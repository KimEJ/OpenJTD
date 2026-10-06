# RFC 0003: DocumentText Initial Text Extraction

Status: draft

Observed: 2026-06-18

Japanese translation: [0003-document-text.ja.md](0003-document-text.ja.md)

## Summary

The `/DocumentText` stream contains recoverable body text.

The stream starts with the ASCII magic:

```text
SsmgV.01
```

Initial extraction shows text runs encoded as UTF-16BE after a `0x001F` marker.

Some visible text is also stored inside inline segments delimited by `0x001D ... 0x001E`.

This is enough for a first `rjtd cat <file.jtd>` implementation. rjtd now has a structured `ParsedDocumentText` token layer for observed text runs, inline text, and control boundaries, but it is not yet a complete `DocumentText` record parser.

## Implemented Commands

```sh
cargo run -p rjtd-cli -- dump-stream ../rjtd-testdata/local-samples/a5.jtd /DocumentText
cargo run -p rjtd-cli -- cat ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-tokens ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-control-context ../rjtd-testdata/local-samples/a5.jtd 0x001c
cargo run -p rjtd-cli -- text-control-ranges ../rjtd-testdata/local-samples/a5.jtd 0x001c
cargo run -p rjtd-cli -- text-map ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- cat ../rjtd-testdata/local-samples/setsuden_05.jttc
cargo run -p rjtd-cli -- cat ../rjtd-testdata/local-samples/ichitaro-20030706231249-success-001-success_data-fujimoto_file.jtd
```

`dump-stream` writes raw stream bytes to stdout.

`cat` reads `/DocumentText`, parses it into `ParsedDocumentText`, and emits the parser's plain-text projection. For observed `.jttc` samples, it first unwraps `/JSCompDocument` `JustCompressedDocument` data and reads `/DocumentText` from the decompressed inner CFB. For observed samples that do not expose a named `/DocumentText` stream, it scans for embedded `SsmgV.01`/`TextV.01` fragments and extracts plausible text lines.

`text-tokens` emits the structured token stream as tab-separated lines:

```text
text	銀河
control	0x001c
skipped-inline	0x0082	20	ごご
text	鉄道\n
```

`text-map` emits the same tokenization with byte ranges, UTF-16 unit ranges, token kind, selector/control metadata, and any `MarkV.01` ids whose raw offsets fall inside each token range. It is a diagnostic bridge between `/DocumentText` and `/DocumentTextPositionTables`.

`text-control-context` emits each control boundary with its byte/unit range, neighboring map entries, and nearest previous/next control boundary. It accepts an optional decimal or hex control-code filter, such as `0x001c`.

`text-control-ranges` emits the intervals before, between, and after control delimiters. Without a filter every mapped control boundary is a delimiter; with a filter such as `0x001c`, only that control code splits the stream, while other controls remain counted inside the interval. Each row includes previous/next delimiter metadata, entry index span, byte/unit span, token-kind counts, control-code counts, and a short text preview.

## Current Structured Token Parser

### Named Text Segment Boundary

Observed streams with `SsmgV.01` at byte 0 and `TextV.01` at byte 20 have a
big-endian `u32` content length at byte 28, measured in UTF-16 units. Content
starts at byte 32; the style section follows those declared units. This is the
same boundary used by the DocumentText style-section parser.

Visible text can begin immediately at byte 32 without a `0x001f` marker. The
controlled `PAGE 01` family and independent local `sample_macro` / `toolbox`
documents expose this case. Treating the low header word as a raw-vs-record
format selector loses that initial text. The parser and source map therefore
enter text mode at the named segment's start and stop at its declared end,
bounded by the available complete units. This also prevents style-tail marker
bytes from becoming body text. Lengths use both words of the `u32` field.

Streams without this named header retain marker-based recovery. Physical-file
embedded-fragment recovery cannot assume contiguous logical stream bytes, so
it retains the existing bounded salvage scan rather than trusting a fragment's
declared logical length to discard later recoverable text. Raw bytes remain
preserved in the document model.

### Token Decoding

The current parser:

