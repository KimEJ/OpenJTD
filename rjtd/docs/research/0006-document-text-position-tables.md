# rjtd research 0006: Position diagnostics and bookmark names

Status: implementation record; not a shared format RFC

Japanese translation: [0006-document-text-position-tables.ja.md](0006-document-text-position-tables.ja.md)

## Implementation Scope

Position-table diagnostics expose raw families and competing byte/unit hypotheses. The existing parser skips six bytes after MarkV.01 as a header; this remains a limitation, not proof that the first item is a header. `mark_tag_stream.rs` and `Document::bookmark_name_candidates` expose names while directory values and navigation coordinates remain undecoded.

The [format-facing record](../../../openjtd-spec/rfc/0006-document-text-position-tables.md) retains source
observations and unresolved interpretations. Rust types, candidate admission,
JSON fields, CLI output, paint approximations, test coverage, and resource
policies are implementation behavior. They do not establish native semantics.

## Reproduction Entry Points

From the repository root, build the CLI with `cargo build --manifest-path rjtd/Cargo.toml -p rjtd-cli`. Use an input whose analysis and disclosure are
permitted. Inspect `rjtd/target/debug/rjtd --help` for each command's required arguments.

```sh
rjtd/target/debug/rjtd text-positions path/to/document.jtd
rjtd/target/debug/rjtd text-position-counts path/to/document.jtd
```

Some commands require a page number, stream, format, or other argument.
Record the actual full command, input ID/hash, authoring version, implementation
commit, executed/skipped checks, and observed limitations with any result.
Private-corpus checks do not become public reproducibility by being listed here.

## Historical Evidence

