# openjtd-spec

This index contains format-facing drafts about JTD bytes, records,
relationships, observations, hypotheses, and validation scope. CLI commands,
Rust models, JSON shapes, rendering approximations, and implementation test
sweeps are separated into [rjtd research records](../rjtd/docs/research/README.md).
The [interpretation/rendering design](../docs/ARCHITECTURE.md) defines the code
boundary and migration gates.

## Shared Research and Local Drafts

Joint specification review takes place in [OpenJTD/spec](https://github.com/OpenJTD/spec).
RFC 0001 and RFC 0003, with their Japanese translations, were imported there
as unchanged historical drafts. The
[import ledger](https://github.com/OpenJTD/spec/blob/main/IMPORTS.md) retains
their source commits and hashes. This local separation does not change those
shared copies or their agreement status.

These local records are drafts prepared for publication review. Separating
format claims from implementation behavior does not establish acceptance.
Before shared acceptance, attach permitted input IDs/hashes, rights, authoring
versions, reproduction steps, and independent results to each claim. Local
native comparisons, synthetic regressions, parser success, and visual agreement
are distinct evidence. Preserve limitations and counterexamples; no complete
format or historical-version coverage is claimed.

## RFC Index

RFC 0004 described implementation model/export contracts and is withdrawn from
the format-facing set; its number stays reserved. Other numbers are retained.
Detailed prior observations, byte examples, counterexamples, and implementation
history remain available in the corresponding research records.

| RFC | English | 日本語 | State |
| --- | --- | --- | --- |
| 0001 | [JTD Container Inventory](rfc/0001-container.md) | [JTD Container Inventory](rfc/0001-container.ja.md) | Publication-preparation draft |
| 0002 | [Historical Ichitaro Filter Metadata](rfc/0002-ichitaro-openoffice-filter.md) | [Historical Ichitaro Filter Metadata](rfc/0002-ichitaro-openoffice-filter.ja.md) | Publication-preparation draft |
| 0003 | [DocumentText Content and Associated Records](rfc/0003-document-text.md) | [DocumentText Content and Associated Records](rfc/0003-document-text.ja.md) | Publication-preparation draft |
| 0004 | [Implementation Model Export Record](rfc/0004-document-model-export.md) | [Implementation Model Export Record](rfc/0004-document-model-export.ja.md) | Implementation record; number reserved |
| 0005 | [JTTC JustCompressedDocument Container](rfc/0005-jttc-just-compressed-document.md) | [JTTC JustCompressedDocument Container](rfc/0005-jttc-just-compressed-document.ja.md) | Publication-preparation draft |
| 0006 | [DocumentText Position Tables and Bookmark Names](rfc/0006-document-text-position-tables.md) | [DocumentText Position Tables and Bookmark Names](rfc/0006-document-text-position-tables.ja.md) | Publication-preparation draft |
| 0007 | [Layout Marks and Page Instructions](rfc/0007-layout-mark-streams.md) | [Layout Marks and Page Instructions](rfc/0007-layout-mark-streams.ja.md) | Publication-preparation draft |
| 0008 | [Object Streams, Anchors, and Paint Candidates](rfc/0008-object-stream-candidates.md) | [Object Streams, Anchors, and Paint Candidates](rfc/0008-object-stream-candidates.ja.md) | Publication-preparation draft |
| 0009 | [DocumentText Record Framing and Ruled-Line Flow](rfc/0009-document-text-paragraph-record.md) | [DocumentText Record Framing and Ruled-Line Flow](rfc/0009-document-text-paragraph-record.ja.md) | Publication-preparation draft |

## License Boundary

Unless otherwise stated, authored writing is covered by the root
[Apache License 2.0](../LICENSE). Referenced software, input documents,
quotations, and product names retain their own rights and terms; this directory
does not relicense them. Implementation priorities belong in the
[roadmap](../docs/ROADMAP.md).
