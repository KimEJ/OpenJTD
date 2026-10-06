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

### Named Text Segment Boundary

byte 0 に `SsmgV.01`、byte 20 に `TextV.01` がある観測済み stream では、byte 28 の
big-endian `u32` が UTF-16 unit 単位の content length を示す。本文は byte 32 から始まり、
宣言した unit 数の後に style section が続く。DocumentText style-section parser も
同じ境界を使用する。

可視テキストは `0x001f` marker なしに byte 32 から始まる場合がある。統制実験の
`PAGE 01` family と、独立したローカル `sample_macro` / `toolbox` 文書で確認した。
header の下位 word を raw/record format の選択値とみなすと先頭テキストを失うため、
parser と source map は named segment の先頭で text mode に入り、宣言された終端までを
利用可能な完全な unit 数で制限する。これにより style tail の marker bytes を本文として
読むことも防ぐ。長さには `u32` の両方の word を使う。

named header のない stream は marker-based recovery を維持する。物理ファイル内の
embedded fragment は連続した logical stream bytes と仮定できないため、fragment の
宣言長で後続の復元可能なテキストを破棄せず、既存の上限付き salvage scan を維持する。
raw bytes は文書モデルに保持する。

### Token Decoding

current parser:

1. `/DocumentText` を big-endian 16-bit units として読む。
2. 検証できた named segment の先頭、または `0x001F` 後で text run を開始する。
3. tab、LF、CR を除く C0/C1 control boundaries で text run を停止する。
4. `0x001D ... 0x001E` に包まれた selected visible inline segments を復元する。
5. decoded pieces を `TextRun`、`InlineText`、`SkippedInlineText`、`ControlBoundary` elements として保存する。
6. structured parser の plain-text projection から `cat` output を生成する。

本文、可視 inline、保存する skipped-inline text では、有効な UTF-16 surrogate pair を
まとめて decode する。source range の単位は UTF-16 のままであり、補助平面の文字は
2 units を占める。pair は宣言された text segment の境界を越えず、不正な unit は
従来の boundary 処理を維持する。

`SkippedInlineText` は plain `cat` output には出力しない。selector、decoded text、raw UTF-16BE bytes とともに保持し、source tag `0x001d` を持つ `UnknownObject` として document model に lift する。

skipped inline segments は、matching `0x001E` terminator が `0x001D` 後 256 UTF-16 units 以内に現れる場合だけ保存する。bounded terminator が見つからない場合、parser は大きな binary または formatting region を text として消費せず、ordinary control/text boundaries として残す。これは final format interpretation ではなく、preservation-first safety rule である。

embedded fallback は raw `SsmgV.01` fragments を見つけた後、同じ heuristic を使う。意図的に制限されている。

- named `/DocumentText` と supported `/JSCompDocument` paths がない場合だけ実行する。
- 各 fragment は next `SsmgV.01` marker または 64 KiB までに bound する。
- implausible noise lines は conservative character filter で落とす。
- document model は source を `/EmbeddedDocumentText` として記録する。

## Observed Font Size Sources

統制実験の文字サイズ変更では、property `2` の big-endian `u16` が 1/100 mm 単位の
サイズを示す。`370` は約 10.5 pt、`494` は約 14 pt に対応する。
`053_font_size_table_plus` のセル範囲と `054_font_size_paragraph_plus` /
`082_plain_paragraph_font_size_plus` の本文範囲が大きい値を持つ。参照 PDF は
10.44/14.04 pt を使い、printer の量子化と整合するが、正確な device rounding は未解読。

対応する `/DocumentViewStyles` sequential `0x1006` profile は 20 または 21 byte の
payload が `1f 00 00` で始まり、payload byte 3–4 に同じ単位の既定サイズを持つ。
他の profile は未対応。property `2 = 0` は既定値へ戻す。欠落、不正、混在、未被覆の
property 範囲を一様な明示サイズとみなしてはならない。予約値 `0xfffd..0xffff` は
巨大なフォントとして描画しない。

model renderer は対応する明示・既定サイズとその根拠を SVG/PDF、layer-tree の text run
へ渡す。文字送りと折返しはまだ fallback metrics を使うため、完全な font shaping や
レイアウト再現を意味しない。

