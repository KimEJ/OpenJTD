# RFC 0009: DocumentText Paragraph Record Structure

Status: draft

Observed: 2026-06-24

Japanese translation: [0009-document-text-paragraph-record.ja.md](0009-document-text-paragraph-record.ja.md)

## Summary

Every `0x001c` control boundary in `/DocumentText` is followed immediately by a
self-describing variable-length header record. The record ends with `0x001f`,
which is the existing text-run start marker. This means every `0x001c` that the
current token parser emits as a plain `ControlBoundary` is actually the opener
of a structured paragraph/layout record; the `0x001f` that follows is the
record's own terminator, not an independent text-run marker.

## Record Layout

Every non-inline `0x001c` record has the form:

```text
word offset  field
w0   0x001c  record opener (same code as the ControlBoundary token)
w1   class   record class: 0x0000 / 0x0010 / 0x0020 / 0x0030
w2   len     total record length in u16 words, including w0 and the footer
w3…          payload words (class-specific, count = len - 7)
w[len-4]     len echo (repeats w2)
w[len-3]     0x0000
w[len-2]     class echo (repeats w1)
w[len-1]     0x001f  record terminator / text-run start
```

The footer `(len_echo, 0x0000, class_echo, 0x001f)` was verified against
948 records across the `論文様式.jtd` and `03新旧（整備令）.jtd` local
samples. One apparent exception at unit 22733 is a run of sequential control
word values (`0x001d`…`0x002a`) that happens to start with `0x001c` but does
not follow the record layout; it is not a paragraph record.

The inline opener `0x001c 0x0001 0x0007 … 0x001d` used for ruby/inline segments
is a distinct form already handled by the current token parser. It uses the same
`0x001c` opener but `class=0x0001`, and its terminator is `0x001d` rather than
`0x001f`.

## Observed Record Classes

### Class 0x0010 — paragraph / line header

The most common class across all tested samples. Found in `論文様式.jtd` (19
records, all len=20), and in multiple-column table samples with len ranging
from 10 to 59.

Short representative record (len=13, `03新旧（整備令）.jtd`):

```text
001c 0010 000d  0000 002e 0001 0001 ffff 0000  000d 0000 0010 001f
```

Long representative record (len=27, `03新旧（整備令）.jtd`):

```text
001c 0010 001b  0000 008f 000f 010c 0000 0003 0023 0000 0000 007e 0023
0000 0000 007e 0023 0000 0000 0006 ffff  001b 0000 0010 001f
```

In `論文様式.jtd` all 19 records are len=20 with a stable payload:

```text
0000 0000 0001 0001 0026 0005 [w9] [w10] 0000 0000 0000 ffff 0000
```

`w9` is always `0x0001` across the sample. `w10` varies:

| w10    | count | following text (first 8 chars) |
| ------ | ----: | ------------------------------ |
| 0x0000 |    14 | normal body paragraphs         |
| 0x0141 |     4 | indented continuation lines    |
| 0x01f4 |     1 | trailing blank line            |

The semantic meaning of `w10` is not decoded. The observation suggests it may
encode an indent level or paragraph continuation flag.

**Class `0x0010` sub-types (decoded:false).** The `w4` field discriminates at least
four sub-types in `03新旧（整備令）.jtd`:

| w4 value | len | count | role (decoded:false) |
| -------- | --- | ----: | -------------------- |
| `0x002e` (46)    | 13  |    18 | single-column paragraph record |
| `0x008f` (143)   | 27–61 | 129 | table-row column-spec header |
| `0x002a` (42)    | 37–47 |   2 | composite transition record (Y-coords + inner 0x008f sub-block) |
| `0xffff` (65535) | 10  |     3 | null / end-of-section marker |

