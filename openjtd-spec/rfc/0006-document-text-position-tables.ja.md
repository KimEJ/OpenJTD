# RFC 0006: DocumentText Position Tables and Bookmark Names

Status: draft for publication preparation; joint review pending

English source: [0006-document-text-position-tables.md](0006-document-text-position-tables.md)

## stream 境界

観測した `/DocumentTextPositionTables` は `SsmgV.01` で始まり、`TCntV.01` と `MarkV.01`
section を持つ。全 section・version が同じ座標系を使う証拠ではない。

## TCntV.01 の観測

一つの family は stream offset `0x24` から 29-byte entry を持つ。先頭二つの BE-u32 は順序のある
range-like 数値である。tail field や shifted family もある。本文に byte/UTF-16 unit として当てると
どちらも部分一致し、style ID・layout word との一致も曖昧である。一般の段落、style 参照、行 index、
ページ座標を確定しない。

## MarkV.01 の観測

初期 5 stream では marker が byte 30 にあり、直後 6 bytes は prefix `00000000`、suffix
`0603` / `0610` / `061c`。後続は BE-u16 ID と BE-u32 value の 6-byte item、`ffff` 終端に合う。
直後 6 bytes は当初 header としたが、対照 bookmark 文書では最初の ID/value item とも読める。
可能な先頭 item を黙って飛ばさず曖昧さを保つ。

値は抽出 plain text の文字位置や LineMark word index と一致しない。`+29` UTF-16-unit probe は
一部の目次 title に一致するが、対照の先頭行 bookmark では反証される。他の入力には 9/30 や
family 別 delta が競合する。固定 bias は未解読である。

## 別の MarkTag directory

対照 `/MarkTag` は `MarkV.01`、BE-u16 count、反復する BE-u32 directory value・BE-u16 の
UTF-16 unit 長・UTF-16BE name からなる。三つの名前に 0、`0x00010000`、`0x00020000` が対応するが、
text address として解読しない。wire form は以下である。

```text
8 bytes   MarkV.01
u16 BE    entry count
repeat:
  u32 BE  directory value
  u16 BE  name length in UTF-16 units
  ...     UTF-16BE name
```

同じ marker を持つ position-table section とは、完全な framing、
Unicode、全 bytes の消費を確認して区別する。

## 検証と未解決事項

他の内容を固定し、一つの bookmark を既知の本文・control・ruby 位置へ移動して、両 stream と
source range を記録する。version を変えて独立に再現し、全対照を説明する normalization が得られるまで
raw 値と先頭 item の曖昧さを保持する。名前が読めても navigation/editing 座標は確定しない。

## 実装記録と履歴

[実装の command・API・出力・過去の詳細記録](../../rjtd/docs/research/0006-document-text-position-tables.ja.md).