1. Reads `/DocumentText` as big-endian 16-bit units.
2. Starts text at a validated named segment's beginning or after `0x001F`.
3. Stops a text run on C0/C1 control boundaries, except tab, LF, and CR.
4. Recovers selected visible inline segments wrapped by `0x001D ... 0x001E`.
5. Preserves decoded pieces as `TextRun`, `InlineText`, `SkippedInlineText`, and `ControlBoundary` elements.
6. Produces `cat` output from the structured parser's plain-text projection.

Valid UTF-16 surrogate pairs are decoded together in body, visible-inline, and
preserved skipped-inline text. Source ranges remain measured in UTF-16 units,
so a supplementary character occupies two units. A pair cannot cross the
declared text-segment boundary; malformed units retain boundary behavior.

`SkippedInlineText` is not emitted in plain `cat` output. It is retained with its selector, decoded text, and raw UTF-16BE bytes, then lifted into the document model as an `UnknownObject` with source tag `0x001d`.

Skipped inline segments are preserved only when the matching `0x001E` terminator appears within 256 UTF-16 units after `0x001D`. If no bounded terminator is found, the parser leaves the region as ordinary control/text boundaries instead of consuming a large binary or formatting region as text. This is a preservation-first safety rule, not a final format interpretation.

Embedded fallback uses the same heuristic after locating raw `SsmgV.01` fragments. It is intentionally limited:

- it runs only when named `/DocumentText` and supported `/JSCompDocument` paths are absent;
- each fragment is bounded to the next `SsmgV.01` marker or 64 KiB;
- implausible noise lines are dropped with a conservative character filter;
- the document model records the source as `/EmbeddedDocumentText`.

## Observed Font Size Sources

Controlled font changes expose property `2` as a big-endian `u16` size in
hundredths of a millimeter: `370` corresponds to approximately 10.5 pt and `494`
to 14 pt. The `053_font_size_table_plus` cell ranges and the
`054_font_size_paragraph_plus` / `082_plain_paragraph_font_size_plus` body ranges
carry the larger value. Reference PDFs use 10.44/14.04 pt, consistent with
printer quantization; exact device rounding is not decoded here.

The supported `/DocumentViewStyles` sequential `0x1006` record profile has a
20- or 21-byte payload beginning `1f 00 00`, with the default size at payload
bytes 3–4 in the same units. Other record profiles remain unsupported. Property
`2 = 0` restores the default. Missing, malformed, mixed, or uncovered property
ranges must not be mistaken for a uniform explicit size; reserved values
`0xfffd..0xffff` are not rendered as giant fonts.

The model renderer now carries supported explicit/default sizes into SVG/PDF
and layer-tree text runs with their basis. Text advance and line wrapping still
use fallback metrics; this is not full font shaping or layout fidelity.

## Bounded Character Style and Font Candidates

Controlled character runs preserve property boundaries in UTF-16 source units;
a surrogate pair is never split. Rendering separates these runs without
changing the model's plain text, source flow, or raw style bytes.

The observed property `20` profiles are `0x84000000` (bold), `0x90000000`
(italic), `0x80000010` with property `13 = 1` (single underline),
`0x80000c00` (upper quarter), and `0x80000400` (lower quarter).
`0` and `0x80000000` are admitted ordinary profiles. Quarter scripts additionally
require both one-byte properties `4` and `5` to equal `50`; they use half width
and height. Other combinations, widths, and flags remain unsupported.

Property `3` selects a unique `/Font` entry by its `u16` ID. `0xffff` restores
the default ID at bytes 5–6 of the supported sequential `0x1006` payload;
an unset property uses that same default. Missing or duplicate IDs and
reserved values do not select an arbitrary font. A range can return to the
default between explicit selections even inside a single paragraph.

SVG/PDF and layer-tree runs retain candidate flags and `decoded:false` evidence.
The horizontal renderer shares the largest unscaled size as a line baseline,
places upper-half scripts at the fallback em top, and keeps lower-half scripts
on that baseline. Regular-only faces receive synthetic bold stroke (0.025 em)
and italic shear (0.25). These paint choices and script placement are renderer
approximations, not decoded font metrics or native device geometry. The PDF
backend and browser supply glyph advances through the existing model rendering
API so adjacent style runs use their selected or substitute fonts' widths.
Layer positions and wrapping still use fallback metrics; native whitespace,
exact print quantization, combined style profiles, and vertical script placement
remain unproven. Synthetic bold may produce repeated searchable text in PDF.

