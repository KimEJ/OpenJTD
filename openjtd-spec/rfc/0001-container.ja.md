# RFC 0001: JTD Container Inventory

Status: draft for publication preparation; joint review pending

English source: [0001-container.md](0001-container.md)

## 範囲と根拠

観測した JTD/JTT は Compound File Binary（CFB）storage を使う。これは調査した文書の
inventory であり、歴史的な全 variant に同じ tree を要求するものではない。初期観測日は
2026-06-18。再配布可能な再現入力一式は、この draft に添付されていない。

## 観測した stream

初期 5 文書に共通する logical stream は以下である。

```text
/\x04JSRV_SegmentInformation
/\x04JSRV_SummaryInformation
/\x05SummaryInformation
/AutoTextInfo
/DocumentEditStyles
/DocumentPeripheralThree
/DocumentPeripheralTwo
/DocumentText
/DocumentTextPositionTables
/DocumentViewStyles
/Font
/Footnote
/Header
/MarkTag
/PageLayoutStyle
/ReferenceInfo
/RelatedDocuments
/TextLayoutStyle
/ThinkingTemplate
```

`\x04` と `\x05` は
名前中の制御文字を示し、文字列としての backslash ではない。
`/DocumentText`、`/DocumentTextPositionTables`、`/DocumentViewStyles`、
`/DocumentEditStyles`、`/TextLayoutStyle`、`/PageLayoutStyle`、`/Font`、
`/Footnote`、`/Header`、`/MarkTag` と metadata stream が含まれる。

一部には `/LineMark`、`/PageMark`、`/PaperMark`、`/PageLayoutStyleHeader` もあるが、
存在しない文書もある。macro storage には `/DocumentMacro/Macros/BaseStorage0` と
`InfoStream`、`MacrosStream`、`MacrosStreamStyle3` がある。存在だけでは payload の意味や
実行許可を証明しない。

## 解釈と反例

調査 profile では `/DocumentText` が本文と対応する。style、position table、layout mark の
意味は別途検証する。stream の大きさや名前だけで役割を確定しない。named `/DocumentText` を
直接読めず、埋込み `SsmgV.01` / `TextV.01` fragment を持つ文書もある。JTTC wrapper は
RFC 0005 に記録する。

一部の local 入力には FAT 不整合がある。reader の回復方針は実装の選択であり、読めたことは
不正 sector chain が JTD の規範構造である証拠にはならない。

## 必要な検証

作成 version、正確な inventory、hash、取得根拠、readability を入力ごとに記録する。
一条件だけ変えた native 文書と過去の文書を独立に比較し、stream の欠落、読めない chain、
未知 payload を明示する。

## 実装記録と履歴

[実装の command・API・出力・過去の詳細記録](../../rjtd/docs/research/0001-container.ja.md).
