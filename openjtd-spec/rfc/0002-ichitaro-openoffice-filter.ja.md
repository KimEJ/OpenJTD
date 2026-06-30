# RFC 0002: Ichitaro OpenOffice Filter Reference

Status: draft

Observed: 2026-06-18

## Summary

historical OpenOffice Ichitaro Document Filter は、rjtd にとって high-priority reference artifact である。

これは Ichitaro 8/9/10/11 `.jtd` と `.jtt` files に対する OpenOffice Writer import path が存在したことを確認し、RFC 0001 の local JTD sample inventory と一致する stream names を露出している。

## Source

Extension page:

```text
https://extensions.openoffice.org/en/project/ichitaro-document-filter.html
```

Download target:

```text
https://sourceforge.net/projects/aoo-extensions/files/1936/0/ichitaro.oxt/download
```

Local copy:

```text
third-party/ichitaro-filter/ichitaro.oxt
third-party/ichitaro-filter/extracted/
```

Hashes:

```text
ddf7b708261b989c95b7552ca181fee160b6ea84349f4845ecf788535cf95ca8  ichitaro.oxt
3add7be73d158ca9b7f81055a83e3413e1dbf792aa0aa23f51d019179e5334bb  jsreadermi.dll
```

## Package Tree

```text
ichitaro.oxt
├── description.xml
├── filters.xcu
├── Ichitaro_Filter_Extension.txt
├── Ichitaro_Filter_Extension_License.txt
├── jsreadermi.dll
├── META-INF/
│   └── manifest.xml
└── types.xcu
```

この package には `.jar` file はない。import implementation は native Windows x86 UNO component である。

## OpenOffice Registration

`description.xml` は extension を次のように識別する。

```text
identifier: com.sun.star.ichitaro-windows_x86
display-name: Ichitaro import filter
platform: windows_x86
publisher: Sun Microsystems
version: 1.0
minimum OpenOffice.org: 3.0
```

`META-INF/manifest.xml` は `jsreadermi.dll` を次のように登録する。

```text
application/vnd.sun.star.uno-component;type=native
```

`filters.xcu` は document と template import filters の両方を次の値で登録する。

```text
FilterService: com.sun.comp.jsimport.IchitaroImportFilter
DocumentService: com.sun.star.text.TextDocument
Flags: IMPORT ALIEN 3RDPARTYFILTER
```

`types.xcu` は次を登録する。

```text
jtd -> writer_JustSystem_Ichitaro_10
jtt -> writer_JustSystem_Ichitaro_10_template
```

## DLL Inventory

`jsreadermi.dll` は次の形式である。

```text
PE32 executable (DLL), Intel 80386, Windows GUI
```

PE export table は standard native UNO component entry points を公開している。

```text
component_getFactory
component_getImplementationEnvironment
component_writeInfo
```

export table は DLL name を次のように報告する。

```text
newjsreader.dll
```

binary には PDB path string も含まれる。

```text
C:\odk641\WINexample.out\\bin\\newjsreader.pdb
```

## Stream Name Evidence

plain string inspection では、local `.jtd` sample streams と一致する stream names が見つかった。

```text
DocumentText
DocumentViewStyles
Header
PageLayoutStyle
PageLayoutStyleHeader
TextLayoutStyle
JSRV_SegmentInformation
```

追加の candidate stream または object names も含まれる。

```text
LayoutBox
LayoutBoxText
Figure
EmbedItems
EmbeddingInfo
Embedding
Contents
EmbeddedPress
FigureData
main_data
FDMIndex
FDMVector
SsmgTextTcntQLST
```

## Conversion Path Evidence

plain strings は、filter が simple text API を公開するのではなく OpenOffice XML/SAX output を書くことを示す。

Notable strings:

```text
com.sun.star.comp.Writer.XMLImporter
com.sun.star.xml.sax.XDocumentHandler
office:document
office:automatic-styles
office:master-styles
office:body
text:section
text:p
text:span
text:s
text:c
text:ruby
table:table
draw:text-box
style:style
style:font-decl
```

## COM Automation Evidence

`JXW.Application` COM automation による独立したクリーンルーム証拠が、DLL バイナリに依存せず Ichitaro text export パスを確認する。

JustSystem は登録済み COM ProgID を公開している。

```text
JXW.Application
```

対応する automation object は `TaroLibrary` member と `SaveDocument` method を公開する。

```text
JWApp.TaroLibrary.SaveDocument(outputPath, "", "", filterNo)
```

`filterNo=10` を設定すると plain-text export が選択される。この VBA call パターンは、官公庁環境で使用される一太郎→Word 一括変換ワークフロー向け automation scripts において独立に観察された。

COM ProgID と filter number は DLL 解析なしに automation scripts から観察可能であり、クリーンルーム入力として適格である。

### Text Export Tab Mode

COM text export の動作は、ドキュメント処理時に Ichitaro でアクティブになっている保存タブモードに依存する。

- `基本`：正しい全文テキストエクスポートを生成する
- `アウトライン`：アウトラインモードのドキュメント状態を反映した構造的に異なる出力を生成する場合がある
- `提出`：特定のコンテンツ領域を除外する提出スコープの出力を生成する場合がある

COM automation 経由で処理されたドキュメントから信頼性の高い `DocumentText` 相当の抽出を行うには、エクスポート時に `基本` タブがアクティブである必要がある。

## License Boundary

package には Sun software license が含まれる。license text は、applicable law により enforcement が禁止されない限り decompilation と reverse engineering を制限する。

rjtd はこの artifact を no-code compatibility reference として扱わなければならない。

- DLL から implementation logic を copy しない。
- 通常の rjtd development の一部として DLL を decompile しない。
- extension metadata、filter registration、file inventory、independently observable sample behavior を clean-room inputs として使う。
- より深い binary analysis の前には legal advice を求める。

## Impact on rjtd

RFC 0001 は `/DocumentText` を local samples で最大の common stream として特定した。

この filter は、`DocumentText` が Ichitaro importer で使われる meaningful stream name であることを独立に確認する。したがって M2 は paragraph や style modeling を試す前に、`DocumentText` stream extraction と byte-level characterization から始めるべきである。

Recommended next steps:

- local research 用の read-only stream dump helper を追加する。
- five local samples 間で `DocumentText` payloads を比較する。
- compression、segmentation、text encoding signatures を探す。
- payload formats が decode される前でも、filter-confirmed stream names を known container names として保存する。