## Auxiliary Footnote Text Preservation

The model now retains `/Footnote`, `/FootnoteLink`, and `/MarkTag` streams
unchanged, including malformed and unsupported forms. They are separate from
body text; a footnote's cached editor tail must not enter the body projection.

A bounded 44-byte, one-entry `/FootnoteLink` profile contains big-endian `u32`
addresses at bytes 10–13 and 18–21. Adding the 16-unit text header bias places
the first at the note's `0x001f` text marker and the second at its body context
record. Both addresses, matching cached marker strings, and complete 13-unit
records are checked together. The note context has word 7 `0x0030`, ID word 8
`0`, followed by an otherwise matching closing context with ID `0xffff`;
the linked body context has word 7 `0x0010`, ID `0`. Only the intervening note
text is exposed as `footnoteTextCandidates`, with its marker, note/body source
spans, raw link addresses, and `decoded:false`/placement/link-role limits.

The candidate reader is capped at 64 KiB of auxiliary text and requires unique
streams. Truncated or mismatched addresses, other link profiles, duplicate
streams, or other note contexts retain raw evidence without a guessed match.
This preserves source text; it does not establish general note numbering,
multiple-note linking, note-area geometry, marker scaling, or bookmark semantics.
Native note placement and field evaluation remain separate unresolved work.

## Visible Inline Ranges and Explicit Page Boundaries

An explicit `0x000c` remains a control boundary but no longer terminates text
reading inside a validated named `TextV.01` segment. Both the token parser and
source map retain the following pages. Unbounded marker-only input keeps its
previous conservative stop behavior.

Core inline map ranges include `0x001d`/`0x001e`. Model text ranges now exclude
those wrappers when their length equals the complete visible UTF-16 string
plus two units; malformed or non-contiguous inline strings have no guessed
text span. Raw map/flow ranges remain unchanged. Physical-line admission also
checks the visible bytes and translates UTF-16 offsets to character offsets,
rejecting boundaries inside a surrogate pair.

The existing fixed-84-byte source-page path now admits complete selector-1
cache groups, the controlled heading context (`0x0010`, 13 units, subfield
`0x002e`, values 1–3), and the observed 13-unit numbered-marker context
(`0x0000`, subfield 10, `391/0x2010`). It applies them only to non-ruled
horizontal flow. Unknown contexts, incomplete wrappers, section styles, and
vertical text retain their fallbacks. This preserves stored marker text and
source line/page assignment; it does not decode editable heading/list semantics,
TOC leaders, dynamic fields, or native whitespace/tracking. Portrait/landscape
page ranges use the shared model layout path; native landscape Japanese
character spacing is still unresolved.

## Bounded Saved TOC Rows

A controlled `/DocumentText` TOC region uses paired 12-unit class `0x0020`
records with word 4 `0x0030`/`0x0031`. Their other observed fields and order must
match, and there must be exactly one pair. A source line containing only that
setting record and its line terminator consumes no text pitch. Ordinary blank
or space-only content does not inherit this rule.

Within this region, the observed 17-unit title context and optional 18-unit leader
context (`1` or `100` raw variant) can pass the existing physical source-page
path. Two visible text runs separated by exactly those contexts provide saved
TOC title/page-label metadata through `tocEntries`, with source spans and
`decoded:false`. Incomplete, duplicate-section, or unknown contexts keep the
fallback and raw evidence. Metadata is cached content, not a decoded generation,
heading-level, bookmark, or editable TOC model.

General horizontal tab stops and exact leader metrics are unproven; the bounded saved leader projection is described below. Page assignment
and text pitch now follow the shared model path, while page labels retain
fallback horizontal placement. Raw context fields remain available for further
analysis; no reference-PDF coordinates are used to place the region.

## Bounded Field Cache Bindings

The controlled JTD, JTT, and JTTC inputs share three class-0 field profiles:
28-unit printing date (`0x0033`), 15-unit page number (`0x0035`), and 12-unit
external link (`0x0048`). Complete selector-1 value wrappers and selector-0
argument wrappers must be contiguous with the record; dates require `DATE`,
page numbers require `PAGENUMBER`, and HTTP(S) links require their target plus
an empty secondary argument. Other record fields and profiles stay raw.
`textFieldCandidates` exposes the cache, argument, and record/value spans with
`decoded:false`. This does not establish a general field expression engine.

