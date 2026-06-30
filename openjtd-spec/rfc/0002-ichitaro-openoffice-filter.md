# RFC 0002: Ichitaro OpenOffice Filter Reference

Status: draft

Observed: 2026-06-18

Japanese translation: [0002-ichitaro-openoffice-filter.ja.md](0002-ichitaro-openoffice-filter.ja.md)

## Summary

The historical OpenOffice Ichitaro Document Filter is a high-priority reference artifact for rjtd.

It confirms that an OpenOffice Writer import path existed for Ichitaro 8/9/10/11 `.jtd` and `.jtt` files, and it exposes stream names that match the local JTD sample inventory from RFC 0001.

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

There is no `.jar` file in this package. The import implementation is a native Windows x86 UNO component.

## OpenOffice Registration

`description.xml` identifies the extension as:

```text
identifier: com.sun.star.ichitaro-windows_x86
display-name: Ichitaro import filter
platform: windows_x86
publisher: Sun Microsystems
version: 1.0
minimum OpenOffice.org: 3.0
```

`META-INF/manifest.xml` registers `jsreadermi.dll` as:

```text
application/vnd.sun.star.uno-component;type=native
```

`filters.xcu` registers both document and template import filters with:

```text
FilterService: com.sun.comp.jsimport.IchitaroImportFilter
DocumentService: com.sun.star.text.TextDocument
Flags: IMPORT ALIEN 3RDPARTYFILTER
```

`types.xcu` registers:

```text
jtd -> writer_JustSystem_Ichitaro_10
jtt -> writer_JustSystem_Ichitaro_10_template
```

## DLL Inventory

`jsreadermi.dll` is:

```text
PE32 executable (DLL), Intel 80386, Windows GUI
```

The PE export table exposes the standard native UNO component entry points:

```text
component_getFactory
component_getImplementationEnvironment
component_writeInfo
```

The export table reports the DLL name as:

```text
newjsreader.dll
```

The binary also contains a PDB path string:

```text
C:\odk641\WINexample.out\\bin\\newjsreader.pdb
```

## Stream Name Evidence

Plain string inspection found stream names that match local `.jtd` sample streams:

```text
DocumentText
DocumentViewStyles
Header
PageLayoutStyle
PageLayoutStyleHeader
TextLayoutStyle
JSRV_SegmentInformation
```

It also contains additional candidate stream or object names:

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

Plain strings indicate the filter writes OpenOffice XML/SAX output rather than exposing a simple text API.

Notable strings include:

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

Independent clean-room evidence from JXW.Application COM automation confirms the Ichitaro text export path without depending on the DLL binary.

JustSystem exposes a registered COM ProgID:

```text
JXW.Application
```

The corresponding automation object exposes a `TaroLibrary` member with a `SaveDocument` method:

```text
JWApp.TaroLibrary.SaveDocument(outputPath, "", "", filterNo)
```

Setting `filterNo=10` selects plain-text export. This VBA call pattern was observed independently in automation scripts written for batch Ichitaro-to-Word conversion workflows used in government office environments.

The COM ProgID and filter number are observable from automation scripts without DLL analysis and therefore qualify as clean-room input.

### Text Export Tab Mode

COM text export behavior depends on which save-tab mode is active in Ichitaro when the document is processed:

- `基本` (basic): produces correct full-body text export
- `アウトライン` (outline): may produce a structurally different output reflecting outline-mode document state
- `提出` (submission): may produce a submission-scoped output that excludes certain content regions

For reliable `DocumentText`-representative extraction from documents processed via COM automation, the `基本` tab should be active during export.

## License Boundary

The package includes a Sun software license. The license text restricts decompilation and reverse engineering unless enforcement is prohibited by applicable law.

rjtd must treat this artifact as a no-code compatibility reference:

- Do not copy implementation logic from the DLL.
- Do not decompile the DLL as part of normal rjtd development.
- Use extension metadata, filter registration, file inventory, and independently observable sample behavior as clean-room inputs.
- Consult legal advice before any deeper binary analysis.

## Impact on rjtd

RFC 0001 identified `/DocumentText` as the largest common stream in local samples.

This filter independently confirms that `DocumentText` is a meaningful stream name used by an Ichitaro importer. M2 should therefore start with `DocumentText` stream extraction and byte-level characterization before trying paragraph or style modeling.

Recommended next steps:

- Add a read-only stream dump helper for local research.
- Compare `DocumentText` payloads across the five local samples.
- Search for compression, segmentation, and text encoding signatures.
- Preserve all filter-confirmed stream names as known container names even before their payload formats are decoded.
