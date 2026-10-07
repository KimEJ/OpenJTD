# RFC 0002: Historical Ichitaro Filter Metadata

Status: draft for publication preparation; joint review pending

English source: [0002-ichitaro-openoffice-filter.md](0002-ichitaro-openoffice-filter.md)

## 範囲と根拠

歴史的な相互運用 artifact の inventory である。metadata と文字列は補助資料であり、
JTD の権威的 decoder や実装 algorithm の供給源ではない。この RFC は binary 解析を許可する
規定ではない。研究上の許可と実装優先順位は別の文書で管理する。

## artifact の識別

記録したものは Sun Microsystems の OpenOffice 一太郎 import filter version 1.0、
Windows x86 向け、最低 OpenOffice.org 3.0。取得 URL と SHA-256 は英語版と同じである。
`description.xml`、`filters.xcu`、`types.xcu`、native `jsreadermi.dll`、manifest、
license/readme を含み、JAR は記録されていない。DLL は PE32/x86 で、UNO entry point
`component_getFactory`、`component_getImplementationEnvironment`、`component_writeInfo`
を公開する。

取得元: [extension page](https://extensions.openoffice.org/en/project/ichitaro-document-filter.html)、[recorded download](https://sourceforge.net/projects/aoo-extensions/files/1936/0/ichitaro.oxt/download)。

記録した SHA-256:

```text
ddf7b708261b989c95b7552ca181fee160b6ea84349f4845ecf788535cf95ca8  ichitaro.oxt
3add7be73d158ca9b7f81055a83e3413e1dbf792aa0aa23f51d019179e5334bb  jsreadermi.dll
```

## 登録と文字列

登録は `com.sun.comp.jsimport.IchitaroImportFilter`、
`com.sun.star.text.TextDocument`、JTD/JTT の文書・template type を示す。
`DocumentText`、`DocumentViewStyles`、`Header`、`PageLayoutStyle`、`TextLayoutStyle`、
`LayoutBoxText`、`EmbeddingInfo`、`EmbeddedPress`、`FDMIndex`、`FDMVector` が文字列にある。
XML/SAX 関連には `com.sun.star.comp.Writer.XMLImporter`、
`com.sun.star.xml.sax.XDocumentHandler`、`text:p`、`text:ruby`、`table:table`、
`draw:text-box` がある。存在する名前の観測であり、record layout、普遍的意味、表の所有構造を
証明しない。

## 独立観測と限界

過去の automation 観測には `JXW.Application`、`TaroLibrary.SaveDocument`、plain-text
filter 番号 10 がある。強い主張には application version、設定、出所、再現が必要であり、
出力は application mode に依存し得る。

package には制限的な license 条件がある。所持、inventory、文字列一致は解析・再配布の許可を
与えない。復元 algorithm はこの RFC に含めない。形式上の関係は、解析を許可された JTD の
独立観測と反例により検証する。

## 実装記録と履歴

[実装の command・API・出力・過去の詳細記録](../../rjtd/docs/research/0002-ichitaro-openoffice-filter.ja.md).