A validated `YYYY/MM/DD` printing date is a model render context, not a mutation
of cached text or raw data. `DocumentCore::set_print_date` supplies it to SVG
and layer output; the WASM wrapper and viewer supply the local browser date.
Native Unix PDF export obtains the local date from the platform `date` command
only when a supported printing-date field exists. The explicit
`to_pdf_with_file_name_and_print_date` API supports reproducible output on all
native targets; without a date provider or supplied context, caches remain.
Page-number caches are preserved without general renumbering.

The controlled external-link profile binds the record's first-unit color and
underline properties to its visible cache. SVG carries the escaped HTTP(S)
target, and layer output retains field/cache evidence. Complete field groups
also pass physical source-page admission. Bookmark positions, other date
formats/types, arbitrary targets, editable field semantics, and PDF link
annotations remain unsupported. Geometry and font metrics retain their
existing candidate/fallback limits.

## LayoutBoxText Content

`/LayoutBoxText` also contains length-delimited `TextV.01` blocks. Their bounded
content is mapped in text mode from its beginning, including leading text and
UTF-16 surrogate pairs. Only visible text/inline entries are projected;
skipped inline annotations remain in the preserved raw stream.

A control-only block must not fall back to treating every printable control
payload word as text. The first block of `054_font_size_paragraph_plus` contains
object references, not a title; the former fallback emitted spurious `0ԇ`
fragments. That fallback has been removed. Text recovery does not by itself
prove the geometry or ownership of the later cell-text blocks.

## Inline Segment Observation

The local samples show repeated inline segment contexts:

```text
001C 0001 0007 0000 0000 0003 001D <visible base text> 001E
001C 0001 0007 0000 0001 0082 001D <phonetic annotation> 001E
```

The first form appears to hold visible ruby base text, such as `午后`, `天気輪`, `捕`, and `切符`.

The second form appears to hold phonetic annotation text, such as `ごご`, `てんきりん`, `と`, and `きっぷ`.

Plain `cat` output currently emits the visible base text and skips the phonetic annotation text.

Template samples also show:

```text
001C 0001 0007 0000 0000 0001 001D <visible placeholder text> 001E
001C 0001 0007 0000 0001 0000 001D <template instruction text> 001E
```

Plain `cat` output emits visible placeholders, such as `○○○`, and skips template instruction text.

Skipped inline segments are still preserved for reverse-engineering. For example, local `a5.jtd` exposes rows such as:

```text
skipped-inline	0x0082	20	ごご
skipped-inline	0x0082	26	てんきりん
skipped-inline	0x0082	22	きっぷ
```

## Control Boundary Observation

`text-control-context` and `text-control-ranges` were added after `TCntV.01` range diagnostics showed that `0x0202` chosen byte ranges repeatedly include `/DocumentText` controls. Across the 61 current local samples, the context command runs without errors and 60 files contain mapped control boundaries.

Top observed control codes:

| Control code | Rows | Files |
| --- | ---: | ---: |
| `0x001c` | 51,971 | 60 |
| `0x000e` | 6,621 | 41 |
| `0x001d` | 1,156 | 32 |
| `0x0000` | 682 | 57 |
| `0x000c` | 166 | 24 |
| `0x0090` | 99 | 13 |

The two controls most relevant to the current `TCntV.01` work have different local context profiles:

| Code | Most common previous/next map-entry kinds | Count |
| --- | --- | ---: |
| `0x001c` | text -> text | 16,717 |
| `0x001c` | text -> control | 10,329 |
| `0x001c` | control -> text | 7,191 |
| `0x001c` | control -> control | 6,561 |
| `0x000e` | control -> control | 3,338 |
| `0x000e` | text -> control | 1,832 |
| `0x000e` | text -> skipped-inline | 844 |
| `0x000e` | control -> text | 356 |

This makes `0x001c` the strongest current generic delimiter candidate. It often separates visible text runs from other visible text or from control clusters. `0x000e` appears more control-cluster-like and often sits directly next to another control boundary, or before skipped inline content. These are observations only; neither code has a final semantic name yet.