## 限定的な文字書式・フォント候補

統制した文字範囲では UTF-16 source unit の書式境界を保持し、surrogate pair は
分割しない。描画時に run を分けても model の plain text、source flow、raw byte は変えない。

観測した property `20` profile は `0x84000000`（太字）、`0x90000000`（斜体）、
property `13 = 1` と組み合わせた `0x80000010`（一本下線）、`0x80000c00`（上付き1/4）、
`0x80000400`（下付き1/4）。`0` と `0x80000000` は通常 profile として許容する。
1/4 書式は one-byte property `4` と `5` が両方 `50` のときだけ幅・高さを半分にする。
その他の組み合わせ、幅、flag は未対応のまま残す。

property `3` は `u16` ID で一意な `/Font` entry を選ぶ。`0xffff` と未設定状態は、
対応する sequential `0x1006` payload の byte 5–6 にある既定 ID へ戻る。
欠落・重複 ID や予約値から任意のフォントを選ばない。同一段落内の明示範囲の間でも
既定フォントへ戻る場合がある。

SVG/PDF と layer-tree は候補 flag と `decoded:false` を保持する。横書きでは行内の
最大の未縮小サイズで共通 baseline を作り、上付きは fallback em 上端、下付きは
通常 baseline に置く。通常 face しかない場合も synthetic stroke（0.025 em）で太字、
shear（0.25）で斜体を表示する。これらの paint と添字位置は renderer 近似であり、
解読済み font metrics や native device geometry ではない。PDF backend と browser は
既存 model API に実フォント・代替フォントの glyph advance を渡し、書式 run を隣接配置する。
layer 位置と折返しは fallback metrics のまま。native 空白、正確な印刷量子化、
複合書式 profile、縦書き添字位置は未証明。synthetic 太字の PDF では検索テキストが
重複する場合がある。

## 補助脚注テキストの保持

model は `/Footnote`、`/FootnoteLink`、`/MarkTag` の不正・未対応形式も含めて
raw stream を変更せず保持する。本文とは分離し、脚注の editor 用末尾 cache を
本文 projection へ混ぜない。

限定した 44-byte・一 entry の `/FootnoteLink` profile は byte 10–13 と 18–21 に
big-endian `u32` address を持つ。text header の 16-unit bias を加えると、前者は
脚注の `0x001f` text marker、後者は本文 context record を指す。両 address、
cache marker 文字列の一致、完全な 13-unit record を同時に確認する。脚注 context の
word 7 は `0x0030`、ID の word 8 は `0`。同じ形の終了 context は ID `0xffff`。
対応する本文 context は word 7 `0x0010`、ID `0`。この間の文字だけを
`footnoteTextCandidates` として公開し、marker、脚注・本文 source span、raw link
address と `decoded:false`、placement/link-role の制限を残す。

候補読み取りは補助 text 64 KiB までで、各 stream は一意でなければならない。
切れた・不一致の address、別 link profile、重複 stream、別 note context は
raw evidence を保ち、対応関係を推測しない。source text の保持であり、一般的な
脚注番号、複数脚注、脚注領域の geometry、marker 縮小、bookmark semantics の解読ではない。
native 脚注配置と field 評価は別の未解決項目として残る。

## 表示インライン範囲と明示改ページ

明示 `0x000c` は control boundary のままだが、検証済み named `TextV.01` segment
では text 読み取りを終了しない。token parser と source map の両方が後続ページを
保持する。境界のない marker-only input は従来の保守的な停止動作を維持する。

core inline map 範囲は `0x001d`/`0x001e` を含む。完全な表示 UTF-16 文字列に
2 unit を加えた長さと一致するとき、model text 範囲は両 wrapper を除く。
不正・非連続 inline には推測の text span を付けない。raw map/flow 範囲は変更しない。
物理行の許容時も表示 byte を確認し、UTF-16 offset を character offset に変換して
surrogate pair 内部の境界を拒否する。