The [complete pre-separation record](https://github.com/OpenJTD/rjtd/blob/cce9cb20611c8c809a7376c8df6130aed14a0437/openjtd-spec/rfc/0006-document-text-position-tables.md) preserves the original raw
examples, commands, local sweeps, rejected hypotheses, and implementation
history at a fixed commit. It mixes source observations with implementation
behavior and includes superseded capability statements; it is a historical
source, not the current format contract or an executable public fixture pack.
The public shared imports of RFC 0001/0003 retain their own import commits and
status; this local edit does not change them.

## Separation Work

Apply the [interpretation/rendering boundary](../../../docs/ARCHITECTURE.md)
before moving mixed functions. Preserve text, source spans, unknown data,
candidate labels, resource limits, and output behavior in focused regressions.
No Rust modules were moved by this documentation change.

## Preserved Pre-Separation Record

The following original mixed research record retains its detailed observations,
raw examples, experiments, counterexamples, and implementation history. Its
capability, policy, and hypothesis statements describe their historical stage;
read them together with the current scope above. Relative links were rebased.

Status: draft

Observed: 2026-06-18

Japanese translation: [0006-document-text-position-tables.ja.md](0006-document-text-position-tables.ja.md)

## Summary

Some JTD samples expose a `/DocumentTextPositionTables` stream next to `/DocumentText`.

The stream starts with the same ASCII magic:

```text
SsmgV.01
```

Observed samples then contain marker strings including:

```text
TCntV.01
MarkV.01
```

The first implemented parser only decodes the observed `MarkV.01` table as big-endian `(u16 id, u32 offset)` entries terminated by `0xffff`.

## Implemented Commands

```sh
cargo run -p rjtd-cli -- text-positions ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-counts ../rjtd-testdata/local-samples/justsystems-20120223023549-jp-just-finance-j200003.jtd
cargo run -p rjtd-cli -- text-position-count-context ../rjtd-testdata/local-samples/justsystems-20120223023549-jp-just-finance-j200003.jtd
cargo run -p rjtd-cli -- text-position-count-tail-context ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- text-position-count-clusters ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- text-position-count-candidates ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- text-position-count-family ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- text-position-count-fields ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- text-position-count-field-deltas ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- text-position-count-tail-delta-scan ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- text-position-count-tail-delta-groups ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- text-position-count-tail-row-deltas ../rjtd-testdata/local-samples/justsystems-20120223023609-jp-just-finance-j200403sc.jtd
cargo run -p rjtd-cli -- text-position-count-tail-row-context ../rjtd-testdata/local-samples/justsystems-20120223023609-jp-just-finance-j200403sc.jtd
cargo run -p rjtd-cli -- text-position-count-range-preview ../rjtd-testdata/local-samples/justsystems-20120223023609-jp-just-finance-j200403sc.jtd
cargo run -p rjtd-cli -- text-position-count-range-boundaries ../rjtd-testdata/local-samples/justsystems-20120223023609-jp-just-finance-j200403sc.jtd
cargo run -p rjtd-cli -- text-position-count-control-ranges ../rjtd-testdata/local-samples/justsystems-20120223023609-jp-just-finance-j200403sc.jtd 0x001c
cargo run -p rjtd-cli -- text-position-count-layout-context ../rjtd-testdata/local-samples/justsystems-20120223023549-jp-just-finance-j200003.jtd
cargo run -p rjtd-cli -- text-boundary-paragraph-like-style-context ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- text-boundary-paragraph-like-discriminators ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- text-paragraph-boundary-targets ../rjtd-testdata/local-samples/ichitaro-20030316043238-success-001-success_data-iwata_file.jtd
cargo run -p rjtd-cli -- stream-meta ../rjtd-testdata/local-samples/ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd /DocumentTextPositionTables
cargo run -p rjtd-cli -- text-map ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-context ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-delta-scan ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-mark-header ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-mark-summary ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-line-context ../rjtd-testdata/local-samples/a5.jtd
```

`text-positions` prints only the parsed `MarkV.01` entries:

```text
1	23802
2	7512
3	40217
4	5058
5	16486
6	32137
7	18627
8	10965
```

`text-position-counts` prints the observed non-Mark `TCntV.01` numeric table:

```text
header	1	0	3	36	3
entry	0	6630	7147	000019e600001beb0202000100490100000000000000000100000000
entry	1	15742	16240	00003d7e00003f70020200010049010000000000000000010000000000
entry	2	42392	43003	0000a5980000a7fb02020013005b010000000000000000010000000000
```

The header columns are currently interpreted as `kind`, `reserved`, `declared_count`, `entries_offset`, and `parsed_entries`. The observed entries start at stream offset `0x0024` and are currently parsed as 29-byte raw records with the first two big-endian `u32` fields exposed as provisional numeric offsets.

`text-position-count-context` compares those first two fields against the `/DocumentText` token map as both byte offsets and UTF-16 unit offsets:

```text
index	start	end	byte_start_context	byte_end_context	unit_start_context	unit_end_context
```

The output intentionally keeps all four contexts because the current samples do not prove a single coordinate system.

`text-position-count-tail-context` applies the same byte/unit context check to the tail `t1/t2` fields instead of the chosen range fields.

`text-position-count-tail-delta-scan` scans positive `0..64` deltas over `t1/t2` as UTF-16 unit offsets:

```text
delta	delta	rows	endpoints	unit_hits	text_hits	both_unit_rows	both_text_rows
```

`text-position-count-tail-delta-groups` summarizes that scan by the current `(family,t0,t3,t4,t7)` pattern key:

```text
group	family	t0	t3	t4	t7	rows	endpoints	best-unit	best-text	d0	d29	d30
```

`text-position-count-tail-row-deltas` exposes the same score per row and includes the document byte/unit length summary:

```text
summary	entries	doc-bytes	doc-units
row	index	family	t0	t3	t4	t7	start	end	span	t1	t2	tspan	best-unit	best-text	d0	d29	d30
```

`text-position-count-tail-row-context` combines the chosen range contexts with the best-delta tail contexts:

```text
row-context	index	family	t0	t3	t4	t7	start	end	t1	t2	best-unit	best-text	start-byte	end-byte	start-unit	end-unit	t1-unit-best	t2-unit-best	t1-text-best	t2-text-best
```

`text-position-count-range-preview` summarizes the `/DocumentText` entries overlapped by the chosen range as byte and UTF-16 unit intervals:

```text
range-preview	index	family	t0	t3	t4	t7	start	end	span	byte-range	unit-range
```

Each `byte-range` and `unit-range` value reports overlapped entry counts by token kind and an escaped text preview:

```text
entries=N,text=N,inline=N,skipped=N,control=N,preview=...
```

`text-position-count-range-boundaries` inspects the same chosen range as byte and UTF-16 unit intervals, but focuses on edge alignment and control delimiters:

```text
range-boundary	index	family	t0	t3	t4	t7	start	end	span	byte-boundary	unit-boundary
```

Each boundary value reports the number of overlapped entries, how many are fully contained, start/end edge classes, first/last/previous/next map entries, and a compact `controls=0xNNNN:N` list.

`text-position-count-control-ranges` compares each chosen `TCntV.01` range against `/DocumentText` intervals split by all controls or by a selected control delimiter:

```text
count-control-range	index	family	delimiter	t0	t3	t4	t7	start	end	span	byte-ranges	unit-ranges
```

Each `byte-ranges` and `unit-ranges` value reports how many control-delimited intervals overlap the chosen range, the first/last interval indexes, combined byte/unit span, entry spans, control-code counts, and a short preview. This is a correlation diagnostic only; it does not assign paragraph semantics to a control code.

Current local sweep with `text-position-count-control-ranges` over the 10 readable `TCntV.01` files and 89 rows:

| Delimiter | Byte interval overlaps | Unit interval overlaps | Byte multi-interval rows | Unit multi-interval rows |
| --- | ---: | ---: | ---: | ---: |
| `0x001c` | 462 | 794 | 40 | 37 |
| `0x000e` | 135 | 195 | 25 | 32 |

Working interpretation: `0x001c` remains the strongest text/control delimiter candidate, but it splits the chosen `TCntV.01` ranges too aggressively to promote directly to real paragraph boundaries. `0x000e` is coarser but still appears cluster-like. Parser/model changes should wait for a stronger boundary rule.

The document model now preserves the same candidate relationship as decoded-false JSON evidence. Each valid `textCountRanges` entry can include `controlRangeOverlaps` rows for observed delimiter candidates, reporting basis, delimiter code, overlapped interval count, first/last interval indexes, and combined source span. The same rows are lifted into top-level `textBoundaryCandidates` for app-core inspection, but these values are diagnostic and must not be treated as decoded paragraph records.

`rjtd text-boundary-candidates <file>` prints those model-derived candidates directly:

```text
text-boundary-candidate <index> kind=controlDelimitedTextCountRange range=<textCountRangeIndex> basis=<byte|unit> delimiter=<code> intervals=<count> interval-kind=<single|multi> first=<interval> last=<interval> source=<start-end> decoded=false
```

A local sweep over the current 61 samples finds candidates in the same 10 files that expose `TCntV.01` entries: 356 candidate rows, 1,586 overlapped intervals, 222 single-interval candidates, and 134 multi-interval candidates. The largest single candidate spans 44 `0x001c`/unit intervals in `justsystems-20120223023609-jp-just-finance-j200403sc.jtd`. This keeps `textBoundaryCandidates` useful as evidence while confirming that direct paragraph promotion remains unsafe.

`rjtd text-boundary-candidate-context <file>` compares those candidates with `/DocumentText` visible text, line breaks, and source edge alignment:

```text
text-boundary-candidate-context <index> range=<textCountRangeIndex> basis=<byte|unit> delimiter=<code> intervals=<count> interval-kind=<single|multi> source=<start-end> line-breaks=<count> text=<range-preview> edges=<edge-summary> decoded=false
```

The current context sweep reports 356 candidate rows, 276 rows with at least one line break, 3,458 total line breaks, and 210 rows that start after a control gap and end on an aligned text boundary. Among `0x001c` single-interval edge-good rows, byte basis has 17 one-line-break and 16 zero-line-break rows, while unit basis has 22 one-line-break and 13 zero-line-break rows. `0x000e` candidates often span many line breaks, so line-break presence alone is not a safe paragraph rule.

`rjtd text-boundary-candidate-agreement <file>` pairs byte-basis and unit-basis candidates with the same text-count range and delimiter:

```text
text-boundary-candidate-agreement <index> range=<textCountRangeIndex> delimiter=<code> byte-index=<candidate> unit-index=<candidate> byte-intervals=<count> unit-intervals=<count> byte-edge-good=<bool> unit-edge-good=<bool> byte-line-breaks=<count> unit-line-breaks=<count> text-match=<bool> line-break-match=<bool> byte-text=<preview> unit-text=<preview> decoded=false
```

The current agreement sweep finds 178 byte/unit pairs across the 10 files with candidates. Exact visible-text match occurs only once, and that row is empty, so text equality is not a useful promotion rule. In the stricter `0x001c` single/single pair set, 43 pairs exist; unit-basis edge-good/non-empty/line-break<=1 keeps 33 rows, while byte-basis keeps 28 rows. This suggests the next paragraph-rule experiment should evaluate unit-basis `0x001c` single candidates first, still as diagnostics until page/layout evidence agrees.

`rjtd text-boundary-candidate-layout-context <file>` compares unit-basis `0x001c` single candidates with direct `/LineMark`, `/PageMark`, and `/PaperMark` index/byte contexts:

```text
text-boundary-candidate-layout-context <file>
summary unit-001c-single-candidates=<count> rule-selected=<count> line-bytes=<bytes> line-words=<words> page-rows=<rows> page-bytes=<bytes> paper-rows=<rows> paper-bytes=<bytes>
candidate <index> range=<textCountRangeIndex> selected=<bool> edge-good=<bool> non-empty=<bool> line-breaks=<count> source=<unit-start-end> text=<preview> line-word-start=<context> ... paper-byte-end=<context> decoded=false
```

The current layout-context sweep finds 52 unit `0x001c` single candidates across 8 files and 35 strict rule-selected rows. None of the selected rows has start/end direct hits in `/LineMark`, `/PageMark`, or `/PaperMark`. Therefore candidate source units and layout mark rows are not the same coordinate space; paragraph promotion still needs a separate layout-mark mapping rule.

`rjtd text-boundary-layout-map <file>` scores the same unit-basis `0x001c` candidates against sparse layout point sets using several global source-unit transforms:

```text
text-boundary-layout-map <file>
summary unit-001c-single-candidates=<count> rule-selected=<count> target-sets=<count> bases=<count> delta-range=<min>..<max>
best scope=<all|selected> target=<point-set> base=<unit-transform> delta=<signed-delta> delta-at-boundary=<bool> points=<count> candidates=<count> endpoints=<count> valid=<count> invalid=<count> exact=<count> total-distance=<sum|-> max-distance=<max|-> decoded=false
```

The current map sweep runs successfully on all 61 local samples. It finds the same 52 unit `0x001c` single candidates across 8 files and 35 strict selected candidates across 4 files. Non-boundary exact hits exist, but they do not converge on a single global transform: `iwata_file` favors `line-word-value` and `page-be32-field` with unit-div2 shifts around `-1140..-1192`, while the selected finance samples favor different `page-be32-field` shifts. Therefore paragraph promotion must look for row-local, section-local, or record-local base offsets instead of a file-global source-unit transform.

`rjtd text-boundary-layout-map-rows <file>` scores each unit-basis `0x001c` candidate independently and includes the linked `TCntV.01` row summary:

```text
text-boundary-layout-map-rows <file>
summary unit-001c-single-candidates=<count> rule-selected=<count> target-sets=<count> bases=<count> local-rows=<count>
local candidate=<candidateIndex> range=<textCountRangeIndex> selected=<bool> target=<point-set> base=<unit-transform> delta=<signed-delta> delta-at-boundary=<bool> exact=<0..2> total-distance=<sum|-> max-distance=<max|-> start-nearest=<source:mapped->nearest:d> end-nearest=<source:mapped->nearest:d> source=<unit-start-end> text=<preview> tcnt=<row-summary> decoded=false
```

The row-local sweep also succeeds on all 61 local samples. It reports 52 unit `0x001c` single candidates and 35 strict selected candidates. Of the 32 strict selected candidates in `iwata_file`, 10 have row-local `exact=2` evidence through both `line-word-value` and `page-be32-field`. The three strict selected finance candidates have no row-local `exact=2` evidence. This means the strict edge/text rule is not sufficient by itself: the future paragraph rule needs a layout-local discriminator that keeps paragraph-like `iwata_file` rows separate from large spans.

`rjtd text-boundary-paragraph-like <file>` applies that discriminator as diagnostic-only output:

```text
text-boundary-paragraph-like <file>
summary unit-001c-single-candidates=<count> strict-selected=<count> paragraph-like=<count> selected-non-paragraph-like=<count> rule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2 decoded=false
candidate <index> range=<textCountRangeIndex> strict-selected=<bool> paragraph-like=<bool> line-word-evidence=<evidence|-> page-field-evidence=<evidence|-> source=<unit-start-end> text=<preview> tcnt=<row-summary> decoded=false
```

The current classifier sweep over 61 samples has 0 failures. It sees 52 unit `0x001c` single candidates, 35 strict selected candidates, 10 paragraph-like candidates, and 25 strict selected but non-paragraph-like candidates. Only `iwata_file` produces paragraph-like candidates under this rule. These rows are still evidence, not decoded paragraph construction.

`rjtd text-boundary-paragraph-like-style-context <file>` joins that classifier with the linked `TCntV.01` tail fields and the existing text/page/view-style diagnostics:

```text
text-boundary-paragraph-like-style-context <file>
summary unit-001c-single-candidates=<count> strict-selected=<count> paragraph-like=<count> selected-non-paragraph-like=<count> text-style-candidates=<count> page-style-candidates=<count> view-style-records=<count> rule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2 decoded=false
candidate <index> range=<textCountRangeIndex> strict-selected=<bool> paragraph-like=<bool> line-word-evidence=<evidence|-> page-field-evidence=<evidence|-> tail-fields=<fields> text-style-id-hits=<hits|-> text-style-index-hits=<hits|-> page-style-id-hits=<hits|-> page-style-index-hits=<hits|-> view-style-group-hits=<hits|-> byte-range=<preview> unit-range=<preview> source=<unit-start-end> text=<preview> tcnt=<row-summary> decoded=false
```

The current style-context sweep over 61 samples also has 0 failures and preserves the same candidate counts: 52 unit candidates, 35 strict selected candidates, 10 paragraph-like candidates, and 25 selected non-paragraph-like candidates. The 10 paragraph-like rows have no `/TextLayoutStyle` or `/PageLayoutStyle` candidate hits, but all 10 have `/DocumentViewStyles` group evidence in `iwata_file`. Strict non-paragraph rows also have view-group hits (25/25), so this is not a paragraph discriminator. Because `f7` is already near-constant in broader `TCntV.01` style summaries, these hits remain default/flag-like evidence only; they do not prove paragraph style attachment.

`rjtd text-boundary-paragraph-like-discriminators <file>` summarizes the same candidates by bucket:

```text
text-boundary-paragraph-like-discriminators <file>
summary unit-001c-single-candidates=<count> strict-selected=<count> paragraph-like=<count> selected-non-paragraph-like=<count> rule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2 decoded=false
bucket <paragraph-like|strict-non-paragraph|non-strict> rows=<count> strict-selected=<count> line-word-exact2=<count> page-field-exact2=<count> dual-exact2=<count> text-style-hit=<count> page-style-hit=<count> view-style-group-hit=<count> missing-tcnt=<count> source-spans=<min..max> range-spans=<min..max> families=<counts> f0=<counts> f4=<counts> f7=<counts> line-evidence=<counts> page-evidence=<counts> decoded=false
```

The current discriminator sweep over 61 samples has 0 failures. It confirms that dual exact layout evidence appears only in paragraph-like rows: 10/10 for paragraph-like, 0/25 for strict-non-paragraph, and 0/17 for non-strict. In `iwata_file`, the paragraph-like bucket is `be0:10` with `range-spans=2..8`, while strict-non-paragraph rows have `range-spans=0..0` and include both `be0` and `be1-shifted`. This makes nonzero chosen `TCntV.01` span plus row-local dual layout exactness the strongest current discriminator. It still does not explain the coordinate target and therefore remains decoded-false evidence.

`rjtd text-paragraph-boundary-targets <file>` traces the model-preserved `textParagraphBoundaryCandidates` back to concrete layout stream hit locations:

```text
text-paragraph-boundary-targets <file>
summary text-paragraph-boundary-candidates=<count> line-words=<count> page-rows=<count> rule=strict-unit-001c-single+nonzero-tcnt-span+line-word-value-exact2+page-be32-field-exact2 decoded=false
text-paragraph-boundary-target <index> boundary=<textBoundaryCandidateIndex> range=<textCountRangeIndex> source=<unit-start-end> span=<textCountRangeSpan> line-word-evidence=<target:base:delta> line-start=<value/hits/refs> line-end=<value/hits/refs> page-field-evidence=<target:base:delta> page-start=<value/hits/refs> page-end=<value/hits/refs> text=<preview> tcnt=<row-summary> decoded=false
```

The current 61-sample sweep has 0 failures and reports the same 10 candidates, all in `iwata_file`. The new provenance output shows why these candidates must remain decoded-false: among the 20 candidate endpoints, 6 line endpoints and 4 page endpoints are non-unique or missing under the current hit formatter. The next proof step is therefore not just "exact hit exists", but identifying which `/LineMark` word positions and `/PageMark` row fields are semantically eligible paragraph/layout targets.

`text-position-count-clusters` groups `TCntV.01` records by their provisional `(start, end)` pair and reports duplicate raw-tail variants. `text-position-count-candidates` prints the raw first bytes as two candidate interpretations:

```text
index	be0_start	be0_end	be1_start	be1_end	raw
```

`be0` means big-endian `u32` at raw offsets `0` and `4`. `be1` means shifted big-endian `u32` at raw offsets `1` and `5`. Both are diagnostic candidates, not stable field names.

`text-position-count-family` applies the current conservative family split and prints `chosen_start`, `chosen_end`, both candidate pairs, the leading byte, and the remaining raw tail:

```text
family	index	family	chosen_start	chosen_end	be0_start	be0_end	be1_start	be1_end	lead	tail
```

`text-map` prints token ranges from `/DocumentText`:

```text
byte_start	byte_end	unit_start	unit_end	kind	meta	byte_marks	unit_marks	text_preview
344	348	172	174	text	-	-	-	一、
```

`text-position-context` compares each `MarkV.01` offset against the token map as:

1. raw byte offset;
2. raw UTF-16 unit offset;
3. provisional `unit + 29` offset.

`text-position-delta-scan` scores UTF-16 unit deltas `0..64` for parsed `MarkV.01` entries:

```text
delta	entry_count	unit_hits	text_hits
```

Example `a5.jtd` observations:

| id | raw offset | strongest current context |
| ---: | ---: | --- |
| 2 | 7512 | `unit + 29` lands on `三、家` |
| 4 | 5058 | `unit + 29` lands on `二、活版所` |
| 5 | 16486 | `unit + 29` lands on `五、` |
| 7 | 18627 | `unit + 29` lands on `六、銀河ステーション` |
| 8 | 10965 | `unit + 29` lands on `四、ケンタウル祭の夜` |

The `unit + 29` probe is a diagnostic comparison only. It is not yet a stable coordinate rule.

Across the five local samples with parsed `MarkV.01` entries, ids 2, 4, 5, 7, and 8 consistently land on visible section heading text through the same `unit + 29` probe. Ids 3 and 6 land on body text or nearby body-text boundaries. Id 1 remains unclear and often lands near control/inline boundaries under this probe.

The broader delta scan weakens the idea that `29` is a unique stable adjustment. Across the same five files (`46.jtd`, `a5.jtd`, `a6.jtd`, `b6.jtd`, `shinsyo.jtd`) and 40 total `MarkV.01` entries, the top current scores are:

| Delta | Unit hits | Visible text hits |
| ---: | ---: | ---: |
| 9 | 34 | 31 |
| 29 | 31 | 31 |
| 30 | 31 | 31 |
| 31 | 31 | 26 |

Therefore `unit + 29` remains a useful probe, but it is not proven as the table/header adjustment. The competing `unit + 9` and adjacent `unit + 30` scores may indicate section-local bases, still-undecoded record boundaries, or broad text spans that make several nearby deltas look plausible.

`text-position-line-context` compares the `MarkV.01` header and entries against `/LineMark` word positions. It reports `/LineMark` word count, tag count, parsed Mark entry count, parsed `/PageMark` and `/PaperMark` row counts, and for each Mark value the nearest `0x1000`/`0x1001`/`0x1002` tag rows.

Across the current 61 local samples:

| Metric | Count |
| --- | ---: |
| files with both readable `/LineMark` and parsed `MarkV.01` | 3 |
| files missing `/LineMark` before this comparison can run | 6 |
| files missing `/DocumentTextPositionTables` | 41 |
| files with `/DocumentTextPositionTables` but no parsed `MarkV.01` | 11 |
| `MarkV.01` entry offsets checked against `/LineMark` | 24 |
| `MarkV.01` entry offsets inside `/LineMark` word range | 0 |

The three files where the comparison runs are:

| Sample | line words | line tags | Mark entries | Page rows | Paper rows | Mark header line index | LineMark word at header index | nearest tag rows |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `46.jtd` | 2597 | 491 | 8 | 97 | 97 | 1539 | `0x0029` | `0x1002@1520`, `0x1000@1564` |
| `a5.jtd` | 2561 | 511 | 8 | 75 | 75 | 1539 | `0x0016` | `0x1002@1534`, `0x1002@1542` |
| `b6.jtd` | 2667 | 502 | 8 | 98 | 98 | 1552 | `0x0000` | `0x1000@1548`, `0x1002@1554` |

Working interpretation: the `MarkV.01` entry offsets are not direct `/LineMark` word indexes. The final big-endian `u16` in the six-byte Mark header does land inside `/LineMark` in the three samples that expose both streams, but it does not land on a stable tag value and is still not decoded.

## Observed Bytes

`a5.jtd` begins:

```text
5373 6d67 562e 3031 0000 0001 0000 0100
0000 0001 5443 6e74 562e 3031 0000 4d61
726b 562e 3031 0000 0000 0603 0001 0000
5cfa 0002 0000 1d58 ...
```

The initial working interpretation is:

```text
SsmgV.01
...
TCntV.01
00 00
MarkV.01
00 00 00 00 06 03
00 01 00 00 5c fa
00 02 00 00 1d 58
...
ff ff ...
```

`0x00000603` is preserved as an unknown table header value for now. It is not treated as an entry count.

## Local Sample Results

Current local sweep:

```text
checked=61 with_position_tables=16 with_mark_entries=5 with_tcnt_entries=10 empty_or_unreadable_position_payload=1
```

Not all readable JTD/JTT/JTTC samples expose this stream.

The initial `MarkV.01` parser currently finds 8 entries in each of these local samples:

```text
46.jtd
a5.jtd
a6.jtd
b6.jtd
shinsyo.jtd
```

The remaining files with `/DocumentTextPositionTables` now split into 10 observed `TCntV.01` numeric tables and 1 inventory entry whose stream size is non-zero but whose payload currently reads back as empty through the safe CFB/ministream path.

The empty-payload sample is:

```text
ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd
```

`stream-meta` reports:

```text
size                 304
start_sector         224
storage              mini
mini_stream_cutoff   4096
mini_stream_bytes    7680
mini_fat_entries     384
```

Since mini-sector `224` would start at byte `224 * 64 = 14336`, it points beyond the observed 7680-byte mini stream. rjtd therefore preserves this as an unreadable/empty position-table payload instead of guessing a regular-sector fallback.

Observed `TCntV.01` samples:

| Sample | declared count | parsed entries | first range |
| --- | ---: | ---: | --- |
| `ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd` | 6 | 6 | `3603..4078` |
| `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` | 50 | 50 | `2404..2406` |
| `justsystems-20120223023549-jp-just-finance-j200003.jtd` | 3 | 3 | `6630..7147` |
| `justsystems-20120223023609-jp-just-finance-j200403sc.jtd` | 5 | 5 | `14947..15781` |
| `justsystems-20120223023614-jp-just-finance-j200503t.jtd` | 5 | 5 | `6434..6950` |
| `justsystems-20120223023906-jp-just-finance-j200003c.jtd` | 5 | 5 | `12484..12913` |
| `justsystems-20120223024135-jp-just-finance-j200403s.jtd` | 3 | 3 | `5749..6606` |
| `justsystems-20120223024139-jp-just-finance-j200409c.jtd` | 4 | 4 | `15289..16089` |
| `justsystems-20120223024144-jp-just-finance-j200409t.jtd` | 3 | 3 | `4888..5710` |
| `justsystems-20120223024150-jp-just-finance-j200503c.jtd` | 5 | 5 | `19682..20100` |

Initial context sweep:

| Sample | entries | byte text hits | unit text hits |
| --- | ---: | ---: | ---: |
| `ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd` | 6 | 2 | 3 |
| `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` | 50 | 14 | 28 |
| `justsystems-20120223023549-jp-just-finance-j200003.jtd` | 3 | 1 | 0 |
| `justsystems-20120223023609-jp-just-finance-j200403sc.jtd` | 5 | 3 | 2 |
| `justsystems-20120223023614-jp-just-finance-j200503t.jtd` | 5 | 2 | 0 |
| `justsystems-20120223023906-jp-just-finance-j200003c.jtd` | 5 | 3 | 0 |
| `justsystems-20120223024135-jp-just-finance-j200403s.jtd` | 3 | 1 | 0 |
| `justsystems-20120223024139-jp-just-finance-j200409c.jtd` | 4 | 3 | 1 |
| `justsystems-20120223024144-jp-just-finance-j200409t.jtd` | 3 | 1 | 1 |
| `justsystems-20120223024150-jp-just-finance-j200503c.jtd` | 5 | 4 | 1 |

This is mixed evidence. The JustSystems finance samples often look closer to byte-oriented `/DocumentText` positions, while `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` produces more text hits through UTF-16 unit contexts. The table may mix coordinate systems, point to layout/object-local regions, or expose gaps in the current `DocumentText` token map.

Candidate-family sweep:

| Sample group | Entries | `be0` plausible against `/DocumentText` bytes | `be1` plausible against `/DocumentText` bytes |
| --- | ---: | ---: | ---: |
| 9 current non-`iwata_file` `TCntV.01` samples | 39 | 39 | 0 |
| `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` | 50 | 32 | 18 |

`iwata_file` therefore appears to contain at least two `TCntV.01` record families. Entries 0-31 fit the provisional `be0` interpretation. Entries 32-49 look implausible as `be0` (`150`, huge end values, etc.) but become natural offsets under `be1`, for example:

```text
index	be0_start	be0_end	be1_start	be1_end
32	150	3388997782	38602	38602
33	150	3388997782	38602	38602
34	151	805306519	38704	38704
```

`text-position-count-family` confirms this split across the current samples: 10 files expose `TCntV.01`, with 89 records total; 71 classify as `be0`, and 18 classify as `be1-shifted`. Every shifted record is in `iwata_file` entries 32-49.

The same sample has duplicate provisional ranges with different raw tails. `text-position-count-clusters` reports 38 clusters, including 12 duplicate clusters. This suggests repeated logical spans are distinguished by hidden subfields after the offset candidates.

`text-position-count-fields` expands each record after the chosen range into `u16be` tail fields named only by position (`t0` through `t9`) plus any extra trailing byte. These are observation labels, not semantic field names.

Current sweep:

| Family | Records | Tail shape |
| --- | ---: | --- |
| `be0` | 71 | 10 `u16be` fields plus extra byte `00` |
| `be1-shifted` | 18 | exactly 10 `u16be` fields, no extra byte |

Common tail-field distributions:

| Family | Field | Most common values |
| --- | --- | --- |
| `be0` | `t0` | `0x0101` x36, `0x0202` x34, `0x0102` x1 |
| `be0` | `t3` | `0x0100` x61, `0x0102` x7, other singleton variants |
| `be0` | `t4` | `0x0000` x43, `0x0001` x28 |
| `be0` | `t7` | `0x0001` x64, `0x0003` x6, `0x0000` x1 |
| `be1-shifted` | `t0` | `0x0101` x16, `0x0202` x2 |
| `be1-shifted` | `t3..t9` | fixed as `0x0100,0x0001,0x0000,0x0000,0x0001,0x0000,0x0000` |

The shifted family is therefore not only a shifted offset interpretation. Its tail shape is also cleaner: the final seven fields are fixed across all 18 shifted records in the current samples.

`text-position-count-field-deltas` compares the chosen family range span with the tail `t1..t2` span and prints signed deltas from chosen start/end to `t1/t2`. This is still diagnostic only.

Across the current local samples:

| Metric | Count |
| --- | ---: |
| checked files | 61 |
| readable files with `TCntV.01` entries | 10 |
| `TCntV.01` rows checked | 89 |
| rows where `t2 >= t1` | 89 |
| rows where chosen span is zero | 32 |
| rows where `t2 - t1` is zero | 2 |
| rows where `t2 - t1` equals chosen span | 0 |

Span relation counts:

| Family | `t2 - t1` vs chosen span | Records |
| --- | --- | ---: |
| `be0` | greater | 27 |
| `be0` | less | 44 |
| `be1-shifted` | greater | 15 |
| `be1-shifted` | less | 3 |

This makes `t1/t2` look like an ordered range-like pair, but not the same coordinate span as the chosen `start/end` range.

The same `t1/t2` pair was compared against `/DocumentText` with `text-position-count-tail-context`:

| Metric | Count |
| --- | ---: |
| checked files | 61 |
| readable files with `TCntV.01` entries and `/DocumentText` | 10 |
| `TCntV.01` rows checked | 89 |
| rows where either `t1` or `t2` hits a byte range | 21 |
| rows where both `t1` and `t2` hit byte ranges | 5 |
| rows where either `t1` or `t2` hits a UTF-16 unit range | 49 |
| rows where both `t1` and `t2` hit UTF-16 unit ranges | 28 |
| rows where both `t1` and `t2` hit text UTF-16 unit ranges | 26 |

The unit-coordinate signal is stronger than the byte-coordinate signal, but it is not complete. Some unit hits land on control entries, and 40 rows still have no direct unit hit for either `t1` or `t2`.

Scanning positive `0..64` deltas over `t1/t2` as UTF-16 unit offsets gives the following aggregate checkpoints:

| Delta | Rows | Endpoints | Unit endpoint hits | Text endpoint hits | Rows with both unit hits | Rows with both text hits |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 89 | 178 | 77 | 70 | 28 | 26 |
| 9 | 89 | 178 | 74 | 69 | 28 | 27 |
| 29 | 89 | 178 | 124 | 98 | 49 | 32 |
| 30 | 89 | 178 | 124 | 95 | 47 | 30 |
| 53 | 89 | 178 | 105 | 102 | 42 | 42 |
| 64 | 89 | 178 | 93 | 83 | 28 | 28 |

Delta 29 and 30 tie for the highest unit endpoint hit count in the current `0..64` scan, and delta 29 has slightly stronger text endpoint and both-unit row counts than delta 30. However, text endpoint hits peak at delta 53. This strengthens the relationship between the tail fields and the MarkV-style `+29` candidate, but still does not prove a single stable adjustment.

Grouping the same scan by `(family,t0,t3,t4,t7)` explains why the aggregate result is mixed:

| Pattern | Rows | Unit best | Text best | Observation |
| --- | ---: | --- | --- | --- |
| `be0,0x0101,0x0100,0x0001,0x0001` | 28 | `29` | `29` | strongest `+29` group; all current rows are in `iwata_file` |
| `be1-shifted,0x0101,0x0100,0x0001,0x0001` | 16 | `31` | `30` | shifted family is close to, but not identical with, the `+29` group |
| `be1-shifted,0x0202,0x0100,0x0001,0x0001` | 2 | `30` | `30` | small shifted subgroup |
| `be0,0x0202,0x0100,0x0000,0x0001` | 28 | spread | spread | rows are spread across multiple files and best deltas (`19`, `23`, `30`, `36`, `46`, `49`, `56`, `57`, etc.) |
| `be0,0x0202,0x0102,0x0000,0x0001` | 5 | mixed | mixed | small group; current rows prefer `29`, `39`, or `50` depending on file |

This supports treating tail patterns as diagnostic subfamilies. A global `+29` rule would explain one major group but would hide the shifted and `0x0202` behavior.

Row-level delta scoring adds another caution. The major `be0,0x0202,0x0100,0x0000,0x0001` pattern spans 28 rows across 8 files. In the current samples it has:

| Metric | Observed range/count |
| --- | ---: |
| rows | 28 |
| files | 8 |
| chosen span range | `398..1212` |
| tail `t2 - t1` range | `46..72` |
| document unit lengths | `65889..146657` |
| distinct row-level best unit deltas | 17 |

This spread does not look like a single file-level or global correction. It more likely reflects row-local structure, missing intermediate records, or a different coordinate/object-local target for the `0x0202` family.

Context-level inspection reinforces that split. Across all current `0x0202` rows:

| Scope | Rows | Files | Chosen start byte | Chosen end byte | Chosen start unit | Chosen end unit | Best tail `t1` | Best tail `t2` |
| --- | ---: | ---: | --- | --- | --- | --- | --- | --- |
| all `t0=0x0202` | 36 | 10 | text 11, boundary 21, control/other 4 | text 15, boundary 19, control 2 | text 4, boundary 31, other 1 | text 1, boundary 34, other 1 | text 25, control 8, inline 3 | text 35, control 1 |
| major `be0,0x0202,0x0100,0x0000,0x0001` | 28 | 8 | text 9, boundary 17, control/other 2 | text 12, boundary 14, control 2 | text 3, boundary 24, other 1 | text 1, boundary 27 | text 21, control 6, inline 1 | text 27, control 1 |

The chosen range therefore often lands in later body byte ranges or nearby body boundaries, while the best-delta tail fields usually land in early heading/date text. This argues against treating `start/end` and `t1/t2` as duplicate endpoints in one text coordinate system.

Range-preview inspection adds one useful distinction: the `0x0202` chosen ranges often cover real `/DocumentText` byte ranges even though they do not behave like direct layout-stream coordinates and do not match the tail `t1/t2` span.

| Scope | Rows | Files | Byte range overlaps text | Unit range overlaps text |
| --- | ---: | ---: | ---: | ---: |
| all `t0=0x0202` | 36 | 10 | 31 | 25 |
| `be0,0x0202,0x0100,0x0000,0x0001` | 28 | 8 | 25 | 21 |
| `be0,0x0202,0x0102,0x0000,0x0001` | 5 | 3 | 5 | 3 |
| `be0,0x0202,0x0100,0x0000,0x0003` | 1 | 1 | 1 | 1 |
| `be1-shifted,0x0202,0x0100,0x0001,0x0001` | 2 | 1 | 0 | 0 |

For the major `be0,0x0202,0x0100,0x0000,0x0001` group, 25 of 28 rows overlap text in the chosen byte range and 21 of 28 overlap text in the chosen unit range. Many overlapping ranges include control entries as well as text, so the chosen range is still not a clean extracted-text span. The result does, however, make `/DocumentText` byte intervals a stronger next target than direct `/LineMark`, `/PageMark`, or `/PaperMark` coordinates for this group.

Boundary inspection further sharpens that target for the major `be0,0x0202,0x0100,0x0000,0x0001` group:

| Metric | Byte interval | UTF-16 unit interval |
| --- | ---: | ---: |
| rows | 28 | 28 |
| overlapped map entries total | 535 | 1031 |
| fully contained entries | 513 | 1026 |
| partial entries | 22 | 5 |
| rows containing controls | 25 | 22 |
| start edge aligned / inside / gap | `1 / 10 / 17` | `0 / 4 / 24` |
| end edge aligned / inside / gap | `1 / 12 / 15` | `3 / 1 / 24` |

The byte interpretation is therefore narrower than the unit interpretation for this group. It still often starts or ends in gaps around control boundaries rather than exactly aligned token edges, but most overlapped byte entries are fully contained. Its current control-code totals are:

| Control code | Byte interval count | UTF-16 unit interval count |
| --- | ---: | ---: |
| `0x001c` | 281 | 522 |
| `0x000e` | 38 | 73 |
| `0x001d` | 2 | 6 |
| `0x0000` | 1 | 15 |
| `0x000c` | 3 | 0 |

This points the next `0x0202` investigation toward `/DocumentText` control-delimited structures, especially `0x001c` and `0x000e`, rather than plain extracted text positions.

The follow-up `text-control-context` sweep across all 61 local samples supports prioritizing those two controls. It runs without errors, finds mapped controls in 60 files, and reports `0x001c` 51,971 times across 60 files and `0x000e` 6,621 times across 41 files. `0x001c` most often appears between text/text, text/control, control/text, or control/control neighbors, making it the stronger generic delimiter candidate. `0x000e` most often appears in control/control, text/control, or text/skipped-inline contexts, making it look more like a control-cluster or inline-adjacent delimiter. These context profiles are still diagnostic and do not assign final semantic names.

`text-position-count-layout-context` compares the chosen `TCntV.01` range for each record against `/LineMark` word offsets, `/LineMark` byte offsets, parsed `/PageMark` rows and bytes, and parsed `/PaperMark` rows and bytes.

Across the current local samples:

| Metric | Count |
| --- | ---: |
| checked files | 61 |
| readable files with `TCntV.01` entries | 10 |
| files missing `/DocumentTextPositionTables` | 45 |
| files with `/DocumentTextPositionTables` but no `TCntV.01` entries | 5 |
| unreadable/invalid position-table payloads | 1 |
| `TCntV.01` rows checked | 89 |
| direct `/LineMark` word hits | 0 |
| direct `/LineMark` byte hits | 0 |
| direct `/PageMark` row hits | 0 |
| direct `/PageMark` byte hits | 0 |
| direct `/PaperMark` row hits | 0 |
| direct `/PaperMark` byte hits | 0 |

This rejects a direct layout-stream coordinate interpretation for the chosen `TCntV.01` ranges in the current samples. The fields may still refer to `/DocumentText`, a layout-local coordinate system not exposed as raw stream offsets, or another table/object boundary that is not decoded yet.

Initial `unit + 29` classification:

| id | observed behavior |
| ---: | --- |
| 1 | unclear, often near control/inline boundaries |
| 2 | section heading `三、家` |
| 3 | body-text anchor |
| 4 | section heading `二、活版所` |
| 5 | section heading prefix `五、` |
| 6 | body-text anchor or nearby body-text boundary |
| 7 | section heading `六、銀河ステーション` |
| 8 | section heading `四、ケンタウル祭の夜` |

## Current Coordinate Hypothesis

Raw `MarkV.01` offsets do not behave like extracted plain-text character offsets. For example, `a5.jtd` has extracted text around 39k characters, while one observed offset is `40217`.

Raw byte offsets are also weak because many offsets are odd, which would land in the middle of a UTF-16BE unit.

The strongest current hypothesis is that `MarkV.01` offsets are UTF-16 unit or internal `/DocumentText` coordinates. In the current five parsed samples, adding 29 UTF-16 units to several offsets lands near visible section heading text, but the broader delta scan shows that 9 and 30 are competitive. This may represent a table-local header adjustment, a section-local coordinate base, or another still-undecoded record boundary.

The six bytes between `MarkV.01` and the first observed entry are not constant across samples:

```text
46.jtd      0000 0000 0603
a5.jtd      0000 0000 0603
a6.jtd      0000 0000 061c
b6.jtd      0000 0000 0610
shinsyo.jtd 0000 0000 061c
```

These values do not directly explain the constant 29-unit probe. They should stay preserved as unknown table header fields until more of the table is decoded.

`text-position-mark-header` exposes this area directly. In the current five MarkV samples, the `MarkV.01` marker itself is always at stream offset 30; the six-byte header always begins with `00000000`, and the final big-endian `u16` is one of `0x0603`, `0x0610`, or `0x061c`.

`text-position-mark-summary` correlates this header with nearby streams. The current five-sample sweep weakens simple interpretations:

| Header | Samples | Related observation |
| --- | --- | --- |
| `0x0603` | `46.jtd`, `a5.jtd` | same Mark header, but different `/PageMark`/`/PaperMark` counts (`96` vs `74`) and different `/LineMark` byte lengths |
| `0x0610` | `b6.jtd` | unique in the current MarkV sample set; has `/LineMark`, `/PageMark`, and `/PaperMark` |
| `0x061c` | `a6.jtd`, `shinsyo.jtd` | same Mark header; `/LineMark`, `/PageMark`, and `/PaperMark` are absent in both current samples |

No direct document-length, page-count, or paper-count meaning is proven yet.

The `/LineMark` comparison also weakens a direct-entry-offset interpretation: all 24 Mark entries in the three samples with both streams are outside the `/LineMark` word range. The header value itself remains a candidate for a LineMark-adjacent pointer or boundary because its final `u16` lands inside `/LineMark` near tag clusters.

Exact four-byte searches for `00000603`, `00000610`, and `0000061c` in representative MarkV samples currently only match `/DocumentTextPositionTables` at offset 40. They do not match the observed `/PageLayoutStyle`, `/PageLayoutStyleHeader`, `/DocumentViewStyles`, or other readable streams through `stream-find-bytes`. This weakens a simple global page-style-code interpretation, although it does not rule out a local table field that is derived from page or layout state.

`text-position-style-context` now compares `TCntV.01` tail fields with observed `/TextLayoutStyle` and `/PageLayoutStyle` label candidates using both one-based candidate IDs and zero-based source record indexes. It also compares the same fields with observed `/DocumentViewStyles` group records such as `0x3104..0x3907`. `text-layout-style-records` adds payload-length, digest, BE16, and preview evidence for all observed `/TextLayoutStyle` records. On the current 61 local samples, that diagnostic exits successfully for every sample: 21 samples have no `/TextLayoutStyle`, 1 sample has the stream but no recognized record boundary, 39 samples expose record candidates, and 38 samples expose at least one labeled candidate. One sample currently has a record candidate without a label.

`document-view-style-groups` adds payload-length, digest, and preview evidence for group records so equal group IDs can be compared at the payload level. On the current 61 local samples, that group diagnostic exits successfully for every sample: 56 samples expose all groups 1..9, 2 samples have no `/DocumentViewStyles`, and 3 samples have the stream without the observed group pattern.

`text-position-style-summary` aggregates the same evidence per field. On the current 61 local samples, the summary diagnostic exits successfully for every sample: 10 samples expose `TCntV.01` entries, 8 samples have an `f1` text-style candidate-range hit, and the same 8 samples have an `f1` `/DocumentViewStyles` group hit. 45 samples have no `/DocumentTextPositionTables`, and 1 sample has a stream at that path that does not start with `SsmgV.01`. In the finance samples, fields such as `f1=0x0001`, `f1=0x0005`, and `f1=0x0013` repeatedly fall inside text style candidate ranges. Values like `f1=0x0001`, `f1=0x0003`, and `f1=0x0005` also match `/DocumentViewStyles` groups 1, 3, and 5. However, two non-finance samples with `TCntV.01` entries have zero `/TextLayoutStyle` records and show large `f1` values such as `0x00c5`, `0x00d5`, `0x004f`, and `0x0087`, outside the 1..9 view-style group range. Across the 10 `TCntV.01` samples, view-style group hits occur on `f1` in 8 samples, `f7` in 10 samples, and `f4` in 1 sample. The `f7=0x0001` or `f7=0x0003` pattern is near-constant, so it currently looks more like a default/flag candidate than a per-range style selector. This is not enough to choose candidate-ID vs source-record-index vs view-style-group semantics, and `f1` cannot be a universal TextLayoutStyle reference, so it must remain diagnostic-only.

`text-position-count-tail-field-roles` compares each tail field and adjacent field pair with document-text unit/text hits at deltas 0, 29, and 30, and also searches the best delta in the existing 0..64 range. On the current 61 local samples, the command exits successfully for every sample and reports the same 10 `TCntV.01` entry-bearing samples. In the two non-finance samples, `f1` has strong direct or shifted text-coordinate evidence (`unit-d0=5/text-d0=5` on six entries in `shanai_lan`, and `unit-d29=31/text-d29=30` on fifty entries in `iwata_file`). The adjacent `f1-f2` pair also behaves range-like: its best unit delta is 11 with 12/12 endpoint hits in `shanai_lan`, and 30 with 73/100 endpoint hits in `iwata_file`. In the finance samples, `f1` has no direct delta-0 text hit, but the `f1-f2` pair still finds best unit deltas between 19 and 57, which weakens a pure style-id interpretation. `f7` remains near-constant (`0x0001` or `0x0003`) and has no delta-0 text hits; only the finance-like `0x0001` rows hit unit positions at deltas 29/30, with no text hits. This keeps `f7` closer to default/flag or view-style selector evidence than to a visible text range.

The parser now preserves parsed `/DocumentText` byte/UTF-16 source spans on `TextRun` model data and preserves valid `TCntV.01` entries as decoded-false `textCountRanges` in the document model, JSON export, and app-core `getDocumentInfo`. Each range also exposes byte/unit `documentTextOverlaps` against model text runs when the chosen range intersects visible source text. This keeps the observed range/coordinate evidence available to renderers and future decoders while avoiding premature style or layout attachment. A local JSON export sweep over all 61 samples succeeds with 0 failures; 10 samples have non-empty `textCountRanges`, matching the samples with `TCntV.01` entries, the same 10 samples expose at least one `documentTextOverlaps` entry, and the largest observed counts are 50 ranges and 107 overlaps in one sample.

The parser also preserves the current strict paragraph-boundary discriminator as decoded-false `textParagraphBoundaryCandidates` in the document model, JSON export, and app-core `getDocumentInfo`. The rule requires a strict unit-basis `0x001c` single boundary candidate, a nonzero chosen `TCntV.01` span, and row-local exact endpoint evidence in both `line-word-value` and `page-be32-field`. The same 61-sample JSON export sweep succeeds with 0 failures and preserves 10 candidates total, all from `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd`.

These hypotheses must remain outside semantic document-model generation until they are proven across more streams and samples. They are preserved as decoded-false evidence, not promoted to real paragraph construction.

## Known Gaps

- The meaning of the `TCntV.01` section is not decoded.
- `TCntV.01` entries are preserved as 29-byte raw records. The first two fields look offset-like, but their semantic target is not proven.
- The first two `TCntV.01` fields show mixed byte/unit context behavior across samples and must not drive model generation yet.
- `documentTextOverlaps` records source-text intersections only. It is not proof that a range is a paragraph boundary, style reference, or final layout coordinate.
- `textParagraphBoundaryCandidates` records a stricter nonzero-span plus dual-layout-exact subset, but it is still diagnostic-only and does not prove real paragraph boundaries.
- Tail `t1/t2` fields show a stronger UTF-16 unit-coordinate signal than byte-coordinate signal, but the hit pattern is incomplete and includes some control hits.
- Positive delta scanning over `t1/t2` improves unit hits around delta 29/30, but text hits peak elsewhere, so no single adjustment is proven.
- Grouped delta scanning shows one major `be0` pattern prefers `+29`, shifted patterns prefer `+30/+31`, and the major `0x0202` `be0` pattern is spread across many best deltas.
- Row-level delta scoring shows the major `0x0202` `be0` pattern remains spread across many best deltas even inside one pattern key, so it should not be collapsed into a corrected `+29` text-coordinate family.
- Row-level context inspection shows `0x0202` chosen ranges often touch later body byte ranges while best-delta tail fields usually touch early heading/date text.
- Range-preview inspection shows `0x0202` chosen byte ranges often overlap real `/DocumentText` text entries, but the overlaps include controls and do not explain `t1/t2` as duplicate endpoints.
- Range-boundary inspection shows the major `0x0202` chosen byte ranges mostly contain whole `/DocumentText` map entries and repeatedly include `0x001c`/`0x000e` controls, so plain text extraction alone is not enough to decode the range semantics.
- Style-context inspection shows some `TCntV.01` tail fields land inside observed text/page style candidate ranges and `/DocumentViewStyles` group ranges. Field-level summary makes `f1` stronger than near-constant `f7` as a variable style-reference candidate, but the same values can still match one-based candidate IDs, zero-based source record indexes, and view-style group numbers. These hits must not drive parse-time paragraph or text-run style assignment until the ambiguity is resolved.
- Control-context inspection shows `0x001c` is a high-frequency delimiter candidate and `0x000e` is more control-cluster or inline-adjacent, but neither code has final semantics.
- One observed `TCntV.01` sample contains a shifted `be1` record family. The parser therefore exposes raw bytes and candidate/family/field diagnostics instead of committing to one fixed field layout.
- Current `TCntV.01` range candidates do not appear to be direct `/LineMark`, `/PageMark`, or `/PaperMark` word/row/byte offsets.
- One `/DocumentTextPositionTables` inventory entry has a non-zero size but an out-of-range mini-sector start, so its payload is currently unreadable through the safe CFB/ministream path.
- The meaning of the `MarkV.01` ids is unknown.
- The meaning of the varying `MarkV.01` header value `0x0603`/`0x0610`/`0x061c` is unknown.
- `MarkV.01` entry offsets do not appear to be direct `/LineMark` word indexes in the current samples.
- Offsets are parsed as raw observed values and are compared against byte/unit token maps, but the final coordinate system is not yet proven.
- The observed `unit + 29` probe may be a real adjustment or a sample-specific coincidence.
- The parser is intentionally diagnostic and should not drive document model generation yet.

## Next Steps

- Compare `MarkV.01` offsets with `/DocumentText`, `/LineMark`, `/PageMark`, and extracted text positions.
- Decode why the final Mark header `u16` lands inside `/LineMark` near tag clusters while the Mark entries do not.
- Run `text-position-context` across every sample with parsed `MarkV.01` entries and classify each id.
- Decode the `t0..t9` tail fields inside `TCntV.01` records and determine what coordinate system or object-local structure makes `t1/t2` ordered while not matching the chosen `start/end` span.
- Compare `0x0202` chosen byte-range previews with table/object boundaries inside `/DocumentText` so that broad text overlap is separated from exact paragraph/table range semantics.
- Decode `/DocumentText` controls `0x001c` and `0x000e` before promoting `0x0202` chosen ranges beyond diagnostics.
- Explain the row-local coordinate target that makes the 10 `iwata_file` `textParagraphBoundaryCandidates` exact in both line-word and page-field point sets before constructing real paragraphs from them.
- Identify whether the `be1-shifted` leading byte is a flag, prefix, version, or evidence that the current 29-byte stride starts one byte too early for that subfamily.
- Determine whether the out-of-range mini-sector entry is stale metadata, a malformed ministream chain, or data reachable through another object boundary.
- Decode the bytes between `MarkV.01` and the first entry to determine what `0x0603`, `0x0610`, and `0x061c` represent.
- Identify whether ids map to document marks, pages, sections, or layout regions.
- Preserve any additional tables after new samples show more than the current `MarkV.01` shape.