Synthetic tests cover:

- text-run extraction after `0x001F`;
- bytes before the first text marker are ignored;
- C1 control values such as `0x0090` are treated as boundaries;
- visible inline ruby base text is emitted while phonetic annotations are skipped;
- visible template placeholders are emitted while template instructions are skipped;
- `ParsedDocumentText` preserves observed text runs, inline text segments, and control boundaries before plain-text projection;
- `text-control-context` reports previous/next map entries and nearest previous/next control boundaries, including optional code filtering;
- `text-control-ranges` reports control-delimited intervals and preserves non-delimiter controls inside filtered ranges;
- skipped phonetic/template inline segments are preserved as `SkippedInlineText` tokens and document-model `UnknownObject` payloads;
- observed ruby base plus phonetic annotation pairs are promoted to document-model `Inline::Ruby`, preserving annotation text and raw payload while visible text output uses the base text;
- unbounded inline starts do not consume the rest of a large control or binary region as `SkippedInlineText`;
- `/JSCompDocument` payloads with `JustCompressedDocument` are decompressed when they match the observed `-lh5-` profile;
- invalid synthetic compressed payloads fail clearly;
- embedded `SsmgV.01` fragments are recovered when `/DocumentText` is absent.

## Local Sample Results

| Sample | `/DocumentText` bytes | `cat` output characters |
| --- | ---: | ---: |
| `46.jtd` | 239844 | 39281 |
| `a5.jtd` | 240104 | 39348 |
| `a6.jtd` | 239324 | 39394 |
| `b6.jtd` | 239324 | 39333 |
| `ichitaro-success-report-20030316045810.jtd` | 14604 | 5997 |
| `shinsyo.jtd` | 239064 | 39319 |
| `fax02.jtt` | 1864 | 159 |
| `raihoumemo01.jtt` | 6804 | 273 |
| `syojo01.jtt` | 1084 | 74 |
| `setsuden_05.jttc` | 564 | 38 |
| `setsuden_06.jttc` | 564 | 39 |
| `ichitaro-20030706231249-success-001-success_data-fujimoto_file.jtd` | embedded | 641 |
| `ichitaro-20030706231543-success-001-success_data-iwata_file.jtd` | embedded | 8178 |

The extracted text begins with:

```text
銀河鉄道の夜				宮沢 賢治

目次
```

The samples appear to contain 宮沢賢治「銀河鉄道の夜」 text.

After inline base-text recovery, the table of contents begins with:

```text
一、午后の授業
二、活版所
三、家
四、ケンタウル祭の夜
五、天気輪の柱
```

Template `.jtt` samples also expose `/DocumentText` and can be read by the same command.

The `.jttc` samples do not expose `/DocumentText` directly. They contain `/JSCompDocument` streams beginning with:

```text
2600 4a75 7374 436f 6d70 7265 7373 6564 446f 6375 6d65 6e74
```

This decodes to the length-prefixed marker `JustCompressedDocument`, followed by an LHA `-lh5-` member in the observed payload. Decompressing that member yields an inner CFB file with its own `/DocumentText` stream.

The observed `.jttc` template samples are mostly blank/control-heavy after plain text extraction. They currently produce no non-empty document model blocks.

Two local `.jtd` samples open as `cfb-embedded-document-text`. They do not expose a named `/DocumentText` stream, but raw file bytes contain repeated `SsmgV.01` and `TextV.01` markers. The recovered text includes visible document content such as:

```text
参加者募集中団体名氏　名
ハイキングクラブ会報・第２０号
```

## COM Text Export Observation

Plain-text export via `JXW.Application` COM automation (`TaroLibrary.SaveDocument`, `filterNo=10`) produces output that uses box-drawing characters from the Unicode U+2500 series to represent Ichitaro table structure. This independently corroborates the DocumentText control code assignments observed in local samples.

### Table Cell Delimiter Corroboration

COM text export uses U+2502 VERTICAL LINE (`│`) as a column delimiter between adjacent table cells:

```text
項目│値
合計│100
```

This matches the observed DocumentText control code `0x001c` (51,971 occurrences across 60 local sample files), confirming its role as a table cell boundary. The code is defined in rjtd-core as:

