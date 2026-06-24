# RFC 0007: Layout Mark Streams Initial Inventory

Status: draft

Observed: 2026-06-18

## Summary

一部の JTD samples は `/DocumentText` の隣に layout-oriented streams を expose する。

```text
/LineMark
/PageMark
/PaperMark
```

これらの streams はまだ document model へ parse されていない。`DocumentText` record structure がより理解された後に page、line、paper、anchor positions を説明する可能性がある layout/cache candidates である。

## Local Samples

current local samples のうち、three streams すべてを持つもの:

| Sample | `/LineMark` bytes | `/PageMark` bytes | `/PaperMark` bytes |
| --- | ---: | ---: | ---: |
| `46.jtd` | 5194 | 8160 | 788 |
| `a5.jtd` | 5122 | 6312 | 612 |
| `b6.jtd` | 5334 | 8244 | 796 |
| `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | 190 | 272 | 108 |

`a6.jtd` と `shinsyo.jtd` は current inventory ではこれらの streams を expose しない。

## PageMark Observation

observed large samples では first 12 bytes が compact header に見える。raw `u32be` diagnostics は `stream-dwords` と `stream-dword-frequencies` で確認できる。

```text
46.jtd  00000060 00000010 0000005f
a5.jtd  0000004a 00000010 00000049
b6.jtd  00000062 00000010 00000061
```

Working interpretation:

- first `u32be`: count-like value。
- second `u32be`: `0x10`。size または stride-like value の可能性が高い。
- third `u32be`: first value minus one。
- next `u32be` は observed 84-byte row family における row 0 の index-like value。

total stream size は simple `header + count * 0x10` formula には還元されない。ただし current large samples は一つの stable family を証明している。

```text
12-byte header
N rows of 84 raw bytes
```

各 84-byte row の first dword は index-like value である。remaining row は field semantics が decode されていないため raw bytes として保存する。

| Sample | header value 0 | header value 1 | header value 2 | rows | stream length formula |
| --- | ---: | ---: | ---: | ---: | --- |
| `46.jtd` | 96 | 16 | 95 | 97 | `12 + 97 * 84 = 8160` |
| `a5.jtd` | 74 | 16 | 73 | 75 | `12 + 75 * 84 = 6312` |
| `b6.jtd` | 98 | 16 | 97 | 98 | `12 + 98 * 84 = 8244` |

high-frequency `u32be` values は raw rows 内に packed coordinate-like tuples があることを示すが、internal field layout はまだ decode されていない。

| Sample | top non-zero dword patterns |
| --- | --- |
| `46.jtd` | `0x01610161` x182, `0x01610008` x100, `0x00000161` x97, `0x00f60000` x95 |
| `a5.jtd` | `0x01610161` x138, `0x01610008` x76, `0x00000161` x74, `0x00f60000` x73 |
| `b6.jtd` | `0x01610161` x184, `0x01610008` x102, `0x00000161` x98, `0x00f60000` x97 |

`rjtd page-marks <file>` は fixed 84-byte family、preserved trailing bytes を持つ fixed 84-byte rows、count-plus-one variable-row families、count-variable family の raw-preserving parsers を expose する。current 61 local `.jtd`/`.jtt`/`.jttc` samples のうち 55 が `/PageMark` を expose し、52 が `page-marks` で parse され、3 は unsupported shapes として意図的に reject される。

`rjtd page-mark-shape <file>` は remaining variants の non-failing shape candidates を expose する。current initial groups:

| Group | Count | Parser status | Representative | Observation |
| --- | ---: | --- | --- | --- |
| fixed 84-byte rows | 17 | parsed | `justsystems-20120223023549-jp-just-finance-j200003.jtd` | tail is divisible into 84-byte raw rows |
| fixed 84-byte rows matching count-plus-one | 3 | parsed | `a5.jtd` | `fixed84` and `count-plus-one` both fit: 75 rows of 84 bytes |
| count-plus-one variable rows | 14 | parsed | `ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd` | header `3,16,2`; tail divides into 4 rows of 274 bytes |
| count-plus-one with 2-byte tail/trim | 9 | parsed | `ichitaro-20041103143104-seminar2004-part2_2-img-shortcutkey2.jtd` | raw stream is not u32-aligned; `(tail - 2)` divides into 7 rows of 556 bytes |
| count-variable rows | 2 | parsed | `ichitaro-20030415170937-success-001-success_data-fujimoto_file.jtd` | tail divides by header count but not count-plus-one |
| fixed 84-byte rows with trailing bytes | 7 | parsed | `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` | one or more 84-byte rows followed by preserved trailing bytes |
| non-PageMark-looking payload | 3 | unsupported | `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | payload contains stream/object names or legacy class/control metadata rather than numeric row headers |

