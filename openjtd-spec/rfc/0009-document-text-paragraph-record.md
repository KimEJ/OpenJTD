# RFC 0009: DocumentText Record Framing and Ruled-Line Flow

Status: draft for publication preparation; joint review pending

Japanese translation: [0009-document-text-paragraph-record.ja.md](0009-document-text-paragraph-record.ja.md)

## Scope and Counterexamples

The inspected non-inline `0x001c` records have echoed class/length framing.
This does not mean every `0x001c` occurrence is a paragraph record: inline
openers and literal control sequences are counterexamples. Validate complete
records within their owning content extent before interpreting their fields.

## Observed Wire Layout

Words are u16 in the observed big-endian text/control stream:

```text
w0              0x001c
w1              class
w2              total length in words, including opener/footer
w3 .. w[len-5]  class-specific payload
w[len-4]        length echo
w[len-3]        0x0000
w[len-2]        class echo
w[len-1]        0x001f
```

The footer was checked in an early 948-record two-document inventory. That
historical count is not a new public reproduction. An inline form starts
`001c 0001 0007 ... 001d` and has a distinct cache/terminator structure.
Observed non-inline classes include `0x0010`, `0x0020`, `0x0030`, and zero.
Class labels such as paragraph, settings, and cell remain profile-dependent.

## Ruled-Line Flow

Class `0x0010` with `w4=0x008f` occurs in controlled ruled-line parent
profiles. Fully framed `0x0030` records precede cell text and carry
boundary/coordinate-like fields. `0x000e` is a one-word separator in the
observed table contexts; `0x000a` also occurs within cell/paragraph content.
Neither a separator count nor a fixed rectangular hierarchy defines ownership.

A preceding body paragraph can share a `0x000e` interval with the first row
while ending before its parent header. Wrapped text, blank cells, erased
boundaries/merges, trailing empty rows, single rows/columns, and tables spanning
pages require physical source ranges and boundary declarations to be preserved.
A high-level table reconstruction must not suppress intervening or surrounding
text. An unframed coincidence does not become a table solely by matching text.

Source margins, row/column spans, declared boundaries, line/page marks, and
source character properties provide separate constraints. The historical
placement formulas and border/font defaults are implementation candidates;
reference-PDF coordinate agreement does not establish universal source units.

## Trailing Style Events

After the content ending at `32 + 2 * content_unit_count`, an observed
byte-oriented style stream uses a source-unit cursor starting at unit 16:

```text
00 <u32 BE length>                         run of length source units
fe (<property ID> <value length> <bytes>)* ff 00
                                          property change, one source unit
ff                                        terminal marker
```

Changes persist through subsequent runs until replaced. Observed widths are:

| Property IDs | Value bytes |
| --- | ---: |
| 4–7, 9–12 | 1 |
| 1–3, 8, 13, 14, 18, 19 | 2 |
| 15–17, 20 | 4 |

Unknown IDs, width mismatches, and trailing bytes remain uninterpreted.
Property 15 matches `0x00BBGGRR` color on controlled text ranges, including
black/red/blue and automatic/default `0xffffffff`. Its occurrence on non-text
table-state ranges is a counterexample to a universal color interpretation.
Character-property associations and their bounds are in RFC 0003.

## Validation Still Needed

Attach permitted input IDs/hashes and native authoring versions to each
profile. Reproduce independent changes in wrapping, margins, column count,
line spacing, merges, border style, and page flow. Separate source observation,
interpretive hypothesis, output approximation, and parser regression. Preserve
counterexamples; no general row ownership, style inheritance, coordinate
normalization, or edit/save grammar is established by this draft.

## Implementation Record and History

[Commands, APIs, output behavior, and detailed historical research](../../rjtd/docs/research/0009-document-text-paragraph-record.md).
