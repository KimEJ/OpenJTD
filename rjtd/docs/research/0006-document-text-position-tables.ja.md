# rjtd 研究記録 0006: Position diagnostics and bookmark names

Status: 実装記録。共同形式 RFC ではない

English source: [0006-document-text-position-tables.md](0006-document-text-position-tables.md)

## 実装の範囲

position 診断は raw family と byte/unit 仮説を公開する。既存 parser が MarkV.01 後の 6 bytes を飛ばすことは制限であり header の証明ではない。名前 candidate は公開するが directory value と navigation 座標は未解読。

[形式向け記録](../../../openjtd-spec/rfc/0006-document-text-position-tables.ja.md) は原本観測と未確定解釈を記録する。
Rust type、candidate 条件、JSON、CLI、paint 近似、test、資源方針は実装動作であり、
native semantics の証明ではない。

## 再現の入口

repository root で `cargo build --manifest-path rjtd/Cargo.toml -p rjtd-cli` を実行する。
解析・開示を許可された入力を使い、command の必須引数は `rjtd/target/debug/rjtd --help` を確認する。

```sh
rjtd/target/debug/rjtd text-positions path/to/document.jtd
rjtd/target/debug/rjtd text-position-counts path/to/document.jtd
```

page number、stream、format などを要する command がある。結果には実際の全 command、
入力 ID/hash、作成 version、実装 commit、実行/skip、制限を記録する。private corpus の
検証を列挙しても public 再現性の証明にはならない。

## 過去の根拠

