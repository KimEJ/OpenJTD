# rjtd 研究記録 0002: Historical filter research

Status: 実装記録。共同形式 RFC ではない

English source: [0002-ichitaro-openoffice-filter.md](0002-ichitaro-openoffice-filter.md)

## 実装の範囲

package 調査、local artifact、automation、研究優先順位は実装側の記録である。追加解析前に現行研究規定を参照する。この記録は規定を変更しない。

[形式向け記録](../../../openjtd-spec/rfc/0002-ichitaro-openoffice-filter.ja.md) は原本観測と未確定解釈を記録する。
Rust type、candidate 条件、JSON、CLI、paint 近似、test、資源方針は実装動作であり、
native semantics の証明ではない。

## 再現の入口

repository root で `cargo build --manifest-path rjtd/Cargo.toml -p rjtd-cli` を実行する。
解析・開示を許可された入力を使い、command の必須引数は `rjtd/target/debug/rjtd --help` を確認する。

```sh
rjtd/target/debug/rjtd streams path/to/document.jtd
rjtd/target/debug/rjtd info path/to/document.jtd
```

page number、stream、format などを要する command がある。結果には実際の全 command、
入力 ID/hash、作成 version、実装 commit、実行/skip、制限を記録する。private corpus の
検証を列挙しても public 再現性の証明にはならない。

## 過去の根拠

[分離前の完全な記録](https://github.com/OpenJTD/rjtd/blob/cce9cb20611c8c809a7376c8df6130aed14a0437/openjtd-spec/rfc/0002-ichitaro-openoffice-filter.ja.md) に raw example、command、local sweep、棄却仮説、実装履歴を
固定 commit で保持する。原本観測と実装動作が混在し、古い能力記述もある歴史資料であり、
現行形式の契約や実行可能な public fixture 一式ではない。共有 RFC 0001/0003 の import commit
と status は、この local 編集では変更しない。

## 分離作業

[解釈と描画の境界](../../../docs/ARCHITECTURE.ja.md) に沿い、混在関数を分ける。
本文、source span、unknown data、candidate、資源上限、出力動作を焦点を絞った回帰で保持する。
今回の文書変更で Rust module は移動していない。

## 保存した分離前の記録

以下は元の混合研究記録であり、能力・規定・仮説の記述には当時の状態が含まれる。上記の現在の範囲と区別する。

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
