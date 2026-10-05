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
948 records across the `sample-academic.jtd` and `sample-table.jtd` local
samples. One apparent exception at unit 22733 is a run of sequential control
word values (`0x001d`…`0x002a`) that happens to start with `0x001c` but does
not follow the record layout; it is not a paragraph record.

The inline opener `0x001c 0x0001 0x0007 … 0x001d` used for ruby/inline segments
is a distinct form already handled by the current token parser. It uses the same
`0x001c` opener but `class=0x0001`, and its terminator is `0x001d` rather than
`0x001f`.

## Observed Record Classes

### Class 0x0010 — paragraph / line header

The most common class across all tested samples. Found in `sample-academic.jtd` (19
records, all len=20), and in multiple-column table samples with len ranging
from 10 to 59.

Short representative record (len=13, `sample-table.jtd`):

```text
001c 0010 000d  0000 002e 0001 0001 ffff 0000  000d 0000 0010 001f
```

Long representative record (len=27, `sample-table.jtd`):

```text
001c 0010 001b  0000 008f 000f 010c 0000 0003 0023 0000 0000 007e 0023
0000 0000 007e 0023 0000 0000 0006 ffff  001b 0000 0010 001f
```

In `sample-academic.jtd` all 19 records are len=20 with a stable payload:

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
four sub-types in `sample-table.jtd`:

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
The `w2` value and `w8` echo are the record length, not font size. A 2026-09-26
sweep found 12,315 windows matching the supported envelope (`w6=0x00ff`, `w7=0`)
across 83 local files; every window had `w2=w8=12`. The fixed-width reader now
requires both lengths to match. Other envelopes remain unproven.

