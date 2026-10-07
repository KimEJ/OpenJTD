# CLI Diagnostics

This reference covers selected investigation commands in the development
source tree. Run `cargo run --manifest-path rjtd/Cargo.toml -p rjtd-cli -- --help`
from the repository root for the full current command list. The commands below
assume the `rjtd/` workspace directory. Replace example paths and optional
arguments with your actual permitted inputs.

Outputs are implementation diagnostics, not accepted format semantics.
Bookmark offsets and fixed-bias probes remain undecoded; see
[format drafts](../../openjtd-spec/README.md) and
[implementation records](research/README.md). Current capabilities and test
limits are in [feature status](../../docs/FEATURE-STATUS.md) and
[validation](../../docs/VALIDATION.md).

## Commands and Diagnostic Meaning


```sh
cargo run -p rjtd-cli -- streams path/to/document.jtd
cargo run -p rjtd-cli -- info path/to/document.jtd
cargo run -p rjtd-cli -- dump-stream path/to/document.jtd /DocumentText
cargo run -p rjtd-cli -- cfb-map path/to/document.jtd
cargo run -p rjtd-cli -- cfb-dir path/to/document.jtd
cargo run -p rjtd-cli -- stream-meta path/to/document.jtd /DocumentText
cargo run -p rjtd-cli -- stream-chain path/to/document.jtd /DocumentText
cargo run -p rjtd-cli -- stream-words path/to/document.jtd /LineMark
cargo run -p rjtd-cli -- stream-word-frequencies path/to/document.jtd /LineMark
cargo run -p rjtd-cli -- line-mark-tags path/to/document.jtd
cargo run -p rjtd-cli -- line-mark-text-context path/to/document.jtd
cargo run -p rjtd-cli -- stream-dwords path/to/document.jtd /PageMark
cargo run -p rjtd-cli -- stream-dword-frequencies path/to/document.jtd /PageMark
cargo run -p rjtd-cli -- stream-text-probe path/to/document.jtd /PageMark
cargo run -p rjtd-cli -- stream-find path/to/document.jtd /PageMark
cargo run -p rjtd-cli -- stream-find-bytes path/to/document.jtd 534f0000
cargo run -p rjtd-cli -- so-records path/to/document.jtd
cargo run -p rjtd-cli -- so-record-clusters path/to/document.jtd
cargo run -p rjtd-cli -- so-record-fields path/to/document.jtd
cargo run -p rjtd-cli -- so-record-geometry path/to/document.jtd
cargo run -p rjtd-cli -- so-record-halves path/to/document.jtd
cargo run -p rjtd-cli -- cat path/to/document.jtd
cargo run -p rjtd-cli -- text-tokens path/to/document.jtd
cargo run -p rjtd-cli -- text-control-context path/to/document.jtd [control-code]
cargo run -p rjtd-cli -- text-positions path/to/document.jtd
cargo run -p rjtd-cli -- text-position-counts path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-context path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-tail-context path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-clusters path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-candidates path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-family path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-fields path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-field-deltas path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-tail-delta-scan path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-tail-delta-groups path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-tail-row-deltas path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-tail-row-context path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-range-preview path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-range-boundaries path/to/document.jtd
cargo run -p rjtd-cli -- text-position-count-layout-context path/to/document.jtd
cargo run -p rjtd-cli -- text-position-mark-header path/to/document.jtd
cargo run -p rjtd-cli -- text-position-mark-summary path/to/document.jtd
cargo run -p rjtd-cli -- paper-marks path/to/document.jtd
cargo run -p rjtd-cli -- paper-mark-shape path/to/document.jtd
cargo run -p rjtd-cli -- page-marks path/to/document.jtd
cargo run -p rjtd-cli -- page-mark-shape path/to/document.jtd
cargo run -p rjtd-cli -- text-map path/to/document.jtd
cargo run -p rjtd-cli -- text-position-context path/to/document.jtd
cargo run -p rjtd-cli -- text-position-line-context path/to/document.jtd
cargo run -p rjtd-cli -- text-position-delta-scan path/to/document.jtd
cargo run -p rjtd-cli -- export path/to/document.jtd --format json
cargo run -p rjtd-cli -- export path/to/document.jtd --format md
cargo run -p rjtd-cli -- export path/to/document.jtd --format html
cargo run -p rjtd-cli -- export path/to/document.jtd --format text
cargo run -p rjtd-cli -- export path/to/document.jtd --format pdf -o output.pdf
```

