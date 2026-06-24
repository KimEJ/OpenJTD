# RFC 0007: Layout Mark Streams Initial Inventory

Status: draft

Observed: 2026-06-18

Japanese translation: [0007-layout-mark-streams.ja.md](0007-layout-mark-streams.ja.md)

## Summary

Some JTD samples expose layout-oriented streams next to `/DocumentText`:

```text
/LineMark
/PageMark
/PaperMark
```

These streams are not parsed into the document model yet. They are layout/cache candidates that may explain page, line, paper, or anchor positions after the `DocumentText` record structure is better understood.

## Local Samples

The current local samples with all three streams include:

| Sample | `/LineMark` bytes | `/PageMark` bytes | `/PaperMark` bytes |
| --- | ---: | ---: | ---: |
| `46.jtd` | 5194 | 8160 | 788 |
| `a5.jtd` | 5122 | 6312 | 612 |
| `b6.jtd` | 5334 | 8244 | 796 |
| `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | 190 | 272 | 108 |

`a6.jtd` and `shinsyo.jtd` do not expose these streams in the current inventory.

## PageMark Observation

The first 12 bytes look like a compact header in the observed large samples. Raw `u32be` diagnostics are available through `stream-dwords` and `stream-dword-frequencies`.

```text
46.jtd  00000060 00000010 0000005f
a5.jtd  0000004a 00000010 00000049
b6.jtd  00000062 00000010 00000061
```

Working interpretation:

- first `u32be`: count-like value;
- second `u32be`: `0x10`, likely a size or stride-like value;
- third `u32be`: first value minus one;
- the next `u32be` is row 0's index-like value in the observed 84-byte row family.

The total stream size does not reduce to a simple `header + count * 0x10` formula. However, the current large samples prove one stable family:

```text
12-byte header
N rows of 84 raw bytes
```

The first dword in each 84-byte row is an index-like value. The rest of the row is preserved as raw bytes because the fields are not semantically decoded yet.

The `count_value = last_index_value + 1` invariant holds for all 14 tested samples (Ginga large + 11 new local government/academic). `/PageMark` and `/PaperMark` share the same `count_value` in every sample. The meaning of `count_value` is not decoded.

| Sample | header value 0 | header value 1 | header value 2 | rows | stream length formula |
| --- | ---: | ---: | ---: | ---: | --- |
| `46.jtd` | 96 | 16 | 95 | 97 | `12 + 97 * 84 = 8160` |
| `a5.jtd` | 74 | 16 | 73 | 75 | `12 + 75 * 84 = 6312` |
| `b6.jtd` | 98 | 16 | 97 | 98 | `12 + 98 * 84 = 8244` |
| `論文様式.jtd` | 3 | 16 | 2 | 3 | `12 + 3 * 84 = 264` |
| `01要綱（事務局組織令）.jtd` | 7 | 16 | 6 | 135 | `12 + 135 * 84 = 11352` |
| `03新旧（整備令）.jtd` | 11 | 16 | 10 | 12 | `12 + 12 * 84 = 1020` |
| `04参照条文（施行日政令）.jtd` | 7 | 16 | 6 | 154 | `12 + 154 * 84 = 12948` |
| `04参照条文（組織令）.jtd` | 6 | 16 | 5 | 7 | count-plus-one-trim2, row_bytes=1852 (7 large variable entries) |
| `04参照条文（整備政令）.jtd` | 25 | 16 | 24 | 154 | `12 + 154 * 84 = 12948` |

High-frequency `u32be` values show packed coordinate-like tuples inside the raw rows, but the internal field layout is not decoded:

| Sample | top non-zero dword patterns |
| --- | --- |
| `46.jtd` | `0x01610161` x182, `0x01610008` x100, `0x00000161` x97, `0x00f60000` x95 |
| `a5.jtd` | `0x01610161` x138, `0x01610008` x76, `0x00000161` x74, `0x00f60000` x73 |
| `b6.jtd` | `0x01610161` x184, `0x01610008` x102, `0x00000161` x98, `0x00f60000` x97 |

`rjtd page-marks <file>` exposes raw-preserving parsers for the fixed 84-byte family, fixed 84-byte rows with preserved trailing bytes, the count-plus-one variable-row families, and the count-variable family. Across the current 61 local `.jtd`/`.jtt`/`.jttc` samples, 55 expose `/PageMark`; 52 parse through `page-marks` and 3 are intentionally rejected as unsupported shapes.

`rjtd page-mark-shape <file>` exposes non-failing shape candidates for the remaining variants. The current initial groups are:

| Group | Count | Parser status | Representative | Observation |
| --- | ---: | --- | --- | --- |
| fixed 84-byte rows | 17 | parsed | `justsystems-20120223023549-jp-just-finance-j200003.jtd` | tail is divisible into 84-byte raw rows |
| fixed 84-byte rows matching count-plus-one | 3 | parsed | `a5.jtd` | `fixed84` and `count-plus-one` both fit: 75 rows of 84 bytes |
| count-plus-one variable rows | 14 | parsed | `ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd` | header `3,16,2`; tail divides into 4 rows of 274 bytes |
| count-plus-one with 2-byte tail/trim | 9 | parsed | `ichitaro-20041103143104-seminar2004-part2_2-img-shortcutkey2.jtd` | raw stream is not u32-aligned; `(tail - 2)` divides into 7 rows of 556 bytes |
| count-variable rows | 2 | parsed | `ichitaro-20030415170937-success-001-success_data-fujimoto_file.jtd` | tail divides by header count but not count-plus-one |
| fixed 84-byte rows with trailing bytes | 7 | parsed | `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` | one or more 84-byte rows followed by preserved trailing bytes |
| non-PageMark-looking payload | 3 | unsupported | `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | payload contains stream/object names or legacy class/control metadata rather than numeric row headers |