一部の parsed regular-stream samples は、safe readable payload よりずっと大きい declared CFB sizes をまだ持つ。例えば `ichitaro-20030120133129-0007-sp-dat-tmogi3_2.jtd` は 304 readable bytes を expose する一方、directory entry は `9434469490474615088` bytes を declare する。parser は safely read payload を使い、declared-size anomaly は `page-mark-shape` に保持する。

three unsupported `non-page-header` payloads は parser family に昇格しない。`stream-text-probe` はそれらが unrelated-looking text payloads を指していることを示す。

| Sample | Probe evidence |
| --- | --- |
| `ichitaro-20030706231945-success-001-success_data-kaisya_annai.jtd` | UTF-16LE names such as `LayoutBoxTextPositionTables`, `TextLayoutStyle`, `DocumentEditStyles`, `DocumentViewStyles`, and `SummaryInformation` |
| `ichitaro-20030706232401-success-001-success_data-kazoku_ryoko.jtd` | ASCII/class-like strings such as `Ver.2.3 for Windows95`, `JSFart2`, and `JS.FartCtrl.1` |
| `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | UTF-16 names such as `JSRV_SegmentInformation`; its `/PaperMark` similarly exposes `ReferenceInfo` |

Working interpretation: these three `/PageMark` directory entries は normal layout rows ではなく stale、foreign、または alternate object payload bytes を指している可能性が高い。

`stream-chain` は unsupported entries が単に broken miniFAT chains ではないことを確認する。`/PageMark` chains は complete だが、mini-stream offsets の bytes は unrelated payloads と decode される。

| Sample | `/PageMark` chain evidence | Payload evidence |
| --- | --- | --- |
| `ichitaro-20030706231945-success-001-success_data-kaisya_annai.jtd` | miniFAT start 133, complete chain, 832 bytes capacity for 796 declared bytes | CFB directory-entry-looking fragments including `LayoutBoxTextPositionTables` |
| `ichitaro-20030706232401-success-001-success_data-kazoku_ryoko.jtd` | miniFAT start 88, complete chain, 192 bytes capacity for 162 declared bytes | OLE/ActiveX-like metadata strings `Ver.2.3 for Windows95`, `JSFart2`, `JS.FartCtrl.1` |
| `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | miniFAT start 96, complete chain, 320 bytes capacity for 272 declared bytes | CFB directory-entry-looking fragments including `JSRV_SegmentInformation` |

`cfb-map` はこれをさらに絞り込む。

- `kaisya_annai`: root mini-stream chain は sector `13` 以降で directory chain と overlap するため、directory-entry-looking bytes は root mini-stream/directory chain overlap で説明できる。
- `shanai_lan`: root mini-stream chain も sector `13` 以降で directory chain と overlap する。
- `kazoku_ryoko`: `cfb-map` では同じ directory/root mini-stream overlap は見えない。`stream-find` は `/PageMark` を `/EmbedItems/Embedding 1/JSFart2Contents` offset 1664 の exact 162-byte slice に結びつけるため、この `/PageMark` entry は layout-mark variant ではなく duplicate embedded-control payload である。