`streams` works on observed `.jtd`, `.jtt`, and `.jttc` CFB samples. It first uses the `cfb` crate and falls back to a narrow lenient FAT reader for malformed CFB files. For `.jttc`, this reports the outer CFB inventory.

`info` reports the detected compound-document shape and key stream sizes.

`cfb-map` reports special CFB sector chains: FAT sector ids, directory chain, mini FAT chain, and the root mini-stream chain. It is useful for detecting malformed files where the root mini-stream chain overlaps the directory chain or other special CFB structures.

`cfb-dir` reports raw CFB directory entries with directory id, object type, size, start sector, left/right/child ids, resolved path, raw name, and name length. It is useful when a suspicious stream needs to be compared with nearby sibling entries or embedded object storages.

`stream-meta` reports CFB directory metadata for one stream, including stream size, start sector, whether the stream is stored in the regular FAT or mini FAT, and the observed mini-stream sizes. This is diagnostic only.

`stream-chain` expands a stream's FAT or miniFAT sector chain, including the chain status, sector ids, and file or mini-stream byte offsets. This is useful when a directory entry may point at stale, foreign, or alternate payload bytes even though its sector chain is structurally complete.

`stream-words`, `stream-word-frequencies`, `stream-dwords`, and `stream-dword-frequencies` inspect any stream as raw big-endian 16-bit or 32-bit values. They are generic reverse-engineering diagnostics and do not imply a record parser.

`line-mark-tags` scans `/LineMark` for the current tag-like `0x1000`, `0x1001`, and `0x1002` words and prints each tag's word index, byte offset, previous four words, and next six words. It is a diagnostic for grouping LineMark records before assigning semantics.

`line-mark-text-context` compares each `/LineMark` tag row with the `/DocumentText` token map. It reports the tag's LineMark byte/unit contexts, whether the immediate next word appears in raw `/DocumentText`, the first raw word hit, and the same surrounding LineMark words printed by `line-mark-tags`.

`stream-text-probe` scans any stream for printable ASCII, UTF-16LE, and UTF-16BE string candidates. It is useful when a stream entry appears to point at stale, foreign, or alternate-encoded payload bytes.

`stream-find` searches for the exact bytes of one stream inside every other readable stream in the same CFB file. It helps trace stale or duplicate stream payloads back to a likely owning stream.

`stream-find-bytes` searches every readable stream for a user-provided hex byte sequence. It is useful for tracing markers such as `SO` (`534f0000`) or coordinate-like field values across object/control streams.

`so-records` scans every readable stream for the observed `SO\0\0` object/control marker and prints the stream path, offset, first little-endian 32-bit fields, and preserved raw bytes. It is diagnostic only.

`so-record-clusters` groups `SO` records by preserved raw bytes and reports counts plus stream-offset locations. It is useful for separating repeated default/control records from singleton geometry-like records before decoding field semantics.

`so-record-fields` expands each `SO` record into little-endian 32-bit fields with signed and low/high 16-bit views. It is useful for comparing coordinate-like values against constants such as `0x00000100` and `0x00000064`.

`so-record-geometry` classifies the first four payload fields after the `SO\0\0` marker as diagnostic geometry candidates. It reports the raw `f1..f4` values, `xyxy` width/height deltas, `xywh` right/bottom sums, and the preserved raw bytes. Class names are deliberately conservative: `geometry-like`, `default-control`, `packed-jseq3-like`, `packed-ffff-preamble`, `packed`, `truncated`, or `unknown`.

`so-record-halves` prints each `SO` payload dword as low/high 16-bit unsigned and signed halves. It is useful for comparing packed SO-like records such as `JSEQ3Contents`, where the current samples repeat the low 16 bits of one packed field in a later dword.