Some parsed regular-stream samples still have declared CFB sizes much larger than the safely readable payload; for example `ichitaro-20030120133129-0007-sp-dat-tmogi3_2.jtd` exposes 304 readable bytes while the directory entry declares `9434469490474615088` bytes. The parser uses the safely read payload and keeps the declared-size anomaly in `page-mark-shape`.

The three unsupported `non-page-header` payloads are not promoted to a parser family. `stream-text-probe` shows they point at unrelated-looking text payloads:

| Sample | Probe evidence |
| --- | --- |
| `ichitaro-20030706231945-success-001-success_data-kaisya_annai.jtd` | UTF-16LE names such as `LayoutBoxTextPositionTables`, `TextLayoutStyle`, `DocumentEditStyles`, `DocumentViewStyles`, and `SummaryInformation` |
| `ichitaro-20030706232401-success-001-success_data-kazoku_ryoko.jtd` | ASCII/class-like strings such as `Ver.2.3 for Windows95`, `JSFart2`, and `JS.FartCtrl.1` |
| `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | UTF-16 names such as `JSRV_SegmentInformation`; its `/PaperMark` similarly exposes `ReferenceInfo` |

Working interpretation: these three `/PageMark` directory entries likely point at stale, foreign, or alternate object payload bytes rather than normal layout rows.

`stream-chain` confirms that these unsupported entries are not simply broken miniFAT chains. Their `/PageMark` chains are complete, but the bytes at those mini-stream offsets decode as unrelated payloads:

| Sample | `/PageMark` chain evidence | Payload evidence |
| --- | --- | --- |
| `ichitaro-20030706231945-success-001-success_data-kaisya_annai.jtd` | miniFAT start 133, complete chain, 832 bytes capacity for 796 declared bytes | CFB directory-entry-looking fragments including `LayoutBoxTextPositionTables` |
| `ichitaro-20030706232401-success-001-success_data-kazoku_ryoko.jtd` | miniFAT start 88, complete chain, 192 bytes capacity for 162 declared bytes | OLE/ActiveX-like metadata strings `Ver.2.3 for Windows95`, `JSFart2`, `JS.FartCtrl.1` |
| `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | miniFAT start 96, complete chain, 320 bytes capacity for 272 declared bytes | CFB directory-entry-looking fragments including `JSRV_SegmentInformation` |

`cfb-map` narrows this further:

- `kaisya_annai`: the root mini-stream chain overlaps the directory chain from sector `13` onward, so the directory-entry-looking bytes can be explained by root mini-stream/directory chain overlap.
- `shanai_lan`: the root mini-stream chain similarly overlaps the directory chain from sector `13` onward.
- `kazoku_ryoko`: `cfb-map` does not show the same directory/root mini-stream overlap. `stream-find` ties `/PageMark` to an exact 162-byte slice of `/EmbedItems/Embedding 1/JSFart2Contents` at offset 1664, so this `/PageMark` entry is duplicate embedded-control payload rather than a layout-mark variant.

## PaperMark Observation

The first 12 bytes look related to the `PageMark` header:

```text
46.jtd  00000060 0000000c 0000005f
a5.jtd  0000004a 0000000c 00000049
b6.jtd  00000062 0000000c 00000061
```

Working interpretation:

- first `u32be`: count-like value shared with `/PageMark`;
- second `u32be`: `0x0c`, likely a size or stride-like value;
- third `u32be`: first value minus one.

The observed large samples now prove a stable row shape:

```text
12-byte header
N rows of:
  u32be index
  u32be flags
```

The row count is derived from stream length as `(stream_len - 12) / 8`. The first header value is count-like but is not always equal to either `row_count` or `row_count - 1`, so the parser preserves it as an observed header value instead of assigning semantics.

However, from the 11 new government/academic local samples added in 2026-06-24, a stronger invariant emerges across **all 14 currently tested samples**:

- `header_value_2 = header_value_0 - 1` (always; `last_index_value = count_value - 1`)
- `/PageMark` and `/PaperMark` headers share the same `count_value` in every sample

This means `count_value` and `last_index_value` are not independent — one is derived from the other. Their shared meaning is not decoded. Possible candidates: number of distinct page-section transitions, number of page-layout regions, or an unrelated document-level counter.

| Sample | header value 0 | header value 1 | header value 2 | rows | flag distribution |
| --- | ---: | ---: | ---: | ---: | --- |
| `46.jtd` | 96 | 12 | 95 | 97 | `0x00010000` x89, `0x00010010` x7, `0x00010011` x1 |
| `a5.jtd` | 74 | 12 | 73 | 75 | `0x00010000` x65, `0x00010010` x9, `0x00010011` x1 |
| `b6.jtd` | 98 | 12 | 97 | 98 | `0x00010000` x90, `0x00010010` x6, `0x00010011` x2 |
| `論文様式.jtd` | 3 | 12 | 2 | 3 | `0x00010010` x1, `0x00010000` x2 |
| `01要綱（事務局組織令）.jtd` | 7 | 12 | 6 | 138 | `0x00010010` x129, `0x00010000` x9 |
| `03新旧（整備令）.jtd` | 11 | 12 | 10 | 16 | `0x00010010` x2, `0x00010000` x14 |
| `04参照条文（施行日政令）.jtd` | 7 | 12 | 6 | 158 | `0x00010010` x107, `0x00010000` x51 |
| `04参照条文（組織令）.jtd` | 6 | 12 | 5 | 158 | `0x00010010` x105, `0x00010000` x53 |
| `04参照条文（整備政令）.jtd` | 25 | 12 | 24 | 158 | `0x00010010` x114, `0x00010000` x44 |

The `0x00010011` flag, observed in Ginga vertical samples (`46`/`a5`/`b6`), does not appear in any of the 11 new horizontal government/academic samples. The vertical samples also have a much higher ratio of `0x00010000` to `0x00010010` entries compared to the government document samples.

The flags `0x00010000`/`0x00010010` interleave in alternating groups — runs of consecutive `0x00010010` entries followed by runs of consecutive `0x00010000` entries. The number of such groups is not the same as `count_value`.

Cross-referencing PaperMark flag runs with the `/PageMark` `lineStart`/`lineEnd` values for the corresponding entry index (same entry index, same document position) reveals a consistent structural pattern across the 11 government/academic samples. For each transition from a `0x00010010` run to a `0x00010000` run, the `lineStart` of the first `0x00010000` entry is larger than the `lineEnd` of the last `0x00010010` entry before it. The gap is approximately 30 lines in the `04参照条文` samples, 11–13 lines in the `02案文` samples. The `0x00010000` runs in `04参照条文` samples correspond to spans of `lineStart`/`lineEnd` values that span approximately 30 contiguous lines each — strongly suggesting a section of body text such as one referenced statute article.

