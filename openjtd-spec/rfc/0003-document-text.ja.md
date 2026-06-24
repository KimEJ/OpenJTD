# RFC 0003: DocumentText Initial Text Extraction

Status: draft

Observed: 2026-06-18

## Summary

`/DocumentText` stream には復元可能な body text が含まれる。

stream は次の ASCII magic で始まる。

```text
SsmgV.01
```

初期抽出では、`0x001F` marker の後に UTF-16BE で encoded された text runs が見える。

visible text の一部は `0x001D ... 0x001E` で区切られた inline segments の中にも保存されている。

これは最初の `rjtd cat <file.jtd>` implementation には十分である。rjtd は現在、観察済み text runs、inline text、control boundaries のための structured `ParsedDocumentText` token layer を持つが、まだ完全な `DocumentText` record parser ではない。

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

`dump-stream` は raw stream bytes を stdout に書く。

`cat` は `/DocumentText` を読み、`ParsedDocumentText` に parse し、その parser の plain-text projection を出力する。観察済み `.jttc` samples では、まず `/JSCompDocument` `JustCompressedDocument` data を unwrap し、decompressed inner CFB から `/DocumentText` を読む。named `/DocumentText` stream を expose しない観察済み samples では、embedded `SsmgV.01`/`TextV.01` fragments を scan し、plausible text lines を抽出する。

`text-tokens` は structured token stream を tab-separated lines として出力する。

```text
text	銀河
control	0x001c
skipped-inline	0x0082	20	ごご
text	鉄道\n
```

`text-map` は同じ tokenization を byte ranges、UTF-16 unit ranges、token kind、selector/control metadata、各 token range 内に raw offsets が入る `MarkV.01` ids とともに出力する。これは `/DocumentText` と `/DocumentTextPositionTables` の diagnostic bridge である。

`text-control-context` は各 control boundary を byte/unit range、neighboring map entries、nearest previous/next control boundary とともに出力する。`0x001c` のような optional decimal/hex control-code filter を受け取る。

`text-control-ranges` は control delimiters の前、間、後にある intervals を出力する。filter がなければ mapped control boundary すべてが delimiter になり、`0x001c` のような filter があればその code だけが stream を分割し、他の controls は interval 内で count される。各 row には previous/next delimiter metadata、entry index span、byte/unit span、token-kind counts、control-code counts、short text preview が含まれる。

## Current Structured Token Parser

current parser:

1. `/DocumentText` を big-endian 16-bit units として読む。
2. `0x001F` 後に text run を開始する。
3. tab、LF、CR を除く C0/C1 control boundaries で text run を停止する。
4. `0x001D ... 0x001E` に包まれた selected visible inline segments を復元する。
5. decoded pieces を `TextRun`、`InlineText`、`SkippedInlineText`、`ControlBoundary` elements として保存する。
6. structured parser の plain-text projection から `cat` output を生成する。

`SkippedInlineText` は plain `cat` output には出力しない。selector、decoded text、raw UTF-16BE bytes とともに保持し、source tag `0x001d` を持つ `UnknownObject` として document model に lift する。

skipped inline segments は、matching `0x001E` terminator が `0x001D` 後 256 UTF-16 units 以内に現れる場合だけ保存する。bounded terminator が見つからない場合、parser は大きな binary または formatting region を text として消費せず、ordinary control/text boundaries として残す。これは final format interpretation ではなく、preservation-first safety rule である。

embedded fallback は raw `SsmgV.01` fragments を見つけた後、同じ heuristic を使う。意図的に制限されている。

- named `/DocumentText` と supported `/JSCompDocument` paths がない場合だけ実行する。
- 各 fragment は next `SsmgV.01` marker または 64 KiB までに bound する。
- implausible noise lines は conservative character filter で落とす。
- document model は source を `/EmbeddedDocumentText` として記録する。

## Inline Segment Observation

local samples は repeated inline segment contexts を示す。

```text
001C 0001 0007 0000 0000 0003 001D <visible base text> 001E
001C 0001 0007 0000 0001 0082 001D <phonetic annotation> 001E
```

first form は `午后`、`天気輪`、`捕`、`切符` など visible ruby base text を保持しているように見える。

second form は `ごご`、`てんきりん`、`と`、`きっぷ` など phonetic annotation text を保持しているように見える。