既存の fixed84 source-page 経路は、完全な selector-1 cache group、統制した
heading context（`0x0010`、13 unit、subfield `0x002e`、値1–3）、観測した
13-unit 番号 marker context（`0x0000`、subfield 10、`391/0x2010`）を許容する。
横書き・罫線なし flow のみに適用し、未知 context、不完全 wrapper、section style、
縦書きは fallback を保つ。保存済み marker と source 行・ページ割当の保持であり、
編集可能な見出し・一覧 semantics、目次 leader、動的 field、native 空白・tracking の
解読ではない。縦長・横長の page 範囲は共通 model layout を使うが、横長文書の
native 和文文字間隔は未解決のまま。

## 限定的な保存済み目次行

統制した `/DocumentText` 目次領域は、word 4 `0x0030`/`0x0031` を持つ
12-unit class `0x0020` record の組を使う。観測した他の field と順序が一致し、
一組だけであることを要求する。その設定 record と行末だけの source 行は text pitch
を消費しない。通常の空行や空白だけの本文へこの規則を拡張しない。

領域内の観測した 17-unit title context と任意の 18-unit leader context
（raw variant `1`/`100`）は既存の物理 source-page 経路を通せる。
この context で区切られた二つの表示 text run を、保存済み title/page label の
`tocEntries` metadata として source span・`decoded:false` 付きで保持する。
不完全、重複 section、未知 context は fallback と raw evidence を保つ。
生成・目次レベル・bookmark・編集可能な目次 model の解読ではなく、保存済み内容である。

一般の横方向 tab stop と正確な leader metrics は未証明。限定した保存済み leader 投影は後述する。page 割当と text pitch は共通 model 経路に
従うが、page label の横位置は fallback のまま。追加分析用の raw field を保全し、
参照 PDF の座標で領域の配置を補正しない。

## 限定的な field cache の対応

統制した JTD・JTT・JTTC は class0 の三つの field profile を共有する。
28-unit 印刷日（`0x0033`）、15-unit page number（`0x0035`）、12-unit 外部 link
（`0x0048`）。完全な selector1 value wrapper と selector0 argument wrapper が record
と連続することを確認する。日付は `DATE`、page number は `PAGENUMBER`、HTTP(S) link は
URL target と空の第二 argument を要求する。他の field/profile は raw のまま保つ。
`textFieldCandidates` は cache・argument・record/value span を `decoded:false` で公開し、
一般的な field expression engine の解読とはしない。

有効な `YYYY/MM/DD` 印刷日は model の render context であり、cache/raw を書き換えない。
`DocumentCore::set_print_date` が SVG と layer へ値を渡し、WASM wrapper/viewer は
browser の local date を供給する。native Unix PDF export は対応する印刷日 field がある
場合だけ platform の `date` command を使う。明示 API
`to_pdf_with_file_name_and_print_date` は全 native target で再現可能な出力を支える。
provider/context がなければ cache を維持する。page number cache は一般的な再番号付けをしない。

統制した外部 link profile は record 最初の unit の color/underline property を表示 cache
へ対応させる。SVG は escape した HTTP(S) target を持ち、layer は field/cache evidence を保つ。
完全な field group は物理 source-page 経路も通せる。bookmark 位置、別の日付形式・種類、
任意の target、編集可能な field semantics、PDF link annotation は未対応。
geometry/font metrics は既存 candidate/fallback の制限を維持する。

## LayoutBoxText Content

`/LayoutBoxText` にも長さで区切られた `TextV.01` block がある。区切られた content は
先頭から text mode で map し、先頭テキストと UTF-16 surrogate pair も保持する。
可視 text/inline だけを投影し、skipped inline annotation は raw stream に保持する。

制御だけの block で printable な payload word をすべて文字として読む fallback は
使わない。`054_font_size_paragraph_plus` の最初の block はタイトルではなく object
reference を持ち、以前の fallback は誤った `0ԇ` を出力した。この fallback は削除した。
テキストの復元だけでは、後続セル block の位置や所有関係は証明されない。

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

## 限定的な modern 縦書き本文