Example from `04参照条文（施行日政令）.jtd`:

```text
PaperMark entry 3–6: flags=0x00010000, PageMark lineStart=[70,100,130,160], lineEnd=[99,129,159,160]
  (entry 6 has lineStart=lineEnd=160 — a zero-extent sentinel page)
PaperMark entry 7–13: flags=0x00010010, PageMark lineStart=[209,239,...,389], lineEnd=[238,268,...,418]
  (gap from lineEnd=160 to lineStart=209 is 49 — the boundary between legal text blocks)
PaperMark entry 14–20: flags=0x00010000, PageMark lineStart=[419,449,...,569]
  (gap from lineEnd=418 to lineStart=419 is 1; transition within a legal block)
```

Working interpretation (decoded:false): `0x00010010` marks pages that belong to a continuous body section (one legal statute article or one document segment), while `0x00010000` marks transitional pages — section separators, front matter, or blank spacers — that separate distinct layout regions. The `lineStart=lineEnd` zero-extent page (entry 6) may be a sentinel or empty section marker rather than a visible page. The `0x00010011` flag (Ginga vertical samples only) remains undecoded.

`rjtd paper-marks <file>` exposes this parser-backed diagnostic. It is not wired into the document model yet because the header and flag semantics are unknown. `rjtd paper-mark-shape <file>` exposes a non-failing shape diagnostic for all observed `/PaperMark` streams.

Across the current 61 local `.jtd`/`.jtt`/`.jttc` samples, 55 expose `/PaperMark`; 52 parse with this row shape. `paper-mark-shape` opens all 55 streams; three are intentionally classified as `non-paper-header` and rejected by `paper-marks`:

| Sample | observed header/stride evidence | Text probe evidence |
| --- | --- | --- |
| `ichitaro-20030706231945-success-001-success_data-kaisya_annai.jtd` | stride-like dword `0xffffffff` | UTF-16 `JSRV_SegmentInfor...` |
| `ichitaro-20030706232401-success-001-success_data-kazoku_ryoko.jtd` | stride-like dword `0xff090000` | short non-row payload; paired `/PageMark` contains `Ver.2.3 for Windows95`, `JSFart2`, `JS.FartCtrl.1` |
| `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | stride-like dword `0xffffffff` | UTF-16 `ReferenceInfo` |

Working interpretation: these three `/PaperMark` directory entries likely point at stale, foreign, or alternate object payload bytes rather than normal paper-mark rows.

`stream-chain` shows the same pattern for `/PaperMark`: the miniFAT chains are structurally complete, but the payload bytes are not paper-mark rows.

| Sample | `/PaperMark` chain evidence | Payload evidence |
| --- | --- | --- |
| `ichitaro-20030706231945-success-001-success_data-kaisya_annai.jtd` | miniFAT start 157, complete chain, 128 bytes capacity for 100 declared bytes | CFB directory-entry-looking fragment containing `JSRV_SegmentInfor...` |
| `ichitaro-20030706232401-success-001-success_data-kazoku_ryoko.jtd` | miniFAT start 97, complete chain, 64 bytes capacity for 36 declared bytes | short control/object payload beginning with `SO`-like bytes, not a `0x0c` paper-mark header |
| `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | miniFAT start 111, complete chain, 128 bytes capacity for 108 declared bytes | CFB directory-entry-looking fragment containing `ReferenceInfo` |

The same `cfb-map` interpretation applies to `/PaperMark`: the `kaisya_annai` and `shanai_lan` directory-entry fragments are explained by directory/root mini-stream chain overlap. `kazoku_ryoko` does not exact-match another complete stream, but `cfb-dir` places its `/PaperMark` entry in the root-level object/control sequence between `/EmbedFrame` and `/Figure`. The payload begins with the `SO` marker also seen in `/Figure` and `/EmbedItems/Embedding 1/\x01CompObj`; `stream-find-bytes 534f0000` reproduces those marker hits. The first 20 bytes after the marker also match `/EmbedItems/Embedding 2/JSFart2Contents` offset 192 (`stream-find-bytes ff090000a00800009a130000a008000000000000`). This makes it an embedded-control/object record candidate rather than a paper-mark row variant.