`cat` currently uses a structured `ParsedDocumentText` token parser. It reads observed `.jtd` and `.jtt` samples with `/DocumentText`, including common visible inline segments and control boundaries. It also opens observed `.jttc` samples by decoding `/JSCompDocument` `JustCompressedDocument` `-lh5-` payloads without adding an LHA dependency. When a local sample does not expose a named `/DocumentText`, `cat` can recover observed embedded `SsmgV.01`/`TextV.01` fragments and reports the format as `cfb-embedded-document-text`.

`text-tokens` prints the structured `ParsedDocumentText` stream as tab-separated `text`, `inline`, `skipped-inline`, and `control` rows for reverse-engineering work.

`text-control-context` prints each `/DocumentText` control boundary with its byte/unit range, previous and next map entries, and nearest previous/next control boundary. An optional decimal or hex control code filters the output, for example `0x001c` or `14`. This is diagnostic only and does not assign final control semantics.

`text-control-clusters` groups adjacent `/DocumentText` control boundaries and prints each cluster's entry range, code sequence, byte/unit range, and neighboring map entries. This is diagnostic only and is intended to narrow paragraph/table/object boundary candidates.

`text-positions` prints the initial parsed `MarkV.01` entries from `/DocumentTextPositionTables` as tab-separated `id` and raw offset rows. This is diagnostic only and does not yet drive model generation.

`text-position-counts` prints the observed `TCntV.01` numeric entries from `/DocumentTextPositionTables` as diagnostic rows. The table currently appears as 29-byte records beginning at stream offset `0x0024`.

`text-position-count-context` compares the first two `TCntV.01` fields against the `/DocumentText` token map as both byte offsets and UTF-16 unit offsets. This is diagnostic only; observed samples are mixed and the final coordinate semantics are not decoded yet.

`text-position-count-tail-context` compares the tail `t1/t2` fields against the `/DocumentText` token map as both byte offsets and UTF-16 unit offsets. Current samples show a stronger unit-coordinate signal than byte-coordinate signal, but not a complete rule.

`text-position-count-clusters` groups `TCntV.01` records by their provisional `(start, end)` pair and shows duplicate raw-tail variants. `text-position-count-candidates` prints both `be32@0/4` and shifted `be32@1/5` field candidates; one observed sample uses both candidate families. `text-position-count-family` classifies each record as the current `be0` or `be1-shifted` diagnostic family and prints both candidate offsets plus the remaining raw tail. `text-position-count-fields` expands that tail into `u16be` fields plus any extra trailing byte. `text-position-count-field-deltas` compares the chosen range span with the tail `t1..t2` span and signed deltas without assigning semantic field names. `text-position-count-tail-delta-scan` scans small positive deltas over `t1/t2` as UTF-16 unit offsets to test whether a MarkV-like adjustment improves hits. `text-position-count-tail-delta-groups` summarizes the same scan by `(family,t0,t3,t4,t7)` pattern so that subfamily-specific coordinate behavior can be separated from global offsets. `text-position-count-tail-row-deltas` prints the same score at row granularity with document byte/unit length and chosen/tail spans, which is useful for diagnosing spread-out groups such as `0x0202`. `text-position-count-tail-row-context` adds chosen start/end byte/unit contexts and best-delta tail contexts on the same row. `text-position-count-range-preview` summarizes the `/DocumentText` entries overlapped by the chosen range as byte and UTF-16 unit intervals, including token-kind counts and an escaped text preview. `text-position-count-range-boundaries` adds edge alignment, first/last/previous/next entries, and control-code counts for those same byte and unit intervals. `text-position-count-layout-context` compares the chosen family range against `/LineMark` word/byte offsets and parsed `/PageMark`/`/PaperMark` row/byte offsets.

`paper-marks` prints the observed `/PaperMark` header values and 8-byte `(index, flags)` rows. This is diagnostic only; the row shape is stable in most current local `/PaperMark` streams, but the semantic meaning of the header count values and flags is not decoded yet.

