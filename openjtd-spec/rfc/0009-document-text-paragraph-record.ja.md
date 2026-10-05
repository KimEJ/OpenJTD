# RFC 0009: DocumentText 段落レコード構造

Status: draft

Observed: 2026-06-24

English version: [0009-document-text-paragraph-record.md](0009-document-text-paragraph-record.md)

## 概要

`/DocumentText` ストリーム中のすべての `0x001c` 制御境界の直後には、自己記述型の可変長ヘッダーレコードが続く。このレコードは `0x001f` で終わり、既存のテキストランマーカーでもある。つまり、現在のトークンパーサーが単純な `ControlBoundary` として出力するすべての `0x001c` は、実際には構造化された段落/レイアウトレコードのオープナーである。その後の `0x001f` はレコード自身のターミネーターであって、独立したテキストランマーカーではない。

## レコードレイアウト

インライン形式以外のすべての `0x001c` レコードは次の構造を持つ：

```text
ワード位置   フィールド
w0   0x001c  レコードオープナー（ControlBoundary トークンと同じコード）
w1   class   レコードクラス：0x0000 / 0x0010 / 0x0020 / 0x0030
w2   len     レコード総長（u16 ワード数、w0 とフッターを含む）
w3…          ペイロードワード（クラス固有、数 = len - 7）
w[len-4]     len エコー（w2 の繰り返し）
w[len-3]     0x0000
w[len-2]     class エコー（w1 の繰り返し）
w[len-1]     0x001f  レコードターミネーター / テキストランスタート
```

フッターパターン `(len_echo, 0x0000, class_echo, 0x001f)` は、ローカルサンプル `sample-academic.jtd` と `sample-table.jtd` の合計 948 件のレコードで検証済み。unit 22733 の例外 1 件は `0x001d`〜`0x002a` の連番制御ワード列で、段落レコードではなく誤検出。

ルビ/インラインセグメントに使う `0x001c 0x0001 0x0007 … 0x001d` の形式は既存のトークンパーサーが処理する別形式であり、同じ `0x001c` オープナーを使うが `class=0x0001`、ターミネーターは `0x001d` である。

## 観測されたレコードクラス

### クラス 0x0010 — 段落/行ヘッダー

全サンプル中最も一般的なクラス。`sample-academic.jtd` では 19 件すべて len=20。複数列の表サンプルでは len が 10〜59 の範囲で変化する。

短い代表例（len=13、`sample-table.jtd`）：

```text
001c 0010 000d  0000 002e 0001 0001 ffff 0000  000d 0000 0010 001f
```

長い代表例（len=27、`sample-table.jtd`）：

```text
001c 0010 001b  0000 008f 000f 010c 0000 0003 0023 0000 0000 007e 0023
0000 0000 007e 0023 0000 0000 0006 ffff  001b 0000 0010 001f
```

`sample-academic.jtd` の 19 件はすべて len=20 でペイロードが安定している：

```text
0000 0000 0001 0001 0026 0005 [w9] [w10] 0000 0000 0000 ffff 0000
```

`w9` はサンプル全体で常に `0x0001`。`w10` は変化する：

| w10    | 件数 | 後続テキスト（先頭 8 字付近） |
| ------ | ---: | ----------------------------- |
| 0x0000 |   14 | 通常本文段落                  |
| 0x0141 |    4 | インデント付き続き行          |
| 0x01f4 |    1 | 末尾の空白行                  |

`w10` の意味は未解読。インデントレベルまたは段落継続フラグの可能性がある。

**クラス `0x0010` のサブタイプ（decoded:false）。** `w4` フィールドにより `sample-table.jtd` で少なくとも 4 種類のサブタイプが識別される：

| w4 値 | len | 件数 | 役割（decoded:false） |
| ----- | --- | ---: | --------------------- |
| `0x002e`（46）    | 13 |  18 | 単列段落レコード |
| `0x008f`（143）   | 27〜61 | 129 | 表行列スペックヘッダー |
| `0x002a`（42）    | 37〜47 |   2 | 複合遷移レコード（Y 座標 + 内部 0x008f サブブロック） |
| `0xffff`（65535） | 10 |   3 | ヌル/セクション終端マーカー |

`w4=0x008f` レコードの場合：`len = w5 + 12`（129 件で検証済み）。`w6=268` は後続 `0x0030` 行のセル `b1` 最大値に等しい（表全幅の右端座標仮説と整合）。可変長ペイロード（`w9..w[len-5]`）は 4 ワードのサブエントリ `[tag, v1, v2, v3]` で構成され、`[0xffff, 0x0000]` センチネルで終端する。最頻出タグは `0x23`（546 件；v1=v2=0；v3 はセルスパンと相関するが未解読）、`0x2b`（71 件；v1=0, v2=0x08；v3=0x1d〜0x76）、`0x1b`（11 件；v1=0, v2=0x08, v3=0x1f）。主要ケースでは `n_sub_entries = n_cells − 1` が成立（n_sub=3/n_cells=4 が 68 件、n_sub=9/n_cells=10 が 27 件）。`w8` フィールドの役割は未解読（`0x0023` タグ件数とは一致しない）。サブエントリのタグ意味論および v3 と座標の関係は未解読。