`rjtd so-records <file>` preserves this marker family as a diagnostic. It prints every `SO\0\0` marker with the containing stream path, offset, first little-endian `u32` fields, and raw bytes. Across the current 61 local samples, only 4 samples expose `SO` records, with 24 records total. `kazoku_ryoko` is the only current sample where `/PaperMark` itself contains an `SO` record.

`rjtd so-record-clusters <file>` groups these records by the preserved raw bytes. The observed `JSFart2Contents` samples split into singleton geometry-like clusters and repeated default/control clusters, usually at three repeated offsets. The `kazoku_ryoko` `/PaperMark` record matches the geometry-like `JSFart2Contents` record seen in the older `ichitaro-20030315133825-success-001-success_data-kazoku_ryoko.jtd` sample:

```text
0x00004f53,0x000009ff,0x000008a0,0x0000139a,0x000008a0,...
```

This strengthens the interpretation that `kazoku_ryoko` `/PaperMark` is a leaked or duplicated embedded-control geometry record, not a JTD paper-mark row.

`rjtd so-record-fields <file>` expands each record as little-endian fields. Current evidence fits a 9-dword diagnostic shape:

- field 0 is always the marker `0x00004f53` (`SO\0\0`);
- repeated default/control clusters contain small constants such as `0x00000100` and `0x00000064`;
- singleton geometry-like clusters carry coordinate-like values in fields 1-4, usually with high 16-bit halves equal to zero in the `JSFart2Contents` samples;
- packed records split into `packed-jseq3-like` and `packed-ffff-preamble` families and should remain separate SO-like families until their object payloads are decoded.

`rjtd so-record-geometry <file>` makes this split explicit without treating it as final semantics. Across the same 61 local samples, the command checks all files successfully and reports 24 SO records in 4 files: 9 `geometry-like`, 8 `default-control`, 4 `packed-jseq3-like`, 2 `packed-ffff-preamble`, and 1 `truncated`. The `kazoku_ryoko` `/PaperMark` record is classified as `geometry-like` with fields `2559,2208,5018,2208`, matching the older `JSFart2Contents` geometry-like record.

`rjtd so-record-halves <file>` prints low/high 16-bit unsigned and signed halves for each SO payload dword. In the current samples, every `packed-jseq3-like` record occurs in `JSEQ3Contents`; field 6 repeats the low 16 bits of field 2, while field 7 carries a second small 16-bit value. The two `packed-ffff-preamble` records occur in `JSFart2Contents` at offset 324 and precede repeated geometry-like records in the same streams.

## LineMark Observation

`/LineMark` begins differently and looks closer to a token/control stream than a simple fixed-width numeric table:

```text
a5.jtd  0914 0000 0001 0000 048f 0000 0003 0000
46.jtd  0914 0000 0001 0000 050e 0000 0015 0000
b6.jtd  0914 0000 0001 0000 0531 0000 0002 0000
```

After the first words, common values such as `000d`, `000a`, `0011`, `0019`, `0082`, and `1002` recur. These overlap with values already seen around `/DocumentText` control and inline segments, so `/LineMark` should be investigated together with the structured `DocumentText` token map.

`stream-words` shows the first words as:

| Sample | word 0 | word 4 | word 8 |
| --- | --- | --- | --- |
| `46.jtd` | `0x0914` | `0x050e` | `0x050d` |
| `a5.jtd` | `0x0914` | `0x048f` | `0x048e` |
| `b6.jtd` | `0x0914` | `0x0531` | `0x0530` |

Working interpretation:

- word 0 is a `LineMark` header/type candidate;
- word 4 is count-like;
- word 8 is word 4 minus one;
- words 6 differs by sample and is not decoded.

Raw word-frequency comparison shows that many small `/LineMark` words also appear in raw `/DocumentText`, but the high-frequency `0x1000`, `0x1001`, and `0x1002` words appear to be `/LineMark`-specific tags:

| Sample | `0x1000` | `0x1001` | `0x1002` |
| --- | ---: | ---: | ---: |
| `46.jtd` | 302 | 24 | 165 |
| `a5.jtd` | 293 | 18 | 200 |
| `b6.jtd` | 308 | 25 | 169 |

These tag-like values are not treated as `DocumentText` control codes.