```rust
pub const TABLE_CELL_DELIMITER_CONTROL: u16 = 0x001c;
```

### Table Row Delimiter Corroboration

COM text export uses the following box-drawing characters for horizontal table borders:

```text
┌─┬─┐   (top edge)
├─┼─┤   (inter-row divider)
└─┴─┘   (bottom edge)
```

Characters used: U+2500 (`─`), U+250C (`┌`), U+251C (`├`), U+253C (`┼`), U+2514 (`└`), U+2524 (`┤`), U+252C (`┬`), U+2510 (`┐`), U+2534 (`┴`)

This matches the observed DocumentText control code `0x000e` (6,621 occurrences across 41 local sample files), which appears frequently in control-cluster context (`control -> control` most common pairing). It is defined in rjtd-core as:

```rust
pub const TABLE_ROW_DELIMITER_CONTROL: u16 = 0x000e;
```

### Page Break Corroboration

COM VBA scripts use `Chr(12)` (ASCII form feed, `0x0C`) as the page break character when searching or splitting exported text. This confirms DocumentText control code `0x000c` (166 occurrences across 24 local sample files):

```rust
pub const DOCUMENT_TEXT_PAGE_BREAK_CONTROL: u16 = 0x000c;
```

### Confidence Level

These corroborations are strong but not exhaustive:

- The VBA → box-drawing → DocumentText control mapping is consistent with all observed data.
- Multiple document types (`.jtd`, `.jtt`) were covered by the VBA automation corpus.
- The `基本` tab mode is required for correct full-body export; other tab modes may produce structurally different DocumentText content.
- Semantic naming (TABLE_CELL_DELIMITER vs TABLE_ROW_DELIMITER) is now confirmed by independent cross-format evidence rather than control-code pattern analysis alone.

## Known Gaps

- The inline segment rules are still heuristic and based on observed local samples.
- The structured token layer is not yet a full record parser and does not recover styles, full ruby semantics, tables, or layout objects.
- Embedded fragment recovery is heuristic and should be replaced by proper object/stream boundary parsing.
- `DocumentText` record boundaries are not decoded yet beyond observed token/control boundaries.
- `0x001c` and `0x000e` are high-priority delimiter candidates, but their exact record/table/object/paragraph semantics are not decoded.
- The relationship between `/DocumentText` and `/DocumentTextPositionTables` is now testable through `text-map` and `text-position-context`, but the stable coordinate rule is not yet proven.
- JTTC support is limited to the observed `JustCompressedDocument` plus single `-lh5-` member profile.
- The initial LH5 decoder does not yet validate LHA header checksums or CRC values.

## Next Steps

- Decode true `DocumentText` records beyond the current token layer.
- Identify record or token meanings around `0x001C`, `0x000E`, `0x001D`, `0x001E`, and `0x001F`.
- Use `/DocumentTextPositionTables` to recover missing or reordered text if it participates in text layout.
- Identify the container/object boundary that owns embedded `SsmgV.01` fragments.
- Expand `JustCompressedDocument` documentation as more `.jttc` samples are observed.
- Recover paragraph boundaries and style references from surrounding streams instead of deriving model blocks from plain-text line breaks.

## Bounded Modern Vertical Text

The controlled modern sequential view family uses the same `0x1001` stock-size
profile for horizontal and vertical text. Its `0x1002` payload keeps the margin
quad at offset 2: the 32-byte horizontal form has `0x40` at offset 10; the
33-byte vertical form has `0x50,0x01` at offsets 10/11. Both have the same
ten repeated `0x02bc` words and zero suffix after the direction portion.
Duplicate size/margin records, unknown flags, or explicit page styles do not
select this global profile. The edit section `0x2000` changes with caret state
and is not a writing-direction discriminator. Legacy first-record heuristics
remain diagnostic only.

Plain text uses validated LineMark source columns and the existing PageMark
font-plus-gap pitch across the horizontal axis. The controlled `0x1006` tail
value `0x0266` with the known `0x100b` profile corresponds to the native UI's
60% character spacing. The bounded renderer applies that spacing to Japanese
runs; this association does not establish a general numeric-unit formula.
Latin and whitespace advances, fallback font metrics, and vertical glyph
substitutions remain approximate. Source column boundaries are preserved.