`paper-mark-shape` prints `/PaperMark` stream length, declared CFB size, header values, and fixed 8-byte row candidates. It is a non-failing diagnostic for separating normal `/PaperMark` rows from stale or foreign payload bytes.

`page-marks` prints observed `/PageMark` row families as header values, family name, raw-preserved rows, and any preserved trailing bytes. This is diagnostic only and currently covers fixed 84-byte rows, fixed 84-byte rows with a tail, count-plus-one variable rows, and count-variable rows rather than every local `/PageMark` variant.

`page-mark-shape` prints `/PageMark` stream length, declared CFB size, header values, and candidate row formulas such as fixed 84-byte rows, header-count rows, and 2-byte-trimmed variants. This is a reverse-engineering helper for classifying `/PageMark` variants before broadening the parser.

`text-map` prints the structured `/DocumentText` token map with byte ranges, UTF-16 unit ranges, token kind, selector/code metadata, and any `MarkV.01` ids that land inside each range. This is diagnostic only.

`text-position-context` compares each `MarkV.01` offset against the token map in three ways: raw byte offset, UTF-16 unit offset, and a provisional `unit + 29` probe. `text-position-delta-scan` scores unit deltas `0..64` by unit hits and visible text hits. `text-position-mark-header` prints the raw six bytes between `MarkV.01` and the first entry. `text-position-mark-summary` correlates the Mark header with `/DocumentText`, `/LineMark`, `/PageMark`, and `/PaperMark` metrics. `text-position-line-context` compares the Mark header and entry offsets against `/LineMark` word contexts and nearest tag rows. In the current `a5.jtd` family samples, `unit + 29` often lands on visible heading text, but the delta scan shows it is not unique.

`style-records` prints preserved style stream summaries and record candidates, including family, header candidates, record layout, offsets, codes, payload lengths, and conservative labels. `style-candidates` lists labeled `/TextLayoutStyle` candidates as stable per-document rows for cross-sample correlation. `text-layout-style-records` prints all `/TextLayoutStyle` records with payload digests and previews. `document-view-style-groups` prints `/DocumentViewStyles` group record payload lengths, digests, and short previews. `text-position-style-context` compares `TCntV.01` tail fields against text/page style candidate IDs, record indexes, and `/DocumentViewStyles` group records; `text-position-style-summary` aggregates those hits per tail field; `text-position-count-tail-field-roles` compares tail fields and adjacent pairs against document-text unit/text hits. Parsed `TextRun` values preserve `/DocumentText` byte/UTF-16 source spans, and valid `TCntV.01` entries are preserved as decoded-false `textCountRanges` with byte/unit `documentTextOverlaps` in model JSON and app-core document info. These are reverse-engineering diagnostics, not decoded paragraph style assignments yet.

`export` parses through the `DocumentParser` entry point, consumes `ParsedDocumentText` to build a minimal `Document` model, preserves the raw text source in the model, preserves skipped inline text as `UnknownObject` payloads, promotes observed ruby base/phonetic pairs to `Inline::Ruby`, preserves observed style/layout streams as named `UnknownStyle` entries, and then emits JSON, Markdown, plain text, basic HTML, or native PDF. Plain text, Markdown, and PDF output use the visible ruby base text, while JSON keeps the annotation text, style stream names, observed style stream family/header summaries, neutral record boundary candidates with conservative label candidates, and raw payloads. PDF export requires `-o`/`--output` and uses the model-based SVG-to-PDF path: `DocumentCore` renders text SVG pages and `rjtd-export` converts them with `svg2pdf` plus `pdf-writer`. For observed `.jttc`, the preserved `/DocumentText` and style streams come from the decompressed inner CFB. For embedded samples, the preserved source is `/EmbeddedDocumentText`. If a document has preserved raw streams but no extractable text yet, PDF/SVG output shows a visible diagnostic notice instead of a silent blank page. Basic HTML emits paragraphs and ruby markup; full layout and rich clipboard semantics remain unfinished.