Some older model diagnostics still call this value `fontSizeUnits` and use it
in reference-calibrated row/stroke projections. Those names and formulas are
legacy hypotheses, not decoded font metrics. Actual text-size evidence comes
from style property 2 and the supported view-style default described in
[RFC 0003](0003-document-text.md#observed-font-size-sources).

Analysis of 703 records in `sample-table.jtd`:

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

Observed in two distinct forms in `sample-table.jtd` (92 records total).

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
takes small values (0, 2, 4, 5) with `w13` mostly 0 or 2 (27×`(w9=4,w13=2)`,
20×`(w9=2,w13=2)`, 11×`(w9=0,w13=2)`, 9×`(w9=2,w13=0)`, 2×`(0,0)`, 1×`(5,2)`,
1×`(1,0)`). When `w8=1` (6 occurrences), `w9` takes large values and `w13`
covaries: the pair `(w9=0x025d=605, w13=0xcd=205)` always accompanies the cell
`[b0=4, b1=132]` and `(w9=0x0229=553, w13=0x99=153)` always accompanies the
symmetric cell `[b0=136, b1=264]`. In both `w8=1` pairs `w9 − w13 = 400 =
(b1 − b0) × 3.125` exactly. Between the two cells, w9 and w13 each decrease by
exactly 52 (605→553 and 205→153), while b0 increases by 132; no linear
relationship has been found. The w8=1 records always appear immediately before a
`0x001c/0x0001` ruby/inline record (the heading text), while w8=0 records appear
inside ordinary text runs. No physical meaning decoded for the coordinate
relationship; the constant `w9−w13 = (b1−b0) × 3.125` ratio is an observed
invariant only.

### Class 0x0020 — table-section transition marker (12 words)

Observed 4 times in `sample-table.jtd`. Always appears after `0x000e` (table
row delimiter) and immediately before a `0x001c/0x0010` single-column paragraph:

```text
001c 0020 000c  0000 0010 [w5] 0000 0001  000c 0000 0020 001f
```

`w4=0x0010=16` (class code of the following `0x0010` record), `w7=0x0001=1`
constant. `w5=0x0002` or `0x0000`. Appears to mark the transition from a table
section back to normal single-column text. Semantic meaning is not decoded.

## Verified First-Page Control-Table Text Placement

The controlled source-y corpus establishes one narrow renderable profile for
horizontal first-page control tables. It is intentionally stricter than generic
table-candidate detection. The profile requires all of the following:

- a unit-based `0x000e` table candidate with a stable column pattern;
- one immediate `0x0010` parent record for each row, either starting at the
  row interval or ending immediately before it, with the same nonzero `w6`
  grid extent;
- one `0x0030` header ending exactly at every cell text range;
- a selected `LineMark` interval starting at that parent or row and ending at
  the row interval's end;
- a single first `PageMark` entry whose `w21` is a plausible 1/100-mm line
  pitch; and
- supported page margins and a uniform resolved text style for each cell.

For that profile, cell text uses the `0x0030` `b0` offset scaled over the
parent `w6` extent inside the source margins. Each row baseline uses its
`LineMark` record index multiplied by `PageMark w21`, plus the actual resolved
font size. ASCII spaces preserved immediately before a cell label contribute
two source grid units each; they adjust its position but are not repainted as
new visible glyphs.

The 20 admitted controlled documents cover baseline, horizontal movement,
whole-line vertical movement, and individual row-height changes. The generated
PDFs for `PAGE 01`, `PAGE 01_right_4Tick`, `PAGE 01_down_4Low`, `000_base_a`,
`013_table_moved_right`, and `020_row1_height_plus` place cell text within
0.25 pt in X/Y against their Ichitaro PDFs. This evidence does not include
the ink width: the local substitute font remains up to about 3.9 pt narrower
on a label. Source border-paint semantics remain unproven; the basic border
projection below uses an explicit renderer fallback style.

`050_wrapped_one_cell` is excluded. Its cell header offsets vary between rows
after wrapping, so a simple per-row text placement rule would not be sound.
Multi-page tables, merged cells, empty/sparse cells, vertical writing, and
mixed object trees remain diagnostic-only.

A three-row, two-column probe adds a structural
counterexample to control-only grouping: its `BEFORE-TABLE` paragraph shares a
`0x000e` interval with the first complete `0x0010` / `w4=0x008f` row header,
but all its visible text ends before that header starts. The control-run table
candidate excludes that paragraph while preserving the source text. Its actual
cell headers also contain `w6=0`, alongside the previously observed `0x00ff`,
with the same twelve-word count, echoed count, and terminator. For logical
control-table candidate admission, this word remains opaque; other values are
not admitted by that path. The separate physical-span projection below has its
own explicit-alignment admission. The local regression requires three
rows, six ordered cells, and one SVG instance of each surrounding paragraph.
It does not establish native PDF geometry or font-metric equivalence.

An independent 2×2 text-only probe has no
numeric cell values. Its complete `0x0010/w4=0x008f` parents and immediate,
fully framed `0x0030` cell headers admit the short table without the older
value-marker heuristic. Every non-empty cell range must lie after its parent
header and inside its row; column counts must agree. Unframed two-row text
remains unproven. This is candidate admission, not a general geometry decode.

A single-row 1×2 probe uses the same complete
parent/cell framing. It admits one multi-column row as a control-table candidate;
unframed single-row text is still rejected. Single-row border projection requires
the verified trailing empty-row record, giving explicit source boundaries rather
than inferring a height from a second row. Native PDF geometry remains unproven.

A 2×1 single-column probe supplies a counterexample to the
old minimum-column heuristic. A one-column control row is admitted only with
the complete native parent/cell framing used above. Its two cells and source
border project through the existing narrow profile; unframed one-column text
does not become a table candidate. Geometry and paint semantics stay undecoded.

### Interstitial flow text between two native control tables

One further first-page horizontal profile is renderable only when two complete
control-table projections bracket a single `DocumentText` text run. The run
must contain exactly two non-empty ASCII lines with the same nonzero leading
ASCII-space count, one unique preceding zero-offset `0x0030` header in the
gap after the first table, and exact containment of each line in a distinct
`LineMark` interval. Both intervals must resolve to the same first `PageMark`
entry with the supported `w21` pitch. The renderer preserves the source spaces
and sets each baseline from that interval's record index, the shared pitch,
and the resolved source font size.

`070_two_tables_vertical` satisfies this profile: two 2×2 native-table
projections bracket `BETWEEN01` and `BETWEEN02`, whose `LineMark` records are
7 and 9. Its generated PDF keeps all ten text runs separate and is within
0.51 pt in X and 0.21 pt in top position against the Ichitaro PDF. Those
figures validate only this tightly gated flow bridge; they do not decode
generic paragraph layout, whitespace semantics, or font ink widths.

### Physical ruled-flow spans and explicit alignment

A controlled one-row, two-column pair keeps the same first-cell text on three
physical lines while changing only its explicit cell alignment. Each line
retains its own framed `0x0030` declaration; repeated declarations do not imply
additional logical rows or cells. The first-cell declarations share `b0=2`,
`b1=78`, and parent grid extent 160 in both variants:

| Setting | `w6` | `w7`, successive physical lines |
| --- | ---: | --- |
| `左寄せ` (left) | 0 | 2, 2, 0 |
| `中央揃え` (center) | 1 | 2, 2, 0 |
| `右寄せ` (right) | 2 | 2, 2, 0 |
| `均等` (distributed) | 3 | 2, 2, 0 |

An additional center/right pair preserves the same `b0=2`, `b1=78`, grid extent
160, three physical lines, and second-cell text. Center uses
`marginLeft + (b0+b1)/2 * bodyWidth/gridExtent`; right uses
`marginLeft + b1 * bodyWidth/gridExtent`. SVG `text-anchor="middle"` or `"end"`
lets the paint backend use its real font advances. Each generated PDF's three
center midpoints or right endpoints agree with the native pair within 0.3 pt;
this does not establish equal glyph widths. The layer tree keeps `textAnchor`
and `anchorX` separate from its estimated bbox/positions with `decoded:false`.
This admission excludes edge whitespace, unknown continuation flags, other
writing modes, and logical-row reconstruction.

This associates `w6` with explicit alignment in this profile. The unchanged
`w7` sequence is a continuation-related candidate, not proof of justification,
logical-cell identity, or a complete flag schema. Inherited `w6=0x00ff` remains
distinct: the wrapped counterexample above has spacing that explicit left
alignment does not reproduce.

The model now retains ordered `DocumentTextFlow` events and raw flags before
deriving table views. A bounded first-page horizontal ASCII projection uses an
immediate parent/declaration, exact `LineMark` containment, first `PageMark`
pitch, source margins, and uniform source font size to place physical spans.
Explicit distributed spans without edge whitespace use
`(b1-b0) * bodyWidth / parentGridExtent` as SVG `textLength` with
`lengthAdjust="spacing"`; explicit left spans do not use distributed character
stretching. Their English word spacing also depends on the document style below. This does
not reconstruct logical table rows. Overlapping table fallbacks stop painting,
but their candidates remain diagnostics in the layer tree. Raw flags,
`decoded:false`, and estimated glyph-position labels remain visible.

Both variants pass the local source-span/order/duplicate-render checks, and
their generated PDFs preserve line anchors and the distributed extent. Exact
glyph metrics and ruled borders are not established. The inherited wrapped
`0x00ff` profile is still rejected by this projection pending independent
spacing evidence; distributed character spacing must not stand in for English
word justification.

### Document-default English justification

A controlled ON/OFF pair leaves `/DocumentText` byte-identical, including the
first cell's `w6=0` and `w7=2,2,0` sequence. Thus these cell flags alone cannot
determine English justification. Its `/DocumentViewStyles` default `0x100b`
record changes between these complete payloads:

```text
ON:  02 02 58 00    04 00 00 00 08
OFF: 02 02 58 40 00 04 00 00 00 08
```

The bounded model candidate recognizes only these profiles and a unique
record. Unknown masks, values, duplicate records, and other envelopes stay
unproven; this is not a complete optional-field schema. Raw style bytes remain
preserved and JSON reports `englishJustificationCandidate` with `decoded:false`.

The native PDFs justify the two nonfinal physical lines when ON and leave the
last line unchanged. For the admitted multiword ASCII flow, source span width
minus leading grid padding determines the requested advance extent. The paint
backend measures the natural font advance, and the model divides the positive
remaining width among interior ASCII spaces. It emits SVG `word-spacing`, not
distributed `textLength`. PDF and browser adapters supply their own font
measurements to the same model rule. Without measurements, SVG/layer evidence
explicitly remains unresolved rather than claiming decoded glyph positions.

An exact first/last text-run match to a first-page `LineMark` interval also
places the surrounding body baseline using source pitch, avoiding flattened
paragraph-order placement over the wrapped lines. This does not decode general
paragraph or page assignment. The ON/OFF regression covers unchanged source
flow, two spacing requests versus none, unchanged final-line behavior, and
separate surrounding text. Substitute-font metrics, single-word tracking,
wrapped inherited `0x00ff`, other writing modes, and ruled paint remain outside
this admission.

### Basic control-table border projection

For the admitted profile, vertical border positions use `parent w8 + 1` and
each cell header's `b1 + 1`, scaled by `bodyWidth / parent w6`. Horizontal
positions use `marginTop + recordIndex * pitch + resolvedFontSize / 2`.
The first boundary selects the first nonempty row's record minus one; internal
boundaries select the next nonempty row's record minus one. The last boundary
follows contiguous source-empty control rows only while their parent grid and
left edge match and their source ranges equal the corresponding `LineMark`
intervals. A non-table transition ends the scan; a source gap must not be skipped.

The local `020_row1_height_plus`, `PAGE 01`, and `070_two_tables_vertical`
PDF comparisons place the measured border centers within 0.14 pt. This is
coordinate evidence for that profile, not full border-paint equivalence.
The current SVG/PDF projection uses black 0.8 CSS-px lines (0.6 PDF pt) as a
renderer fallback. Color, thickness, dash patterns, joins, and overhanging corner
marks are not decoded by this projection. Border output remains `decoded:false`
and currently belongs to SVG/PDF; the page-layer tree exposes the table text
but does not emit these border lines.

### Directional rule paint in a controlled strip profile

The black-base, transparent, thick-preset, red, dashed-preset, and one-transparent-
edge native pairs retain identical content records, `LineMark`, and `PageMark`.
Only the trailing style events change. At junction source units, the observed
contextual properties are:

| Direction | Preset | Transparency | BGR24 color |
|-----------|-------:|-------------:|------------:|
| Up        | 1 | 4 | 15 |
| Down      | 2 | 5 | 16 |
| Right     | 3 | 6 | 17 |

These meanings apply to the junction context, not ordinary text: property 2
also supplies font size in text runs. The profile has at least three consecutive
physical parent records, each subtype `0x008f` and length `w5 + 12`. In each
top/bottom band, all but the last junction are `16` / `15` (hex), and the last
is `12` / `11`. Internal bands have `13` or `17` before the last junction and
`13` at the last. Bits 1/2/4 select
up/down/right strokes. A following run is `14` when the right bit is set and
zero otherwise. Junction strips
occupy two source grid units, inter-junction run lengths remain fixed across
bands, and each junction center is its strip start plus one. This is nominal
physical rule geometry, not logical-cell reconstruction or a general topology
decoder. Exact source `LineMark` starts and active page pitch bound placement.

Each complete four-word strip is `[junction,0,right-run-state,run-length]`.
The final junction can have an explicit `[0,right-padding-length]` run, or end
as a two-word `[junction,0]` pair directly before `ffff,0`. Widths and optional
right padding must remain identical across bands and exactly cover the parent
extent: add two grid units per junction and each run length; the explicit-tail
profile includes one additional terminal unit. Padding is not divided into
equal-width cells or stretched to the right body margin.

| Controlled columns | Interior run lengths | Final blank run | Junction centers |
|-------------------:|----------------------|-----------------|------------------|
| 1 | 156 | absent | 1, 159 |
| 2 | 76, 76 | 1 | 1, 79, 157 |
| 3 | 50, 50, 50 | 1 | 1, 53, 105, 157 |
| 5 | 28, 28, 28, 28, 28 | 7 | 1, 31, 61, 91, 121, 151 |

All four examples have parent extent 160 and left offset zero. The five-column
table is physically narrower because its terminal padding is seven units, not
because the renderer assigns a special layout to its file name. Wrong strip
counts, zero interior lengths, inconsistent widths, and extent mismatches reject
the profile. The existing first-page control-table text projection can enable
exact surrounding body `LineMark` positions even when no physical-flow text
slots remain; otherwise its trailing body could overlap the last table row.

Unset/reset presets (`0`/`ffff`) keep thin fallback paint; preset 3 selects the
observed thick solid and preset 4 the observed dash. Transparency 1 hides a
stroke, 0/unset leaves it visible. Value `80` occurs at inactive horizontal ends;
its active-stroke meaning is not admitted. Colors use the existing BGR24 mapping,
with unset/`ffffffff` retaining black. Unsupported values, malformed widths,
truncated style sections, source gaps, and other row patterns reject the profile.

The renderer uses 0.8/2.56 CSS-px thin/thick widths and a 3.2/3.2 CSS-px dash
pattern as bounded paint approximations. Native prints show 0.6/1.92 pt widths
and 2.4 pt dashes, but the UI exposes presets rather than numeric units: these
measurements do not establish a source length-unit decoder. Joins, dash phase,
corner glyph clipping, other presets, logical merge ownership, and general multipage paint remain
unproven. SVG/PDF retain `paint-decoded:false` and do not duplicate the older
black-border fallback. The basic 2x2 layer JSON retains all 30 directional stroke candidates,
including hidden strokes, raw preset/transparency/color, and source unit.
Hiding the internal vertical edge suppresses eight candidates without changing
the source content, geometry, or table candidates. Surrounding first/last text
uses its exact source `LineMark` interval so the trailing body does not overlap
the first row even when text is owned by the older control-table projection.

### Empty spans, partial boundaries, and explicit cell line breaks

Controlled empty-cell, space-only, vertical-merge, explicit-break, Japanese-wrap,
and automatic-row-growth samples share this two-column parent family. Truly
empty spans have consecutive `0030` declarations without a text event. A
space-only span retains its actual spaces and exact source range as a physical
text slot; it does not advance the unrelated body fallback's X position.
The ordered source flow preserves both cases distinctly. Source table/flow
placements suppress overlapping table fallbacks through the same guard, while
keeping their candidate evidence in the layer tree.

The vertical-merge example changes only the left middle junction `17` to `13`
and its following run `14` to zero. Only that horizontal stroke is omitted;
the right middle boundary and original text order remain. This does not infer
logical rowspan, merged-cell editing ownership, or removal of either text run.
The native merge also leaves an LF after the first left text and an LF-only
boundary span; these are retained as separate source-flow evidence.
The explicit-break case adds a vertical-only physical band; the native row-growth
example has 11 physical bands for four logical rows, not a multipage long table.
Rule placement follows the actual band count and boundaries rather than an
alternating-row assumption. Border admission does not require nonempty text.

A source LF at the end of a cell text part is retained in source flow and exposed
as `isLineBreakEnd` in the projected text layer, not as a paragraph boundary.
It suppresses automatic word spacing even with `w7=2`: the explicit-break
native print uses natural spacing. An inherited `w6=ff,w7=2` span is admitted
as a left-alignment candidate only when the immediately preceding physical line
in the same grid/span has known-left alignment and ends with an explicit LF.
The raw flags and `preceding-explicit-break-left-candidate` basis remain visible.
Automatic inherited wrapping without this predecessor still rejects projection.

Horizontal Japanese text now retains each source physical line and uses UTF-16
unit counts for trimmed source spans. Its natural fallback font paint does not
claim decoded tracking, punctuation adjustments, mixed Japanese/English
justification, or exact native glyph metrics. The existing ASCII-only word
spacing candidate remains separate. Unknown scripts and controls remain in
source evidence rather than being promoted by this bounded projection.

### Physical source pagination around ruled bands

The controlled near-bottom, whole-table-on-page-two, and cross-page examples
have respectively 40/53/44 physical LineMark records and 1/2/2 native pages.
Their cell-text records are 33/35/37, 46/48/50, and 37/39/41. Active fixed84
PageMark entries provide inclusive ranges 0..39 and 40..79. Selected-page
placement uses `recordIndex - lineStart`; zero-pitch trailing capacity entries
do not establish blank pages. This fixes source pagination, not just page count.

The model plan maps original text-run source spans into those intervals while
retaining paragraph indices and character ranges. Noncontiguous cell fragments
and empty rule bands remain physical source lines, not logical table rows.
The observed final interval includes one terminal unit beyond named content;
the marker is not painted. Body/rule baselines use page-local pitch. An upward
rule at a page's first line is clipped to the source top margin. The cross-page
print has no closing horizontal rule on page one; the next boundary belongs to
page two, so none is invented on page one.

Complete source coverage, matching ordered paragraph spans, uniform visible-run
font sizes, ASCII/Japanese text,
known rule/control records, and unique active page ranges are required. Edited
or spanless text, Unknown/Ruby/inline content, other scripts, per-page layout
styles, and unsupported framing keep fallback pagination. Page/line budget
preflight uses the same plan before page text allocation. Layer text exposes
global `lineMarkRecordIndex`, candidate page assignment, and `decoded:false` /
`positionsDecoded:false`. Raw source/marks/unknown data stay intact. General
section pagination, native glyph metrics, and Japanese tracking remain unproven.

### Cumulative font height and fixed source-line pitch

Controlled 10.5pt/14pt table and body runs use font sizes 370/494 mm100 units.
The default line advance uses `max(documentDefaultFont, visibleLineFonts)` plus
PageMark gap word14. Word19 retains the document default; it is distinct from
the first body font in words10/13 when only the body is enlarged. With default
370 and gap222, base advance is592. A line containing494 adds124 units to every
following line on that page. Three enlarged table text lines therefore add372
units before the following body; enlarging only the first body line adds124.
These are source-derived advances, not fitted PDF coordinates.

Fixed 10mm line pitch is stored in a framed `0010/0020` prefix:

```text
w3..w9: 0000 0020 0004 0008 03e8 0008 03e8
```

The repeated attributes must agree. A standalone length16 record ends with
`ffff,0` before its footer. A composite record embeds `008f` at word10 and shifts
the usual ruled fields/style-unit positions by six words; the original frame is
not rewritten. The local outgoing advance becomes1000, while subsequent plain
bands retain592. This applies to the initial body line or the explicit-break cell
line carrying the prefix, not the entire table. PageMark word20=8/word21=1000
can describe the first body line and must not override later ruled bands.

Leading source grid padding is excluded only when resolving visible font size;
raw text/ranges and whitespace-only spans remain intact. This avoids rejecting
the first enlarged cell because its padding still carries the old font. Rule
vertical halves meet at neighboring source-line midpoints when advances differ.
SVG/layer/body and legacy control-table placement consume the same cumulative
metrics. Raw PageMark pitch metadata and unknown fields remain available, and
geometry/paint stay candidate / `decoded:false`. Mixed visible fonts, other
paragraph attributes, shrinking-font behavior, and exact glyph metrics are not
generalized by these controlled examples.

## Correlation with LineMark unit-start

The `sample-academic.jtd` sample (25 parsed LineMark records) shows exact correspondence
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

### Corroboration: `sample-draft.jtd`

The law-document draft sample `sample-draft.jtd` (43 parsed
LineMark records, `base-unit=16`) shows the same pattern at larger scale:
25 of 43 `unit-start` values fall exactly on a `0x001c` record position.

Selected representative rows (first 31 of 43):

| LineMark record | LineMark unit-start | 0x001c unit in /DocumentText |
| --------------- | ------------------: | ---------------------------: |
| 0               | 16                  | — (no 0x001c at 16)          |
| 1               | 41                  | 41                           |
| 2               | 103                 | — (falls inside text run)    |
| 3               | 114                 | 114                          |
| 4–7             | 178–322             | — (all fall inside text run) |
| 8               | 353                 | 353                          |
| 9               | 386                 | 386                          |
| 10              | 445                 | 445                          |
| 11              | 513                 | 513                          |
| 12              | 579                 | 579                          |
| 13              | 634                 | 634                          |
| 14              | 668                 | 668                          |
| 15              | 729                 | 729                          |
| 16              | 793                 | 793                          |
| 17              | 918                 | 918                          |
| 18              | 1036                | 1036                         |
| 19              | 1070                | 1070                         |
| 20              | 1128                | 1128                         |
| 21              | 1184                | 1184                         |
| 22              | 1223                | 1223                         |
| 23              | 1288                | 1288                         |
| 24              | 1351                | — (falls inside text run)    |
| 25              | 1372                | 1372                         |
| 26              | 1436                | 1436                         |
| 27              | 1462                | 1462                         |
| 28              | 1512                | 1512                         |
| 29              | 1551                | 1551                         |
| 30              | 1572                | 1572                         |
| 31              | 1636                | — (falls inside text run)    |
| 32–40           | 1667–1675 (delta=1) | — (all fall inside text run) |
| 41              | 1684                | 1684                         |
| 42              | 1748                | — (falls inside text run)    |

Records 32–40 have `delta=1` each, with unit-start values 1667–1675
consecutively. These fall inside a single long text run spanning units
1589–1684 that contains multiple embedded `\n` characters (the main-body
施行 sentence followed by the 理由 section preamble). Each delta=1 LineMark
record corresponds to one of the embedded newlines, confirming that LineMark
enumerates physical display-line starts regardless of whether a `0x001c`
paragraph boundary exists at that position.

## Impact on Current Token Parser

The current `parse_document_text` function (in `rjtd-core/src/document_text.rs`)
reads the stream as big-endian UTF-16 and treats `0x001c` as a plain
`ControlBoundary`. When `0x001c` appears, the next `0x001f` it encounters
starts a new text run. This works for text extraction because the header words
between `0x001c` and `0x001f` do not decode as valid Unicode text (they are
control-range values). The parser effectively skips the header by stopping on
`0x001c` as a boundary, then resuming on `0x001f`.

The model additionally preserves complete length/echo/class/terminator frames
as ordered `DocumentTextFlow` record events, bounded by named TextV.01 content.
Unknown gaps and raw record words remain available; framing is not a decode of
indent levels, style references, logical cells, or border paint. Table candidates
are derived views and do not replace the source events. The `decoded:false`
principle applies.

## Trailing TextV.01 Style Event Section

`/DocumentText` carries a second, byte-oriented event stream after its UTF-16BE
content. The big-endian `u32` at byte offset 28 is the content-unit count, and
the observed style section begins at `32 + content_unit_count * 2`. Its event
cursor starts at source unit 16, matching the 32-byte `/DocumentText` header.

The observed event grammar is:

- `00 <u32-be length>`: a run covering `length` source units;
- `fe (<property-id> <value-length> <value-bytes>)* ff 00`: a property-change
  event covering one source unit;
- `ff`: terminal marker, with any remaining bytes preserved as trailing data.

Property changes form persistent state: values apply to the change unit and
remain active across following run events until another change replaces them.
The currently observed typed widths are 1 byte for property IDs 4–7 and 9–12,
2 bytes for IDs 1–3, 8, 13, 14, 18, and 19, and 4 bytes for IDs 15–17 and 20.
Unknown IDs and width mismatches remain raw evidence. This event stream is
separate from, and must not be confused with, the `0x001c/0x0010 w4=0x008f`
table-row header family described above.

The `shanai_lan` label probes and controlled body contrast associate property 15
with text color. When one value covers a text fragment's complete source range, the observed
`0x00BBGGRR` values map as follows:

| Property 15 value | CSS color | Observed use |
|------------------:|-----------|--------------|
| `0x00008000` | `#008000` | diagram title |
| `0x00800000` | `#000080` | blue device and server labels |
| `0x00660000` | `#000066` | dark-blue NAS label |
| `0x00000000` | `#000000` | controlled black body text |
| `0x000000ff` | `#ff0000` | controlled red body text |
| `0x00ff0000` | `#0000ff` | controlled blue body text |
| `0xffffffff` | default | automatic/default color sentinel |

This proves the packed-color encoding for those ranges, but not a universal
property role. In the cross-sample `hyo` fixture, property 15 also occurs over
non-text/control ranges associated with table state. Therefore the renderer
uses it as a source color for uniform, exact text ranges in the supported body
SVG/layer paths and as a decoded-false candidate in the `shanai_lan` projection.
It does not reinterpret control/table-state ranges as colors. Mixed, uncovered,
or unsupported high-byte values retain the default fill. The controlled body
contrast verifies the three raw values and their SVG/layer colors, not general
style inheritance or border color.

## 0x000e and 0x000a Control Codes

### 0x000e Row Delimiter

In `sample-table.jtd` (a table-heavy new-vs-old comparison document), every
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

### 0x000a Line Break (decoded:false)

`0x000a` appears 210 times in `sample-table.jtd` and is present in every
current sample (range: 2–4671 per file). Unlike `0x000e`, it is not confined
to inter-cell positions between `0x0030` records. Context analysis:

- Most common predecessor: `0x001f` (74×) — the text-run start/record terminator;
  also follows CJK characters (字 etc.) and ASCII space `0x0020`
- Followed in 169 of 210 cases by `0x001c 0x0030` (table cell header) and in 24
  cases by `0x001c 0x0010` (paragraph header)

This suggests `0x000a` acts as a **within-cell line break** or **intra-paragraph
newline**, separating text runs that continue with a new `0x001c` record — either
the next cell header (`0x0030`) or a fresh paragraph header (`0x0010`). Semantic
meaning is not decoded; it may correspond to a soft return, a hard line break
within a table cell, or a paragraph-level newline in non-table context. Not to be
confused with `0x000e`, which is strictly a between-cell row delimiter bounded by
`0x0030` records on both sides.

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
  cells with a stable constant block plus varying `w8`/`w9`/`w13` fields — when
  `w8=0` the values are small flag-like integers (dominant: `w9=4,w13=2` 27×;
  `w9=2,w13=2` 20×; `w9=0,w13=2` 11×); when `w8=1` the values are large and
  cell-specific (`w9−w13=400=(b1−b0)×3.125` invariant holds; w8=1 records
  always appear immediately before `0x001c/0x0001` inline/heading content);
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
  `0x1b`, `0x24`–`0x27` and the role of `w8` are not decoded. Cross-record
  analysis shows `w8` takes only two values: `0x0001` (22 records) or `0x0003`
  (107 records). Notably, all 14 records with `n_cells=12` (the widest rows in this
  sample) have `w8=0x0001`, while `n_cells=4` rows are overwhelmingly `w8=0x0003`
  (72×) with only 5 exceptions (`w8=0x0001`). The `w8=0x0001` / `0x0003` split is
  not equal to the count of `0x23`-tagged sub-entries and no clean rule has been
  found; it may encode a row-type flag (e.g. header row vs data row).
- Class `0x0010` records of varying length appear to share a common sub-header
  signature `0x0026 0x0005` at words `w4/w5` (seen in `sample-academic.jtd` len=20 and
  `sample-outline/sample-draft/sample-reference` len=17 samples). Detailed analysis of `sample-reference.jtd`
  len=17 records (142 total, `w4=0x0026 w5=0x0005`) reveals 9 distinct payload
  combinations driven by `w6`/`w7`/`w8`/`w9`/`w10`. When `w6=1` (102 records):
  `w7=0x01ec=492` and `w8=w10=0x01cc=460` are constant, forming what appears to be
  a hanging-indent group (if 1/20 mm: 24.6/23 mm; if 1/10 mm: 49.2/46 mm). When
  `w6=0` (40 records): `w7` takes 0/2/4, `w8` is mostly 0, indicating no hanging
  indent. In `sample-academic.jtd` len=20, `w10=0x0141=321` appears on indented continuation
  lines, consistent with a ~32 mm hanging indent. In `sample-table.jtd`, the
  `w4=0x002e` variant (18 records, len=13) is fully constant (`w5=w6=1`, `w7=0xffff`,
  `w8=0`), suggesting uniform single-column layout. The unit scale and exact field role
  are not decoded.
  Cross-sample analysis of 246 `w4=0x0026 len=17` records across 11 tested samples
  reveals a new structural split by `(w8, w10)` magnitude: `(w8=w10=0x01cc=460)`
  appears only in `sample-reference` (also with `w6=1`); `(w8=w10=1)` or `(w8=w10=0/2)`
  appears in `sample-outline` and `sample-draft` samples exclusively. The `sample-draft` sample
  has 21 records and `w7` takes values 0/1/2/6/8. Correlation of `w7` with the
  following text in `sample-draft` shows the mapping is not simple visual-indent
  depth: `w7=0` appears on flush-left statute headings, clauses, and article openers;
  `w7=1` on article-clause continuation text; `w7=2` on item-list and appendix entries
  (some with one leading fullwidth space, some flush); `w7=6` on preamble body text;
  `w7=8` on supplementary-provision headings. This pattern is consistent with `w7`
  encoding a paragraph-style ID rather than a visual indent count. The mapping of `w7`
  values to Ichitaro named paragraph styles is not proven. `(w8, w10)` remains either a
  document-type discriminator or encodes style IDs (short-text samples) vs physical
  coordinates (reference-statute samples). No physical unit scale is yet proven.

  Extended sweep across all 14 tested samples confirms the `w7` value set
  and adds two further values. Observed `w7` values (`w4=0x0026 len=17`) and their
  associated text contexts (decoded:false):

  | `w7` | `w8`/`w10` | Text context | Candidate style role |
  | ---- | ---------- | ------------ | -------------------- |
  | 0    | 0x0001     | Article/clause body, flush-left headings (sample-outline/sample-draft) | Standard body paragraph |
  | 0    | 0x0002     | Short flush-left body entries (sample-reference-b) | Standard body paragraph (mixed family) |
  | 0    | 0x0000     | Body paragraphs (sample-draft-b) | Standard body paragraph (w8=0 family) |
  | 0    | 0x01cc     | Hanging-indent first entry or TOC heading (sample-reference) | Standard body / TOC heading |
  | 1    | 0x0001     | Article clause continuation (sample-draft) | Body continuation line |
  | 2    | 0x0001     | Item list / appendix entries (sample-draft) | Item / indent paragraph |
  | 2    | 0x0000     | Article body (sample-draft-b) | Item / indent paragraph (w8=0) |
  | 2    | 0x0002     | Article body (sample-reference-b, sample-reference-c) | Item / indent paragraph (mixed family) |
  | 3    | 0x0000     | Supplementary-provision sub-heading e.g. 「（施行期日）」 | Provision sub-heading |
  | 4    | 0x0000     | 別表 / deeper appendix indent (sample-draft-b, sample-reference) | Deep indent / appendix |
  | 4    | 0x0002     | Law section heading (sample-reference-b) | Law section heading (mixed) |
  | 6    | 0x0001/0x0000 | Statute title / preamble body (all sample-outline/sample-draft samples) | Title / preamble style |
  | 8    | 0x0001     | Supplementary-provision heading 「附　則」 (sample-draft) | Supplementary-provision heading |
  | 10   | 0x0000     | Reason section heading 「理　由」 (sample-draft-b) | Reason heading |
  | 492 (0x01ec) | 0x01cc | Hanging-indent law text (sample-reference, w6=1) | Hanging-indent body |

  Across all `sample-outline` samples `w7=6` appears on the law title or preamble and is
  the only non-zero value (records ≤ 3 per file). Across all `sample-draft` samples
  `w7=6` appears on the law title or preamble headline, `w7=8` on supplementary
  provisions, and `w7=0` on all regular clause body paragraphs. The consistent
  recurrence of `w7=6` for the opening title line across every `sample-outline`/`sample-draft`
  sample strengthens the interpretation that `w7` encodes a named paragraph-style
  identifier, not a visual indent depth. The exact Ichitaro style names corresponding
  to each `w7` value remain unproven (decoded:false).

## Samples Used

| Sample | Records | Families observed |
| --- | ---: | --- |
| `sample-academic.jtd` | 19 | `0x0010 len=20` only |
| `sample-table.jtd` | 1039 | `0x0010` (all len), `0x0030 len=12`, `0x0000 len=12/21`, `0x0020 len=12` |
| `sample-draft.jtd` | 33 | `0x0010`, `0x0030 len=12` |
| `sample-reference.jtd` | 504 | `0x0010`, `0x0030 len=12`, `0x0000 len=12` |