For `w4=0x008f` records: `len = w5 + 12` (verified 129 records). `w6=268`
equals the maximum cell `b1` coordinate in following `0x0030` rows — consistent
with `w6` encoding the total table width. The variable-length payload (words
`w9..w[len-5]`) consists of 4-word sub-entries `[tag, v1, v2, v3]` terminated by
`[0xffff, 0x0000]`. The most common tags are `0x23` (546 occurrences; v1=v2=0;
v3 varies — correlates with cell span), `0x2b` (71; v1=0, v2=0x08; v3 in 0x1d–0x76),
and `0x1b` (11; v1=0, v2=0x08, v3=0x1f). The pattern `n_sub_entries = n_cells − 1`
holds for the dominant cases (68 records with n_sub=3/n_cells=4 and 27 records with
n_sub=9/n_cells=10). The `w8` field value does not directly equal the count of
`0x0023`-tagged entries; its role is not decoded. Sub-entry tag semantics and the
relationship between v3 and cell coordinates are not decoded.

For `w4=0x002a` records: `w7` holds the count of large-valued (`> 1000`) word
pairs in `w8..w(8+2*w7-1)`; those values are in the thousands and may encode
vertical Y-coordinates of horizontal rules (candidate interpretation: 1/100 mm;
138mm and 140mm are plausible table-separator positions on an A4 page). After the
Y-coordinate block, an inner `0x008f` sub-block encodes the following table's
column layout with the same structure as a standalone `w4=0x008f` record.

For `w4=0xffff` records: the entire payload is `(0xffff, 0x0000)` — just the
`0xffff` sentinel and a zero — with no column entries.

### Class 0x0030 — table cell header (12 words fixed)

Appears inside table-heavy samples, one per table cell per display row. Fixed
12-word structure:

```text
001c 0030 000c  0000 [b0] [b1] 00ff 0000  000c 0000 0030 001f
```

**`b0` and `b1` are cell boundary coordinates (decoded:false for scale/unit).**
Analysis of 703 records in `03新旧（整備令）.jtd`:

- `b0` = left edge of cell in the table coordinate space
- `b1` = right edge of cell; `b1 − b0` = cell width in the same units
- Cells within a row are non-overlapping and ordered left-to-right
- Adjacent cells in the same row are separated by a gap of exactly 4 units

Representative layout for the main two-column comparison table (4 cells per row):

```text
gap  [b0, b1]   width  role
  0  [  0,  2]      2  left border strip
  4  [  6,130]    124  column A (改正案)
  4  [134,258]    124  column B (現行)
  4  [262,268]      6  right border strip
```

Total table span = 268 (`max(b1)` = `0x010c`), which matches `w6` in the
preceding `0x0010` row-header record. The physical unit of the coordinates is not
decoded; 268 does not correspond to a simple mm or 1/10 mm value for the text
area of an A4 page.

This is the format referenced in RFC 0003 §COM Text Export Observation as the
`shanai_lan` `0x001c/0x0030 line header` context.

### Class 0x0000 — inline-segment context marker (12 or 21 words)

Observed in two distinct forms in `03新旧（整備令）.jtd` (92 records total).

**len=12 (14 records).** Always appears immediately after a `0x001c/0x0030` cell
header and immediately before a `0x001c/0x0001` ruby/inline segment. Structure:

```text
001c 0000 000c  0000 [w4] [w5] [w6] 0000  000c 0000 0000 001f
```

`w4=0x0007` matches the `len` field of the following `0x0001` inline record (7
words). `w6=0x020d=525` is constant across all 14 occurrences (style/type code
candidate). `w5` varies: `0x00dc=220` appears in the wide content columns
(width=124, b0=6 or b0=134) before "改正案"/"現行" column headers; `0x0098=152`
and `0x008e=142` appear in narrow columns (width=28, b0=12) before ministry-name
labels; `0x008c=140` appears in the symmetric narrow column (width=28, b0=144).
The same ministry name can appear with different `w5` values depending on
which column it occupies. The physical meaning of `w5` is not decoded — it
appears sensitive to the containing cell's position or a style selector tied to
the cell type rather than directly encoding the label text.

**len=21 (77 records).** The most common `0x0000` form. Structure has a stable
header block and a variable tail:

```text
001c 0000 0015  0000 0056 0000 0406 0010 [w8] [w9] 0000 0000
                0000 [w13] 0000 0000 0000  0015 0000 0000 001f
```