統制した modern sequential view family は、横書き・縦書きで同じ `0x1001` stock-size
profile を使用する。`0x1002` payload の margin quad は offset2 にあり、32-byte
横書き形は offset10 の `0x40`、33-byte 縦書き形は offset10/11 の `0x50,0x01` を
持つ。方向部分の後には同じ十個の `0x02bc` word と zero suffix がある。size/margin
record の重複、未知 flag、明示 page style はこの global profile を選択しない。
edit section `0x2000` は caret 状態で変わり、書字方向の判別に使用しない。
legacy の first-record heuristic は diagnostic のみ。

plain 本文は検証した LineMark source 列と既存 PageMark font-plus-gap pitch を
横方向に再利用する。既知 `0x100b` profile と `0x1006` tail の `0x0266` は
native UI の字間60%に対応する。限定 renderer は和文 run にその字間を適用するが、
一般的な数値単位の式を確定したものではない。英字・空白の送り、代替 font metrics、
縦用 glyph 置換は近似のまま。source の列境界を保持する。

class0 の 12-unit record `[1c,0,12,0,47,0,5,0210,12,0,0,1f]` に空の index0
selector1 cache と index1 selector`0x0101` の二桁 cache が続くと、統制した
幅に収めない縦中横候補になる。連続した完全な wrapper と terminator を要求する。
数字を source 付き TextRun に復元し、元 skipped cache と raw control を保全する。
SVG/PDF と layer は二桁を一つの縦方向 cell に横向きに配置し、通常の `123` は
一般の縦組み方向を維持する。他の長さ・非数字・fit mode・ruby・不完全 record を
この規則では昇格しない。JSON は record/value span と `decoded:false` を公開し、
geometry、font/spacing 解釈は候補のまま保持する。

統制した global 横長 profile も、縦書きと同じ `0x0266`・既知 `0x100b` の
字間60%対応を再利用する。和文 fullwidth run に字間を適用し、英字 run は
backend advance を保持する。source LineMark 行と PageMark pitch で page・baseline
を選び、SVG/PDF と layer は source range と候補字間を共有する。限定 uniform-font・
plain-flow 投影のまま。他字間値、rich/table/field/section profile、正確な英字・
空白 metrics、一般数値単位の解釈は未解決。

## 限定した保存済み目次 leader

既存の保存済み目次 scope 内で、厳密な17-unit title context
`[1c,0,17,0,9,375,31,0090,0,2,f81e,0,0,17,0,0,1f]` と任意の18-unit
leader record `[1c,0,18,0,21,0,23,0090,K,2,a77c,0,0,0,18,0,0,1f]` を照合する。
`K=1` は実線、`K=100` は点線、leader record が無ければ隣接番号という
三つの統制 variant を保持する。各 record の直後に完全な空 cache
`[1c,1,7,0,0,1,1d,C,1e,5,0,1,1f]` を要求する。title context の後は `C=5`、
leader record の後は `C=3` とし、literal title と保存済み
数字 label が同一 source 行・metadata entry に対応することを検証する。

統制 title context は全角一 cell の区切りを確保する。leader があれば label の
右端を source 本文右余白に置き、無ければ区切りの後で題名に隣接させる。
文字幅は backend の glyph advance、page と baseline は source LineMark/PageMark
で決める。SVG/PDF と layer は text span と leader-record span を保持する。
未知 context/cache、rich・縦書き行、物理 source 行欠落、幅の矛盾は fallback
本文を維持し、model を書き換えない。

限定した source 対応の描画候補である。実線・点線の選択は native 作成設定と
出力で照合したが、全角区切り、右 anchor、leader 縦位置、線幅、点間隔は
`decoded:false`/`geometryDecoded:false` のまま。leader metrics は backend 中立の
近似を使用し、native PDF 座標で補正しない。任意 tab stop、見出し navigation、
目次再生成と編集 semantics は未解明。

## 限定した脚注 marker と ruby source

単一 link の脚注候補は両方の marker source span を保持する。統制した横書き
profile の本文 marker は property1 が1、property20 が`0x80000000`、脚注領域
marker は2を参照する。明示 size/scale override は無い。唯一の SsmgSlots
TextLayoutStyle の`0x114`/`0x214`にある二つの`0x5555` record と kind・character・
paragraph subrecord を照合する。slot1の厳密な`0x5004` profile は`-50`・`60`・
`50`・`50` field と半サイズ上付き marker に対応し、slot2は通常 profile を保つ。
完全な link/span/reference/style 対応だけで既存半サイズ上付き renderer を再利用する。
別参照・倍率・不完全 profile は通常 fallback 本文と raw data を保つ。SVG/layerは
継承 script の basis、JSONは脚注側 marker span を公開する。一般 property1 継承
decoder を確定したものではない。