## PaperMark Observation

first 12 bytes は `PageMark` header と関連して見える。

```text
46.jtd  00000060 0000000c 0000005f
a5.jtd  0000004a 0000000c 00000049
b6.jtd  00000062 0000000c 00000061
```

Working interpretation:

- first `u32be`: `/PageMark` と共有される count-like value。
- second `u32be`: `0x0c`。size または stride-like value の可能性が高い。
- third `u32be`: first value minus one。

observed large samples は stable row shape を証明している。

```text
12-byte header
N rows of:
  u32be index
  u32be flags
```

row count は stream length から `(stream_len - 12) / 8` として derive する。first header value は count-like だが、常に `row_count` または `row_count - 1` と等しいわけではないため、parser は semantics を割り当てず observed header value として保存する。

| Sample | header value 0 | header value 1 | header value 2 | rows | flag distribution |
| --- | ---: | ---: | ---: | ---: | --- |
| `46.jtd` | 96 | 12 | 95 | 97 | `0x00010000` x89, `0x00010010` x7, `0x00010011` x1 |
| `a5.jtd` | 74 | 12 | 73 | 75 | `0x00010000` x65, `0x00010010` x9, `0x00010011` x1 |
| `b6.jtd` | 98 | 12 | 97 | 98 | `0x00010000` x90, `0x00010010` x6, `0x00010011` x2 |

`rjtd paper-marks <file>` はこの parser-backed diagnostic を expose する。header と flag semantics が unknown のため、document model にはまだ wire されていない。`rjtd paper-mark-shape <file>` は observed `/PaperMark` streams すべてに対する non-failing shape diagnostic を expose する。

current 61 local `.jtd`/`.jtt`/`.jttc` samples では、55 が `/PaperMark` を expose し、52 がこの row shape で parse される。`paper-mark-shape` は 55 streams すべてを開き、3 件を `non-paper-header` として分類し `paper-marks` では reject する。

