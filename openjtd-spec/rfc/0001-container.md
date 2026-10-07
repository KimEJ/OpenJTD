# RFC 0001: JTD Container Inventory

Status: draft for publication preparation; joint review pending

Japanese translation: [0001-container.ja.md](0001-container.ja.md)

## Scope and Evidence

Observed JTD/JTT documents use Compound File Binary (CFB) storage. This is an
inventory of the inspected files, not a requirement that every historical
variant have the same tree. Initial observations date from 2026-06-18;
complete redistributable reproduction inputs are not attached to this draft.

## Observed Streams

The initial five-file inventory shares these logical streams:

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

Here `\x04` and `\x05` denote control characters in the names, not literal
backslash sequences. Some inspected files also contain `/LineMark`,
`/PageMark`, `/PaperMark`, and `/PageLayoutStyleHeader`; others do not.
The macro storage family includes `/DocumentMacro/Macros/BaseStorage0` and
its `InfoStream`, `MacrosStream`, and `MacrosStreamStyle3` children.
Presence does not establish their payload meaning or permission to execute them.

## Interpretation and Counterexamples

`/DocumentText` is associated with body text in the inspected profiles.
Style-named streams, position tables, and layout marks require separate
interpretation. A large stream or a suggestive name alone does not prove a role.
Some files lack a directly readable named `/DocumentText` and contain embedded
`SsmgV.01` / `TextV.01` fragments. JTTC has the wrapper described in RFC 0005.

FAT inconsistencies occur in some local inputs. A reader's recovery policy is
an implementation choice; successful recovery is not evidence that malformed
sector chains are a normative part of JTD.

## Validation Still Needed

Record the authoring version, exact inventory, hashes, acquisition basis, and
readability of each input. Compare minimal native documents with one feature
changed at a time, and independently check older documents. Stream absence,
unreadable chains, and unknown payloads must remain explicit.

## Implementation Record and History

[Commands, APIs, output behavior, and detailed historical research](../../rjtd/docs/research/0001-container.md).