A class-zero 12-unit record `[1c,0,12,0,47,0,5,0210,12,0,0,1f]`, followed by
an empty index-0 selector-1 cache and an index-1 selector-`0x0101` two-digit
cache, identifies the controlled no-fit tatechuyoko candidate. Complete
contiguous wrappers and terminators are required. The numeric value becomes
a source-linked TextRun while its original skipped cache and raw controls
remain preserved. SVG/PDF and layer output place this pair horizontally in
one vertical cell; ordinary `123` retains the normal vertical orientation.
Other lengths, nonnumeric caches, fit modes, ruby, and incomplete records are
not promoted by this rule. JSON exposes both record/value spans with
`decoded:false`; geometry and font/spacing interpretation remain candidates.

The controlled global landscape profile now reuses the same `0x0266`/known
`0x100b` 60% spacing association as vertical text. Japanese fullwidth runs
receive the spacing; Latin runs retain backend advances. Source LineMark rows
and PageMark pitch own pages/baselines. SVG/PDF and layer data share source
ranges and candidate spacing. This remains a bounded uniform-font/plain-flow
projection; other spacing values, rich/table/field/section profiles, exact
Latin/whitespace metrics and general numeric-unit interpretation stay unresolved.

## Bounded saved TOC leaders

Within the previously framed saved TOC scope, the exact 17-unit title context
`[1c,0,17,0,9,375,31,0090,0,2,f81e,0,0,17,0,0,1f]` and optional 18-unit
leader record `[1c,0,18,0,21,0,23,0090,K,2,a77c,0,0,0,18,0,0,1f]` retain
three corroborated variants: `K=1` solid, `K=100` dotted, or no leader record.
Every record must be followed contiguously by the complete empty cache
`[1c,1,7,0,0,1,1d,C,1e,5,0,1,1f]`, with `C=5` after the title context and
`C=3` after a leader record; the literal title and numeric saved
label must bind to the same source row and metadata entry.

The controlled title context reserves one fullwidth separator cell. With a
leader, the label ends at the source body's right margin; without a leader it
stays after that separator beside the title. Backend glyph advances determine
text widths, and source LineMark/PageMark data determines pages and baselines.
SVG/PDF and layer output preserve text spans and leader-record spans. Unknown
contexts/caches, rich or vertical rows, missing physical source rows, and
conflicting widths retain fallback text. The model is never rewritten.

This is a bounded source-associated rendering candidate. Solid/dotted selection
is corroborated by native creation settings and output; the fullwidth gap,
right anchor, leader vertical position, stroke width and dot pitch retain
`decoded:false`/`geometryDecoded:false`. Leader metrics use a backend-neutral
approximation, without native-PDF coordinate fitting. Arbitrary tab stops,
heading navigation, TOC regeneration and editing remain unproven.

## Bounded linked footnote marker and ruby source

The single-link footnote candidate now retains both marker source spans. In the
controlled horizontal profile, the visible body marker has property 1 value 1
and property 20 value `0x80000000`, while its note-area marker references 2.
Explicit size/scale overrides are absent. The unique SsmgSlots TextLayoutStyle
has the corroborated two `0x5555` records at `0x114`/`0x214`, matching kind,
character and paragraph subrecords. Slot 1's exact `0x5004` profile includes
`-50`, `60`, `50`, `50` fields and the half-size upper marker; slot 2 retains
the normal profile. Only that complete link/span/reference/style association
reuses the existing upper half-size renderer. Other references, scales and
incomplete profiles retain normal fallback text and all raw data. SVG/layer
output names the inherited script basis; JSON retains the note-marker span.
This does not establish a general property-1 inheritance decoder.

The controlled no-layout-mark body profile has two literal paragraphs, one
source newline, default size 370 and the known `0x100b` 600 line-gap profile.
It contains only the known note and optional grouped-ruby records, has no
other object/table/section data, and each paragraph fits one line. Source and
model text must agree. SVG and layer placement use the 60% gap association
without adding a fallback blank paragraph row. Edits, wrapping, unknown gap
values or other layouts retain fallback; numeric units and geometry remain
candidates. The note area's baseline and separator placement remain unproven.