統制した layout mark 無し本文 profile は literal な二段落、source 改行一つ、
既定 size370、既知`0x100b`の600行間 profile を持つ。既知脚注と任意の grouped
ruby record だけを含み、他 object/table/section data が無く、各段落が一行に収まる。
source と model の文字列一致を要求する。SVG/layer は60%行間の対応を用い、
fallback の空段落行を追加しない。編集・折返し・未知行間・別 layout は fallback。
数値単位と geometry は候補のまま。脚注領域の baseline・区切り配置は未証明。

ruby 昇格時に base TextRun の source span を破棄せず保持する。JSON/layer はspan、
SVG/PDF はsource font size とbackend base advance を再利用する。source font が
ある ruby は半サイズ annotation と em 相対上方 baseline を描画候補として使い、
source font が無い手動 annotation は従来 fallback を保つ。

統制 grouped kana record は`[1c,0,12,0,5,0,517,512,12,0,0,1f]`と、連続した
完全な selector3 base・selector`0x0082` annotation cache で構成する。source の
base・annotation と model の一致、annotation font/scale override 無しを要求する。
測定した base 幅に収まる全角 kana に対し、残り幅を cell 間に分配し、両端には
半 gap を置く。`data-group-ruby-candidate`とdecoded-false geometryを保持する。
別 grouping record・font 変更・script・長いまたは非 kana annotation は fallback。
一般 ruby 単位、正確な printer metrics と編集 semantics は未解明。

## 限定した選択 page の字間

既存の単一書式・縦長/横長/縦長対応を、中間pageの統制字間にも使用する。
既知apply/reset・fixed84の三page entry・用紙寸法swap・同じsource余白を持つ
page2だけを認める。global view の横書き判定も独立に要求し、明示styleに対する
既存global-only方向guardは維持する。

唯一の`0x4006` subrecordは26byte、prefix`0000c10000`、5..7にglobal既定値と
同じ百分の一mm font size、7..14に`8000003f000266`、14..18に100/100 scale、
suffix`8000800002800000`を持つ。唯一の`0x400a` profileは`c300000d000050400100`。
global viewと同じ統制字間60%対応を照合したが、一般数値式やstyle継承を
確定したものではない。

共有tracking投影は物理source行とbackend英字advanceを再利用し、和文cellには
60%字間候補を維持する。SVG/layerは`page-layout-style-4006`と
`document-view-style-1006`を区別する。未知size/scale/spacing/line profile・
書字方向・layout/余白対応はfallback。別pageは従来描画を維持する。
geometry、glyph baseline、空白とprinter量子化は候補のまま。

## 限定した単独段落の改行幅

統制した plain 段落は、class `0x0010` の16語 record を使う:
`001c 0010 0010 0000 0020 0004 0008 P 0000 0000 ffff 0000 0010 0000 0010 001f`。
既存の表・本文 profile と異なり、二つ目の pitch pair は繰返しではなく
未設定の `0,0` である。統制した `P=1000` は保存済み10mmの改行幅と対応する。
限定した百分の一mm pitch 候補であり、一般段落 decoder ではない。

完全な単独 profile が model 段落の保存済み source span の直前にあり、
global 横書きで rule grid がない場合だけ使う。すべての該当 record に段落の
対応を要求する。LineMark の物理範囲から折返し行を選び、pitch 候補はその段落の
source 範囲内で続き、末尾で終了する。後続段落には伝播しない。既存の表の
繰返し属性の検査は維持する。本文・空白・raw record・PageMark 値を変更せず、
SVG/PDF と layer は同じ source page plan を使い、予算検査も物理行数を数える。

未知の field、source/line 範囲の不足、縦書きや grid の混在、失われた source 対応は
fallback を保つ。他の段落 profile、正確な glyph metrics・印刷時の量子化は未解明。
Geometry は decoded-false のまま。