[分離前の完全な記録](https://github.com/OpenJTD/rjtd/blob/cce9cb20611c8c809a7376c8df6130aed14a0437/openjtd-spec/rfc/0006-document-text-position-tables.ja.md) に raw example、command、local sweep、棄却仮説、実装履歴を
固定 commit で保持する。原本観測と実装動作が混在し、古い能力記述もある歴史資料であり、
現行形式の契約や実行可能な public fixture 一式ではない。共有 RFC 0001/0003 の import commit
と status は、この local 編集では変更しない。

## 分離作業

[解釈と描画の境界](../../../docs/ARCHITECTURE.ja.md) に沿い、混在関数を分ける。
本文、source span、unknown data、candidate、資源上限、出力動作を焦点を絞った回帰で保持する。
今回の文書変更で Rust module は移動していない。

## 保存した分離前の記録

以下は元の混合研究記録であり、能力・規定・仮説の記述には当時の状態が含まれる。上記の現在の範囲と区別する。

Status: draft

Observed: 2026-06-18

## Summary

一部の JTD samples は `/DocumentText` の隣に `/DocumentTextPositionTables` stream を expose する。

この stream は同じ ASCII magic で始まる。

```text
SsmgV.01
```

観察済み samples には次の marker strings が含まれる。

```text
TCntV.01
MarkV.01
```

最初に実装した parser は、観察済み `MarkV.01` table だけを big-endian `(u16 id, u32 offset)` entries として decode し、`0xffff` で終端する。

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
cargo run -p rjtd-cli -- stream-meta ../rjtd-testdata/local-samples/ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd /DocumentTextPositionTables
cargo run -p rjtd-cli -- text-map ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-context ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-delta-scan ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-mark-header ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-mark-summary ../rjtd-testdata/local-samples/a5.jtd
cargo run -p rjtd-cli -- text-position-line-context ../rjtd-testdata/local-samples/a5.jtd
```

`text-positions` は parsed `MarkV.01` entries だけを出力する。

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

`text-position-counts` は observed non-Mark `TCntV.01` numeric table を出力する。

```text
header	1	0	3	36	3
entry	0	6630	7147	000019e600001beb0202000100490100000000000000000100000000
entry	1	15742	16240	00003d7e00003f70020200010049010000000000000000010000000000
entry	2	42392	43003	0000a5980000a7fb02020013005b010000000000000000010000000000
```

header columns は現在 `kind`、`reserved`、`declared_count`、`entries_offset`、`parsed_entries` として解釈している。observed entries は stream offset `0x0024` から始まり、first two big-endian `u32` fields を provisional numeric offsets として expose する 29-byte raw records として parse される。

`text-position-count-context` はその first two fields を `/DocumentText` token map に対し byte offsets と UTF-16 unit offsets の両方で比較する。

```text
index	start	end	byte_start_context	byte_end_context	unit_start_context	unit_end_context
```

current samples は single coordinate system を証明しないため、output は意図的に all four contexts を保持する。

`text-position-count-tail-context` は chosen range fields ではなく tail `t1/t2` fields に同じ byte/unit context check を適用する。

`text-position-count-tail-delta-scan` は `t1/t2` を UTF-16 unit offsets として positive `0..64` deltas で scan する。

```text
delta	delta	rows	endpoints	unit_hits	text_hits	both_unit_rows	both_text_rows
```

`text-position-count-tail-delta-groups` はその scan を current `(family,t0,t3,t4,t7)` pattern key ごとにまとめる。

```text
group	family	t0	t3	t4	t7	rows	endpoints	best-unit	best-text	d0	d29	d30
```

`text-position-count-tail-row-deltas` は row ごとの score と document byte/unit length summary を出す。

```text
summary	entries	doc-bytes	doc-units
row	index	family	t0	t3	t4	t7	start	end	span	t1	t2	tspan	best-unit	best-text	d0	d29	d30
```

`text-position-count-tail-row-context` は chosen range contexts と best-delta tail contexts を結合する。

```text
row-context	index	family	t0	t3	t4	t7	start	end	t1	t2	best-unit	best-text	start-byte	end-byte	start-unit	end-unit	t1-unit-best	t2-unit-best	t1-text-best	t2-text-best
```

`text-position-count-range-preview` は chosen range が overlap する `/DocumentText` entries を byte と UTF-16 unit intervals として要約する。

```text
range-preview	index	family	t0	t3	t4	t7	start	end	span	byte-range	unit-range
```

各 range value は token kind ごとの overlapped entry counts と escaped text preview を含む。

```text
entries=N,text=N,inline=N,skipped=N,control=N,preview=...
```

`text-position-count-range-boundaries` は同じ chosen range を byte/UTF-16 unit intervals として調べるが、edge alignment と control delimiters に注目する。

```text
range-boundary	index	family	t0	t3	t4	t7	start	end	span	byte-boundary	unit-boundary
```

各 boundary value は overlapped entries、fully contained count、start/end edge classes、first/last/previous/next map entries、compact `controls=0xNNNN:N` list を報告する。

`text-position-count-control-ranges` は each chosen `TCntV.01` range を all controls または selected control delimiter で分割した `/DocumentText` intervals と比較する。

```text
count-control-range	index	family	delimiter	t0	t3	t4	t7	start	end	span	byte-ranges	unit-ranges
```

各 `byte-ranges` と `unit-ranges` value は chosen range と overlap する control-delimited interval count、first/last interval indexes、combined byte/unit span、entry spans、control-code counts、short preview を報告する。これは correlation diagnostic にすぎず、control code に paragraph semantics を割り当てない。

readable `TCntV.01` files 10 件、89 rows に対する current local sweep:

| Delimiter | Byte interval overlaps | Unit interval overlaps | Byte multi-interval rows | Unit multi-interval rows |
| --- | ---: | ---: | ---: | ---: |
| `0x001c` | 462 | 794 | 40 | 37 |
| `0x000e` | 135 | 195 | 25 | 32 |

working interpretation: `0x001c` は最も強い text/control delimiter candidate のままだが、chosen `TCntV.01` ranges を細かく分割しすぎるため、real paragraph boundaries に直接 promote するには危険である。`0x000e` はより coarse だが、まだ cluster-like に見える。parser/model changes はより強い boundary rule が得られるまで待つべきである。

document model は同じ candidate relationship を decoded-false JSON evidence として保存する。各 valid `textCountRanges` entry は observed delimiter candidates に対する `controlRangeOverlaps` rows を含むことがあり、basis、delimiter code、overlapped interval count、first/last interval indexes、combined source span を報告する。同じ rows は app-core inspection 用に top-level `textBoundaryCandidates` に lift されるが、これらは diagnostic values であり、decoded paragraph records として扱ってはならない。

`rjtd text-boundary-candidates <file>` はこれらの model-derived candidates を直接出力する。

```text
text-boundary-candidate <index> kind=controlDelimitedTextCountRange range=<textCountRangeIndex> basis=<byte|unit> delimiter=<code> intervals=<count> interval-kind=<single|multi> first=<interval> last=<interval> source=<start-end> decoded=false
```

current 61 samples の local sweep では、`TCntV.01` entries を expose する同じ 10 files に candidates があり、356 candidate rows、1,586 overlapped intervals、222 single-interval candidates、134 multi-interval candidates が見つかる。最大 single candidate は `justsystems-20120223023609-jp-just-finance-j200403sc.jtd` の `0x001c`/unit 44 intervals である。したがって `textBoundaryCandidates` は evidence として有用だが、direct paragraph promotion はまだ unsafe である。

`rjtd text-boundary-candidate-context <file>` はこれらの candidates を `/DocumentText` visible text、line breaks、source edge alignment と比較する。

```text
text-boundary-candidate-context <index> range=<textCountRangeIndex> basis=<byte|unit> delimiter=<code> intervals=<count> interval-kind=<single|multi> source=<start-end> line-breaks=<count> text=<range-preview> edges=<edge-summary> decoded=false
```

current context sweep は candidate rows 356、line break を 1 つ以上持つ rows 276、total line breaks 3,458、control gap 直後に始まり aligned text boundary で終わる rows 210 を報告する。`0x001c` single-interval edge-good rows のうち byte basis は one-line-break 17 と zero-line-break 16、unit basis は one-line-break 22 と zero-line-break 13 である。`0x000e` candidates は many line breaks を含むことが多いため、line-break presence だけでは safe paragraph rule ではない。

`rjtd text-boundary-candidate-agreement <file>` は同じ text-count range と delimiter を持つ byte-basis candidates と unit-basis candidates を pair する。

```text
text-boundary-candidate-agreement <index> range=<textCountRangeIndex> delimiter=<code> byte-index=<candidate> unit-index=<candidate> byte-intervals=<count> unit-intervals=<count> byte-edge-good=<bool> unit-edge-good=<bool> byte-line-breaks=<count> unit-line-breaks=<count> text-match=<bool> line-break-match=<bool> byte-text=<preview> unit-text=<preview> decoded=false
```

current agreement sweep では candidates を持つ 10 files で 178 byte/unit pairs が見つかる。exact visible-text match は 1 row だけで、その row も empty であるため、text equality は useful promotion rule ではない。より strict な `0x001c` single/single pair set は 43 pairs で、unit-basis edge-good/non-empty/line-break<=1 は 33 rows、byte-basis は 28 rows を残す。したがって次の paragraph-rule experiment は unit-basis `0x001c` single candidates を最初に評価すべきだが、page/layout evidence が一致するまでは diagnostics に留める。

`rjtd text-boundary-candidate-layout-context <file>` は unit-basis `0x001c` single candidates を `/LineMark`、`/PageMark`、`/PaperMark` の direct index/byte contexts と比較する。

```text
text-boundary-candidate-layout-context <file>
summary unit-001c-single-candidates=<count> rule-selected=<count> line-bytes=<bytes> line-words=<words> page-rows=<rows> page-bytes=<bytes> paper-rows=<rows> paper-bytes=<bytes>
candidate <index> range=<textCountRangeIndex> selected=<bool> edge-good=<bool> non-empty=<bool> line-breaks=<count> source=<unit-start-end> text=<preview> line-word-start=<context> ... paper-byte-end=<context> decoded=false
```

current layout-context sweep では 8 files に unit `0x001c` single candidates 52 と strict rule-selected rows 35 が見つかる。selected rows のうち `/LineMark`、`/PageMark`、`/PaperMark` に start/end direct hits を持つものは 0 である。したがって candidate source units と layout mark rows は同じ coordinate space ではなく、paragraph promotion には別の layout-mark mapping rule が必要である。

`rjtd text-boundary-layout-map <file>` は同じ unit-basis `0x001c` candidates を、複数の global source-unit transforms で sparse layout point sets に対して score する。

```text
text-boundary-layout-map <file>
summary unit-001c-single-candidates=<count> rule-selected=<count> target-sets=<count> bases=<count> delta-range=<min>..<max>
best scope=<all|selected> target=<point-set> base=<unit-transform> delta=<signed-delta> delta-at-boundary=<bool> points=<count> candidates=<count> endpoints=<count> valid=<count> invalid=<count> exact=<count> total-distance=<sum|-> max-distance=<max|-> decoded=false
```

current map sweep は local samples 61 件すべてで成功する。同じ 8 files に unit `0x001c` single candidates 52、4 files に strict selected candidates 35 が見つかる。non-boundary exact hits は存在するが、single global transform には収束しない。`iwata_file` は `line-word-value` と `page-be32-field` を unit-div2 shifts around `-1140..-1192` で好み、selected finance samples は異なる `page-be32-field` shifts を好む。したがって paragraph promotion は file-global source-unit transform ではなく、row-local、section-local、または record-local base offsets を探す必要がある。

`rjtd text-boundary-layout-map-rows <file>` は unit-basis `0x001c` candidate ごとに独立して score し、linked `TCntV.01` row summary を含める。

```text
text-boundary-layout-map-rows <file>
summary unit-001c-single-candidates=<count> rule-selected=<count> target-sets=<count> bases=<count> local-rows=<count>
local candidate=<candidateIndex> range=<textCountRangeIndex> selected=<bool> target=<point-set> base=<unit-transform> delta=<signed-delta> delta-at-boundary=<bool> exact=<0..2> total-distance=<sum|-> max-distance=<max|-> start-nearest=<source:mapped->nearest:d> end-nearest=<source:mapped->nearest:d> source=<unit-start-end> text=<preview> tcnt=<row-summary> decoded=false
```

row-local sweep も local samples 61 件すべてで成功する。unit `0x001c` single candidates 52 と strict selected candidates 35 が報告される。`iwata_file` の strict selected candidates 32 のうち 10 件は `line-word-value` と `page-be32-field` の両方で row-local `exact=2` evidence を持つ。一方、strict selected finance candidates 3 は row-local `exact=2` evidence を持たない。したがって strict edge/text rule だけでは不十分であり、future paragraph rule は paragraph-like `iwata_file` rows と large spans を分離する layout-local discriminator を必要とする。

`rjtd text-boundary-paragraph-like <file>` はこの discriminator を diagnostic-only output として適用する。

```text
text-boundary-paragraph-like <file>
summary unit-001c-single-candidates=<count> strict-selected=<count> paragraph-like=<count> selected-non-paragraph-like=<count> rule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2 decoded=false
candidate <index> range=<textCountRangeIndex> strict-selected=<bool> paragraph-like=<bool> line-word-evidence=<evidence|-> page-field-evidence=<evidence|-> source=<unit-start-end> text=<preview> tcnt=<row-summary> decoded=false
```

current classifier sweep は 61 samples で 0 failures。unit `0x001c` single candidates 52、strict selected candidates 35、paragraph-like candidates 10、strict selected but non-paragraph-like candidates 25 を報告する。この rule で paragraph-like candidates を出すのは `iwata_file` だけである。これらの rows はまだ evidence であり、decoded paragraph construction ではない。

`rjtd text-boundary-paragraph-like-style-context <file>` はこの classifier に linked `TCntV.01` tail fields と既存の text/page/view-style diagnostics を結合する。

```text
text-boundary-paragraph-like-style-context <file>
summary unit-001c-single-candidates=<count> strict-selected=<count> paragraph-like=<count> selected-non-paragraph-like=<count> text-style-candidates=<count> page-style-candidates=<count> view-style-records=<count> rule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2 decoded=false
candidate <index> range=<textCountRangeIndex> strict-selected=<bool> paragraph-like=<bool> line-word-evidence=<evidence|-> page-field-evidence=<evidence|-> tail-fields=<fields> text-style-id-hits=<hits|-> text-style-index-hits=<hits|-> page-style-id-hits=<hits|-> page-style-index-hits=<hits|-> view-style-group-hits=<hits|-> byte-range=<preview> unit-range=<preview> source=<unit-start-end> text=<preview> tcnt=<row-summary> decoded=false
```

current style-context sweep も 61 samples で 0 failures であり、candidate counts は同じである。unit candidates 52、strict selected candidates 35、paragraph-like candidates 10、selected non-paragraph-like candidates 25。paragraph-like rows 10 件は `/TextLayoutStyle` や `/PageLayoutStyle` candidate hits を持たないが、すべて `iwata_file` の `/DocumentViewStyles` group evidence を持つ。strict non-paragraph rows も 25/25 で view-group hit を持つため、これは paragraph discriminator ではない。広い `TCntV.01` style summaries では `f7` が near-constant であるため、これらの hits は default/flag-like evidence に留めるべきであり、paragraph style attachment を証明しない。

`rjtd text-boundary-paragraph-like-discriminators <file>` は同じ candidates を bucket ごとに要約する。

```text
text-boundary-paragraph-like-discriminators <file>
summary unit-001c-single-candidates=<count> strict-selected=<count> paragraph-like=<count> selected-non-paragraph-like=<count> rule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2 decoded=false
bucket <paragraph-like|strict-non-paragraph|non-strict> rows=<count> strict-selected=<count> line-word-exact2=<count> page-field-exact2=<count> dual-exact2=<count> text-style-hit=<count> page-style-hit=<count> view-style-group-hit=<count> missing-tcnt=<count> source-spans=<min..max> range-spans=<min..max> families=<counts> f0=<counts> f4=<counts> f7=<counts> line-evidence=<counts> page-evidence=<counts> decoded=false
```

current discriminator sweep は 61 samples で 0 failures。dual exact layout evidence は paragraph-like rows だけに現れる。paragraph-like は 10/10、strict-non-paragraph は 0/25、non-strict は 0/17 である。`iwata_file` では paragraph-like bucket は `be0:10` かつ `range-spans=2..8` だが、strict-non-paragraph rows は `range-spans=0..0` で `be0` と `be1-shifted` が混在する。したがって nonzero chosen `TCntV.01` span と row-local dual layout exactness が現在もっとも強い discriminator である。ただし coordinate target はまだ説明できていないため、decoded-false evidence に留める。

`text-position-count-clusters` は `TCntV.01` records を provisional `(start, end)` pair で group し、duplicate raw-tail variants を報告する。`text-position-count-candidates` は raw first bytes を二つの candidate interpretations として出力する。

```text
index	be0_start	be0_end	be1_start	be1_end	raw
```

`be0` は raw offsets `0` and `4` の big-endian `u32` を意味する。`be1` は raw offsets `1` and `5` の shifted big-endian `u32` を意味する。どちらも diagnostic candidates であり、stable field names ではない。

`text-position-count-family` は current conservative family split を適用し、`chosen_start`、`chosen_end`、both candidate pairs、leading byte、remaining raw tail を出力する。

```text
family	index	family	chosen_start	chosen_end	be0_start	be0_end	be1_start	be1_end	lead	tail
```

`text-map` は `/DocumentText` token ranges を出力する。

```text
byte_start	byte_end	unit_start	unit_end	kind	meta	byte_marks	unit_marks	text_preview
344	348	172	174	text	-	-	-	一、
```

`text-position-context` は each `MarkV.01` offset を raw byte offset、raw UTF-16 unit offset、provisional `unit + 29` offset として token map と比較する。

`text-position-delta-scan` は parsed `MarkV.01` entries に対して UTF-16 unit deltas `0..64` を score する。

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

`unit + 29` probe は diagnostic comparison にすぎず、stable coordinate rule ではない。

parsed `MarkV.01` entries を持つ five local samples 全体で、ids 2、4、5、7、8 は同じ `unit + 29` probe により visible section heading text に一貫して着地する。ids 3 と 6 は body text または nearby body-text boundaries に着地する。id 1 は不明で、この probe では control/inline boundaries 付近に落ちることが多い。

broader delta scan は `29` が unique stable adjustment であるという考えを弱める。同じ five files (`46.jtd`, `a5.jtd`, `a6.jtd`, `b6.jtd`, `shinsyo.jtd`) と 40 total `MarkV.01` entries に対する top scores:

| Delta | Unit hits | Visible text hits |
| ---: | ---: | ---: |
| 9 | 34 | 31 |
| 29 | 31 | 31 |
| 30 | 31 | 31 |
| 31 | 31 | 26 |

したがって `unit + 29` は useful probe だが、table/header adjustment として証明されていない。competing `unit + 9` と adjacent `unit + 30` scores は、section-local bases、未解読 record boundaries、または broad text spans によって複数の近い deltas が plausible に見えることを示している可能性がある。

`text-position-line-context` は `MarkV.01` header と entries を `/LineMark` word positions と比較する。`/LineMark` word count、tag count、parsed Mark entry count、parsed `/PageMark` and `/PaperMark` row counts、各 Mark value の nearest `0x1000`/`0x1001`/`0x1002` tag rows を報告する。

current 61 local samples:

| Metric | Count |
| --- | ---: |
| files with both readable `/LineMark` and parsed `MarkV.01` | 3 |
| files missing `/LineMark` before this comparison can run | 6 |
| files missing `/DocumentTextPositionTables` | 41 |
| files with `/DocumentTextPositionTables` but no parsed `MarkV.01` | 11 |
| `MarkV.01` entry offsets checked against `/LineMark` | 24 |
| `MarkV.01` entry offsets inside `/LineMark` word range | 0 |

comparison が実行できる three files:

| Sample | line words | line tags | Mark entries | Page rows | Paper rows | Mark header line index | LineMark word at header index | nearest tag rows |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `46.jtd` | 2597 | 491 | 8 | 97 | 97 | 1539 | `0x0029` | `0x1002@1520`, `0x1000@1564` |
| `a5.jtd` | 2561 | 511 | 8 | 75 | 75 | 1539 | `0x0016` | `0x1002@1534`, `0x1002@1542` |
| `b6.jtd` | 2667 | 502 | 8 | 98 | 98 | 1552 | `0x0000` | `0x1000@1548`, `0x1002@1554` |

working interpretation: `MarkV.01` entry offsets は direct `/LineMark` word indexes ではない。six-byte Mark header の final big-endian `u16` は両 streams を expose する three samples で `/LineMark` 内に入るが、stable tag value には落ちず、まだ decode されていない。

## Observed Bytes

`a5.jtd` begins:

```text
5373 6d67 562e 3031 0000 0001 0000 0100
0000 0001 5443 6e74 562e 3031 0000 4d61
726b 562e 3031 0000 0000 0603 0001 0000
5cfa 0002 0000 1d58 ...
```

initial working interpretation:

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

`0x00000603` は現時点では unknown table header value として保存する。entry count としては扱わない。

## Local Sample Results

Current local sweep:

```text
checked=61 with_position_tables=16 with_mark_entries=5 with_tcnt_entries=10 empty_or_unreadable_position_payload=1
```

すべての readable JTD/JTT/JTTC samples がこの stream を expose するわけではない。

initial `MarkV.01` parser は現在、次の local samples でそれぞれ 8 entries を見つける。

```text
46.jtd
a5.jtd
a6.jtd
b6.jtd
shinsyo.jtd
```

`/DocumentTextPositionTables` を持つ残りの files は、10 件の observed `TCntV.01` numeric tables と、stream size は non-zero だが safe CFB/ministream path では payload が empty と読まれる inventory entry 1 件に分かれる。

empty-payload sample:

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

mini-sector `224` は byte `224 * 64 = 14336` から始まるため、observed 7680-byte mini stream の範囲外を指す。rjtd は regular-sector fallback を推測せず、この payload を unreadable/empty position-table payload として保存する。

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

これは mixed evidence である。JustSystems finance samples は byte-oriented `/DocumentText` positions に近く見えることが多い一方、`ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` は UTF-16 unit contexts でより多くの text hits を出す。この table は coordinate systems を混在させているか、layout/object-local regions を指しているか、current `DocumentText` token map の gap を expose している可能性がある。

Candidate-family sweep:

| Sample group | Entries | `be0` plausible against `/DocumentText` bytes | `be1` plausible against `/DocumentText` bytes |
| --- | ---: | ---: | ---: |
| 9 current non-`iwata_file` `TCntV.01` samples | 39 | 39 | 0 |
| `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` | 50 | 32 | 18 |

`iwata_file` は少なくとも二つの `TCntV.01` record families を含むように見える。entries 0-31 は provisional `be0` interpretation に合う。entries 32-49 は `be0` では implausible (`150`, huge end values など) だが、`be1` では自然な offsets になる。

```text
index	be0_start	be0_end	be1_start	be1_end
32	150	3388997782	38602	38602
33	150	3388997782	38602	38602
34	151	805306519	38704	38704
```

`text-position-count-family` は current samples 全体でこの split を確認する。10 files が `TCntV.01` を expose し、89 records total、71 は `be0`、18 は `be1-shifted`。shifted record はすべて `iwata_file` entries 32-49 である。

同じ sample には異なる raw tails を持つ duplicate provisional ranges がある。`text-position-count-clusters` は 38 clusters を報告し、そのうち 12 は duplicate clusters である。これは hidden subfields after offset candidates が repeated logical spans を区別していることを示唆する。

`text-position-count-fields` は chosen range 後の各 record を positional labels (`t0` through `t9`) の `u16be` tail fields と extra trailing byte に展開する。これは observation labels であり semantic field names ではない。

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

shifted family は offset interpretation が shifted であるだけではない。tail shape も cleaner で、current samples の 18 shifted records すべてで final seven fields が fixed である。

`text-position-count-field-deltas` は chosen family range span と tail `t1..t2` span を比較し、chosen start/end から `t1/t2` への signed deltas を出力する。diagnostic only。

current local samples:

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

`t1/t2` は ordered range-like pair に見えるが、chosen `start/end` range と同じ coordinate span ではない。

同じ `t1/t2` pair を `text-position-count-tail-context` で `/DocumentText` と比較した。

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

unit-coordinate signal は byte-coordinate signal より強いが完全ではない。一部の unit hits は control entries に着地し、40 rows は `t1`/`t2` どちらにも direct unit hit がない。

`t1/t2` を UTF-16 unit offsets として positive `0..64` deltas で scan した aggregate checkpoints:

| Delta | Rows | Endpoints | Unit endpoint hits | Text endpoint hits | Rows with both unit hits | Rows with both text hits |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 89 | 178 | 77 | 70 | 28 | 26 |
| 9 | 89 | 178 | 74 | 69 | 28 | 27 |
| 29 | 89 | 178 | 124 | 98 | 49 | 32 |
| 30 | 89 | 178 | 124 | 95 | 47 | 30 |
| 53 | 89 | 178 | 105 | 102 | 42 | 42 |
| 64 | 89 | 178 | 93 | 83 | 28 | 28 |

Delta 29 と 30 は current `0..64` scan で highest unit endpoint hit count に tie し、delta 29 は text endpoint と both-unit row counts で delta 30 より少し強い。しかし text endpoint hits は delta 53 で peak する。これは tail fields と MarkV-style `+29` candidate の関係を強めるが、single stable adjustment はまだ証明しない。

同じ scan を `(family,t0,t3,t4,t7)` で group すると aggregate result が混ざる理由が見える。

| Pattern | Rows | Unit best | Text best | Observation |
| --- | ---: | --- | --- | --- |
| `be0,0x0101,0x0100,0x0001,0x0001` | 28 | `29` | `29` | strongest `+29` group; all current rows are in `iwata_file` |
| `be1-shifted,0x0101,0x0100,0x0001,0x0001` | 16 | `31` | `30` | shifted family is close to, but not identical with, the `+29` group |
| `be1-shifted,0x0202,0x0100,0x0001,0x0001` | 2 | `30` | `30` | small shifted subgroup |
| `be0,0x0202,0x0100,0x0000,0x0001` | 28 | spread | spread | rows are spread across multiple files and best deltas (`19`, `23`, `30`, `36`, `46`, `49`, `56`, `57`, etc.) |
| `be0,0x0202,0x0102,0x0000,0x0001` | 5 | mixed | mixed | small group; current rows prefer `29`, `39`, or `50` depending on file |

これにより tail patterns は diagnostic subfamilies として扱うべきである。global `+29` rule は一つの major group を説明できても、shifted と `0x0202` behavior を隠してしまう。

Row-level delta scoring はさらに注意を促す。major `be0,0x0202,0x0100,0x0000,0x0001` pattern は 8 files にまたがる 28 rows を持ち、current samples では次の通り。

| Metric | Observed range/count |
| --- | ---: |
| rows | 28 |
| files | 8 |
| chosen span range | `398..1212` |
| tail `t2 - t1` range | `46..72` |
| document unit lengths | `65889..146657` |
| distinct row-level best unit deltas | 17 |

この spread は single file-level または global correction には見えない。row-local structure、missing intermediate records、または `0x0202` family の別 coordinate/object-local target を示す可能性が高い。

context-level inspection もその split を補強する。

| Scope | Rows | Files | Chosen start byte | Chosen end byte | Chosen start unit | Chosen end unit | Best tail `t1` | Best tail `t2` |
| --- | ---: | ---: | --- | --- | --- | --- | --- | --- |
| all `t0=0x0202` | 36 | 10 | text 11, boundary 21, control/other 4 | text 15, boundary 19, control 2 | text 4, boundary 31, other 1 | text 1, boundary 34, other 1 | text 25, control 8, inline 3 | text 35, control 1 |
| major `be0,0x0202,0x0100,0x0000,0x0001` | 28 | 8 | text 9, boundary 17, control/other 2 | text 12, boundary 14, control 2 | text 3, boundary 24, other 1 | text 1, boundary 27 | text 21, control 6, inline 1 | text 27, control 1 |

chosen range は later body byte ranges または nearby body boundaries に着地することが多い一方、best-delta tail fields は early heading/date text に着地することが多い。したがって `start/end` と `t1/t2` を一つの text coordinate system の duplicate endpoints と扱うべきではない。

Range-preview inspection は有用な区別を加える。`0x0202` chosen ranges は direct layout-stream coordinates のようには振る舞わず、tail `t1/t2` span とも一致しないが、real `/DocumentText` byte ranges を cover することが多い。

| Scope | Rows | Files | Byte range overlaps text | Unit range overlaps text |
| --- | ---: | ---: | ---: | ---: |
| all `t0=0x0202` | 36 | 10 | 31 | 25 |
| `be0,0x0202,0x0100,0x0000,0x0001` | 28 | 8 | 25 | 21 |
| `be0,0x0202,0x0102,0x0000,0x0001` | 5 | 3 | 5 | 3 |
| `be0,0x0202,0x0100,0x0000,0x0003` | 1 | 1 | 1 | 1 |
| `be1-shifted,0x0202,0x0100,0x0001,0x0001` | 2 | 1 | 0 | 0 |

major group では 25/28 rows が chosen byte range で text と overlap し、21/28 rows が chosen unit range で text と overlap する。多くの overlaps には text だけでなく control entries も含まれるため clean extracted-text span ではない。それでもこの group では direct `/LineMark`、`/PageMark`、`/PaperMark` coordinates より `/DocumentText` byte intervals が次の target として強い。

major group の boundary inspection:

| Metric | Byte interval | UTF-16 unit interval |
| --- | ---: | ---: |
| rows | 28 | 28 |
| overlapped map entries total | 535 | 1031 |
| fully contained entries | 513 | 1026 |
| partial entries | 22 | 5 |
| rows containing controls | 25 | 22 |
| start edge aligned / inside / gap | `1 / 10 / 17` | `0 / 4 / 24` |
| end edge aligned / inside / gap | `1 / 12 / 15` | `3 / 1 / 24` |

byte interpretation はこの group では unit interpretation より narrow である。start/end は control boundaries 周辺の gaps に落ちることが多いが、overlapped byte entries の大半は fully contained である。current control-code totals:

| Control code | Byte interval count | UTF-16 unit interval count |
| --- | ---: | ---: |
| `0x001c` | 281 | 522 |
| `0x000e` | 38 | 73 |
| `0x001d` | 2 | 6 |
| `0x0000` | 1 | 15 |
| `0x000c` | 3 | 0 |

次の `0x0202` investigation は plain extracted text positions ではなく、特に `0x001c` と `0x000e` を含む `/DocumentText` control-delimited structures に向けるべきである。

follow-up `text-control-context` sweep はその二つの controls を優先する根拠を補強する。61 local samples 全体で error なく動き、60 files に mapped controls を見つけ、`0x001c` を 60 files で 51,971 回、`0x000e` を 41 files で 6,621 回報告する。`0x001c` は text/text、text/control、control/text、control/control neighbors の間に多く、strong generic delimiter candidate である。`0x000e` は control/control、text/control、text/skipped-inline contexts に多く、control-cluster または inline-adjacent delimiter らしく見える。これらはまだ diagnostic profiles で、final semantic names ではない。

`text-position-count-layout-context` は各 record の chosen `TCntV.01` range を `/LineMark` word offsets、`/LineMark` byte offsets、parsed `/PageMark` rows/bytes、parsed `/PaperMark` rows/bytes と比較する。

current local samples:

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

これは current samples において chosen `TCntV.01` ranges を direct layout-stream coordinates と解釈することを reject する。fields は `/DocumentText`、raw stream offsets として expose されない layout-local coordinate system、またはまだ decode されていない別 table/object boundary を参照している可能性がある。

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

raw `MarkV.01` offsets は extracted plain-text character offsets のようには振る舞わない。例えば `a5.jtd` は extracted text が約 39k characters だが、observed offset の一つは `40217` である。

raw byte offsets も弱い。多くの offsets は odd であり、UTF-16BE unit の途中に入る。

最も強い current hypothesis は、`MarkV.01` offsets が UTF-16 unit または internal `/DocumentText` coordinates であるというもの。current five parsed samples では、いくつかの offsets に 29 UTF-16 units を足すと visible section heading text 付近に着地する。ただし broader delta scan は 9 と 30 も competitive であることを示す。これは table-local header adjustment、section-local coordinate base、またはまだ decode されていない record boundary を示している可能性がある。

`MarkV.01` と first observed entry の間の six bytes は samples 間で constant ではない。

```text
46.jtd      0000 0000 0603
a5.jtd      0000 0000 0603
a6.jtd      0000 0000 061c
b6.jtd      0000 0000 0610
shinsyo.jtd 0000 0000 061c
```

これらの values は constant 29-unit probe を直接説明しない。table がさらに decode されるまでは unknown table header fields として保存する。

`text-position-mark-header` はこの領域を直接 expose する。current five MarkV samples では、`MarkV.01` marker 自体は常に stream offset 30、six-byte header は常に `00000000` で始まり、final big-endian `u16` は `0x0603`、`0x0610`、`0x061c` のいずれかである。

`text-position-mark-summary` はこの header を nearby streams と相関させる。current five-sample sweep は単純な解釈を弱める。

| Header | Samples | Related observation |
| --- | --- | --- |
| `0x0603` | `46.jtd`, `a5.jtd` | same Mark header, but different `/PageMark`/`/PaperMark` counts (`96` vs `74`) and different `/LineMark` byte lengths |
| `0x0610` | `b6.jtd` | unique in the current MarkV sample set; has `/LineMark`, `/PageMark`, and `/PaperMark` |
| `0x061c` | `a6.jtd`, `shinsyo.jtd` | same Mark header; `/LineMark`, `/PageMark`, and `/PaperMark` are absent in both current samples |

direct document-length、page-count、paper-count meaning はまだ証明されていない。

`/LineMark` comparison も direct-entry-offset interpretation を弱める。両 streams を持つ three samples の all 24 Mark entries は `/LineMark` word range 外である。header value 自体は LineMark-adjacent pointer または boundary candidate として残る。final `u16` が `/LineMark` 内の tag clusters 近くに入るためである。

representative MarkV samples で `00000603`、`00000610`、`0000061c` を exact four-byte search すると、現在は `/DocumentTextPositionTables` offset 40 だけに match する。observed `/PageLayoutStyle`、`/PageLayoutStyleHeader`、`/DocumentViewStyles`、その他 readable streams には `stream-find-bytes` で match しない。これは simple global page-style-code interpretation を弱めるが、page/layout state から derivation された local table field は否定しない。

`text-position-style-context` は `TCntV.01` tail fields と observed `/TextLayoutStyle`・`/PageLayoutStyle` label candidates を、one-based candidate IDs と zero-based source record indexes の両方で比較する。同じ fields を `0x3104..0x3907` のような observed `/DocumentViewStyles` group records とも比較する。`text-layout-style-records` は all observed `/TextLayoutStyle` records に payload-length、digest、BE16、preview evidence を追加する。current 61 local samples では、この diagnostic は全 samples で成功する。21 samples は `/TextLayoutStyle` を持たず、1 sample は stream を持つが recognized record boundary を持たず、39 samples は record candidates を expose し、38 samples は labeled candidate を少なくとも 1 つ expose する。現在 1 sample は label のない record candidate を持つ。

`document-view-style-groups` は group records に payload-length、digest、preview evidence を追加し、同じ group IDs を payload level で比較できるようにする。current 61 local samples では、この group diagnostic は全 samples で成功する。56 samples は groups 1..9 をすべて expose し、2 samples は `/DocumentViewStyles` を持たず、3 samples は stream を持つが observed group pattern を持たない。

`text-position-style-summary` は同じ evidence を field ごとに aggregate する。current 61 local samples では summary diagnostic は全 samples で成功する。10 samples が `TCntV.01` entries を expose し、8 samples は `f1` text-style candidate-range hit を持ち、同じ 8 samples は `f1` `/DocumentViewStyles` group hit も持つ。45 samples は `/DocumentTextPositionTables` を持たず、1 sample はその path の stream が `SsmgV.01` で始まらない。finance samples では `f1=0x0001`、`f1=0x0005`、`f1=0x0013` などが text style candidate ranges 内に繰り返し入る。`f1=0x0001`、`f1=0x0003`、`f1=0x0005` のような values は `/DocumentViewStyles` groups 1、3、5 にも match する。しかし、`TCntV.01` entries を持つ two non-finance samples は `/TextLayoutStyle` records が 0 で、`0x00c5`、`0x00d5`、`0x004f`、`0x0087` のような large `f1` values が 1..9 の view-style group range 外に現れる。10 `TCntV.01` samples 全体では view-style group hits は `f1` で 8 samples、`f7` で 10 samples、`f4` で 1 sample に現れる。`f7=0x0001` または `f7=0x0003` pattern は near-constant であるため、現時点では per-range style selector より default/flag candidate に見える。ただし candidate-ID semantics、source-record-index semantics、view-style-group semantics のどれかを選ぶには不十分で、`f1` は universal TextLayoutStyle reference ではあり得ないため、diagnostic-only に留める。

`text-position-count-tail-field-roles` は各 tail field と adjacent field pair を deltas 0、29、30 の document-text unit/text hits と比較し、既存の 0..64 range でも best delta を探す。current 61 local samples では command は全 samples で成功し、同じ 10 `TCntV.01` entry-bearing samples を報告する。two non-finance samples では `f1` が強い direct または shifted text-coordinate evidence を持つ。`shanai_lan` の six entries では `unit-d0=5/text-d0=5`、`iwata_file` の fifty entries では `unit-d29=31/text-d29=30` である。adjacent `f1-f2` pair も range-like に振る舞う。`shanai_lan` では best unit delta が 11 で 12/12 endpoint hits、`iwata_file` では 30 で 73/100 endpoint hits である。finance samples では `f1` に direct delta-0 text hit はないが、`f1-f2` pair は 19..57 の best unit deltas を見つけるため、pure style-id interpretation は弱まる。`f7` は near-constant (`0x0001` または `0x0003`) のままで delta-0 text hits を持たない。finance-like `0x0001` rows だけが deltas 29/30 で unit positions に hit するが、text hits はない。これは `f7` を visible text range より default/flag または view-style selector evidence に近いものとして残す。

parser は parsed `/DocumentText` byte/UTF-16 source span を `TextRun` model data に保存し、valid `TCntV.01` entries を decoded-false `textCountRanges` として document model、JSON export、app-core `getDocumentInfo` に保存する。各 range は chosen range が visible source text と交差する場合、model text runs に対する byte/unit `documentTextOverlaps` も expose する。これにより observed range/coordinate evidence を renderers と future decoders が利用できる一方、style または layout attachment を早まらない。local 61 samples の JSON export sweep は 0 failures で成功する。10 samples が non-empty `textCountRanges` を持ち、これは `TCntV.01` entries を持つ samples と一致し、同じ 10 samples が少なくとも 1 件の `documentTextOverlaps` を expose する。最大 observed counts は 1 sample の 50 ranges と 107 overlaps である。

parser は現在の strict paragraph-boundary discriminator も decoded-false `textParagraphBoundaryCandidates` として document model、JSON export、app-core `getDocumentInfo` に保存する。この rule は strict unit-basis `0x001c` single boundary candidate、nonzero chosen `TCntV.01` span、`line-word-value` と `page-be32-field` の両方に row-local exact endpoint evidence があることを要求する。同じ 61-sample JSON export sweep は 0 failures で、合計 10 candidates を保存する。すべて `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` 由来である。

これらの hypotheses はより多くの streams と samples で証明されるまで semantic document-model generation の外に置く。decoded-false evidence として保存するだけで、real paragraph construction には promote しない。

## Known Gaps

- `TCntV.01` section の意味は decode されていない。
- `TCntV.01` entries は 29-byte raw records として保存される。first two fields は offset-like に見えるが、semantic target は未証明。
- first two `TCntV.01` fields は samples 間で mixed byte/unit context behavior を示すため、まだ model generation を駆動してはならない。
- `documentTextOverlaps` は source-text intersections だけを記録する。paragraph boundary、style reference、final layout coordinate の証明ではない。
- `textParagraphBoundaryCandidates` は nonzero span plus dual layout exact evidence を満たす stricter subset を記録するが、まだ diagnostic-only であり real paragraph boundaries の証明ではない。
- tail `t1/t2` fields は byte-coordinate signal より強い UTF-16 unit-coordinate signal を示すが、hit pattern は incomplete で control hits も含む。
- `t1/t2` の positive delta scanning は delta 29/30 付近で unit hits を改善するが、text hits は elsewhere で peak するため single adjustment は未証明。
- grouped delta scanning は one major `be0` pattern が `+29` を好み、shifted patterns は `+30/+31`、major `0x0202` `be0` pattern は many best deltas に分散することを示す。
- row-level delta scoring は major `0x0202` `be0` pattern が one pattern key 内でも many best deltas に分散することを示すため、corrected `+29` text-coordinate family に collapse してはならない。
- row-level context inspection は `0x0202` chosen ranges が later body byte ranges に触れることが多く、best-delta tail fields は early heading/date text に触れることが多いことを示す。
- range-preview inspection は `0x0202` chosen byte ranges が real `/DocumentText` text entries と overlap することを示すが、その overlaps は controls を含み、`t1/t2` を duplicate endpoints として説明しない。
- range-boundary inspection は major `0x0202` chosen byte ranges が mostly whole `/DocumentText` map entries を含み、`0x001c`/`0x000e` controls を繰り返し含むことを示す。plain text extraction だけでは range semantics を decode できない。
- style-context inspection は一部の `TCntV.01` tail fields が observed text/page style candidate ranges と `/DocumentViewStyles` group ranges に入ることを示す。field-level summary は near-constant な `f7` より `f1` の方が variable style-reference candidate として強いことを示すが、同じ values は one-based candidate IDs、zero-based source record indexes、view-style group numbers に match し得る。この ambiguity が解決されるまで、parse-time paragraph/text-run style assignment を駆動してはならない。
- control-context inspection は `0x001c` を high-frequency delimiter candidate、`0x000e` をより control-cluster or inline-adjacent と示すが、どちらも final semantics はない。
- observed `TCntV.01` sample の一つは shifted `be1` record family を含む。parser は一つの fixed field layout に commit せず、raw bytes と candidate/family/field diagnostics を expose する。
- current `TCntV.01` range candidates は direct `/LineMark`、`/PageMark`、`/PaperMark` word/row/byte offsets ではなさそうである。
- one `/DocumentTextPositionTables` inventory entry は non-zero size を持つが out-of-range mini-sector start のため、safe CFB/ministream path では payload が unreadable。
- `MarkV.01` ids の meaning は unknown。
- varying `MarkV.01` header value `0x0603`/`0x0610`/`0x061c` の meaning は unknown。
- current samples では `MarkV.01` entry offsets は direct `/LineMark` word indexes ではなさそうである。
- offsets は raw observed values として parse し、byte/unit token maps と比較するが、final coordinate system は未証明。
- observed `unit + 29` probe は real adjustment か sample-specific coincidence か未確定。
- parser は意図的に diagnostic であり、まだ document model generation を駆動すべきではない。

## Next Steps

- `MarkV.01` offsets を `/DocumentText`、`/LineMark`、`/PageMark`、extracted text positions と比較する。
- final Mark header `u16` が `/LineMark` tag clusters 近くに入る一方で Mark entries が入らない理由を decode する。
- parsed `MarkV.01` entries を持つ every sample で `text-position-context` を実行し、各 id を分類する。
- `TCntV.01` records 内の `t0..t9` tail fields を decode し、`t1/t2` が ordered でありながら chosen `start/end` span と一致しない coordinate system または object-local structure を判断する。
- `0x0202` chosen byte-range previews を `/DocumentText` 内の table/object boundaries と比較し、broad text overlap と exact paragraph/table range semantics を分離する。
- `0x0202` chosen ranges を diagnostics から昇格する前に、`/DocumentText` controls `0x001c` と `0x000e` を decode する。
- `iwata_file` の 10 `textParagraphBoundaryCandidates` が line-word と page-field point sets の両方で exact になる row-local coordinate target を説明してから real paragraphs を構築する。
- `be1-shifted` leading byte が flag、prefix、version、または current 29-byte stride がその subfamily では one byte too early である証拠かを特定する。
- out-of-range mini-sector entry が stale metadata、malformed ministream chain、または another object boundary から到達可能な data かを判断する。
- `MarkV.01` と first entry の間の bytes を decode し、`0x0603`、`0x0610`、`0x061c` が何を表すか決める。
- ids が document marks、pages、sections、layout regions のいずれに map するか特定する。
- new samples が current `MarkV.01` shape を超える tables を示したら、追加 tables も保存する。