Ruby promotion retains the base TextRun's source span instead of discarding it.
JSON and layer source data expose that span, and SVG/PDF reuse source font
sizes and backend base advances. Source-font ruby uses a half-size annotation
and an em-relative upper baseline as rendering candidates; manual annotations
without source font data retain the prior fallback.

The controlled grouped-kana record is
`[1c,0,12,0,5,0,517,512,12,0,0,1f]` with complete contiguous selector-3 base
and selector-`0x0082` annotation caches. The source base and annotation must
agree with the model, and annotation font/scale overrides must be absent.
For fullwidth kana fitting the measured base width, the renderer distributes
the remaining width between cells, with half a gap at either edge. It retains
`data-group-ruby-candidate` and decoded-false geometry. Other grouping records,
font changes, scripts, long or non-kana annotations retain fallback; general
ruby units, exact printer metrics and editing remain unresolved.

## Bounded selected-page character spacing

The previously validated single-style portrait/landscape/portrait association
also owns the middle page's controlled spacing. Only page 2 with the known
apply/reset, three fixed84 page entries, stock swap and equal source margins
is admitted. The global view must independently identify horizontal writing;
the existing global-only direction guard for explicit styles remains intact.

The unique `0x4006` subrecord has length 26, prefix `0000c10000`, the same
hundredth-millimeter font size as the global default at bytes 5..7,
`8000003f000266` at 7..14, 100/100 scales at 14..18, and suffix
`8000800002800000`. The unique `0x400a` profile is `c300000d000050400100`.
This corroborates the same controlled 60% spacing association as the global
view profile; it does not decode a general numeric formula or style inheritance.

The shared tracking projection uses physical source rows and backend Latin
advances, while Japanese cells retain the 60% spacing candidate. SVG/layer
output distinguishes `page-layout-style-4006` from `document-view-style-1006`.
Unknown size/scale/spacing/line profiles, writing direction or layout/margin
associations retain fallback. Other pages retain their prior rendering.
Geometry, glyph baselines, whitespace and printer quantization remain candidates.

## Bounded standalone paragraph pitch

The controlled plain paragraph uses a 16-word class `0x0010` record:
`001c 0010 0010 0000 0020 0004 0008 P 0000 0000 ffff 0000 0010 0000 0010 001f`.
Unlike the previously admitted table/body profile, its second pitch pair is
absent (`0,0`), rather than a repeated attribute. The controlled `P=1000`
corroborates the saved 10mm paragraph line advance. This remains a bounded
hundredth-millimeter pitch candidate, not a general paragraph decoder.

Only a complete standalone profile immediately preceding a model paragraph's
preserved source span is admitted, with global horizontal writing and no rule
grid. All such records must have paragraph bindings. Physical LineMark ranges
supply wrapped rows; the candidate advance follows that paragraph's source
bounds and stops at its end, rather than leaking to later paragraphs. Existing
repeated table attributes retain their checks. Source text, spaces, raw records
and PageMark values remain unchanged. SVG/PDF and layer data share the source
page plan; resource preflight counts those physical rows.

Unknown fields, incomplete source/line coverage, vertical or grid mixtures and
lost source bindings retain fallback. Other paragraph profiles, exact glyph
metrics and printer quantization remain unresolved. Geometry stays decoded-false.

## Bounded control-cell padding typography

In the admitted horizontal control grid, leading ASCII spaces have their own
source character properties. A controlled 14pt cell selection starts after the
first cell's three padding spaces, but includes the padding of subsequent cells.
Their property-2 ranges therefore differ even when every visible label is 14pt.

The existing two-grid-unit space advance now scales by the padding's source
font size relative to the document default. The visible label's font size is
resolved independently. Default padding retains its prior position exactly.
This is a bounded placement candidate; it does not decode general space units,
proportional-font metrics or cell ownership. No reference-PDF position is used.

SVG and layer data retain the raw cell source range and additionally name the
padding prefix range, space count and candidate width. Raw text, original spaces,
source records and line metrics are preserved. Mixed/uncovered padding font
ranges, malformed font values or explicit width/height scale properties retain
fallback. Other cell typography remains unresolved; geometry stays decoded-false.