Fields `w4=0x56=86`, `w5=0x0000`, `w6=0x0406=1030`, `w7=0x0010=16` are constant
(style/context codes, not decoded). `w8` is a flag (0 or 1). When `w8=0`, `w9`
takes small values (0, 1, 2, 4, 5). When `w8=1`, `w9` takes large values
(`0x025d=605` or `0x0229=553`); `w13` covaries with `w9` (`0xcd=205` or
`0x99=153`). No physical meaning decoded.

### Class 0x0020 — table-section transition marker (12 words)

Observed 4 times in `03新旧（整備令）.jtd`. Always appears after `0x000e` (table
row delimiter) and immediately before a `0x001c/0x0010` single-column paragraph:

```text
001c 0020 000c  0000 0010 [w5] 0000 0001  000c 0000 0020 001f
```

`w4=0x0010=16` (class code of the following `0x0010` record), `w7=0x0001=1`
constant. `w5=0x0002` or `0x0000`. Appears to mark the transition from a table
section back to normal single-column text. Semantic meaning is not decoded.

## Correlation with LineMark unit-start

The `線文様式.jtd` sample (25 parsed LineMark records) shows exact correspondence
between LineMark `unit-start` values and `0x001c` record positions in
`/DocumentText`:

| LineMark record | LineMark unit-start | 0x001c unit in /DocumentText |
| --------------- | ------------------: | ---------------------------: |
| 0               | 16                  | 16                           |
| 1               | 83                  | — (no 0x001c at 83)          |
| 2               | 129                 | 129                          |
| 3               | 150                 | 150                          |
| 4               | 179                 | 179                          |
| 5               | 248                 | 248                          |
| 6               | 332                 | — (no 0x001c at 332)         |
| 7               | 360                 | 360                          |
| 8               | 416                 | 416                          |
| 9               | 484                 | 484                          |
| 10              | 546                 | 546                          |
| 11              | 618                 | 618                          |
| 12              | 681                 | 681                          |
| 13              | 759                 | 759                          |
| 14              | 846                 | — (falls inside a text run)  |
| 15              | 877                 | 877                          |
| 16              | 957                 | — (falls inside a text run)  |
| 17              | 971                 | 971                          |
| 18              | 1051                | — (falls inside a text run)  |
| 19              | 1076                | 1076                         |
| 20              | 1141                | 1141                         |
| 21              | 1217                | 1217                         |
| 22              | 1238                | 1238                         |
| 23              | 1259                | 1259                         |
| 24              | 1280                | 1280 (0x0000 terminator)     |

Fourteen of the twenty-five LineMark `unit-start` values fall exactly on a
`0x001c` record position. The remaining eleven fall inside text runs or at
the `0x0000` document terminator. This partial overlap is consistent with
LineMark records representing physical display lines while `0x001c` paragraph
records represent logical paragraphs; a single paragraph can wrap across
multiple display lines.

The LineMark `flag` values are not yet correlated with `0x001c` record payload
fields. `flag=0x0002` is the most common LineMark value in this sample (18/25),
`flag=0x0003` appears at record 0 (start of document), and `flag=0x0000`
appears at records 1, 6, 14, 16, 18 (which do not coincide with `0x001c`
positions). This may indicate that `flag=0x0000` marks display-line
continuations within a paragraph rather than paragraph boundaries.

## Impact on Current Token Parser

The current `parse_document_text` function (in `rjtd-core/src/document_text.rs`)
reads the stream as big-endian UTF-16 and treats `0x001c` as a plain
`ControlBoundary`. When `0x001c` appears, the next `0x001f` it encounters
starts a new text run. This works for text extraction because the header words
between `0x001c` and `0x001f` do not decode as valid Unicode text (they are
control-range values). The parser effectively skips the header by stopping on
`0x001c` as a boundary, then resuming on `0x001f`.

No change to the parser is warranted until paragraph-record semantics (indent
levels, style references, column/cell geometry) are proven. The `decoded:false`
principle applies.

## 0x000e Row Delimiter Role

In `03新旧（整備令）.jtd` (a table-heavy new-vs-old comparison document), every
`0x000e` occurrence is immediately preceded and followed by a `0x001c` record.
The `text-control-context` diagnostic confirms that every `0x000e` has
`prev-control=0x001c` (class `0x0030`) and `next-control=0x001c` (class `0x0030`).