`rjtd line-mark-tags <file>` scans `/LineMark` for these tag-like words and prints their word index, byte offset, previous four words, and next six words. A sample-wide sweep across the current 61 local samples produced:

| Group | Count |
| --- | ---: |
| files with tag rows | 5 |
| files without `/LineMark` | 6 |
| readable `/LineMark` streams with no tag rows | 50 |
| total tag rows | 1536 |
| `0x1000` rows | 915 |
| `0x1001` rows | 67 |
| `0x1002` rows | 554 |

The five files with tag rows are:

| Sample | `0x1000` | `0x1001` | `0x1002` | total |
| --- | ---: | ---: | ---: | ---: |
| `46.jtd` | 302 | 24 | 165 | 491 |
| `a5.jtd` | 293 | 18 | 200 | 511 |
| `b6.jtd` | 308 | 25 | 169 | 502 |
| `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` | 0 | 0 | 5 | 5 |
| `ichitaro-20030706234132-success-004-success_data-asobinin_24.jtd` | 12 | 0 | 15 | 27 |

The first word after a tag is not a unique family discriminator. Frequent next words such as `0x0025`, `0x0027`, `0x002d`, `0x004d`, `0x0037`, and `0x0035` overlap across tag families, so the next word is currently treated as payload-like context rather than a decoded tag subtype.

`rjtd line-mark-text-context <file>` compares each tag row with the `/DocumentText` token map. It tests two weak hypotheses: whether the LineMark word/byte offset directly lands in a `DocumentText` map entry, and whether the immediate next word appears anywhere in raw `/DocumentText`.

Across the same 61 local samples:

| Metric | Count |
| --- | ---: |
| files checked successfully | 55 |
| files without `/LineMark` | 6 |
| tag rows reported | 1536 |
| direct LineMark byte-offset hits in `DocumentText` map | 587 |
| direct LineMark unit-offset hits in `DocumentText` map | 587 |
| tag rows whose immediate next word appears in raw `/DocumentText` | 1511 |

Per tag family:

| Tag | direct byte hits | direct unit hits | next-word raw `DocumentText` hit rows |
| --- | ---: | ---: | ---: |
| `0x1000` | 344 | 344 | 904 |
| `0x1001` | 27 | 27 | 67 |
| `0x1002` | 216 | 216 | 540 |

Working interpretation: `/LineMark` tag-next words usually reuse values that occur somewhere in `/DocumentText`, but this does not make them direct offsets. Directly treating the LineMark tag index or byte offset as a `DocumentText` coordinate only hits 587 of 1536 rows, so `/LineMark` likely uses its own record coordinate system or layout-local payload fields.

`rjtd text-position-line-context <file>` compares `MarkV.01` header/entry offsets against `/LineMark` word positions and nearest LineMark tag rows. Only three current samples expose both readable `/LineMark` and parsed `MarkV.01`: `46.jtd`, `a5.jtd`, and `b6.jtd`.

| Sample | line words | line tags | Mark entries | Page rows | Paper rows | Mark header line index | LineMark word at header index | nearest tag rows |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `46.jtd` | 2597 | 491 | 8 | 97 | 97 | 1539 | `0x0029` | `0x1002@1520`, `0x1000@1564` |
| `a5.jtd` | 2561 | 511 | 8 | 75 | 75 | 1539 | `0x0016` | `0x1002@1534`, `0x1002@1542` |
| `b6.jtd` | 2667 | 502 | 8 | 98 | 98 | 1552 | `0x0000` | `0x1000@1548`, `0x1002@1554` |

All 24 `MarkV.01` entry offsets in those three samples are outside the `/LineMark` word range. Therefore the entries are not direct LineMark word indexes. The final `u16` in the Mark header remains a LineMark-adjacent candidate because it lands inside `/LineMark` near tag clusters, but it is not decoded and does not land on a stable tag value.

`rjtd text-position-count-layout-context <file>` extends the same direct-layout-coordinate check to `TCntV.01` tables. Across the 10 readable `TCntV.01` samples and 89 checked rows, the chosen range candidates produce 0 direct hits against `/LineMark` word offsets, `/LineMark` byte offsets, `/PageMark` rows/bytes, and `/PaperMark` rows/bytes. This means the current `TCntV.01` ranges also should not be treated as direct layout stream coordinates.

