# RFC 0006: DocumentText Position Tables and Bookmark Names

Status: draft for publication preparation; joint review pending

Japanese translation: [0006-document-text-position-tables.ja.md](0006-document-text-position-tables.ja.md)

## Stream Framing

Observed `/DocumentTextPositionTables` begins with `SsmgV.01` and contains
`TCntV.01` and `MarkV.01` sections. Their occurrence does not establish a
single coordinate system shared by every section or version.

## TCntV.01 Observations

One observed family has entries at stream offset `0x24`, each 29 bytes long.
The first two BE-u32 fields are ordered, range-like numeric values. Tail
fields and shifted families also occur. Byte-range and UTF-16-unit
interpretations each produce partial matches with `/DocumentText`; matches
with style IDs or layout words are ambiguous. No universal paragraph,
style-reference, line index, or page coordinate interpretation is established.

## MarkV.01 Observations

In the initial five inspected streams the marker starts at byte 30. The six
following bytes have prefix `00000000` and suffix `0603`, `0610`, or `061c`.
Subsequent six-byte items fit BE-u16 ID plus BE-u32 value, with `ffff`
termination. The leading six bytes were initially treated as a header;
controlled bookmark files also permit a first ID/value-item interpretation.
That ambiguity must be retained rather than silently skipping a possible item.

The candidate values do not behave as extracted plain-text character offsets
or direct LineMark word indexes. A `+29` UTF-16-unit probe matches some saved
TOC titles, but fails the controlled first-line bookmark case. Deltas 9/30
and family-dependent values compete in other inputs. No fixed bias is decoded.

## Separate MarkTag Directory

The controlled `/MarkTag` form is:

```text
8 bytes   MarkV.01
u16 BE    entry count
repeat:
  u32 BE  directory value
  u16 BE  name length in UTF-16 units
  ...     UTF-16BE name
```

Recorded directory values 0, `0x00010000`, and `0x00020000` accompany three
names. They are neutral directory values, not decoded text addresses.
Complete framing, valid Unicode, and exact consumption distinguish this
profile from position-table sections carrying the same marker string.

## Validation and Remaining Questions

Move one named bookmark across known plain, control, and ruby positions while
holding other content constant. Record both streams and source ranges for each
save. Repeat across authoring versions. Preserve raw values and the leading
item ambiguity until an independently reproduced normalization explains all
contrasts. Readable names do not establish navigation or editing coordinates.

## Implementation Record and History

[Commands, APIs, output behavior, and detailed historical research](../../rjtd/docs/research/0006-document-text-position-tables.md).
