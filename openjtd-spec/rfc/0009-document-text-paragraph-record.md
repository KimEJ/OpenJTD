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

### Class 0x0030 — short line header (12 words)

Appears inside table-heavy samples. Fixed 12-word structure:

```text
001c 0030 000c  0000 [b0] [b1] 00ff 0000  000c 0000 0030 001f
```

Fields `b0` and `b1` vary. Representative examples:

```text
001c 0030 000c 0000 0000 0002 00ff 0000 000c 0000 0030 001f
001c 0030 000c 0000 0000 00c4 00ff 0000 000c 0000 0030 001f
001c 0030 000c 0000 0000 00c8 00ff 0000 000c 0000 0030 001f
```

This is the format referenced in RFC 0003 §COM Text Export Observation as the
`shanai_lan` `0x001c/0x0030 line header` context.

### Class 0x0000 — compact line marker (12 or 21 words)

```text
001c 0000 000c  0000 [a] [b] [c] 0000  000c 0000 0000 001f
001c 0000 0015  0000 [a] 0000 [c] 0010 0000 0002 0000 0000
                0000 0002 0000 0000 0000  0015 0000 0000 001f
```

Semantic meaning is not decoded.

### Class 0x0020 — compact marker (12 words)

```text
001c 0020 000c  0000 0010 0002 0000 0001  000c 0000 0020 001f
```

Observed sparsely. Semantic meaning is not decoded.

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
- Class `0x0030` fields `b0`/`b1` are not decoded.
- Classes `0x0000` and `0x0020` are observed but not interpreted.
- The partial LineMark overlap (14/25 matches) is consistent with the
  logical/physical line hypothesis but not proven.
- No multi-column sample was used to test whether table-cell `0x001c` records
  differ structurally from paragraph `0x001c` records within the same family.
- The `0x000e` row delimiter is confirmed as a single 1-word control code with no
  additional payload. How the table-cell count, column widths, and cell geometry
  are encoded elsewhere is not yet decoded.
- Class `0x0010` records of varying length appear to share a common sub-header
  signature `0x0026 0x0005` at words `w4/w5` (seen in `論文様式.jtd` len=20 and
  `01要綱/02案文/04参照` len=17 samples). The len=17 variant shows variant fields
  `w6/w7/w8/w9/w10`; in `04参照条文（整備政令）` the repeated value `0x01cc = 460`
  could be a 1/10 mm indent unit (46 mm). In `論文様式.jtd` len=20, `w10=0x0141=321`
  appears on indented continuation lines, which is consistent with a ~32 mm first-line
  or hanging indent. These are layout-unit candidates only; the unit scale and field
  role are not decoded.

## Samples Used

| Sample | Records | Families observed |
| --- | ---: | --- |
| `論文様式.jtd` | 19 | `0x0010 len=20` only |
| `03新旧（整備令）.jtd` | 1039 | `0x0010` (all len), `0x0030 len=12`, `0x0000 len=12/21`, `0x0020 len=12` |
| `02案文・理由（整備令）.jtd` | 33 | `0x0010`, `0x0030 len=12` |
| `04参照条文（整備政令）.jtd` | 504 | `0x0010`, `0x0030 len=12`, `0x0000 len=12` |