plain `cat` output は現在 visible base text を出力し、phonetic annotation text を skip する。

template samples には次も見える。

```text
001C 0001 0007 0000 0000 0001 001D <visible placeholder text> 001E
001C 0001 0007 0000 0001 0000 001D <template instruction text> 001E
```

plain `cat` output は `○○○` のような visible placeholders を出力し、template instruction text を skip する。

skipped inline segments は reverse-engineering のために保存される。たとえば local `a5.jtd` は次のような rows を expose する。

```text
skipped-inline	0x0082	20	ごご
skipped-inline	0x0082	26	てんきりん
skipped-inline	0x0082	22	きっぷ
```

## Control Boundary Observation

`text-control-context` と `text-control-ranges` は、`TCntV.01` range diagnostics が `0x0202` chosen byte ranges に `/DocumentText` controls が繰り返し含まれることを示した後に追加された。現在の local samples 61 件では、context command は error なく動き、60 files が mapped control boundaries を含む。

Top observed control codes:

| Control code | Rows | Files |
| --- | ---: | ---: |
| `0x001c` | 51,971 | 60 |
| `0x000e` | 6,621 | 41 |
| `0x001d` | 1,156 | 32 |
| `0x0000` | 682 | 57 |
| `0x000c` | 166 | 24 |
| `0x0090` | 99 | 13 |

current `TCntV.01` work に最も関連する二つの controls は異なる local context profiles を持つ。

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

これにより `0x001c` は現在最も強い generic delimiter candidate になる。visible text runs を他の visible text や control clusters から分けることが多い。`0x000e` はより control-cluster-like で、別の control boundary の直隣や skipped inline content の前に置かれることが多い。これは observation にすぎず、どちらにも final semantic name はまだない。

synthetic tests cover:

- `0x001F` 後の text-run extraction。
- first text marker 前の bytes は無視される。
- `0x0090` のような C1 control values は boundaries として扱われる。
- visible inline ruby base text は出力され、phonetic annotations は skip される。
- visible template placeholders は出力され、template instructions は skip される。
- `ParsedDocumentText` は plain-text projection の前に observed text runs、inline text segments、control boundaries を保存する。
- `text-control-context` は previous/next map entries と nearest previous/next control boundaries を報告し、optional code filtering を含む。
- `text-control-ranges` は control-delimited intervals を報告し、filtered ranges 内の non-delimiter controls を preserve/count する。
- skipped phonetic/template inline segments は `SkippedInlineText` tokens と document-model `UnknownObject` payloads として保存される。
- observed ruby base と phonetic annotation の pair は document-model `Inline::Ruby` に promote され、visible text output は base text を使いつつ annotation text と raw payload を保存する。
- unbounded inline starts は large control または binary region の残りを `SkippedInlineText` として消費しない。
- `/JSCompDocument` payloads with `JustCompressedDocument` は observed `-lh5-` profile と一致すると decompressed される。
- invalid synthetic compressed payloads は明確に失敗する。
- `/DocumentText` がない場合に embedded `SsmgV.01` fragments が復元される。

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

extracted text は次で始まる。

```text
銀河鉄道の夜				宮沢 賢治

目次
```

samples は宮沢賢治「銀河鉄道の夜」の text を含んでいるように見える。

inline base-text recovery 後、table of contents は次のように始まる。

```text
一、午后の授業
二、活版所
三、家
四、ケンタウル祭の夜
五、天気輪の柱
```

template `.jtt` samples も `/DocumentText` を expose し、同じ command で読める。

`.jttc` samples は `/DocumentText` を直接 expose しない。`/JSCompDocument` streams は次で始まる。

```text
2600 4a75 7374 436f 6d70 7265 7373 6564 446f 6375 6d65 6e74
```

これは length-prefixed marker `JustCompressedDocument` と decode され、observed payload では後続に LHA `-lh5-` member がある。その member を decompress すると own `/DocumentText` stream を持つ inner CFB file が得られる。

observed `.jttc` template samples は plain text extraction 後、ほぼ blank/control-heavy である。現在は non-empty document model blocks を生成しない。

二つの local `.jtd` samples は `cfb-embedded-document-text` として開く。named `/DocumentText` stream は expose しないが、raw file bytes には repeated `SsmgV.01` と `TextV.01` markers が含まれる。recovered text には次のような visible document content が含まれる。