The `text-control-ranges` diagnostic shows that consecutive `0x000e` records are
separated by exactly 2 bytes (1 u16 word). This means **`0x000e` itself is a
single-word control code with no additional payload**; it acts as a raw
one-word table-row delimiter. The pattern in a two-column new-vs-old table is:

```text
[prev column text content]
0x001c 0x0030 [12 words = cell A header] 0x001f [cell A text...]
0x000e                                          ← 1-word row delimiter
0x001c 0x0030 [12 words = cell B header] 0x001f [cell B text...]
```

This corroborates the COM text export evidence in RFC 0003 §COM Text Export
Observation (where `0x001c/0x0030` line headers and `0x000e` row delimiters
were observed in the `shanai_lan` table context).

## Known Gaps

- The semantic meaning of class `0x0010` payload words beyond the footer
  pattern is not decoded. The varying `w10` field likely encodes style or indent
  but has not been matched against rendered output.
- Class `0x0030` fields `b0`/`b1` are partially decoded: `b0` is the left edge and
  `b1` the right edge of the cell in the table coordinate space; cells are
  non-overlapping with 4-unit inter-cell gaps. The physical unit of the coordinate
  values is not decoded.
- Classes `0x0000` and `0x0020` are observed with structural patterns documented but
  not semantically decoded: `0x0000 len=12` precedes ruby/inline segments with
  `w4=7` (inline len) and constant `w6=525`; `0x0000 len=21` appears inside table
  cells with a stable constant block plus varying `w8`/`w9`/`w13` fields;
  `0x0020 len=12` marks table-to-paragraph transitions with `w4=0x0010` and
  `w7=1`.
- The partial LineMark overlap (14/25 matches) is consistent with the
  logical/physical line hypothesis but not proven.
- No multi-column sample was used to test whether table-cell `0x001c` records
  differ structurally from paragraph `0x001c` records within the same family.
- The `0x000e` row delimiter is confirmed as a single 1-word control code with no
  additional payload. The `0x0010 w4=0x008f` record encodes per-row column layout
  via its 4-word sub-entries `[tag, v1, v2, v3]`; the count of sub-entries equals
  `n_cells − 1` in the dominant cases. Sub-entry `v3` values correlate with cell
  spans but the exact formula is not decoded. The sub-entry tags `0x23`, `0x2b`,
  `0x1b`, `0x24`–`0x27` and the role of `w8` are not decoded.
- Class `0x0010` records of varying length appear to share a common sub-header
  signature `0x0026 0x0005` at words `w4/w5` (seen in `論文様式.jtd` len=20 and
  `01要綱/02案文/04参照` len=17 samples). Detailed analysis of `04参照条文（整備政令）.jtd`
  len=17 records (142 total, `w4=0x0026 w5=0x0005`) reveals 9 distinct payload
  combinations driven by `w6`/`w7`/`w8`/`w9`/`w10`. When `w6=1` (102 records):
  `w7=0x01ec=492` and `w8=w10=0x01cc=460` are constant, forming what appears to be
  a hanging-indent group (if 1/20 mm: 24.6/23 mm; if 1/10 mm: 49.2/46 mm). When
  `w6=0` (40 records): `w7` takes 0/2/4, `w8` is mostly 0, indicating no hanging
  indent. In `論文様式.jtd` len=20, `w10=0x0141=321` appears on indented continuation
  lines, consistent with a ~32 mm hanging indent. In `03新旧（整備令）.jtd`, the
  `w4=0x002e` variant (18 records, len=13) is fully constant (`w5=w6=1`, `w7=0xffff`,
  `w8=0`), suggesting uniform single-column layout. The unit scale and exact field role
  are not decoded.

## Samples Used

| Sample | Records | Families observed |
| --- | ---: | --- |
| `論文様式.jtd` | 19 | `0x0010 len=20` only |
| `03新旧（整備令）.jtd` | 1039 | `0x0010` (all len), `0x0030 len=12`, `0x0000 len=12/21`, `0x0020 len=12` |
| `02案文・理由（整備令）.jtd` | 33 | `0x0010`, `0x0030 len=12` |
| `04参照条文（整備政令）.jtd` | 504 | `0x0010`, `0x0030 len=12`, `0x0000 len=12` |