`w4=0x002a` レコードの場合：`w7` が大値（> 1000）ワードペアの個数を保持（`w8..w(8+2*w7-1)` に格納）。これらの値（例：13200、13800、14000）は垂直 Y 座標（候補解釈：1/100mm；138mm や 140mm は A4 の表区切り線として妥当）の可能性がある。Y 座標ブロックの後に続く内部 `0x008f` サブブロックが次の表の列レイアウトを独立した `w4=0x008f` レコードと同じ構造でエンコードする。

`w4=0xffff` レコードの場合：ペイロード全体が `(0xffff, 0x0000)` のみで、列エントリを持たない。

### クラス 0x0030 — 表セルヘッダー（12 ワード固定）

表を多用するサンプルに出現。表示行ごとにセル 1 件につき 1 レコード。固定 12 ワード構造：

```text
001c 0030 000c  0000 [b0] [b1] 00ff 0000  000c 0000 0030 001f
```

`w2` と `w8` の繰返し値はレコード長であり、文字サイズではない。2026-09-26 の調査では、
対応済み envelope（`w6=0x00ff`、`w7=0`）に一致する 12,315 windows をローカル 83 files で
確認し、すべて `w2=w8=12` だった。固定長 reader は両方の長さの一致を必須とする。
他の envelope は未解読のまま保持する。