`rjtd text-position-count-fields <file>` exposes the remaining `TCntV.01` tail as positional `u16be` fields, and `rjtd text-position-count-field-deltas <file>` compares the chosen range span with the tail `t1..t2` span. In the current samples, all 89 rows have `t2 >= t1`, but no row has `t2 - t1` equal to the chosen range span. `rjtd text-position-count-tail-context <file>` also shows that `t1/t2` has a stronger `/DocumentText` UTF-16 unit hit pattern than byte hit pattern, though it is not universal. `rjtd text-position-count-tail-delta-scan <file>` raises unit hits most around delta 29/30, but text hits peak elsewhere. `rjtd text-position-count-tail-delta-groups <file>` then splits that aggregate signal into tail-pattern groups: one major `be0` pattern prefers `+29`, shifted patterns prefer `+30/+31`, and a major `0x0202` pattern is spread across many best deltas. `rjtd text-position-count-tail-row-deltas <file>` confirms the major `0x0202` pattern remains spread across many row-level best deltas. `rjtd text-position-count-tail-row-context <file>` shows that `0x0202` chosen ranges often touch later body byte ranges while best-delta tail fields usually touch early heading/date text. `rjtd text-position-count-range-preview <file>` then shows that `0x0202` chosen byte ranges often overlap real `/DocumentText` text entries even though they are not direct layout-stream coordinates. `rjtd text-position-count-range-boundaries <file>` adds that the major `0x0202` byte ranges mostly contain whole `/DocumentText` map entries and repeatedly include `0x001c`/`0x000e` controls. `rjtd text-control-context <file>` then shows `0x001c` as a high-frequency delimiter candidate and `0x000e` as more control-cluster or inline-adjacent. This keeps `TCntV.01` tail fields promising and makes `/DocumentText` control-delimited byte intervals the next target for chosen-range analysis, but rejects treating `t1/t2` as a simple duplicate of the chosen range.

## LineMark Header Word 0 Variation

The samples available in 2026-06-24 expose two `LineMark` header word 0 values:

| Value | Samples |
| --- | --- |
| `0x0914` | `46.jtd`, `a5.jtd`, `b6.jtd` (Ginga vertical samples, RFC 0007 original set) |
| `0x090b` | All 10 government-document local samples (`01要綱`, `02案文`, `03新旧`, `04参照`) |
| `0x0912` | `論文様式.jtd` (A4 horizontal academic paper template) |

All three values share the `0x0900` family prefix. The lower byte differs:
`0x14 = 20`, `0x0b = 11`, `0x12 = 18`. The difference between Ginga vertical
and A4 horizontal samples (`0x0914` vs `0x0912`) is 2; the difference between
Ginga and government documents (`0x0914` vs `0x090b`) is 9. The meaning of the
lower byte is not decoded.

## LineMark to DocumentText 0x001c Correlation

RFC 0009 established that every `0x001c` in `/DocumentText` is the opener of a
self-describing paragraph/layout record (class, length, payload, footer). The
`be16-delta-v1` LineMark profile emits one record per display line, while
`0x001c` marks logical paragraphs (which can wrap to multiple display lines).

In `論文様式.jtd` (25 LineMark records, 19 `0x001c` records), 14 of the 25
LineMark `unit-start` values fall exactly on a `0x001c` record position; the
remaining 11 fall inside text runs or at the `0x0000` document terminator.
LineMark records whose `flag=0x0000` tend to fall inside text runs without a
corresponding `0x001c`, which is consistent with continuation lines in a wrapped
paragraph.

In `03新旧（整備令）.jtd` (157 parsed LineMark records), the `0x001c` records
include table-cell class `0x0030` (703 records) and paragraph class `0x0010`
(151 records), reflecting the table-heavy structure of the new-vs-old comparison
document. The LineMark delta values are correspondingly smaller and more varied
compared to the simple paragraph-only `論文様式.jtd`.

## PageLayoutStyle Observation

Seven local government/academic samples expose `/PageLayoutStyle` and
`/PageLayoutStyleHeader` streams in addition to `/LineMark`/`/PageMark`/`/PaperMark`.