```text
参加者募集中団体名氏　名
ハイキングクラブ会報・第２０号
```

## COM Text Export Observation

`JXW.Application` COM automation（`TaroLibrary.SaveDocument`、`filterNo=10`）による plain-text export は、Ichitaro テーブル構造を表現するために Unicode U+2500 系の罫線文字を使用した出力を生成する。これは local samples で観察された DocumentText control code の割り当てを独立に裏付ける。

### Table Cell Delimiter Corroboration

COM text export は隣接するテーブルセル間の列区切りとして U+2502 VERTICAL LINE（`│`）を使用する。

```text
項目│値
合計│100
```

これは観察済み DocumentText control code `0x001c`（local sample ファイル 60 件で 51,971 回出現）と一致し、テーブルセル境界としての役割を確認する。このコードは rjtd-core で次のように定義されている。

```rust
pub const TABLE_CELL_DELIMITER_CONTROL: u16 = 0x001c;
```

### Table Row Delimiter Corroboration

COM text export は水平テーブル罫線に次の罫線文字を使用する。

```text
┌─┬─┐   (上端)
├─┼─┤   (行間区切り)
└─┴─┘   (下端)
```

使用文字：U+2500（`─`）、U+250C（`┌`）、U+251C（`├`）、U+253C（`┼`）、U+2514（`└`）、U+2524（`┤`）、U+252C（`┬`）、U+2510（`┐`）、U+2534（`┴`）

これは観察済み DocumentText control code `0x000e`（local sample ファイル 41 件で 6,621 回出現）と一致する。このコードは control-cluster コンテキスト（`control -> control` が最頻ペア）で頻繁に現れる。rjtd-core での定義は次のとおり。

```rust
pub const TABLE_ROW_DELIMITER_CONTROL: u16 = 0x000e;
```

### Page Break Corroboration

COM VBA スクリプトは、エクスポートテキストの検索や分割時に `Chr(12)`（ASCII フォームフィード、`0x0C`）を改ページ文字として使用する。これにより DocumentText control code `0x000c`（local sample ファイル 24 件で 166 回出現）が確認される。

```rust
pub const DOCUMENT_TEXT_PAGE_BREAK_CONTROL: u16 = 0x000c;
```

### Confidence Level

これらの裏付けは強力だが網羅的ではない。

- VBA → 罫線文字 → DocumentText control code のマッピングは観察データすべてと一致する。
- VBA automation corpus は複数のドキュメントタイプ（`.jtd`、`.jtt`）をカバーしていた。
- 正確な全文エクスポートには `基本` タブモードが必要であり、他のタブモードでは構造的に異なる DocumentText コンテンツが生成される場合がある。
- セマンティック命名（TABLE_CELL_DELIMITER vs TABLE_ROW_DELIMITER）は、control-code パターン分析だけでなく独立したクロスフォーマット証拠によって確認された。

## Known Gaps

- inline segment rules はまだ heuristic であり、observed local samples に基づく。
- structured token layer は full record parser ではなく、styles、full ruby semantics、tables、layout objects をまだ復元しない。
- embedded fragment recovery は heuristic で、proper object/stream boundary parsing に置き換えるべきである。
- `DocumentText` record boundaries は observed token/control boundaries を超えてまだ decode されていない。
- `0x001c` と `0x000e` は high-priority delimiter candidates だが、exact record/table/object/paragraph semantics は未解読。
- `/DocumentText` と `/DocumentTextPositionTables` の関係は `text-map` と `text-position-context` で検査可能になったが、stable coordinate rule はまだ証明されていない。
- JTTC support は observed `JustCompressedDocument` plus single `-lh5-` member profile に限定される。
- initial LH5 decoder は LHA header checksums や CRC values をまだ検証しない。

## Next Steps

- current token layer を超えて true `DocumentText` records を decode する。
- `0x001C`、`0x000E`、`0x001D`、`0x001E`、`0x001F` 周辺の record または token meanings を特定する。
- `/DocumentTextPositionTables` が text layout に参加するなら、missing または reordered text の復元に使う。
- embedded `SsmgV.01` fragments を所有する container/object boundary を特定する。
- より多くの `.jttc` samples が観察されたら `JustCompressedDocument` documentation を拡張する。
- plain-text line breaks から model blocks を derive する代わりに、surrounding streams から paragraph boundaries と style references を復元する。
