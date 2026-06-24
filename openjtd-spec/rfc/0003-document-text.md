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

The current parser:

1. Reads `/DocumentText` as big-endian 16-bit units.
2. Starts a text run after `0x001F`.
3. Stops a text run on C0/C1 control boundaries, except tab, LF, and CR.
4. Recovers selected visible inline segments wrapped by `0x001D ... 0x001E`.
5. Preserves decoded pieces as `TextRun`, `InlineText`, `SkippedInlineText`, and `ControlBoundary` elements.
6. Produces `cat` output from the structured parser's plain-text projection.

`SkippedInlineText` is not emitted in plain `cat` output. It is retained with its selector, decoded text, and raw UTF-16BE bytes, then lifted into the document model as an `UnknownObject` with source tag `0x001d`.

Skipped inline segments are preserved only when the matching `0x001E` terminator appears within 256 UTF-16 units after `0x001D`. If no bounded terminator is found, the parser leaves the region as ordinary control/text boundaries instead of consuming a large binary or formatting region as text. This is a preservation-first safety rule, not a final format interpretation.

Embedded fallback uses the same heuristic after locating raw `SsmgV.01` fragments. It is intentionally limited:

- it runs only when named `/DocumentText` and supported `/JSCompDocument` paths are absent;
- each fragment is bounded to the next `SsmgV.01` marker or 64 KiB;
- implausible noise lines are dropped with a conservative character filter;
- the document model records the source as `/EmbeddedDocumentText`.

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