古い model diagnostics の一部はこの値を `fontSizeUnits` と呼び、参照画像で調整した
row/stroke projection に使っている。この名称と式は旧仮説であり、解読済み font metrics
ではない。実際の文字サイズの根拠は style property 2 と対応する view-style 既定値であり、
[RFC 0003](0003-document-text.ja.md#observed-font-size-sources) を参照する。

**`b0`/`b1` はセル境界座標（スケール・単位は decoded:false）。** `sample-table.jtd` の 703 件の分析結果：

- `b0` = セル左端座標（表座標空間）
- `b1` = セル右端座標；`b1 − b0` = セル幅（同単位）
- 同一行内のセルは非重複かつ左から右へ順序付き
- 隣接セル間のギャップは常に 4 ユニット

主要な 2 列比較表（1 行あたり 4 セル）の代表的なレイアウト：

```text
gap  [b0, b1]   幅   役割
  0  [  0,  2]    2  左ボーダー
  4  [  6,130]  124  A 列（改正案）
  4  [134,258]  124  B 列（現行）
  4  [262,268]    6  右ボーダー
```

表全体のスパン = 268（`max(b1)` = `0x010c`）は、直前の `0x0010` 行ヘッダーレコードの `w6` フィールドと一致する。座標値の物理単位は未解読であり、268 は A4 ページの本文領域に対して単純な mm 単位や 1/10 mm 単位では対応しない。

RFC 0003 §COM テキストエクスポート観測の「shanai_lan `0x001c/0x0030` 行ヘッダー」コンテキストに対応するフォーマット。

### クラス 0x0000 — インラインセグメントコンテキストマーカー（12 または 21 ワード）

`sample-table.jtd` で 2 種類の形式が観測された（計 92 件）。

**len=12（14 件）。** 常に `0x001c/0x0030` セルヘッダーの直後、かつ `0x001c/0x0001` ルビ/インラインセグメントの直前に現れる。構造：

```text
001c 0000 000c  0000 [w4] [w5] [w6] 0000  000c 0000 0000 001f
```

`w4=0x0007` は後続 `0x0001` インラインレコードの `len` フィールド（7 ワード）に一致。`w6=0x020d=525` は 14 件すべてで定数（スタイル/タイプコード候補）。`w5` は変化する：`0x00dc=220` は広コンテンツ列（幅=124、b0=6 または b0=134）で "改正案"/"現行" 列ヘッダーの前に現れる；`0x0098=152` と `0x008e=142` は狭い列（幅=28、b0=12）で省庁名ラベルの前に現れる；`0x008c=140` は対称的な狭列（幅=28、b0=144）に現れる。同一省庁名でも所属する列によって異なる `w5` 値を持つ。`w5` の物理的意味は未解読 — ラベルテキストより、セル位置またはセル種別に紐付くスタイルセレクタに敏感であるように見える。

**len=21（77 件）。** 最多の `0x0000` 形式。安定したヘッダーブロックと可変テールを持つ：

```text
001c 0000 0015  0000 0056 0000 0406 0010 [w8] [w9] 0000 0000
                0000 [w13] 0000 0000 0000  0015 0000 0000 001f
```

フィールド `w4=0x56=86`、`w5=0x0000`、`w6=0x0406=1030`、`w7=0x0010=16` は定数（スタイル/コンテキストコード、未解読）。`w8` はフラグ（0 または 1）。`w8=0` のとき `w9` は小値（0/2/4/5）、`w13` はほぼ 0 または 2（27×`(w9=4,w13=2)`、20×`(w9=2,w13=2)`、11×`(w9=0,w13=2)`、9×`(w9=2,w13=0)`、2×`(0,0)`、1×`(5,2)`、1×`(1,0)`）。`w8=1`（6 件）のとき `w9` と `w13` は常にペアで出現：`(w9=0x025d=605, w13=0xcd=205)` は常にセル `[b0=4, b1=132]` に対応し、`(w9=0x0229=553, w13=0x99=153)` は常に対称セル `[b0=136, b1=264]` に対応する。両 `w8=1` ペアで `w9 − w13 = 400 = (b1 − b0) × 3.125` が一定。2 セル間で w9 と w13 はともに正確に 52 減少（605→553 および 205→153）するが b0 は 132 増加しており、線形関係は見つかっていない。`w8=1` レコードは常に `0x001c/0x0001` ルビ/インラインレコード（列見出しテキスト）の直前に現れ、`w8=0` レコードは通常テキストラン内に現れる。物理的意味は未解読；`w9 − w13 = (b1 − b0) × 3.125` という一定比は観測された不変量にすぎない。

### クラス 0x0020 — 表セクション遷移マーカー（12 ワード）

`sample-table.jtd` で 4 件のみ観測。常に `0x000e`（表行区切り）の後、かつ `0x001c/0x0010` 単列段落の直前に現れる：

```text
001c 0020 000c  0000 0010 [w5] 0000 0001  000c 0000 0020 001f
```

`w4=0x0010=16`（後続 `0x0010` レコードのクラスコード）、`w7=0x0001=1` は定数。`w5=0x0002` または `0x0000`。表セクションから通常の単列テキストへの遷移を示すと考えられる。意味は未解読。

## 検証済みの先頭ページ Control-Table Text 配置

統制された source-y corpus では、横書きの先頭ページ control table に限定した
描画可能な profile を確認した。この profile は一般的な table-candidate 検出より意図的に
厳しい。次のすべてを必要とする。

- unit 基準の `0x000e` table candidate と安定した column pattern。
- 各 row に対し、row interval で始まるか直前で終わる 1 個の `0x0010` parent record。
  すべて同じ非ゼロの `w6` grid extent を持つこと。
- 各 cell text range の直前で正確に終わる `0x0030` header。
- parent または row で始まり row interval の終端で終わる選択済み `LineMark` interval。
- `w21` が妥当な 1/100-mm line pitch である単一の最初の `PageMark` entry。
- 対応する page margins と、各 cell の一様に解決できる text style。

この profile では、cell text は `0x0030` の `b0` offset を parent `w6` extent 内で
source margin に対して scale する。各 row baseline は `LineMark` record index と
`PageMark w21` の積に実際に解決した font size を加える。cell label 直前に保存された
ASCII space は 1 文字あたり 2 source grid units として位置だけを調整し、新しい可視
glyph としては描画しない。

許可した 20 の統制文書は、baseline、横方向移動、行単位の縦方向移動、個別 row-height
変更を含む。`PAGE 01`、`PAGE 01_right_4Tick`、`PAGE 01_down_4Low`、`000_base_a`、
`013_table_moved_right`、`020_row1_height_plus` の生成 PDF は、各一太郎 PDF に対して
cell text の X/Y を 0.25 pt 以内で配置する。この根拠には ink width は含まれず、ローカル
代替 font は label により約 3.9 pt 狭い。source border-paint semantics は未解決であり、
下記の基本 border projection は明示的な renderer fallback style を使う。

`050_wrapped_one_cell` は除外する。wrap 後に cell header offset が row 間で変わるため、
単純な row ごとの text placement rule は健全ではない。複数ページ table、merged cell、
empty/sparse cell、縦書き、mixed object tree は診断専用のままである。

3-row、2-column probe は control のみの
grouping に対する構造的な反例を追加する。`BEFORE-TABLE` 段落は最初の完全な
`0x0010` / `w4=0x008f` row header と同じ `0x000e` interval に入るが、可視
text はすべてその header の開始前に終わる。control-run table candidate は source
text を保存したまま、この段落を row から除外する。実際の cell header では、既存の
`0x00ff` に加えて `w6=0` も観測し、12-word count、echoed count、terminator は
同じである。論理 control-table candidate の許可経路ではこの word は未解読のままで、
他の値は許可しない。下記の別の物理 span projection は明示的な alignment に対して
独立した許可条件を持つ。local の
回帰検査は 3 row、順序を保った 6 cell、前後の各段落が SVG に一度ずつ現れることを
要求する。native PDF の geometry や font metrics の一致を証明するものではない。

数値 cell を持たない独立した 2×2 text-only probe
である。完全な `0x0010/w4=0x008f` parent と各 cell の直前にある完全な `0x0030`
header により、旧来の value-marker heuristic を使わず短い table を許可する。
非空 cell の範囲はすべて parent header の後かつ row 内にあり、column count は
一致する必要がある。frame を持たない 2-row text は未証明のままである。これは
candidate の許可条件であり、一般的な geometry の解読ではない。

同じ完全な parent/cell frame を持つ 1×2 probe
である。複数 column を持つ 1 row を control-table candidate として許可するが、
frame のない 1-row text は引き続き拒否する。1-row border projection は検証済みの
末尾 empty-row record を必要とし、2 row 目から高さを推測せず source の境界を使う。
native PDF geometry の一致は未証明のままである。

2×1 probe は旧来の最小 column 数 heuristic に対する
反例である。1-column control row は前述の完全な native parent/cell frame を
持つ場合に限り許可する。既存の狭い profile で 2 cell と source border を描画するが、
frame のない 1-column text は table candidate にしない。geometry と paint の
意味は未解読のままである。

### 2 つの native control table 間の flow text

さらに、先頭ページ横書きでは、完全な control-table projection が 2 つあり、その間に
`DocumentText` の text run が 1 つだけある場合に限り、狭い flow-text profile を描画できる。
この run は、同じ非ゼロ個数の先頭 ASCII space を持つ非空 ASCII 行をちょうど 2 行含み、
最初の table の後の gap に zero-offset `0x0030` header が一意に 1 個あり、各行が別々の
`LineMark` interval に完全に含まれなければならない。両 interval は対応する `w21` pitch を
持つ同一の最初の `PageMark` entry に解決される必要がある。renderer は source space を保存し、
各 baseline を interval の record index、共通 pitch、および解決済み source font size から置く。

`070_two_tables_vertical` はこの profile を満たす。2 個の 2×2 native-table projection が
`BETWEEN01` と `BETWEEN02` を挟み、それぞれの `LineMark` record は 7 と 9 である。生成 PDF は
10 個の text run をすべて分離し、一太郎 PDF との差は X が最大 0.51 pt、top position が最大
0.21 pt だった。この数値は厳しく gated した flow bridge だけを検証するものであり、一般的な
paragraph layout、whitespace semantics、font ink width を解読したことは意味しない。

### 物理 ruled-flow span と明示的な文字揃え

統制した 1-row、2-column の対照 pair では、最初の cell の text と 3 行の物理表示を
保ち、明示的な cell alignment だけを変更した。各行は frame を持つ独立した `0x0030`
declaration を保持する。同じ declaration の繰返しは論理 row/cell の追加を意味しない。
両 variant の最初の cell は `b0=2`、`b1=78`、parent grid extent 160 を共有する。

| 設定 | `w6` | 各物理行の `w7` |
| --- | ---: | --- |
| `左寄せ` | 0 | 2, 2, 0 |
| `均等` | 3 | 2, 2, 0 |

この profile では `w6` と明示的な alignment の対応を観測できる。変化しない `w7`
sequence は continuation 関連の候補であり、justification、論理 cell identity、完全な
flag schema の証明ではない。継承値 `w6=0x00ff` は別扱いとする。前述の wrapped
counterexample の spacing は明示的な左寄せでは再現できない。

model は table view を導出する前に、順序付き `DocumentTextFlow` events と raw flags を
保存する。先頭ページ横書き ASCII の限定 projection は、直前の parent/declaration、
正確な `LineMark` containment、最初の `PageMark` pitch、source margins、一様な source
font size を使い物理 span を配置する。両端に空白がない明示的な均等 span は
`(b1-b0) * bodyWidth / parentGridExtent` を SVG `textLength` とし、
`lengthAdjust="spacing"` を使う。左寄せは均等な文字間 stretching を使わないが、欧文の
単語間 spacing は下記の文書 style にも依存する。論理 table row を
再構成するものではない。重なる table fallback は描画を止めるが、候補は layer tree の
診断に残す。raw flags、`decoded:false`、推定 glyph-position の表示を維持する。

両 variant は local の source-span/order/重複描画検査を通過し、生成 PDF は line anchor と
均等 extent を保持した。厳密な glyph metrics と ruled border は証明していない。
継承された wrapped `0x00ff` profile は、独立した spacing 根拠が得られるまでこの
projection で拒否する。均等な文字間 spacing を欧文の単語間 justification の代用にしない。

### 文書既定の欧文 justification

統制した ON/OFF pair では `/DocumentText` の全 bytes が同一であり、最初の cell の
`w6=0` と `w7=2,2,0` sequence も変わらない。cell flags だけでは欧文 justification を
決定できない。`/DocumentViewStyles` の既定 `0x100b` record は次の完全 payload に変わる。

```text
ON:  02 02 58 00    04 00 00 00 08
OFF: 02 02 58 40 00 04 00 00 00 08
```

限定した model candidate はこの profile と一意の record だけを認識する。unknown mask、
value、重複 record、他の envelope は未証明であり、完全な optional-field schema ではない。
raw style bytes を保存し、JSON は `englishJustificationCandidate` を `decoded:false` で示す。

native PDF は ON のとき末尾以外の物理2行を justify し、最後の行は変えない。対応する
multiword ASCII flow では source span width から先頭 grid padding を引き、要求する advance
extent を求める。paint backend が natural font advance を測定し、model が正の残り幅を内部
ASCII space に配分する。均等 `textLength` ではなく SVG `word-spacing` を出力する。PDF と
browser adapter は実際の font measurement を同じ model rule に渡す。未測定の場合は
SVG/layer を明示的に unresolved とし、解読済み glyph position と主張しない。

先頭/末尾 text run が先頭ページの `LineMark` interval と正確に一致する場合、前後の body
baseline も source pitch から置き、flattened paragraph order による wrapped line 上への
重なりを避ける。一般的な paragraph/page assignment の解読ではない。ON/OFF 回帰は同一の
source flow、spacing request2件と0件、最後の行が不変であること、前後textの分離を対象とする。
代替 font metrics、single-word tracking、wrapped 継承値 `0x00ff`、他の writing mode、ruled
paint はこの許可範囲に含めない。

### 基本 control-table border projection

対応 profile の縦方向 border 位置は `parent w8 + 1` と各 cell header の `b1 + 1` を
`bodyWidth / parent w6` で scale する。横方向位置は
`marginTop + recordIndex * pitch + resolvedFontSize / 2` を使う。最初の境界は最初の非空 row の
record minus one、内部境界は次の非空 row の record minus one を選ぶ。最後の境界は、parent grid と
left edge が一致し、source range が対応する `LineMark` interval と一致する連続した空の control row
だけを追跡する。非 table への遷移で scan を終了し、source gap を飛び越えない。

ローカルの `020_row1_height_plus`、`PAGE 01`、`070_two_tables_vertical` の PDF 比較では、
測定した border center の差は 0.14 pt 以内だった。これは profile の座標根拠であり、完全な
border-paint 同等性ではない。現在の SVG/PDF projection は、黒色 0.8 CSS-px line（0.6 PDF pt）を
renderer fallback として使う。color、thickness、dash pattern、join、corner のはみ出し mark は
この projection では解読していない。border 出力は `decoded:false` のままで、現在は SVG/PDF に
含まれる。page-layer tree は table text を公開するが、これらの border line は出力しない。

## LineMark unit-start との相関

`sample-academic.jtd` サンプル（解析済み LineMark レコード 25 件）では、LineMark の `unit-start` 値と `/DocumentText` の `0x001c` レコード位置の間に明確な対応関係がある：

| LineMark record | LineMark unit-start | /DocumentText 中の 0x001c 位置 |
| --------------- | ------------------: | -----------------------------: |
| 0               | 16                  | 16                             |
| 1               | 83                  | — （83 に 0x001c なし）        |
| 2               | 129                 | 129                            |
| 3               | 150                 | 150                            |
| 4               | 179                 | 179                            |
| 5               | 248                 | 248                            |
| 6               | 332                 | — （332 に 0x001c なし）       |
| 7               | 360                 | 360                            |
| 8               | 416                 | 416                            |
| 9               | 484                 | 484                            |
| 10              | 546                 | 546                            |
| 11              | 618                 | 618                            |
| 12              | 681                 | 681                            |
| 13              | 759                 | 759                            |
| 14              | 846                 | — （テキストラン内部）         |
| 15              | 877                 | 877                            |
| 16              | 957                 | — （テキストラン内部）         |
| 17              | 971                 | 971                            |
| 18              | 1051                | — （テキストラン内部）         |
| 19              | 1076                | 1076                           |
| 20              | 1141                | 1141                           |
| 21              | 1217                | 1217                           |
| 22              | 1238                | 1238                           |
| 23              | 1259                | 1259                           |
| 24              | 1280                | 1280（0x0000 ドキュメントターミネーター） |

25 件の LineMark `unit-start` のうち 14 件が `0x001c` レコード位置に完全一致する。残り 11 件はテキストラン内部または `0x0000` ドキュメントターミネーター位置に相当する。この部分的な重複は、LineMark が物理的な表示行を表し、`0x001c` 段落レコードが論理段落を表すという仮説と整合する。1 つの段落が複数の表示行に折り返される場合、そのすべての表示行に LineMark が生成されるが、`0x001c` は段落開始時のみ出現する。

LineMark の `flag` 値と `0x001c` レコードペイロードフィールドの相関は未確認。`flag=0x0002` が最多（25 件中 18 件）、`flag=0x0003` はレコード 0（文書先頭）、`flag=0x0000` はレコード 1, 6, 14, 16, 18（`0x001c` 位置と一致しない）。`flag=0x0000` は段落内の続き表示行を示す可能性がある。

### 裏付け：`sample-draft.jtd`

law-document draft サンプル `sample-draft.jtd`（解析済み LineMark レコード 43 件、`base-unit=16`）でも同一のパターンが確認される：43 件の `unit-start` のうち 25 件が `0x001c` レコード位置に完全一致する。

代表的な行（43 件中前半 31 件）：

| LineMark record | LineMark unit-start | /DocumentText 中の 0x001c 位置     |
| --------------- | ------------------: | ---------------------------------: |
| 0               | 16                  | — （16 に 0x001c なし）            |
| 1               | 41                  | 41                                 |
| 2               | 103                 | — （テキストラン内部）             |
| 3               | 114                 | 114                                |
| 4–7             | 178–322             | — （すべてテキストラン内部）       |
| 8               | 353                 | 353                                |
| 9               | 386                 | 386                                |
| 10              | 445                 | 445                                |
| 11              | 513                 | 513                                |
| 12              | 579                 | 579                                |
| 13              | 634                 | 634                                |
| 14              | 668                 | 668                                |
| 15              | 729                 | 729                                |
| 16              | 793                 | 793                                |
| 17              | 918                 | 918                                |
| 18              | 1036                | 1036                               |
| 19              | 1070                | 1070                               |
| 20              | 1128                | 1128                               |
| 21              | 1184                | 1184                               |
| 22              | 1223                | 1223                               |
| 23              | 1288                | 1288                               |
| 24              | 1351                | — （テキストラン内部）             |
| 25              | 1372                | 1372                               |
| 26              | 1436                | 1436                               |
| 27              | 1462                | 1462                               |
| 28              | 1512                | 1512                               |
| 29              | 1551                | 1551                               |
| 30              | 1572                | 1572                               |
| 31              | 1636                | — （テキストラン内部）             |
| 32–40           | 1667–1675（各 delta=1） | — （すべてテキストラン内部）   |
| 41              | 1684                | 1684                               |
| 42              | 1748                | — （テキストラン内部）             |

レコード 32–40 は各 `delta=1`、unit-start 値が 1667–1675 と連続する。これらは unit 1589–1684 にまたがる単一の長いテキストランの内部に存在する（施行日条文と理由文の間の複数 `\n` 連続を含む段落）。delta=1 の連続 LineMark レコードがそれぞれ埋め込み改行 1 個に対応しており、`0x001c` 段落境界が存在しない位置でも LineMark が物理的な表示行先頭を列挙することを確認する。

## 現在のトークンパーサーへの影響

`rjtd-core/src/document_text.rs` の `parse_document_text` 関数は現在ストリームをビッグエンディアン UTF-16 として読み取り、`0x001c` を単純な `ControlBoundary` として扱う。`0x001c` が現れると次の `0x001f` まで新しいテキストランを開始しない。これはテキスト抽出としては機能している。なぜならヘッダーワードは有効な Unicode テキストとして解読されないからである（制御範囲の値）。パーサーは実質的に `0x001c` を境界として停止し、`0x001f` で再開することでヘッダーをスキップしている。

model は named TextV.01 content 内の完全な length/echo/class/terminator frame を
順序付き `DocumentTextFlow` record event として保存する。unknown gap と raw words は
保持し、frame の認識をインデント、style reference、論理 cell、border paint の解読と
同一視しない。table candidate は派生 view であり source event を置き換えない。
`decoded:false` の原則を適用する。

## 末尾の TextV.01 スタイルイベントセクション

`/DocumentText` には UTF-16BE content の後ろに byte-oriented event stream が存在する。byte offset 28 の big-endian `u32` は content-unit count であり、観測された style section は `32 + content_unit_count * 2` から始まる。event cursor の source unit は 16 から始まり、32-byte の `/DocumentText` header と一致する。

観測された event grammar：

- `00 <u32-be length>`：`length` source units を cover する run。
- `fe (<property-id> <value-length> <value-bytes>)* ff 00`：1 source unit を cover する property-change event。
- `ff`：terminal marker。残りの bytes は trailing data として保存する。

property changes は persistent state を構成する。value は change unit 自体に適用され、別の change で置き換えられるまで後続 run events に引き継がれる。現在観測済みの typed widths は property IDs 4〜7/9〜12 が 1 byte、1〜3/8/13/14/18/19 が 2 bytes、15〜17/20 が 4 bytes である。unknown IDs と width mismatch は raw evidence のまま保存する。この event stream は前述の `0x001c/0x0010 w4=0x008f` table-row header family とは別構造であり、混同してはならない。

`shanai_lan` の label probe と統制した body 対照は property 15 と text color の対応を示す。
単一 value が text fragment の exact source range 全体を cover する場合、観測された
`0x00BBGGRR` values は次のように対応する。

| Property 15 value | CSS color | 観測された用途 |
|------------------:|-----------|----------------|
| `0x00008000` | `#008000` | diagram title |
| `0x00800000` | `#000080` | blue device/server labels |
| `0x00660000` | `#000066` | dark-blue NAS label |
| `0x00000000` | `#000000` | 統制した黒の body text |
| `0x000000ff` | `#ff0000` | 統制した赤の body text |
| `0x00ff0000` | `#0000ff` | 統制した青の body text |
| `0xffffffff` | default | automatic/default color sentinel |

これは該当 ranges の packed-color encoding を証明するが、property role の universal
semantics は証明しない。cross-sample `hyo` では property 15 が table state に関連する
non-text/control ranges にも現れる。そのため、対応する body SVG/layer path の uniform
exact text ranges に source color として適用し、`shanai_lan` projection では decoded-false
candidate として使う。control/table-state ranges を color として解釈しない。mixed、
uncovered、未対応 high-byte value は default fill を保つ。body 対照は 3 個の raw value と
SVG/layer color を検証するが、一般的な style inheritance や border color は証明しない。

## 0x000e と 0x000a 制御コード

### 0x000e 行区切り

`sample-table.jtd`（表が多い新旧対照文書）では、すべての `0x000e` が直前と直後に `0x001c` レコードを持つ。`text-control-context` 診断ツールで、すべての `0x000e` が `prev-control=0x001c`（クラス `0x0030`）および `next-control=0x001c`（クラス `0x0030`）を持つことが確認された。

これは `0x000e` がクラス `0x0030` セルヘッダーレコード間の**表行区切り**として機能するという仮説と整合する：

```text
0x001c 0x0030 ...行 N セル A ヘッダー...
... セル A のテキスト ...
0x000e  ← セル A とセル B の間の行区切り
0x001c 0x0030 ...行 N セル B ヘッダー...
... セル B のテキスト ...
```

`text-control-ranges` 診断ツールは連続する `0x000e` レコードが正確に 2 バイト（1 u16 ワード）間隔であることを示す。つまり **`0x000e` 自体は追加ペイロードなしの 1 ワード制御コード**で、未加工の 1 ワード表行区切りとして機能する。2 列の新旧対照表のパターン：

```text
[前の列のテキストコンテンツ]
0x001c 0x0030 [12 ワード = セル A ヘッダー] 0x001f [セル A テキスト...]
0x000e                                              ← 1 ワード行区切り
0x001c 0x0030 [12 ワード = セル B ヘッダー] 0x001f [セル B テキスト...]
```

これは RFC 0003 §COM テキストエクスポート観測（`shanai_lan` の表コンテキストで `0x001c/0x0030` 行ヘッダーと `0x000e` 行区切りが観測された）を裏付ける。

### 0x000a 改行コード（decoded:false）

`0x000a` は `sample-table.jtd` に 210 件出現し、現在の全サンプルに存在する（ファイルごとの件数は 2〜4671）。`0x000e` とは異なり、`0x0030` レコード間のセル間位置に限定されない。コンテキスト分析：

- 最頻出の直前ワード：`0x001f`（74 件）— テキストランスタート／レコードターミネーター；CJK 文字や ASCII スペース `0x0020` の後にも現れる
- 210 件中 169 件が `0x001c 0x0030`（表セルヘッダー）に続き、24 件が `0x001c 0x0010`（段落ヘッダー）に続く

このことから `0x000a` はセル内改行またはセル内テキストランを継続する新しい `0x001c` レコードへの接続に使われる **セル内行区切り** または **段落内改行** として機能していると考えられる。意味論は未解読であり、ソフトリターン、表セル内のハード改行、または非表コンテキストでの段落レベル改行に対応する可能性がある。`0x000e`（`0x0030` レコードで両端が挟まれたセル間行区切り）と混同しないこと。

## 未解決の課題

- クラス `0x0010` のペイロードワードのうち、フッターパターン以外の意味は未解読。`w10` フィールドはスタイルまたはインデントをエンコードしていると考えられるが、レンダリング出力との照合は未実施。
- クラス `0x0030` のフィールド `b0`/`b1` は部分的に解読済み：`b0` がセル左端、`b1` がセル右端（表座標空間）；セルは非重複で 4 ユニットのセル間ギャップを持つ。座標値の物理単位は未解読。
- クラス `0x0000`/`0x0020` は構造的パターンが文書化されたが意味論的には未解読：`0x0000 len=12` はルビ/インライン直前（`w4=7`=インライン len、`w6=525` 定数）；`0x0000 len=21` は表セル内で多発（定数ブロック+可変 `w8`/`w9`/`w13`）— `w8=0` のとき小値フラグ（支配的：`w9=4,w13=2` 27 件；`w9=2,w13=2` 20 件；`w9=0,w13=2` 11 件）；`w8=1` のとき大値でセル固有（`w9−w13=400=(b1−b0)×3.125` が不変量；`w8=1` レコードは常に `0x001c/0x0001` インライン/見出しコンテンツの直前に現れる）；`0x0020 len=12` は表→段落遷移（`w4=0x0010`、`w7=1`）。
- LineMark との部分的な重複（25 件中 14 件一致）は論理/物理行仮説と整合するが未証明。
- 表セルの `0x001c` レコードが同じファミリー内で段落 `0x001c` レコードと構造的に異なるかどうかは、複数列サンプルでの検証が未実施。
- `0x000e` 行区切りは追加ペイロードなしの 1 ワード制御コードであることを確認済み。`0x0010 w4=0x008f` レコードが 4 ワードサブエントリ `[tag, v1, v2, v3]` を通じて行ごとの列レイアウトをエンコードしており、支配的なケースでは `n_sub_entries = n_cells − 1` が成立する。サブエントリの `v3` 値はセルスパンと相関するが正確な式は未解読。サブエントリタグ `0x23`/`0x2b`/`0x1b`/`0x24`–`0x27` の意味と `w8` の役割は未解読。クロスレコード分析で `w8` が `0x0001`（22 件）または `0x0003`（107 件）の 2 値のみを取ることが判明。特筆すべき点として、`n_cells=12`（本サンプル最大幅の行）の全 14 件は `w8=0x0001`、一方 `n_cells=4` 行は大多数が `w8=0x0003`（72×）で例外 5 件のみ `w8=0x0001`。`w8` の値は `0x23` タグ件数とは一致せず、明確な規則は見つかっていない；行種別フラグ（ヘッダー行 vs データ行など）の可能性がある。
- クラス `0x0010` の可変長レコードは `w4/w5` に共通サブヘッダーシグネチャ `0x0026 0x0005` を共有しているようだ（`sample-academic.jtd` len=20 と `sample-outline/sample-draft/sample-reference` len=17 で確認）。`sample-reference.jtd` の len=17 レコード（計 142 件、`w4=0x0026 w5=0x0005`）詳細分析では、`w6`/`w7`/`w8`/`w9`/`w10` の組み合わせによる 9 種類のペイロードパターンが識別される。`w6=1`（102 件）のとき `w7=0x01ec=492`、`w8=w10=0x01cc=460` が固定で、ハンギングインデントグループとみられる（1/10mm 単位なら 49.2mm/46mm）。`w6=0`（40 件）のとき `w7=0/2/4`、`w8` はほぼ 0 で通常段落（インデントなし）と一致。`sample-academic.jtd` len=20 ではインデント付き続き行に `w10=0x0141=321` が現れ、約 32mm のハンギングインデントと整合する。`sample-table.jtd` の `w4=0x002e` バリアント（18 件、len=13）は完全に固定（`w5=w6=1`, `w7=0xffff`, `w8=0`）で均一な単列レイアウトを示す。単位スケールとフィールド役割は未解読。11 件のテストサンプル全体での `w4=0x0026 len=17` レコード 246 件のクロスサンプル分析では、`(w8, w10)` の大きさによる構造的な分割が新たに判明した：`(w8=w10=0x01cc=460)` は `sample-reference` にのみ出現し（常に `w6=1` を伴う）、`(w8=w10=1)` または `(w8=w10=0/2)` は `sample-outline` と `sample-draft` に専属。`sample-draft` の 21 件では `w7` が 0/1/2/6/8 をとる。`sample-draft` で `w7` と後続テキストを照合した結果、`w7` は視覚的インデント深さと単純対応しない：`w7=0` は左揃えの法令見出し・条・号見出しに出現；`w7=1` は条文本文継続；`w7=2` は号リスト・別表（先頭全角スペースあり/なし混在）；`w7=6` は前文本文；`w7=8` は附則見出し。このパターンは `w7` が視覚的インデント数ではなく段落スタイル ID をエンコードしているという仮説と整合する。`w7` 値と一太郎の段落スタイル名の対応は未証明。`(w8, w10)` は文書種別識別子か短テキストではスタイル ID・参照条文長文では物理座標をエンコードしている可能性がある。物理単位スケールは未証明。

  14 件のテストサンプル全体でのスイープ（`paragraph-style-records`）により、`w7` 値のセットが確認され追加値も判明した。`w4=0x0026 len=17` レコードにおける観測済み `w7` 値とテキストコンテキスト（decoded:false）：

  | `w7` | `w8`/`w10` | テキストコンテキスト | スタイル役割候補 |
  | ---- | ---------- | -------------------- | ---------------- |
  | 0    | 0x0001     | 条・条文本文・左揃え見出し（sample-outline/sample-draft） | 標準本文段落 |
  | 0    | 0x0002     | 短い左揃え本文（sample-reference-b） | 標準本文（混在ファミリー） |
  | 0    | 0x0000     | 本文段落（sample-draft-b） | 標準本文（w8=0 ファミリー） |
  | 0    | 0x01cc     | ハンギングインデント先頭行または目次見出し（sample-reference） | 標準本文 / 目次見出し |
  | 1    | 0x0001     | 条文本文継続行（sample-draft） | 本文継続行 |
  | 2    | 0x0001     | 号リスト・別表（sample-draft） | 号 / インデント段落 |
  | 2    | 0x0000     | 条本文（sample-draft-b） | 号 / インデント段落（w8=0） |
  | 2    | 0x0002     | 条本文（sample-reference-b / sample-reference-c） | 号 / インデント段落（混在） |
  | 3    | 0x0000     | 附則小見出し「（施行期日）」 | 附則内小見出し |
  | 4    | 0x0000     | 別表・深いインデント（sample-draft-b、sample-reference） | 深インデント / 別表 |
  | 4    | 0x0002     | 法令節見出し（sample-reference-b） | 法令節見出し（混在） |
  | 6    | 0x0001/0x0000 | 法令タイトル・前文（全 sample-outline/sample-draft サンプル） | タイトル / 前文スタイル |
  | 8    | 0x0001     | 附則見出し「附　則」（sample-draft） | 附則見出し |
  | 10   | 0x0000     | 理由見出し「理　由」（sample-draft-b） | 理由見出し |
  | 492 (0x01ec) | 0x01cc | ハンギングインデント本文（sample-reference, w6=1） | ハンギングインデント本文 |

  全 `sample-outline` サンプルで `w7=6` が法令タイトルまたは前文に出現し、ファイル内で唯一の非ゼロ値（レコード ≤ 3 件）となる。全 `sample-draft` サンプルで `w7=6` は法令タイトル/前文見出しに出現し、`w7=8` は附則、`w7=0` は通常条文本文全体に出現する。全 `sample-outline`/`sample-draft` サンプルで開頭タイトル行に `w7=6` が一貫して出現することは、`w7` が名前付き段落スタイル識別子をエンコードするという解釈を強化する。各 `w7` 値に対応する一太郎スタイル名は未証明（decoded:false）。

## 使用サンプル

| サンプル | レコード数 | 観測ファミリー |
| --- | ---: | --- |
| `sample-academic.jtd` | 19 | `0x0010 len=20` のみ |
| `sample-table.jtd` | 1039 | `0x0010`（全 len）、`0x0030 len=12`、`0x0000 len=12/21`、`0x0020 len=12` |
| `sample-draft.jtd` | 33 | `0x0010`、`0x0030 len=12` |
| `sample-reference.jtd` | 504 | `0x0010`、`0x0030 len=12`、`0x0000 len=12` |