| Sample | observed header/stride evidence | Text probe evidence |
| --- | --- | --- |
| `ichitaro-20030706231945-success-001-success_data-kaisya_annai.jtd` | stride-like dword `0xffffffff` | UTF-16 `JSRV_SegmentInfor...` |
| `ichitaro-20030706232401-success-001-success_data-kazoku_ryoko.jtd` | stride-like dword `0xff090000` | short non-row payload; paired `/PageMark` contains `Ver.2.3 for Windows95`, `JSFart2`, `JS.FartCtrl.1` |
| `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | stride-like dword `0xffffffff` | UTF-16 `ReferenceInfo` |

Working interpretation: these three `/PaperMark` directory entries は normal paper-mark rows ではなく stale、foreign、または alternate object payload bytes を指している可能性が高い。

`stream-chain` は `/PaperMark` でも同じ pattern を示す。miniFAT chains は structurally complete だが、payload bytes は paper-mark rows ではない。

| Sample | `/PaperMark` chain evidence | Payload evidence |
| --- | --- | --- |
| `ichitaro-20030706231945-success-001-success_data-kaisya_annai.jtd` | miniFAT start 157, complete chain, 128 bytes capacity for 100 declared bytes | CFB directory-entry-looking fragment containing `JSRV_SegmentInfor...` |
| `ichitaro-20030706232401-success-001-success_data-kazoku_ryoko.jtd` | miniFAT start 97, complete chain, 64 bytes capacity for 36 declared bytes | short control/object payload beginning with `SO`-like bytes, not a `0x0c` paper-mark header |
| `ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd` | miniFAT start 111, complete chain, 128 bytes capacity for 108 declared bytes | CFB directory-entry-looking fragment containing `ReferenceInfo` |

同じ `cfb-map` interpretation が `/PaperMark` にも当てはまる。`kaisya_annai` と `shanai_lan` の directory-entry fragments は directory/root mini-stream chain overlap で説明できる。`kazoku_ryoko` は another complete stream と exact-match しないが、`cfb-dir` はその `/PaperMark` entry を `/EmbedFrame` と `/Figure` の間の root-level object/control sequence に置く。payload は `/Figure` と `/EmbedItems/Embedding 1/\x01CompObj` にも見える `SO` marker で始まり、`stream-find-bytes 534f0000` がそれらの marker hits を再現する。marker 後の first 20 bytes も `/EmbedItems/Embedding 2/JSFart2Contents` offset 192 と一致する。これは paper-mark row variant ではなく embedded-control/object record candidate である。

`rjtd so-records <file>` はこの marker family を diagnostic として保存する。各 `SO\0\0` marker を containing stream path、offset、first little-endian `u32` fields、raw bytes とともに出力する。current 61 local samples で、`SO` records を expose するのは 4 samples、24 records total だけである。`/PaperMark` 自体に `SO` record を含む current sample は `kazoku_ryoko` だけである。

`rjtd so-record-clusters <file>` はこれらの records を preserved raw bytes で group する。observed `JSFart2Contents` samples は singleton geometry-like clusters と repeated default/control clusters に分かれ、通常 three repeated offsets に現れる。`kazoku_ryoko` `/PaperMark` record は older `ichitaro-20030315133825-success-001-success_data-kazoku_ryoko.jtd` sample の geometry-like `JSFart2Contents` record と一致する。

```text
0x00004f53,0x000009ff,0x000008a0,0x0000139a,0x000008a0,...
```

これは `kazoku_ryoko` `/PaperMark` が JTD paper-mark row ではなく、leaked または duplicated embedded-control geometry record であるという解釈を強める。

`rjtd so-record-fields <file>` は各 record を little-endian fields として展開する。current evidence は 9-dword diagnostic shape に合う。

- field 0 は常に marker `0x00004f53` (`SO\0\0`)。
- repeated default/control clusters は `0x00000100` や `0x00000064` のような small constants を含む。
- singleton geometry-like clusters は fields 1-4 に coordinate-like values を持つ。通常 `JSFart2Contents` samples では high 16-bit halves は zero。
- packed records は `packed-jseq3-like` と `packed-ffff-preamble` families に分かれ、object payloads が decode されるまでは separate SO-like families として残す。

`rjtd so-record-geometry <file>` は final semantics として扱わず、この split を明示する。same 61 local samples で command は all files を successfully check し、4 files に 24 SO records を報告する: 9 `geometry-like`, 8 `default-control`, 4 `packed-jseq3-like`, 2 `packed-ffff-preamble`, 1 `truncated`。`kazoku_ryoko` `/PaperMark` record は fields `2559,2208,5018,2208` の `geometry-like` と分類され、older `JSFart2Contents` geometry-like record と一致する。

`rjtd so-record-halves <file>` は each SO payload dword の low/high 16-bit unsigned/signed halves を出力する。current samples では、すべての `packed-jseq3-like` record が `JSEQ3Contents` に現れる。field 6 は field 2 の low 16 bits を繰り返し、field 7 は second small 16-bit value を持つ。二つの `packed-ffff-preamble` records は `JSFart2Contents` offset 324 に現れ、same streams 内の repeated geometry-like records に先行する。

## LineMark Observation

`/LineMark` は別の始まり方をし、simple fixed-width numeric table より token/control stream に近く見える。

```text
a5.jtd  0914 0000 0001 0000 048f 0000 0003 0000
46.jtd  0914 0000 0001 0000 050e 0000 0015 0000
b6.jtd  0914 0000 0001 0000 0531 0000 0002 0000
```

first words の後、`000d`、`000a`、`0011`、`0019`、`0082`、`1002` のような common values が繰り返される。これらは `/DocumentText` control and inline segments 周辺で既に見た values と overlap するため、`/LineMark` は structured `DocumentText` token map と一緒に調べるべきである。

`stream-words` の first words:

| Sample | word 0 | word 4 | word 8 |
| --- | --- | --- | --- |
| `46.jtd` | `0x0914` | `0x050e` | `0x050d` |
| `a5.jtd` | `0x0914` | `0x048f` | `0x048e` |
| `b6.jtd` | `0x0914` | `0x0531` | `0x0530` |

Working interpretation:

- word 0 は `LineMark` header/type candidate。
- word 4 は count-like。
- word 8 は word 4 minus one。
- word 6 は sample により異なり、未解読。

raw word-frequency comparison は、多くの small `/LineMark` words が raw `/DocumentText` にも現れることを示す。一方 high-frequency `0x1000`、`0x1001`、`0x1002` words は `/LineMark`-specific tags らしい。

| Sample | `0x1000` | `0x1001` | `0x1002` |
| --- | ---: | ---: | ---: |
| `46.jtd` | 302 | 24 | 165 |
| `a5.jtd` | 293 | 18 | 200 |
| `b6.jtd` | 308 | 25 | 169 |

これらの tag-like values は `DocumentText` control codes として扱わない。

`rjtd line-mark-tags <file>` は `/LineMark` から tag-like words を scan し、word index、byte offset、previous four words、next six words を出力する。current 61 local samples の sweep:

| Group | Count |
| --- | ---: |
| files with tag rows | 5 |
| files without `/LineMark` | 6 |
| readable `/LineMark` streams with no tag rows | 50 |
| total tag rows | 1536 |
| `0x1000` rows | 915 |
| `0x1001` rows | 67 |
| `0x1002` rows | 554 |

tag rows を持つ five files:

| Sample | `0x1000` | `0x1001` | `0x1002` | total |
| --- | ---: | ---: | ---: | ---: |
| `46.jtd` | 302 | 24 | 165 | 491 |
| `a5.jtd` | 293 | 18 | 200 | 511 |
| `b6.jtd` | 308 | 25 | 169 | 502 |
| `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` | 0 | 0 | 5 | 5 |
| `ichitaro-20030706234132-success-004-success_data-asobinin_24.jtd` | 12 | 0 | 15 | 27 |

tag の直後の first word は unique family discriminator ではない。`0x0025`、`0x0027`、`0x002d`、`0x004d`、`0x0037`、`0x0035` のような frequent next words は tag families 間で overlap するため、current interpretation では decoded tag subtype ではなく payload-like context として扱う。

`rjtd line-mark-text-context <file>` は各 tag row を `/DocumentText` token map と比較する。two weak hypotheses を test する: LineMark word/byte offset が direct に `DocumentText` map entry に落ちるか、immediate next word が raw `/DocumentText` のどこかに現れるか。

same 61 local samples:

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

Working interpretation: `/LineMark` tag-next words は通常 `/DocumentText` のどこかに現れる values を再利用するが、それだけで direct offsets になるわけではない。LineMark tag index または byte offset を direct `DocumentText` coordinate として扱うと 1536 rows 中 587 rows だけが hit する。したがって `/LineMark` は own record coordinate system または layout-local payload fields を使っている可能性が高い。

`rjtd text-position-line-context <file>` は `MarkV.01` header/entry offsets を `/LineMark` word positions と nearest LineMark tag rows と比較する。current samples で readable `/LineMark` と parsed `MarkV.01` の両方を expose するのは `46.jtd`、`a5.jtd`、`b6.jtd` の三つだけである。

| Sample | line words | line tags | Mark entries | Page rows | Paper rows | Mark header line index | LineMark word at header index | nearest tag rows |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `46.jtd` | 2597 | 491 | 8 | 97 | 97 | 1539 | `0x0029` | `0x1002@1520`, `0x1000@1564` |
| `a5.jtd` | 2561 | 511 | 8 | 75 | 75 | 1539 | `0x0016` | `0x1002@1534`, `0x1002@1542` |
| `b6.jtd` | 2667 | 502 | 8 | 98 | 98 | 1552 | `0x0000` | `0x1000@1548`, `0x1002@1554` |

three samples の all 24 `MarkV.01` entry offsets は `/LineMark` word range 外にある。したがって entries は direct LineMark word indexes ではない。Mark header の final `u16` は `/LineMark` 内の tag clusters 近くに入るため LineMark-adjacent candidate として残るが、decode されておらず stable tag value にも落ちない。

`rjtd text-position-count-layout-context <file>` は同じ direct-layout-coordinate check を `TCntV.01` tables に広げる。10 readable `TCntV.01` samples と 89 checked rows において、chosen range candidates は `/LineMark` word offsets、`/LineMark` byte offsets、`/PageMark` rows/bytes、`/PaperMark` rows/bytes のいずれにも direct hit しない。current `TCntV.01` ranges も direct layout stream coordinates として扱うべきではない。

`rjtd text-position-count-fields <file>` は remaining `TCntV.01` tail を positional `u16be` fields として expose し、`rjtd text-position-count-field-deltas <file>` は chosen range span と tail `t1..t2` span を比較する。current samples では all 89 rows が `t2 >= t1` だが、`t2 - t1` が chosen range span と等しい row はない。`rjtd text-position-count-tail-context <file>` は `t1/t2` が byte hit pattern より強い `/DocumentText` UTF-16 unit hit pattern を示すが universal ではない。`rjtd text-position-count-tail-delta-scan <file>` は unit hits を delta 29/30 付近で最も増やすが、text hits は elsewhere で peak する。`rjtd text-position-count-tail-delta-groups <file>` は aggregate signal を tail-pattern groups に分ける: one major `be0` pattern は `+29`、shifted patterns は `+30/+31`、major `0x0202` pattern は many best deltas に分散する。`rjtd text-position-count-tail-row-deltas <file>` は major `0x0202` pattern が row-level best deltas でも分散したままであることを確認する。`rjtd text-position-count-tail-row-context <file>` は `0x0202` chosen ranges が later body byte ranges に触れることが多く、best-delta tail fields は early heading/date text に触れることが多いと示す。`rjtd text-position-count-range-preview <file>` は `0x0202` chosen byte ranges が direct layout-stream coordinates ではないにもかかわらず real `/DocumentText` text entries と overlap することを示す。`rjtd text-position-count-range-boundaries <file>` は major `0x0202` byte ranges が mostly whole `/DocumentText` map entries を含み、`0x001c`/`0x000e` controls を繰り返し含むことを追加する。`rjtd text-control-context <file>` は `0x001c` を high-frequency delimiter candidate、`0x000e` をより control-cluster or inline-adjacent と示す。これにより `TCntV.01` tail fields は有望なまま残るが、`t1/t2` を chosen range の単純な duplicate として扱うことは reject され、chosen-range analysis の次の target は `/DocumentText` control-delimited byte intervals になる。

## LineMark ヘッダーワード 0 の変化

2026-06-24 時点のローカルサンプルで `LineMark` ヘッダーワード 0 の値が 3 種類確認された：

| 値 | サンプル |
| --- | --- |
| `0x0914` | `46.jtd`、`a5.jtd`、`b6.jtd`（Ginga 縦書きサンプル、RFC 0007 初期セット） |
| `0x090b` | ローカルの行政文書サンプル 10 件すべて（`01要綱`、`02案文`、`03新旧`、`04参照`） |
| `0x0912` | `論文様式.jtd`（A4 横書き学術論文テンプレート） |

いずれも `0x0900` 系プレフィックスを共有し、下位バイトのみ異なる：
`0x14 = 20`、`0x0b = 11`、`0x12 = 18`。Ginga 縦書きと A4 横書きの差は `2`（`0x0914` vs `0x0912`）、Ginga と行政文書の差は `9`（`0x0914` vs `0x090b`）。
下位バイトの意味は未解読。文書種別・書字方向・その他の文書レベル属性をエンコードしている可能性がある。

## LineMark と DocumentText 0x001c の相関

RFC 0009 により、`/DocumentText` の各 `0x001c` が自己記述型の段落/レイアウトレコードのオープナーであることが確立された。`be16-delta-v1` LineMark プロファイルは表示行ごとに 1 レコードを出力し、`0x001c` は論理段落（複数の表示行に折り返す場合がある）をマークする。

`論文様式.jtd`（LineMark レコード 25 件、`0x001c` レコード 19 件）では、25 件の LineMark `unit-start` 値のうち 14 件が `0x001c` レコード位置と完全一致する。残り 11 件はテキストラン内部または `0x0000` ドキュメントターミネーターに対応する。`flag=0x0000` を持つ LineMark レコードは `0x001c` 位置と一致しない傾向があり、折り返した段落内の続き表示行と整合する。

`03新旧（整備令）.jtd`（解析済み LineMark レコード 157 件）では、`0x001c` レコードが表セルクラス `0x0030`（703 件）と段落クラス `0x0010`（151 件）を含み、新旧対照文書の表多用構造を反映している。LineMark のデルタ値も対応して小さく多様である。

## Known Gaps

- `LineMark` record parser はまだ存在しない。
- `/PageMark` には fixed 84-byte rows、preserved trailing bytes 付き fixed 84-byte rows、count-plus-one variable-row families、count-variable rows の raw-preserving parsers があるが、一部 local `/PageMark` streams はまだ unsupported variants を使う。
- `/PaperMark` には initial row parser があるが、semantic model mapping はまだない。
- count-like header values が entry counts であることは未証明。
- これらの streams と `MarkV.01` / `TCntV.01` の関係は decode されていない。current evidence は direct Mark-entry-to-LineMark-word indexing と direct `TCntV.01` range-to-layout-stream offsets を reject する。
- small malformed samples は mini-stream chains が safely readable ではない inventory entries を expose することがある。small layout streams を解釈する前に `stream-meta` を使うべきである。
- `/LineMark` は semantics unknown の tag-like values (`0x1000`, `0x1001`, `0x1002`) を持つ。current tag-context evidence は immediate next word を unique family discriminator にせず、direct `DocumentText` coordinates も証明しない。
- LineMark ヘッダーワード 0 の値（`0x0914` / `0x090b` / `0x0912`）の意味は未解読。文書種別・書字方向・その他の文書レベル属性をエンコードしている可能性がある。

## Next Steps

- header と flag semantics が decode されるまで `/PaperMark` は parser-backed diagnostic として保つ。
- `SO` object/control record family field semantics を decode する。current evidence は singleton records の fields 1-4 が geometry-like tuples を持ち、repeated records が default/control constants を持つことを示す。
- `/PageMark` と `/PaperMark` count-like values を known samples の rendered page または paper counts と比較する。
- `/PaperMark` flags を page breaks、paper sections、visible layout changes と比較する。
- tag offsets と contexts を text/layout boundaries と比較し、`/LineMark` 内の `0x1000`、`0x1001`、`0x1002` tag families を decode する。
- final Mark header `u16` が `/LineMark` tag clusters 近くに入る一方で Mark entries が入らない理由を decode する。
- direct `/LineMark`、`/PageMark`、`/PaperMark` coordinates が rejected された後の `TCntV.01` ranges の actual target を decode する。`text-position-count-fields`、`text-position-count-field-deltas`、`text-position-count-tail-context`、`text-position-count-tail-delta-scan`、`text-position-count-tail-delta-groups`、`text-position-count-tail-row-deltas`、`text-position-count-tail-row-context`、`text-position-count-range-preview`、`text-position-count-range-boundaries`、`text-control-context` を使って tail field patterns と chosen `/DocumentText` byte-range/control overlap を比較する。
