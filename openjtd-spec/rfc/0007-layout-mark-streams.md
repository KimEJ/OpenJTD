# RFC 0007: Layout Marks and Page Instructions

Status: draft for publication preparation; joint review pending

Japanese translation: [0007-layout-mark-streams.ja.md](0007-layout-mark-streams.ja.md)

## Scope

`/LineMark`, `/PageMark`, `/PaperMark`, `/PageLayoutStyle`, and `/Header`
provide observed layout-related records. Their presence and storage shape do
not establish a universal layout algorithm or a page count. Early archival
observations and controlled modern profiles must remain separately scoped.

## Mark Families

An observed PageMark family has a 12-byte header and 84-byte rows. Header
values include `(N, 16, N-1)`, but N need not equal the stored row count or
visible pages. Other variable-row families exist. Some row values correlate
with source line ranges; mixed payload and sentinel rows are counterexamples
to treating every row as an active body page.

An observed PaperMark family has header `(N, 12, N-1)` followed by BE-u32
index/flags pairs. The same N occurs in PageMark for the inspected files.
Its semantic meaning and all flag combinations are not decoded.

LineMark starts with words including `0x0914`, `0x090b`, or `0x0912` in
different inputs and contains tag-like `0x1000`/`0x1001`/`0x1002` values.
Controlled profiles correlate source-unit intervals with physical lines or
vertical columns. Initial header-word guesses alone do not decode direction
or generalize those interval rules to other families.

## Page Styles and Direction

Controlled class-`0x0020` records apply explicit page style ID 1 and reset ID 0.
The known three-page contrast has a landscape middle page with PageMark flags
`0x00050100`; neighbors use `0x00010000`. These values are corroborated in that
complete profile, not a general page-style inheritance grammar.

A 258-byte sequential view `0x1001` payload repeats LE-u32 stock sizes at
126/130 and 154/158. The related 267-byte form adds a 9-byte orientation
prefix containing BE-u32 dimensions at 1/5. Dimension copies and source
margins must agree. Direction and spacing associations are in RFC 0003.
Source units and page-relative anchors belong to format interpretation;
conversion to pixels and font baselines belongs to an output implementation.

## Plain Header Slots

The observed `/Header` form starts with `SsmgV.01`, a BE-u32 slot count at
byte 8, and `0x100` at byte 12. Slots are 512 bytes from `16 + 512*i`:
`TextV.01` at +4, BE-u32 text-unit count at +12, text at +16, then a zero byte
and repeated count. An empty `TCntV.01` part starts at +260. The trailing table
has a zero prefix/suffix and one six-BE-u32 row per slot:
`[slotID, 0, 17+2*units, 1, 1, 2*i]`.

Controlled slot IDs 0/1/2/5/6 correspond to page-number and primary/secondary
header/footer candidates. Odd/even and cover settings are corroborated only
for their known view profiles. A trailer after the sequential view extent
uses code/u32-length sections `0x1001`/`0x1002`, not sequential u16-length
style records. In the observed 2026-byte settings section, BE-u32 at payload
byte 8 changes from 0 to 2 with page-number printing. Slot presence alone
does not enable printing.

## Remaining Questions

Resolve other row families, header counters, source origins, flags, section
inheritance, styled/multiline slots, and arbitrary numbering. Record input
version and settings with each claim; saved source positions and backend font
measurements are separate evidence. A successful render does not decode
unknown fields, footnote layout, or general pagination.

## Implementation Record and History

[Commands, APIs, output behavior, and detailed historical research](../../rjtd/docs/research/0007-layout-mark-streams.md).
