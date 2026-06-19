# TODO

## M1: Container Explorer

Goal: implement `rjtd streams <file.jtd>` as the first executable milestone.

Status: implemented for CFB entry inventory, including a rhwp-style lenient fallback for malformed FAT files.

Why this comes first:

- rjtd must follow rhwp's layered parsing order.
- rhwp starts binary parsing from the CFB container layer before header, record, body text, model, renderer, or exporter work.
- JTD structure should be observed and documented before text extraction is attempted.
- Exporters must never read raw file data directly; all later work depends on stable lower layers.

Immediate tasks:

- [x] Keep local `.jtd`, `.jtt`, and `.jttc` samples under `rjtd-testdata/local-samples/`.
- [x] Write tests before implementation.
- [x] Add a minimal CFB container reader to `rjtd-core`.
- [x] Expose stream/storage entries with normalized paths, byte sizes, and entry kind.
- [x] Add a rhwp-style lenient CFB fallback for malformed FAT samples.
- [x] Implement `rjtd streams <file.jtd>` in `rjtd-cli`.
- [x] Implement `rjtd info <file>` as a rhwp-style lightweight inventory command.
- [x] Record early observations in `openjtd-spec`.

Acceptance criteria:

- [x] Unit tests create a tiny synthetic CFB file and prove stream listing works.
- [x] CLI integration tests run `rjtd streams` against a synthetic CFB file.
- [x] Local sample files can be inspected manually with `cargo run -p rjtd-cli -- streams ../rjtd-testdata/local-samples/<name>.jtd`.
- [x] `cargo fmt --all --check`, `cargo check --workspace`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` pass from `rjtd/`.

## M2: Text Extraction

Goal: implement `rjtd cat <file.jtd>` after M1 reveals the relevant stream layout.

Do not start M2 until M1 has produced documented stream observations for the local samples.

Status: structured `ParsedDocumentText` token layer implemented for observed text runs and display inline segments. Observed `.jttc` samples are opened through the `/JSCompDocument` `JustCompressedDocument` wrapper. Embedded `SsmgV.01` fragments are recovered for the current no-`/DocumentText` samples.

Priority prework:

- [x] Download the historical Ichitaro Document Filter `.oxt`.
- [x] Unpack the `.oxt` and record its file tree.
- [x] Confirm whether it contains Java, native code, or OpenOffice registry files.
- [x] Record filter/type registration metadata.
- [x] Search for stream and conversion strings without decompiling the DLL.
- [x] Investigate `/DocumentText` bytes in local samples against the stream names confirmed by the filter.
- [x] Add a read-only `rjtd dump-stream <file.jtd> <path>` helper or test utility if needed.
- [x] Implement initial `rjtd cat <file.jtd>` using `DocumentText` text-run markers.
- [x] Recover common display inline segments wrapped by `0x001D ... 0x001E`, including ruby base text and template placeholders.
- [x] Skip known phonetic annotation and template instruction inline segments in plain `cat` output.
- [x] Detect `.jttc` samples that store `/JSCompDocument` as `JustCompressedDocument`.
- [x] Decode observed `.jttc` `/JSCompDocument` `-lh5-` payloads without adding an LHA dependency.
- [x] Report invalid or unsupported compressed payloads clearly.
- [x] Confirm local sample inventory opens after malformed FAT fallback.
- [x] Recover embedded `SsmgV.01`/`TextV.01` fragments when `/DocumentText` is absent.
- [x] Confirm `rjtd cat` succeeds on all 61 current local samples.
- [x] Add `rjtd text-tokens <file>` to inspect structured `ParsedDocumentText` output.
- [x] Add `rjtd text-control-context <file> [control-code]` to inspect `/DocumentText` control boundaries with neighboring map entries and nearest control distances.
- [x] Replace the plain string extractor path with a structured `ParsedDocumentText` parser layer.
- [x] Preserve skipped inline text segments as structured tokens and document-model unknown objects.
- [x] Make CLI stdout writes tolerate broken pipes for token/stream inspection pipelines.
- [x] Add `rjtd text-positions <file>` for initial `/DocumentTextPositionTables` `MarkV.01` diagnostics.
- [x] Add `rjtd text-map <file>` to map `/DocumentText` tokens to byte and UTF-16 unit ranges.
- [x] Add `rjtd text-position-context <file>` to compare `MarkV.01` offsets against byte, unit, and provisional `unit + 29` token contexts.
- [x] Add `rjtd text-position-delta-scan <file>` to score MarkV.01 UTF-16 unit delta candidates from 0 through 64.
- [x] Add `rjtd text-position-mark-header <file>` to expose the raw six bytes between `MarkV.01` and the first entry plus raw `(id, offset)` entries.
- [x] Add `rjtd text-position-mark-summary <file>` to correlate the MarkV.01 header with `/DocumentText`, `/LineMark`, `/PageMark`, and `/PaperMark` metrics.
- [x] Add `rjtd text-position-line-context <file>` to compare the MarkV.01 header and entry offsets against `/LineMark` word contexts and nearest tag rows.
- [x] Record `/DocumentTextPositionTables` observations in `openjtd-spec`.
- [x] Record the current hypothesis that `MarkV.01` offsets are closer to UTF-16 unit/internal coordinates than raw byte or extracted-character positions.
- [x] Sweep `text-position-context` output across the 5 current samples with parsed `MarkV.01` entries and record the initial id classification.
- [x] Sweep `text-position-delta-scan` across the 5 current MarkV.01 samples: 40 entries total; `delta 9`, `delta 29`, and `delta 30` tie at 31 visible text hits, while `delta 9` has more unit hits than `delta 29`.
- [x] Sweep `text-position-mark-header` across the 5 current MarkV.01 samples: marker offset is consistently 30, header prefix is `00000000`, and the last big-endian `u16` is `0x0603`, `0x0610`, or `0x061c`, so the header does not directly encode a constant 29-unit adjustment.
- [x] Sweep `text-position-mark-summary` across the 5 current MarkV.01 samples: `0x0603` appears with different page/paper counts, `0x061c` appears in samples without `/LineMark`/`/PageMark`/`/PaperMark`, and no direct document-length or page-count meaning is proven.
- [x] Sweep `text-position-line-context` across the 61 current local samples: only `46.jtd`, `a5.jtd`, and `b6.jtd` have both readable `/LineMark` and parsed `MarkV.01`; all 24 MarkV.01 entry offsets are outside the `/LineMark` word range, while the MarkV.01 header's final `u16` lands inside `/LineMark`.
- [x] Search the four-byte MarkV.01 header values `00000603`, `00000610`, and `0000061c` across representative MarkV samples and confirm the exact four-byte values only appear in `/DocumentTextPositionTables`, not in the observed layout/style streams.
- [x] Add `rjtd text-position-counts <file>` for observed non-Mark `TCntV.01` numeric tables.
- [x] Classify the 11 non-Mark position-table samples as 10 `TCntV.01` tables plus 1 empty/unreadable position-table payload despite a non-zero inventory size.
- [x] Add `rjtd text-position-count-context <file>` to compare the first two `TCntV.01` fields against `/DocumentText` byte and UTF-16 unit token ranges.
- [x] Sweep `text-position-count-context` across the 10 current `TCntV.01` samples and record that the coordinate behavior is mixed.
- [x] Add `rjtd text-position-count-tail-context <file>` to compare tail `t1/t2` fields against `/DocumentText` byte and UTF-16 unit token ranges.
- [x] Sweep `text-position-count-tail-context` across current local samples: 10 readable `TCntV.01` files, 89 rows, any byte hit 21, both byte hit 5, any unit hit 49, both unit hit 28, and both unit text hit 26; this makes UTF-16 unit coordinates a stronger but still incomplete signal for `t1/t2`.
- [x] Add `rjtd text-position-count-tail-delta-scan <file>` to scan positive `0..64` UTF-16 unit deltas over tail `t1/t2` endpoints.
- [x] Sweep `text-position-count-tail-delta-scan` across current local samples: 10 readable `TCntV.01` files, 89 rows, 178 endpoints; delta 29 and 30 tie for best unit endpoint hits at 124, delta 29 has better text endpoint hits and both-unit rows than 30, while text endpoint hits peak at delta 53 with 102, so no single offset is proven.
- [x] Add `rjtd text-position-count-tail-delta-groups <file>` to summarize tail delta scores by `(family,t0,t3,t4,t7)` pattern.
- [x] Sweep `text-position-count-tail-delta-groups` across current local samples: the 28-row `be0/0x0101/0x0100/0x0001/0x0001` group prefers delta 29 for both unit and text; the 16-row shifted `0x0101/0x0100/0x0001/0x0001` group prefers unit delta 31 and text delta 30; the 28-row `be0/0x0202/0x0100/0x0000/0x0001` group is spread across many best deltas, so a single global adjustment is unlikely.
- [x] Add `rjtd text-position-count-tail-row-deltas <file>` to expose per-row best unit/text deltas, chosen spans, tail spans, and document byte/unit lengths.
- [x] Sweep `text-position-count-tail-row-deltas` across current local samples: the major `be0/0x0202/0x0100/0x0000/0x0001` group has 28 rows across 8 files, chosen spans 398..1212, tail spans 46..72, document unit lengths 65889..146657, and 17 different row-level best unit deltas; this looks like row-local structure rather than one file-level or global correction.
- [x] Add `rjtd text-position-count-tail-row-context <file>` to place chosen start/end byte/unit contexts and best-delta tail contexts on the same row.
- [x] Sweep `text-position-count-tail-row-context` across current local samples: all `0x0202` rows total 36 rows across 10 files; in the major 28-row `be0/0x0202/0x0100/0x0000/0x0001` group, chosen start/end are often byte-context body hits or boundaries while unit-context is mostly `between`, and best-delta tail `t2` lands on text in 27/28 rows. This supports treating chosen range and tail fields as different roles rather than duplicate text coordinates.
- [x] Add `rjtd text-position-count-range-preview <file>` to summarize `/DocumentText` entries overlapped by each chosen `TCntV.01` range as byte and UTF-16 unit intervals.
- [x] Sweep `text-position-count-range-preview` across current local samples: 61 checked, 10 readable `TCntV.01` files, 89 rows; all `t0=0x0202` rows total 36 rows, and 31/36 have chosen byte ranges overlapping text. The major `be0/0x0202/0x0100/0x0000/0x0001` group has 25/28 chosen byte ranges overlapping text and 21/28 chosen unit ranges overlapping text.
- [x] Add `rjtd text-position-count-range-boundaries <file>` to expose chosen range edge alignment, first/last/previous/next `/DocumentText` entries, and overlapped control-code counts.
- [x] Sweep `text-position-count-range-boundaries` across current local samples: in the major 28-row `be0/0x0202/0x0100/0x0000/0x0001` group, chosen byte ranges overlap 535 map entries total, 513 fully contained and 22 partial; 25/28 rows include controls, dominated by `0x001c` 281 times and `0x000e` 38 times. This makes `/DocumentText` control-delimited range structure the next target.
- [x] Sweep `text-control-context` across current local samples: 61 checked, 0 errors, 60 files with controls. `0x001c` appears 51,971 times in 60 files and most often separates text/text (16,717 rows), text/control (10,329), control/text (7,191), or control/control (6,561). `0x000e` appears 6,621 times in 41 files and most often appears in control/control (3,338), text/control (1,832), or text/skipped-inline (844) contexts.
- [x] Add `rjtd text-position-count-control-ranges <file> [control-code]` to compare chosen `TCntV.01` ranges against `/DocumentText` intervals split by a selected control delimiter.
- [x] Sweep `text-position-count-control-ranges` across the 10 readable `TCntV.01` files: 89 rows all map into some interval for both `0x001c` and `0x000e`, but `0x001c` produces many more overlaps (462 byte-basis intervals, 794 unit-basis intervals; 40 byte and 37 unit multi-interval rows) than `0x000e` (135 byte, 195 unit; 25 byte and 32 unit multi-interval rows). This makes direct `0x001c => paragraph` promotion too risky.
- [x] Expose candidate control-delimited overlap summaries on decoded-false `textCountRanges` model/export/app-core JSON as `controlRangeOverlaps` without promoting them to paragraph semantics.
- [x] Expose derived decoded-false `textBoundaryCandidates` model/export/app-core JSON from `controlRangeOverlaps` as diagnostic paragraph-boundary evidence without changing parsed paragraph semantics.
- [x] Add `rjtd text-boundary-candidates <file>` to print model-derived decoded-false boundary candidates with basis, delimiter, interval count, single/multi classification, source span, and `decoded=false`.
- [x] Sweep `text-boundary-candidates` across current local samples: 61 checked, 10 files with candidates, 356 candidate rows, 1,586 overlapped intervals, 222 single-interval candidates, and 134 multi-interval candidates. The largest single candidate spans 44 intervals (`0x001c`/unit) in `justsystems-20120223023609-jp-just-finance-j200403sc.jtd`, so paragraph promotion still needs a stricter rule.
- [x] Add `rjtd text-boundary-candidate-context <file>` to compare decoded-false boundary candidates against `/DocumentText` visible text, line breaks, and edge alignment.
- [x] Sweep `text-boundary-candidate-context` across current local samples: 356 candidate rows, 276 rows with at least one line break, 3,458 total line breaks, and 210 rows that start after a control gap and end on an aligned text boundary. Among `0x001c` single-interval edge-good rows, byte basis has 17 one-line-break and 16 zero-line-break rows, while unit basis has 22 one-line-break and 13 zero-line-break rows; `0x000e` often spans many line breaks and remains too coarse for direct paragraph promotion.
- [x] Add `rjtd text-boundary-candidate-agreement <file>` to pair byte/unit decoded-false boundary candidates by text-count range and delimiter, reporting edge-good flags, line-break counts, visible text previews, and match flags.
- [x] Sweep `text-boundary-candidate-agreement` across current local samples: 178 byte/unit pairs across 10 files. Exact visible-text match appears only once and that row is empty, so text equality is not a useful promotion rule. In the stricter `0x001c` single/single set, 43 pairs exist; unit-basis edge-good/non-empty/line-break<=1 keeps 33 rows, while byte-basis keeps 28 rows.
- [x] Add `rjtd text-boundary-candidate-layout-context <file>` to compare unit-basis `0x001c` single candidates with `/LineMark`, `/PageMark`, and `/PaperMark` direct index/byte contexts while keeping the rule diagnostic-only.
- [x] Sweep `text-boundary-candidate-layout-context` across current local samples: 52 unit `0x001c` single candidates across 8 files, 35 strict rule-selected rows, but 0 selected rows have start/end direct hits in `/LineMark`, `/PageMark`, or `/PaperMark`. This means candidate source units and layout mark rows are not the same coordinate space.
- [x] Add `rjtd text-boundary-layout-map <file>` to score unit-basis `0x001c` candidates against sparse `/LineMark` tag positions, `/PageMark` entry indexes/raw fields/byte boundaries, and `/PaperMark` entry indexes/byte boundaries under several global unit transform hypotheses.
- [x] Sweep `text-boundary-layout-map` across current local samples: 61 checked, 0 failures, 52 unit `0x001c` single candidates across 8 files, and 35 strict selected candidates across 4 files. Non-boundary exact hits exist, but the winning target/base/delta combinations are file-specific (`iwata_file` favors line-word-value/page-be32-field with unit-div2 shifts around -1140..-1192; finance samples favor different page-be32-field shifts). This rejects a single global layout-map transform for paragraph promotion.
- [x] Add `rjtd text-boundary-layout-map-rows <file>` to score each unit `0x001c` single candidate separately and print its linked `TCntV.01` row, local delta, nearest start/end layout points, and exact endpoint count.
- [x] Sweep `text-boundary-layout-map-rows` across current local samples: 61 checked, 0 failures, the same 52 unit `0x001c` single candidates and 35 strict selected candidates. The 32 strict selected candidates in `iwata_file` include 10 candidates with row-local `exact=2` evidence through both `line-word-value` and `page-be32-field`; the 3 strict selected finance candidates have no row-local `exact=2` evidence. This keeps strict candidates diagnostic-only and suggests the next rule must separate paragraph-like rows from non-paragraph large spans.
- [x] Add `rjtd text-boundary-paragraph-like <file>` as a diagnostic-only classifier requiring strict unit `0x001c` single candidates plus both `line-word-value` and `page-be32-field` row-local exact endpoint evidence before reporting `paragraph-like=true`.
- [x] Sweep `text-boundary-paragraph-like` across current local samples: 61 checked, 0 failures, 52 unit `0x001c` single candidates, 35 strict selected candidates, 10 paragraph-like candidates, and 25 strict selected but non-paragraph-like candidates. Only `iwata_file` produces paragraph-like candidates under this rule.
- [x] Add `rjtd text-boundary-paragraph-like-style-context <file>` to print the paragraph-like classifier together with linked `TCntV.01` tail fields, text/page style hits, view-style group hits, and byte/unit range previews.
- [x] Sweep `text-boundary-paragraph-like-style-context` across current local samples: 61 checked, 0 failures, the same 52 unit candidates, 35 strict selected candidates, 10 paragraph-like candidates, and 25 selected non-paragraph-like candidates. The 10 paragraph-like rows have no text/page style candidate hits, but all 10 hit `/DocumentViewStyles` group evidence in `iwata_file`; strict non-paragraph rows also have view-group hits (25/25), so this is not a paragraph discriminator and does not prove paragraph style attachment.
- [x] Add `rjtd text-boundary-paragraph-like-discriminators <file>` to summarize paragraph-like, strict-non-paragraph, and non-strict buckets by exact layout evidence, `TCntV.01` family/span, tail field counts, and style/view-style hits.
- [x] Sweep `text-boundary-paragraph-like-discriminators` across current local samples: 61 checked, 0 failures. Only paragraph-like rows have dual exact layout evidence (10/10); strict-non-paragraph and non-strict rows have 0/25 and 0/17. In `iwata_file`, paragraph-like rows are `be0` with nonzero `range-spans=2..8`, while strict-non-paragraph and non-strict rows have `range-spans=0..0`. This makes nonzero chosen `TCntV.01` span plus dual row-local layout exactness the next discriminator to test, still decoded-false.
- [x] Preserve that stricter discriminator as decoded-false `textParagraphBoundaryCandidates` in model/export/app-core JSON without rebuilding real paragraphs. A 61-sample JSON export sweep has 0 failures and preserves 10 candidates total, all in `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd`.
- [x] Add `rjtd text-paragraph-boundary-targets <file>` to trace preserved `textParagraphBoundaryCandidates` back to concrete `/LineMark` word indexes and `/PageMark` raw row fields.
- [x] Sweep `text-paragraph-boundary-targets` across current local samples: 61 checked, 0 failures, 1 file with candidates, 10 total candidates. Among the 20 candidate endpoints, 6 line endpoints and 4 page endpoints are non-unique or missing under the current hit formatter, so exactness alone is not sufficient for real paragraph construction.
- [x] Add `rjtd text-position-count-clusters <file>` to group duplicate `TCntV.01` ranges and expose raw-tail variants.
- [x] Add `rjtd text-position-count-candidates <file>` to compare `be32@0/4` and shifted `be32@1/5` offset candidates.
- [x] Confirm 9 current `TCntV.01` samples fit `be32@0/4`, while `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` splits into 32 `be32@0/4`-plausible entries and 18 shifted `be32@1/5`-plausible entries.
- [x] Add `rjtd text-position-count-family <file>` to classify `TCntV.01` entries as `be0` or `be1-shifted` while preserving both candidate offsets and the raw tail.
- [x] Sweep `text-position-count-family` across current `TCntV.01` samples: 10 files, 89 records total, 71 `be0`, 18 `be1-shifted`; all shifted entries are `iwata_file` entries 32-49.
- [x] Add `rjtd text-position-count-fields <file>` to expand each `TCntV.01` record into the chosen range plus tail `u16be` fields without assigning final semantics.
- [x] Sweep `text-position-count-fields` across current `TCntV.01` samples: `be0` records have 10 tail `u16be` fields plus extra byte `00`; `be1-shifted` records have exactly 10 tail `u16be` fields and no extra byte.
- [x] Record current tail-field invariants: shifted records have fixed `t3=0x0100`, `t4=0x0001`, `t5=0x0000`, `t6=0x0000`, `t7=0x0001`, `t8=0x0000`, `t9=0x0000`; `be0` records share several mostly fixed fields but retain more variation.
- [x] Add `rjtd text-position-count-field-deltas <file>` to compare the chosen `TCntV.01` range span against the tail `t1..t2` span plus signed deltas.
- [x] Sweep `text-position-count-field-deltas` across current local samples: 10 readable `TCntV.01` files, 89 rows, all rows have `t2 >= t1`, but no row has `t2 - t1` equal to the chosen range span; relation counts are `be0` `gt` 27 / `lt` 44 and `be1-shifted` `gt` 15 / `lt` 3.
- [x] Add `rjtd text-position-count-layout-context <file>` to compare chosen `TCntV.01` ranges against `/LineMark` word/byte offsets plus parsed `/PageMark` and `/PaperMark` row/byte offsets.
- [x] Sweep `text-position-count-layout-context` across current local samples: the 10 readable `TCntV.01` files expose 89 rows and all chosen start/end values are outside direct `/LineMark`, `/PageMark`, and `/PaperMark` ranges under word, row, and byte interpretations.
- [x] Add `rjtd stream-meta <file> <stream-path>` to inspect CFB regular/mini stream placement.
- [x] Confirm the one empty `/DocumentTextPositionTables` payload has mini-sector start `224`, which points beyond the observed 7680-byte mini stream.
- [x] Record initial `/LineMark`, `/PageMark`, and `/PaperMark` observations in `openjtd-spec`.
- [x] Add generic `rjtd stream-words <file> <stream-path>` and `rjtd stream-word-frequencies <file> <stream-path>` diagnostics.
- [x] Compare `/LineMark` raw words against `/DocumentText` raw words and record that `0x1000`, `0x1001`, and `0x1002` look like LineMark-specific tags.
- [x] Add `rjtd line-mark-tags <file>` to print `/LineMark` `0x1000`/`0x1001`/`0x1002` tag positions with surrounding word context.
- [x] Sweep `line-mark-tags` across the 61 current local samples: 5 files contain these tags, 6 files lack `/LineMark`, 50 readable `/LineMark` streams have no such tags, and the tag totals are `0x1000` 915, `0x1001` 67, `0x1002` 554.
- [x] Confirm the first word after each `/LineMark` tag is not a unique tag-family discriminator: high-frequency next words overlap strongly across `0x1000`, `0x1001`, and `0x1002`.
- [x] Add `rjtd line-mark-text-context <file>` to compare `/LineMark` tag offsets and immediate next words against the `/DocumentText` token map.
- [x] Sweep `line-mark-text-context` across the 61 current local samples: 55 files run successfully, 6 lack `/LineMark`, all 1536 known tag rows are reported, direct LineMark byte/unit offsets hit mapped `DocumentText` entries for 587 rows, and tag-next words appear in raw `/DocumentText` for 1511 rows.
- [x] Record that direct `/LineMark` word/byte offsets are not proven `DocumentText` coordinates even though tag-next words usually occur somewhere in raw `/DocumentText`.
- [x] Add generic `rjtd stream-dwords <file> <stream-path>` and `rjtd stream-dword-frequencies <file> <stream-path>` diagnostics for u32-oriented streams.
- [x] Prove the observed `/PaperMark` row shape as a 12-byte header followed by 8-byte `(index, flags)` rows in the current large layout samples.
- [x] Add parser-backed `rjtd paper-marks <file>` diagnostics while keeping `/PaperMark` out of the document model until semantics are decoded.
- [x] Sweep `paper-marks` across local samples and record that 52 of 55 `/PaperMark` streams parse with the observed row shape.
- [x] Add parser-backed `rjtd page-marks <file>` diagnostics for the observed 84-byte `/PageMark` row family while preserving each row as raw bytes.
- [x] Sweep `page-marks` across local samples and record that 20 of 55 `/PageMark` streams match the observed 84-byte row family.
- [x] Add `rjtd page-mark-shape <file>` to classify `/PageMark` stream length, declared CFB size, header values, and row-shape candidates.
- [x] Identify initial `/PageMark` reject families: count-plus-one variable rows, count-plus-one with 2-byte tail/trimming, declared-size mismatch regular streams, and non-PageMark-looking payloads.
- [x] Promote count-plus-one variable and count-plus-one-trim2 `/PageMark` families into the parser as raw-preserved row families.
- [x] Confirm parser-backed `page-marks` opened 43 of 55 current `/PageMark` streams after count-plus-one family promotion.
- [x] Promote count-variable `/PageMark` family into the parser as a raw-preserved row family.
- [x] Confirm parser-backed `page-marks` now opens 45 of 55 current `/PageMark` streams.
- [x] Promote fixed84-tail `/PageMark` family into the parser with preserved trailing bytes.
- [x] Confirm parser-backed `page-marks` now opens 52 of 55 current `/PageMark` streams.
- [x] Add `rjtd stream-text-probe <file> <stream-path>` to detect ASCII/UTF-16 string candidates in suspicious raw streams.
- [x] Confirm the remaining 3 unsupported `/PageMark` payloads contain stream/object names or legacy class/control metadata rather than numeric PageMark rows.
- [x] Add `rjtd paper-mark-shape <file>` to classify `/PaperMark` stream length, declared CFB size, header values, and fixed-row candidates.
- [x] Confirm `paper-mark-shape` opens all 55 current `/PaperMark` streams and separates the 3 unsupported payloads as `non-paper-header`.
- [x] Add `rjtd stream-chain <file> <stream-path>` to inspect FAT/miniFAT sector chains for suspicious stream entries.
- [x] Confirm the remaining 3 unsupported `/PageMark` and `/PaperMark` entries have complete miniFAT chains whose payload bytes decode as CFB directory-entry fragments, OLE/ActiveX control metadata, or unrelated stream names rather than layout rows.
- [x] Add `rjtd cfb-map <file>` to inspect FAT, directory, mini FAT, and root mini-stream sector chains.
- [x] Explain the CFB directory-entry fragments in `kaisya_annai` and `shanai_lan`: their root mini-stream chains overlap the CFB directory chains, so `/PageMark` and `/PaperMark` can read directory-entry bytes through structurally complete miniFAT chains.
- [x] Add `rjtd stream-find <file> <stream-path>` to search for exact duplicate stream payloads inside the same CFB file.
- [x] Trace `kazoku_ryoko` `/PageMark` to an exact slice of `/EmbedItems/Embedding 1/JSFart2Contents` at offset 1664.
- [x] Add `rjtd cfb-dir <file>` to inspect raw CFB directory ids, sibling links, start sectors, and resolved paths.
- [x] Trace `kazoku_ryoko` `/PaperMark` to the root-level object/control stream sequence around `/EmbedFrame` and `/Figure`; its `SO` marker also appears in `/Figure` and `\x01CompObj`, and its first nonzero fields match `/EmbedItems/Embedding 2/JSFart2Contents` offset 192.
- [x] Add `rjtd stream-find-bytes <file> <hex>` to search arbitrary byte markers across all readable streams.
- [x] Reproduce the `kazoku_ryoko` `/PaperMark` object/control evidence with `stream-find-bytes`: `534f0000` appears in `/PaperMark`, `/Figure`, and `\x01CompObj`; the 20-byte coordinate-like suffix appears in `/PaperMark` and `/EmbedItems/Embedding 2/JSFart2Contents`.
- [x] Add `rjtd so-records <file>` to scan all readable streams for the `SO\0\0` object/control marker and print preserved little-endian fields plus raw bytes.
- [x] Sweep `so-records` across local samples: 4 of 61 samples expose SO records, 24 records total, and only `kazoku_ryoko` exposes an SO record through `/PaperMark`.
- [x] Add `rjtd so-record-clusters <file>` to group SO records by preserved raw bytes and report repeated locations.
- [x] Confirm `JSFart2Contents` SO records split into singleton geometry-like records and repeated default/control clusters; `kazoku_ryoko` `/PaperMark` matches the geometry-like cluster seen in the older `kazoku_ryoko` `JSFart2Contents` sample.
- [x] Add `rjtd so-record-fields <file>` to expand SO records into little-endian 32-bit fields plus signed and low/high 16-bit views.
- [x] Confirm current SO records are best treated as 9 little-endian dwords: field 0 is marker `0x00004f53`, repeated default/control records use small constants such as `0x00000100` and `0x00000064`, and singleton geometry-like records carry coordinate-like values in fields 1-4.
- [x] Add `rjtd so-record-geometry <file>` to classify SO records as `geometry-like`, `default-control`, packed subfamilies, `truncated`, or `unknown` without assigning final semantics.
- [x] Add `rjtd so-record-halves <file>` to print SO payload dwords as low/high 16-bit unsigned and signed halves.
- [x] Sweep `so-record-geometry` across local samples: 61 checked, 0 failures, 4 files with SO records, 24 records total (`geometry-like` 9, `default-control` 8, `packed-jseq3-like` 4, `packed-ffff-preamble` 2, `truncated` 1).
- [x] Split the previous generic `packed` SO bucket: `packed-jseq3-like` appears only in `JSEQ3Contents` records, while `packed-ffff-preamble` appears only as the `JSFart2Contents` offset-324 preamble before repeated geometry-like records.
- [x] Add `rjtd object-stream-candidates <file>` to classify readable CFB streams with object/image/shape/table path evidence, `SO\0\0` markers, binary image signatures, SVG text signatures, payload prefixes, and `decoded=false`.
- [x] Sweep `object-stream-candidates` across local samples: 61 checked, 43 files with candidates, 933 candidate rows across 2,253 readable streams, 27 files with object-path evidence, 42 with shape-path evidence, 17 with image-signature evidence, 4 with SO-marker evidence, 0 with SVG signatures, 0 with table-path evidence, and 0 unreadable streams.
- [x] Record the object/image stream inventory in `openjtd-spec` RFC 0008, including the conclusion that embedded JPEG recovery can start from `/EmbedItems/Embedding */Contents` while the `hyo` table sample likely requires `/DocumentText` control/layout decoding rather than named table streams.
- [x] Promote object/image/shape stream candidate evidence into model/export/app-core decoded-false `objectStreamCandidates` before wiring any non-text PDF rendering.
- [x] Sweep JSON export for model-preserved `objectStreamCandidates`: 61 checked, 0 failures, 43 positive files, 933 candidates total, 17 image-signature files, 4 SO-marker files, 0 table-path files, and 0 SVG-signature files.
- [x] Extract complete embedded image payload spans from image-signature object candidates and preserve the bytes in decoded-false `objectStreamCandidates.imagePayloads` without decoding object geometry yet.
- [x] Sweep JSON export for model-preserved `imagePayloads`: 61 checked, 0 failures, 12 payload files, 67 strict complete payloads, 35 dimensioned payloads, 629,024 bytes, JPEG 35 rows/5 files, GIF89a 31 rows/9 files, GIF87a 1 row/1 file.
- [x] Preserve decoded-false image payload object envelopes with header/trailer slices and conservative `le32` declared payload-length candidates.
- [x] Sweep image payload envelopes: 61 checked, 0 failures, 67 envelopes, 20 `le32` declared length matches, all currently in `Embedding */Contents` rows.
- [x] Preserve decoded-false image envelope header field candidates: first prefix `u16/u32` values and source path candidates from `Embedding */Contents` headers.
- [x] Sweep image envelope header fields: 61 checked, 0 failures, 66 source path candidates, all in `Contents` rows (`jpg` 34, `gif` 32); first prefix word `9` appears in 59 payload rows and `4` in 6 rows.
- [x] Preserve decoded-false path-derived `ownershipCandidate` evidence for object stream candidates, including `Embedding N`, stream role, and figure/frame/layout stream families.
- [x] Sweep ownership candidates: 61 checked, 0 failures, 933 candidates, 474 with ownership, and all 67 strict image payload rows covered by an ownership candidate (`contents` 67).
- [x] Preserve decoded-false `ownershipReferences` evidence by matching `Embedding N` byte patterns from embedded image candidates into `FigureData`, `/Figure`, `/Frame`, `/LayoutBox`, `/PageMark`, and `/PaperMark` streams.
- [x] Sweep ownership references: 61 checked, 0 failures, 12 files with references, 52 embedded image candidates with references, 604 reference rows, 9,949 total byte matches, and 67 of 67 strict image payload rows covered by cross-stream reference candidates.
- [x] Add `rjtd object-ownership-references <file>` to print model-owned ownership reference matches with source stream, target stream, encoding, offset, total match count, alignment, local context hex, and le/be 16/32-bit values at the match offset.
- [x] Sweep `object-ownership-references`: 61 checked, 0 failures, 12 files with rows, 3,167 reported preview offsets; target families include `figure-data` 1,010, `frame` 897, `layout-box` 528, `page-mark` 498, `paper-mark` 151, and `figure` 83.
- [x] Record reference-context alignment evidence: reported preview offsets split as mod2 `0` 1,401 / `1` 1,872 and mod4 `0` 728 / `1` 855 / `2` 673 / `3` 1,017; `u16-be`/`u16-le` offsets expose the embedding index directly as be/le16 at the match offset, while `u32-le` exposes it in the low 16 bits.
- [x] Add `rjtd object-ownership-reference-fields <file>` to project ownership reference offsets onto candidate record strides and summarize target, encoding, stride, field offset, row indexes, source count, embedding indexes, and cross-row matches.
- [x] Sweep `object-ownership-reference-fields`: 61 checked, 0 failures, 12 files with field groups, 33,492 projected field groups. This is a projection surface, not decoded geometry; every reported offset is tested against 20 candidate strides.
- [x] Record the strongest cross-row-free stride candidates with stride >= 12: `frame/u16-le/12/5` has 102 weighted matches, `frame/u16-be/12/7` has 89, and `frame/u16-be/20/15` has 70.
- [x] Add `rjtd object-frame-reference-records <file>` to expand the strongest `/Frame` reference projections into candidate row bytes with row hex, BE/LE 16-bit fields, and BE/LE 32-bit fields.
- [x] Sweep `object-frame-reference-records`: 61 checked, 0 failures, 12 files with candidate records, 261 expanded rows. Candidate counts are `u16-le/12/5` 102, `u16-be/12/7` 89, and `u16-be/20/15` 70.
- [x] Record the dominant expanded `/Frame` row families: repeated rows include `00010000000N000000020001`-style 12-byte rows and `00000000010200380000000N` rows, but these are still row-family evidence rather than decoded placement geometry.
- [x] Add `rjtd object-frame-record-families <file>` to group expanded `/Frame` candidate rows into decoded-false diagnostic families.
- [x] Sweep `object-frame-record-families`: 61 checked, 0 failures, 12 files with families, 261 records. Family counts are `frame-index-tail-coordinate-row12` 65, `frame-index-tail-window20` 65, `frame-index-mixed-row12` 61, `frame-index-flag-row12` 41, `frame-index-tail-zero-row12` 22, `frame-index-mixed-window20` 5, and `frame-index-tail-mixed-row12` 2.
- [x] Add `rjtd object-frame-row-links <file>` to verify whether 20-byte `/Frame` windows contain matching 12-byte frame rows as suffixes.
- [x] Sweep `object-frame-row-links`: 61 checked, 0 failures, 9 files with 20-byte rows, 70 row20 windows, 65 linked same-source suffix rows, 5 unlinked. All linked rows are `frame-index-tail-window20 -> frame-index-tail-coordinate-row12`, which makes `u16-be/12/7` a stronger authoritative-row candidate than the 20-byte context window.
- [x] Promote decoded-false `/Frame` reference rows and suffix links into model/export `objectStreamCandidates[].frameReferenceRows` so future image placement remains model-first.
- [x] Sweep JSON export for model-preserved `frameReferenceRows`: 61 checked, 0 failures, positive files 12, rows 261, suffix links 65. Family counts match the CLI sweep exactly.
- [x] Add `rjtd object-image-frame-candidates <file>` to summarize each image payload source against model-owned `/Frame` row evidence, payload kinds/dimensions, row families, row20 suffix links, coordinate-looking row12 pairs, and best coordinate/payload aspect delta.
- [x] Sweep `object-image-frame-candidates`: 61 checked, 0 failures, 12 files with image payload sources, 52 image sources, 52 frame-linked sources, 0 missing-frame sources, 261 frame rows, 35 dimensioned payloads, and 13 sources with aspect candidates. Preferred diagnostic buckets are `row12-tail-coordinate` 25, `row12-tail-zero` 7, `u16-le-row12` 19, and `none` 1.
- [x] Record that `row12-tail-coordinate` is a strong placement-analysis candidate but not yet sufficient for rendering: only two sources currently have best coordinate/payload aspect deltas <= 250 permille, both in `natsu.jtd`, so aspect evidence is still sparse.
- [x] Add `rjtd object-fdm-index <file>` to inspect `/FigureData/*/FDMIndex` rows against sibling `FDMVector` segments, preserving vector offsets, kind fields, bounding-box-like fields, segment prefixes, and image signature hits as decoded-false evidence.
- [x] Sweep `object-fdm-index`: 61 checked, 0 failures, 31 files with indexes, 39 index streams, 417 parsed rows, 6 rows with images, 13 image hits, and 2 missing sibling vectors. This proves FDMIndex/FDMVector is a separate image-placement evidence path from `Embedding N` `/Frame` rows.
- [x] Add `rjtd object-fdm-index-shape <file>` to separate exact 22-byte FDMIndex tables, declared-count prefix tables with auxiliary trailing payloads, mixed declared rows, unknown-header streams, and missing vectors.
- [x] Sweep `object-fdm-index-shape`: 61 checked, 0 failures, 39 indexes, 35 `fdm-index-v1` headers, 4 unknown headers, 34 plausible declared counts, 417 raw stream rows, 252 raw invalid offsets, 147 declared-prefix rows, 43 declared-prefix invalid offsets, and 13 declared-prefix image hits. Shape counts are `row22-count-prefix` 17, `row22-exact` 14, `row22-mixed-declared` 3, `unknown-header` 3, and `missing-vector` 2.
- [x] Add `rjtd object-fdm-index-rows <file>` to print row scope (`declared`, `post-declared`, or `raw`), row role (`vector-segment`, `coordinate-like-invalid`, or `invalid-vector-offset`), BE16/i16 field views, row bytes, and segment image hits for FDMIndex analysis.
- [x] Sweep `object-fdm-index-rows`: 61 checked, 0 failures, 31 files with indexes, 39 indexes, 417 rows, 147 declared rows, 253 post-declared rows, 17 raw rows, 165 valid vector rows, 252 invalid rows, 13 image hits, and 2 missing vectors. Role counts are `vector-segment` 165, `coordinate-like-invalid` 231, and `invalid-vector-offset` 21; all 43 declared invalid rows are `coordinate-like-invalid`.
- [x] Promote decoded-false `FDMIndex` row evidence into model/export/app-core JSON as `objectStreamCandidates[].fdmIndexEntries` on the corresponding `FDMVector` candidate, limited to valid `fdm-index-v1` declared-count prefix rows so auxiliary payload bytes are not exposed as false row entries.
- [x] Sweep JSON export for model-preserved `fdmIndexEntries`: 61 checked, 0 failures, 24 files with entries, 30 candidates with entries, 147 rows, 3 files with image-linked rows, 6 image-linked rows, 13 image hits, 104 valid vector offsets, and 43 invalid/out-of-range vector offsets.
- [x] Record that `fdmIndexEntries` can identify all currently observed image-bearing FDMVector segments while reducing false auxiliary rows. The 43 invalid declared-prefix rows are now classified as coordinate-like diagnostic rows in exactly 3 files, not image-bearing vector segments, so they must remain decoded-false and must not be promoted to renderable page geometry or paint resources.
- [x] Add `rjtd object-fdm-image-candidates <file>` to summarize image-bearing FDMVector rows from model-owned `fdmIndexEntries`, including normalized bbox diagnostics, complete payload coverage, and explicit `renderable=false` until page placement is proven.
- [x] Sweep `object-fdm-image-candidates`: 61 checked, 0 failures, 3 files with candidates, 3 FDM sources, 6 image-bearing rows, 13 image hits, 0 strict complete payloads, 5 plausible bbox rows, and 0 renderable rows. The observed `FFD8FF` hits in FDMVector segments are JPEG-like byte patterns inside vector data, not valid JPEG payloads with SOF/SOS structure, so they remain signature-only decoded-false evidence.
- [x] Expose the same FDM image rows through app-core `getPageOverlayImages` as `unplacedDiagnostics` with `imageCount:0`, `placementProven:false`, and `renderable:false`, keeping the rhwp-shaped overlay API callable without pretending page placement is decoded.
- [x] Preserve `/Frame` fixed 60-byte records as decoded-false model/export/app-core `objectFrameRecords`, including observed object id, record kind/type, and geometry-looking fields.
- [x] Add `rjtd object-fdm-frame-links <file>` to correlate image-bearing FDMIndex rows with `/Frame` records by `fdm row index == frame object id`.
- [x] Add strict image payload dimension diagnostics: model/export JSON now carries optional `imagePayloads[].dimensions`, JPEG payload spans must pass SOF/SOS structure validation, and `object-fdm-frame-links` reports frame size, payload dimensions, dimensioned payload counts, and best frame/payload aspect delta.
- [x] Sweep `object-fdm-frame-links`: 61 checked, 0 failures, 3 positive files, 6 FDM image rows, 13 image hits, 6 frame-linked rows, 0 missing-frame rows, 0 strict complete payloads, 0 dimensioned payloads, and 0 renderable rows.
- [ ] Decode `/Frame` geometry units, page association, paint order, and payload-to-image selection before promoting any FDM or `Embedding N` image ownership candidates into page geometry or paint resources.
- [ ] Recover remaining text hidden behind unknown inline formatting/control records.
- [ ] Decode true `DocumentText` record boundaries and control semantics beyond the current token layer.
- [ ] Explain why `MarkV.01` delta candidates 9, 29, and 30 all score strongly; current evidence does not support treating `unit + 29` as a unique stable adjustment.
- [ ] Decode the meaning of the varying MarkV.01 header value `0x0603`/`0x0610`/`0x061c`; current summary, LineMark-context, and exact-byte-search evidence weakens direct page-count, document-length, global page-style-code, or direct LineMark-entry-offset interpretations.
- [ ] Decode the semantic meaning of the remaining fields inside 29-byte `TCntV.01` records; current `text-position-count-field-deltas` evidence shows `t1/t2` form an ordered range-like pair but not the same span as the chosen `start/end` range.
- [ ] Determine whether `TCntV.01` mixes byte coordinates, UTF-16 unit coordinates, and layout/object-local coordinates, or whether the current token map is missing intervening record structure; tail `t1/t2` currently leans toward UTF-16 unit coordinates and delta 29/30 improve unit hits, while `0x0202` chosen ranges now show strong byte-range text/control overlap without matching the same tail coordinate behavior.
- [ ] Decode `/DocumentText` control boundaries `0x001c` and `0x000e`; current evidence suggests `0x001c` is a high-frequency text/control delimiter while `0x000e` often appears inside adjacent control clusters or before skipped-inline content.
- [ ] Find a row-local, section-local, or record-local base offset that explains the file-specific shifts exposed by `text-boundary-layout-map`.
- [ ] Explain why only 10 of the `iwata_file` strict boundary candidates have both line-word and page-field exact endpoint evidence while the remaining strict candidates and selected finance spans do not; since view-style group hits also appear on strict non-paragraph rows, treat them as default/flag-like until proven otherwise before constructing real paragraphs.
- [ ] Promote `TCntV.01` `be0` and `be1-shifted` diagnostics into explicit raw-preserving record family types once the shifted leading byte is explained by a flag, prefix, version, or preceding record boundary.
- [ ] Identify the actual coordinate target for `TCntV.01`; current evidence rejects direct `/LineMark`, `/PageMark`, and `/PaperMark` word/row/byte coordinates.
- [ ] Determine whether the out-of-range mini-sector in the empty `/DocumentTextPositionTables` sample is a stale directory entry, malformed ministream chain, or recoverable via another storage/object boundary.
- [ ] Fully prove the record layout of `/PageMark` variants that are still unsupported by `page-marks`.
- [ ] Decode the `SO` object/control record family field semantics; current evidence suggests fields 1-4 carry geometry-like tuples in singleton records while repeated records carry default/control constants, but the exact meaning of `packed-jseq3-like` 16-bit halves remains unproven.
- [ ] Decode semantic object header fields preceding embedded image payload spans and connect payload ownership to `/Figure`, `/Frame`, `/LayoutBox`, and layout mark evidence.
- [ ] Decode table semantics through `/DocumentText` control ranges plus layout/style streams; current stream-name inventory finds no named table streams, including in the `hyo` sample.
- [ ] Decode the semantic meaning of `/PaperMark` header count-like values and `0x00010000`/`0x00010010`/`0x00010011` flags.
- [ ] Explain the three unsupported `/PaperMark` stride values before broadening the parser shape.
- [ ] Decode the `0x1000`, `0x1001`, and `0x1002` tag families inside `/LineMark`; current evidence shows their immediate next word is payload-like rather than a family discriminator and their LineMark offsets are not direct text coordinates.
- [ ] Explain why the MarkV.01 header's final `u16` lands inside `/LineMark` near tag clusters in the three samples that expose both streams, while MarkV.01 entry offsets do not.
- [ ] Replace embedded fragment plausibility filtering with structured object/stream boundary parsing.

Important constraint:

- The Ichitaro filter license restricts decompilation and reverse engineering. Treat the binary as a no-code reference artifact. Do not copy code or derive implementation from disassembly.

## M3: Document Model

Goal: implement `rjtd export <file> --format json` through the document model layer.

Status: minimal model path implemented.

Completed:

- [x] Build a minimal `Document` from extracted `DocumentText`.
- [x] Add a `DocumentParser` entry point similar to rhwp's parser trait.
- [x] Preserve raw `/DocumentText` bytes in the `Document` model.
- [x] Represent non-empty text lines as `Paragraph` blocks with `TextRun` inlines.
- [x] Implement JSON export from `Document`.
- [x] Implement Markdown and plain text export from `Document`.
- [x] Implement native PDF export from `Document` through a rhwp-style SVG-to-PDF path.
- [x] Wire `rjtd export <file> --format json`.
- [x] Wire `rjtd export <file> --format md`.
- [x] Wire `rjtd export <file> --format pdf -o <output.pdf>`.
- [x] Keep exporters consuming `Document` instead of raw file or stream bytes.
- [x] Add a minimal rhwp-shaped `DocumentCore` facade with `from_bytes`, `page_count`, `get_document_info`, `render_page_svg`, and `render_page_html`.
- [x] Extend the core facade with app-facing `get_page_info`, page/section/page-border settings, `get_page_layer_tree`, `get_page_overlay_images`, `get_canvaskit_replay_plan`, `set_file_name`, DPI, `get_source_format`, and `convert_to_editable` fallbacks.
- [x] Add a `rjtd-wasm` `HwpDocument` wrapper that mirrors the rhwp load/page-info/SVG/layer-tree/source-format method names for browser integration.
- [x] Add text-line range mapping to `DocumentCore` so fallback pages can answer rhwp-shaped cursor, hit-test, line-info, and vertical-navigation APIs.
- [x] Add WASM `renderPageToCanvas`, `renderPageToCanvasFiltered`, and `renderPageToCanvasLegacy` fallback rendering through `web-sys`, following rhwp's browser-canvas dependency direction.
- [x] Add WASM `getCursorRect`, `hitTest`, `getLineInfo`, `moveVertical`, and explicit no-hit header/footer/footnote fallback APIs.
- [x] Add minimal body-text editing APIs to `DocumentCore` and `rjtd-wasm`: `insertText`, `deleteText`, `splitParagraph`, `mergeParagraph`, `getTextRange`, `getParagraphLength`, and `getParagraphCount`.
- [x] Ensure fallback pages are rebuilt after body-text edits so PDF/SVG/canvas/cursor APIs observe the edited model.
- [x] Add a model-owned fallback `WritingMode` path so `DocumentCore`, SVG page rendering, and `getPageLayerTree` can project `horizontal` or `vertical-rl` without exporters reading raw style streams.
- [x] Add rhwp-shaped no-op page/section/page-border setters until JTD page setting streams are decoded.
- [x] Add fallback character/paragraph/style APIs for rhwp Studio formatting panels: default char properties, default paragraph properties, a single `Normal` style, empty numbering/bullet lists, and no-op format/style application.
- [x] Add body selection and plain-text internal clipboard APIs: `getSelectionRects`, `deleteRange`, `copySelection`, `pasteInternal`, `hasInternalClipboard`, `getClipboardText`, and `clipboardHasControl`.
- [x] Add rhwp-shaped undo/search editing APIs for the fallback body model: `saveSnapshot`, `restoreSnapshot`, `discardSnapshot`, `searchText`, `searchAllText`, `replaceText`, `replaceOne`, and `replaceAll`.
- [x] Add low-risk rhwp Studio view/navigation fallback APIs: `setShowParagraphMarks`, `getShowControlCodes`, `setShowControlCodes`, `getShowTransparentBorders`, `setShowTransparentBorders`, `setClipEnabled`, `getPositionOfPage`, `getPageOfPosition`, `findNextEditableControl`, `findNearestControlBackward`, `findNearestControlForward`, `getControlTextPositions`, and `navigateNextEditable`.
- [x] Add no-hit/no-op rhwp Studio field, header/footer, footnote, and endnote fallback APIs so app panels and edit modes fail gracefully until JTD structures are decoded.
- [x] Add no-hit/no-op rhwp Studio table and cell fallback APIs for cell text, table geometry, row/column operations, cell styles, selection, and formula calls until JTD table structures are decoded.
- [x] Add no-hit/no-op rhwp Studio picture, shape, equation, bookmark, form-object, stable-id, control-copy, and image-data fallback APIs until JTD object/control structures are decoded.
- [x] Add explicit fallback behavior for rhwp Studio HWP/HWPX export, HTML paste/export, page/column/new-number commands, header/footer formatting, footnote editing, style mutation, and numbering creation APIs.
- [x] Close direct rhwp Studio WASM API source-surface gaps to the wasm-bindgen-generated `free` method only.
- [x] Preserve observed ruby base plus phonetic annotation pairs as structured `Inline::Ruby` model data while keeping plain text/Markdown/PDF output on the visible base text.
- [x] Preserve observed style/layout streams (`/DocumentEditStyles`, `/DocumentViewStyles`, `/TextLayoutStyle`, `/PageLayoutStyle`, `/PageLayoutStyleHeader`) as named `UnknownStyle` model data before decoding their record semantics.
- [x] Expose preserved JTD style stream sources through app-core document/style JSON (`getDocumentInfo`, `getStyleList`, `getStyleDetail`) without pretending they are decoded paragraph styles.
- [x] Summarize observed style stream families and big-endian header fields (`ssmg` vs table-like prefixes) in JSON so future style record decoding starts from structured evidence instead of raw blobs only.
- [x] Expose observed style stream record boundary candidates in JSON: `ssmg-slots` for `0x5555`/`0x4444` Ssmg slot records and `sequential` for DocumentViewStyles-style `u16 code + u16 payload_len + payload` records.
- [x] Extract conservative UTF-16BE `label` candidates from Ssmg style records, including observed labels such as `脚注(標準)`, `本文(ｵｰﾄｽﾀｲﾙ)`, and page layout labels like `中扉(自動)`.
- [x] Promote labeled `/TextLayoutStyle` records into rhwp-shaped app-core style candidates so `getStyleList` and `getStyleDetail` can expose observed JTD style names while keeping `decoded:false`.
- [x] Make app-core `applyStyle` persist JTD style candidate references in the in-memory `Paragraph` model, and make `getStyleAt` plus split paragraphs observe that fallback style state.
- [x] Verify the WASM `HwpDocument` wrapper exposes and applies JTD style candidates through the rhwp-shaped browser API surface.
- [x] Add `styleCandidateCount` and `styleCandidateNames` to app-core `getDocumentInfo` so apps can detect observed JTD style candidates before opening the style panel.
- [x] Add `rjtd style-records <file>` and `rjtd style-candidates <file>` reverse-engineering diagnostics for preserved style stream summaries, record offsets/codes, payload lengths, and labeled `/TextLayoutStyle` candidates.
- [x] Add `rjtd text-layout-style-records <file>` to inspect all `/TextLayoutStyle` records, including unlabeled records, payload digests, short payload previews, and labeled candidate IDs.
- [x] Add `rjtd document-view-style-groups <file>` to inspect `/DocumentViewStyles` group records with payload lengths, payload digests, and short payload previews before treating group IDs as decoded style references.
- [x] Add `rjtd text-position-style-context <file>` to correlate `TCntV.01` tail fields with observed text/page style candidate IDs and record indexes without treating those hits as decoded style references.
- [x] Add `rjtd text-position-style-summary <file>` to summarize field-level style hit distributions and compare variable fields such as `f1` against text style candidates, page style candidates, and `/DocumentViewStyles` group records while separating near-constant/default-like fields such as `f7`.
- [x] Add `rjtd text-position-count-tail-field-roles <file>` to compare `TCntV.01` tail fields and adjacent field pairs against document-text unit/text hits, showing that `f1/f2` often behaves more like a range/coordinate pair than a pure style reference.
- [x] Preserve valid `TCntV.01` text-count entries as decoded-false `textCountRanges` model/export/app-core metadata instead of dropping the observed range/coordinate evidence after parsing.
- [x] Preserve `/DocumentText` byte/UTF-16 source spans on parsed `TextRun` model data and attach byte/unit `documentTextOverlaps` evidence to decoded-false `textCountRanges`.
- [x] Expose fallback `textRun` ops plus rhwp-shaped `textSources`/`source` spans from app-core `getPageLayerTree`, including JTD byte/unit source ranges where known.
- [x] Add a fallback `pageBackground` paint op to `getPageLayerTree` so layer/replay ordering matches rhwp's background-to-flow model.
- [x] Add rhwp-shaped layer tree envelope metadata: `schema`, `resourceTable`, `outputOptions`, `fontResources`, feature lists, and fallback `textV2` diagnostics.
- [x] Make app-core `getValidationWarnings` report rhwp-shaped JTD fallback/preservation diagnostics instead of always returning an empty report.
- [x] Preserve `/DocumentText` control boundary codes as decoded-false `textControlBoundaries` model/export/app-core metadata with byte/unit source spans where known.
- [x] Project preserved `textControlBoundaries` onto fallback paragraph character offsets for rhwp-shaped `getControlTextPositions` and nearest-control navigation diagnostics.
- [x] Expose projected `textControlBoundaries` through app-core `getPageControlLayout` as `type:"jtdControl"` diagnostics with fallback bounding boxes and `decoded:false`.
- [x] Add `rjtd text-control-ranges <file> [control-code]` to summarize `/DocumentText` intervals split by all controls or a selected control delimiter.
- [x] Make app-core `getCanvasKitReplayPlan` follow rhwp mode policy (`default`/`compat`) and report fallback `pageBackground`/`textRun` ops as direct replay items instead of returning an always-empty plan.
- [x] Verify `rjtd-wasm` against `wasm32-unknown-unknown`.
- [x] Generate PDFs for all 61 current local `.jtd`, `.jtt`, and `.jttc` samples under `openjtd-samples/pdf-output/`.
- [x] Verify all generated PDFs with PDF header/trailer checks, `pypdf`, `pdfinfo`, and representative `pdftoppm` PNG rendering.
- [x] Add a conditional `rjtd-export` local-sample PDF smoke test that parses every available local `.jtd`, `.jtt`, and `.jttc` sample, exports PDF bytes, and checks PDF header, page marker, EOF marker, and minimum size.
- [x] Expose layout-validated but decoded-false `textParagraphBoundaryCandidates` through JSON export and app-core `getDocumentInfo`, and report them through `getValidationWarnings` as diagnostic-only evidence rather than decoded paragraph records.

Remaining:

- [ ] Decode real paragraph boundaries instead of deriving them from extracted newline text.
- [ ] Infer and attach style IDs from decoded `TextLayoutStyle`, `DocumentEditStyles`, `DocumentViewStyles`, or related streams to paragraphs and text runs during parsing.
- [ ] Persist style edits back to decoded JTD style/body streams once the corresponding stream mutation format is proven.
- [ ] Add fixture-based expected JSON outputs once redistributable samples are available.
- [ ] Convert decompressed `.jttc` template text/control placeholders into meaningful model blocks once the inner `DocumentText` structure is understood.
- [ ] Replace the current text-only PDF/SVG layout with a real Ichitaro layout renderer once page geometry, tables, images, and style streams are decoded.
- [ ] Decode Ichitaro vertical-writing flags from style/layout streams and set model `WritingMode` from parsed evidence instead of the current fallback default.
- [ ] Implement real canvas/layer paint operations in the app compatibility layer once the renderer can emit non-text visual primitives.
- [ ] Replace fallback cursor/hit-test geometry with real JTD layout geometry after `/LineMark`, `/PageMark`, `/PaperMark`, style, and object semantics are proven.
- [ ] Replace text-flattening body edit fallback with structure-preserving edits once true JTD paragraph records, inline controls, and styles are decoded.
- [ ] Implement real page/section/page-border mutation once the corresponding JTD streams are decoded.
- [ ] Implement real character, paragraph, style, numbering, and bullet streams instead of default formatting fallback JSON.
- [ ] Replace table/cell no-hit fallback APIs with decoded JTD table structures, nested cell paths, geometry, style, and formula semantics.
- [ ] Replace picture/shape/equation/bookmark/form no-hit fallback APIs with decoded JTD object/control structures and real binary/image/form data.
- [ ] Implement true HTML paste/export and HWP/HWPX/common-format export once the common document model and JTD object semantics are mature enough.
- [ ] Extend selection/clipboard APIs beyond body plain text to preserve inline controls, tables, cells, pictures, shapes, fields, footnotes, and unknown objects.
- [ ] Implement table, cell, header/footer, footnote, picture, shape, field, style, undo/redo, clipboard, and search APIs needed for full rhwp Studio feature parity.
- [ ] Implement save/export back to JTD or a common document format; current `HwpDocument` compatibility is read/render oriented.

## M4: Markdown Export

Goal: implement `rjtd export <file> --format md` through the document model layer.

Status: minimal model-based Markdown export implemented.

Completed:

- [x] Implement Markdown exporter that consumes `Document`.
- [x] Wire `rjtd export <file> --format md`.
- [x] Verify local `a5.jtd` produces recovered section titles such as `一、午后の授業`.

Remaining:

- [ ] Preserve headings, lists, tables, ruby, and layout semantics once the record parser exposes them.
- [ ] Add HTML export after the model has stable block/inline semantics.