**`/PageLayoutStyle` structure (initial inventory):** Starts with `SsmgV.01` magic (8
bytes). The header words are mostly zero except for a small region around offsets
`w5=0x0004` or `w5=0x0005`, `w7=0x0100`, `w9`=entry-count-like value,
`w10=0x0001`, `w11=0x0002`. The first significant cluster of non-zero words starts
at word 138. Across all seven samples, the value `0x4001` appears at word 155 or 156,
followed immediately by `0x010d=269`. This `269` is one more than `0x010c=268`, which
equals the maximum cell `b1` coordinate (table width) in `03新旧（整備令）.jtd`. The
same value `269` appears in samples with no tables (`01要綱`, `02案文` families), so
it more likely encodes a page-level layout parameter (candidate: text-area width in the
same coordinate units as `/DocumentText` `0x0030` cell coordinates). In `03新旧`,
`0x4001/269` appears twice (words 155 and 792), suggesting section-level repetition.

**`/PageLayoutStyleHeader` structure (initial inventory):** Starts with `SsmgV.01`
magic. The first section (words 10–13) contains `TextV.01` magic, word 138 contains
`TCntV.01` magic, and word 266 contains another `TextV.01`. These recurring magic
strings suggest `/PageLayoutStyleHeader` is a composite container of `TextV.01` and
`TCntV.01` sub-records interleaved in a flat byte stream. Words 299, 300, 302–306
contain `0x001c`/`0x001f`/`0x001d`/`0x001e` patterns matching the `/DocumentText`
inline opener/terminator sequence, suggesting that style block payloads may contain
embedded text runs. The `0x0198=408` value repeats at words 279, 341, 440 (and
corresponding positions in other sections); `0x07dd=2013` appears at words 285 and
447. Physical meaning of all fields is not decoded.

## Known Gaps

- No `LineMark` record parser exists yet.
- `/PageMark` has raw-preserving parsers for fixed 84-byte rows, fixed 84-byte rows with preserved trailing bytes, count-plus-one variable-row families, and count-variable rows, but some local `/PageMark` streams still use unsupported variants.
- `/PaperMark` has an initial row parser, but no semantic model mapping exists yet.
- The count-like header values are not proven to be entry counts.
- The relationship between these streams and `MarkV.01` / `TCntV.01` is not decoded; current evidence rejects direct Mark-entry-to-LineMark-word indexing and direct `TCntV.01` range-to-layout-stream offsets.
- Small malformed samples can expose inventory entries whose mini-stream chains are not safely readable; `stream-meta` should be used before interpreting small layout streams.
- `/LineMark` has tag-like values (`0x1000`, `0x1001`, `0x1002`) whose semantics are unknown; current tag-context evidence does not make the immediate next word a unique family discriminator or prove direct `DocumentText` coordinates.
- The semantic meaning of the LineMark header word 0 (`0x0914` / `0x090b` / `0x0912`) is not decoded; it may encode document type, writing direction, or another document-level property.

## Next Steps

- Keep `/PaperMark` as a parser-backed diagnostic until the header and flag semantics are decoded.
- Decode the `SO` object/control record family field semantics; current evidence suggests fields 1-4 carry geometry-like tuples in singleton records while repeated records carry default/control constants.
- Compare `/PageMark` and `/PaperMark` count-like values with rendered page or paper counts in known samples.
- Compare `/PaperMark` flags with page breaks, paper sections, and visible layout changes.
- Decode the `0x1000`, `0x1001`, and `0x1002` tag families inside `/LineMark` by comparing tag offsets and contexts with text and layout boundaries.
- Decode why the final Mark header `u16` lands inside `/LineMark` near tag clusters while the Mark entries do not.
- Decode the actual target for `TCntV.01` ranges after direct `/LineMark`, `/PageMark`, and `/PaperMark` coordinates were rejected; use `text-position-count-fields`, `text-position-count-field-deltas`, `text-position-count-tail-context`, `text-position-count-tail-delta-scan`, `text-position-count-tail-delta-groups`, `text-position-count-tail-row-deltas`, `text-position-count-tail-row-context`, `text-position-count-range-preview`, `text-position-count-range-boundaries`, and `text-control-context` to compare tail field patterns and chosen `/DocumentText` byte-range/control overlap.
