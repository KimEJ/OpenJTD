# RFC 0002: Historical Ichitaro Filter Metadata

Status: draft for publication preparation; joint review pending

Japanese translation: [0002-ichitaro-openoffice-filter.ja.md](0002-ichitaro-openoffice-filter.ja.md)

## Scope and Evidence

This is an inventory of a historical interoperability artifact. Its metadata
and strings are corroborating material, not an authoritative JTD decoder or
a source of implementation algorithms. It is not a policy authorizing binary
analysis. Research permissions and implementation priorities belong in
separate policy and implementation documents.

## Artifact Identity

The recorded package is the OpenOffice Ichitaro import filter version 1.0,
published by Sun Microsystems for Windows x86, with minimum OpenOffice.org 3.0.

- [Extension page](https://extensions.openoffice.org/en/project/ichitaro-document-filter.html)
- [Recorded download source](https://sourceforge.net/projects/aoo-extensions/files/1936/0/ichitaro.oxt/download)

Recorded SHA-256 values:

```text
ddf7b708261b989c95b7552ca181fee160b6ea84349f4845ecf788535cf95ca8  ichitaro.oxt
3add7be73d158ca9b7f81055a83e3413e1dbf792aa0aa23f51d019179e5334bb  jsreadermi.dll
```

The package contains `description.xml`, `filters.xcu`, `types.xcu`, a native
`jsreadermi.dll`, a manifest, and license/readme files. No JAR was recorded.
The DLL is PE32/x86 and exports `component_getFactory`,
`component_getImplementationEnvironment`, and `component_writeInfo`.

## Registration and Strings

Registration identifies `com.sun.comp.jsimport.IchitaroImportFilter`,
`com.sun.star.text.TextDocument`, and document/template types for JTD/JTT.
Strings include `DocumentText`, `DocumentViewStyles`, `Header`,
`PageLayoutStyle`, `TextLayoutStyle`, `LayoutBoxText`, `EmbeddingInfo`,
`EmbeddedPress`, `FDMIndex`, and `FDMVector`.

XML/SAX names include `com.sun.star.comp.Writer.XMLImporter`,
`com.sun.star.xml.sax.XDocumentHandler`, `text:p`, `text:ruby`,
`table:table`, and `draw:text-box`. These show names present in the artifact;
they do not prove record layouts, universal format semantics, or table ownership.

## Independent Observations and Limits

Historical automation observations record `JXW.Application`,
`TaroLibrary.SaveDocument`, and plain-text filter number 10. Their exact
application version, settings, provenance, and reproduction must accompany
any stronger claim. Output can depend on application mode.

The package includes restrictive license terms. Artifact possession, this
inventory, and a string match do not grant analysis or redistribution rights.
No recovered algorithm is part of this RFC. Each proposed format relationship
needs independent observation of authorized JTD inputs and counterexamples.

## Implementation Record and History

[Commands, APIs, output behavior, and detailed historical research](../../rjtd/docs/research/0002-ichitaro-openoffice-filter.md).
