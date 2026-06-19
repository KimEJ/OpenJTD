# TODO

## M1: Container Explorer

Goal: 最初の実行可能マイルストーンとして `rjtd streams <file.jtd>` を実装する。

Status: CFB entry inventory は実装済み。壊れた FAT ファイルに対する rhwp 風の lenient fallback も含む。

これを最初に行う理由:

- rjtd は rhwp の階層的な解析順序に従う必要がある。
- rhwp は header、record、body text、model、renderer、exporter より前に CFB container 層から binary parsing を始める。
- text extraction を試す前に、JTD 構造を観察し文書化する必要がある。
- exporter は raw file data を直接読んではならず、以降の作業は安定した下位層に依存する。

Immediate tasks:

- [x] ローカル `.jtd`、`.jtt`、`.jttc` サンプルを `rjtd-testdata/local-samples/` に置く。
- [x] 実装前にテストを書く。
- [x] 最小限の CFB container reader を `rjtd-core` に追加する。
- [x] 正規化された path、byte size、entry kind を持つ stream/storage entry を公開する。
- [x] malformed FAT samples のために rhwp 風の lenient CFB fallback を追加する。
- [x] `rjtd-cli` に `rjtd streams <file.jtd>` を実装する。
- [x] rhwp 風の軽量 inventory command として `rjtd info <file>` を実装する。
- [x] 初期観察結果を `openjtd-spec` に記録する。

Acceptance criteria:

- [x] Unit tests が小さな synthetic CFB file を作り、stream listing が動くことを証明する。
- [x] CLI integration tests が synthetic CFB file に対して `rjtd streams` を実行する。
- [x] ローカル sample files を `cargo run -p rjtd-cli -- streams ../rjtd-testdata/local-samples/<name>.jtd` で手動確認できる。
- [x] `rjtd/` から `cargo fmt --all --check`、`cargo check --workspace`、`cargo test --workspace`、`cargo clippy --workspace --all-targets -- -D warnings` が通る。

## M2: Text Extraction

Goal: M1 が関連 stream layout を明らかにした後、`rjtd cat <file.jtd>` を実装する。

M1 が local samples の stream observations を文書化するまでは M2 を開始しない。

Status: 観察済み text run と display inline segment のための structured `ParsedDocumentText` token layer を実装済み。観察済み `.jttc` samples は `/JSCompDocument` `JustCompressedDocument` wrapper 経由で開く。現在 `/DocumentText` を持たない samples については embedded `SsmgV.01` fragments を復元する。

Priority prework:

- [x] historical Ichitaro Document Filter `.oxt` をダウンロードする。
- [x] `.oxt` を展開し file tree を記録する。
- [x] Java、native code、OpenOffice registry files の有無を確認する。
- [x] filter/type registration metadata を記録する。
- [x] DLL を逆コンパイルせず、stream と conversion string を検索する。
- [x] filter で確認された stream names に照らして local samples の `/DocumentText` bytes を調べる。
- [x] 必要なら read-only `rjtd dump-stream <file.jtd> <path>` helper または test utility を追加する。
- [x] `DocumentText` text-run marker を使った初期 `rjtd cat <file.jtd>` を実装する。
- [x] ruby base text と template placeholder を含む、`0x001D ... 0x001E` で囲まれた common display inline segments を復元する。
- [x] plain `cat` output では既知の phonetic annotation と template instruction inline segments をスキップする。
- [x] `/DocumentText` がない `.jttc` samples が `/JSCompDocument` を `JustCompressedDocument` として保存していることを検出する。
- [x] LHA dependency を追加せず、観察済み `.jttc` `/JSCompDocument` `-lh5-` payloads を decode する。
- [x] invalid または unsupported compressed payloads を明確に報告する。
- [x] malformed FAT fallback 後に local sample inventory が開けることを確認する。
- [x] `/DocumentText` がない場合に embedded `SsmgV.01`/`TextV.01` fragments を復元する。
- [x] 現在の local samples 61 件すべてで `rjtd cat` が成功することを確認する。
- [x] structured `ParsedDocumentText` output を調べる `rjtd text-tokens <file>` を追加する。
- [x] `/DocumentText` control boundaries を neighboring map entries と nearest control distances 付きで調べる `rjtd text-control-context <file> [control-code]` を追加する。
- [x] plain string extractor path を structured `ParsedDocumentText` parser layer に置き換える。
- [x] スキップした inline text segments を structured tokens と document-model unknown objects として保存する。
- [x] token/stream inspection pipelines のために CLI stdout writes が broken pipes に耐えるようにする。
- [x] 初期 `/DocumentTextPositionTables` `MarkV.01` diagnostics のために `rjtd text-positions <file>` を追加する。
- [x] `/DocumentText` tokens を byte range と UTF-16 unit range に対応付ける `rjtd text-map <file>` を追加する。
- [x] `MarkV.01` offsets を byte、unit、provisional `unit + 29` token contexts と比較する `rjtd text-position-context <file>` を追加する。
- [x] MarkV.01 UTF-16 unit delta candidates 0 through 64 を採点する `rjtd text-position-delta-scan <file>` を追加する。
- [x] `MarkV.01` と first entry の間の raw six bytes と raw `(id, offset)` entries を表示する `rjtd text-position-mark-header <file>` を追加する。
- [x] MarkV.01 header を `/DocumentText`、`/LineMark`、`/PageMark`、`/PaperMark` metrics と相関させる `rjtd text-position-mark-summary <file>` を追加する。
- [x] MarkV.01 header と entry offsets を `/LineMark` word contexts と nearest tag rows と比較する `rjtd text-position-line-context <file>` を追加する。
- [x] `/DocumentTextPositionTables` observations を `openjtd-spec` に記録する。
- [x] `MarkV.01` offsets は raw byte や extracted-character positions より UTF-16 unit/internal coordinates に近いという現在の仮説を記録する。
- [x] parsed `MarkV.01` entries を持つ現在の 5 samples で `text-position-context` output を sweep し、初期 id classification を記録する。
- [x] 現在の MarkV.01 samples 5 件で `text-position-delta-scan` を sweep する。40 entries total; `delta 9`、`delta 29`、`delta 30` は visible text hits 31 で同率、`delta 9` は `delta 29` より unit hits が多い。
- [x] 現在の MarkV.01 samples 5 件で `text-position-mark-header` を sweep する。marker offset は常に 30、header prefix は `00000000`、最後の big-endian `u16` は `0x0603`、`0x0610`、`0x061c` のいずれかなので、この header は constant 29-unit adjustment を直接 encode していない。
- [x] 現在の MarkV.01 samples 5 件で `text-position-mark-summary` を sweep する。`0x0603` は異なる page/paper counts で現れ、`0x061c` は `/LineMark`/`/PageMark`/`/PaperMark` がない samples で現れるため、直接的な document-length または page-count の意味は未証明。
- [x] 現在の local samples 61 件で `text-position-line-context` を sweep する。`46.jtd`、`a5.jtd`、`b6.jtd` だけが readable `/LineMark` と parsed `MarkV.01` の両方を持つ。MarkV.01 entry offsets 24 件はすべて `/LineMark` word range 外だが、MarkV.01 header の final `u16` は `/LineMark` 内に入る。
- [x] 代表的な MarkV samples で four-byte MarkV.01 header values `00000603`、`00000610`、`0000061c` を検索し、観察済み layout/style streams ではなく `/DocumentTextPositionTables` にだけ現れることを確認する。
- [x] 観察済み non-Mark `TCntV.01` numeric tables のために `rjtd text-position-counts <file>` を追加する。
- [x] 11 件の non-Mark position-table samples を 10 件の `TCntV.01` tables と、inventory size は non-zero だが empty/unreadable position-table payload 1 件に分類する。
- [x] `TCntV.01` の first two fields を `/DocumentText` byte と UTF-16 unit token ranges に対して比較する `rjtd text-position-count-context <file>` を追加する。
- [x] 現在の `TCntV.01` samples 10 件で `text-position-count-context` を sweep し、coordinate behavior が mixed であることを記録する。
- [x] tail `t1/t2` fields を `/DocumentText` byte と UTF-16 unit token ranges に対して比較する `rjtd text-position-count-tail-context <file>` を追加する。
- [x] 現在の local samples で `text-position-count-tail-context` を sweep する。readable `TCntV.01` files 10 件、89 rows、any byte hit 21、both byte hit 5、any unit hit 49、both unit hit 28、both unit text hit 26。`t1/t2` には UTF-16 unit coordinates がより強いが未完成な signal として見える。
- [x] tail `t1/t2` endpoints に positive `0..64` UTF-16 unit deltas を scan する `rjtd text-position-count-tail-delta-scan <file>` を追加する。
- [x] 現在の local samples で `text-position-count-tail-delta-scan` を sweep する。delta 29 と 30 が unit endpoint hits 124 で同率首位、delta 29 は text endpoint hits と both-unit rows で 30 より強い。一方 text endpoint hits は delta 53 で peak するため、single offset は未証明。
- [x] `(family,t0,t3,t4,t7)` pattern ごとに tail delta scores をまとめる `rjtd text-position-count-tail-delta-groups <file>` を追加する。
- [x] 現在の local samples で `text-position-count-tail-delta-groups` を sweep する。28-row `be0/0x0101/0x0100/0x0001/0x0001` group は unit/text とも delta 29 を好む。16-row shifted group は unit delta 31、text delta 30 を好む。28-row `be0/0x0202/0x0100/0x0000/0x0001` group は多くの best deltas に分散し、single global adjustment は考えにくい。
- [x] per-row best unit/text deltas、chosen spans、tail spans、document byte/unit lengths を出す `rjtd text-position-count-tail-row-deltas <file>` を追加する。
- [x] 現在の local samples で `text-position-count-tail-row-deltas` を sweep する。主要 `be0/0x0202/0x0100/0x0000/0x0001` group は 8 files 28 rows、chosen spans 398..1212、tail spans 46..72、document unit lengths 65889..146657、row-level best unit deltas 17 種を持つ。これは file-level/global correction ではなく row-local structure に見える。
- [x] chosen start/end byte/unit contexts と best-delta tail contexts を同じ row に置く `rjtd text-position-count-tail-row-context <file>` を追加する。
- [x] 現在の local samples で `text-position-count-tail-row-context` を sweep する。major 28-row `be0/0x0202/0x0100/0x0000/0x0001` group では chosen start/end が byte-context body hits または boundaries に出ることが多く、unit-context は mostly `between`、best-delta tail `t2` は 27/28 rows で text に着地する。chosen range と tail fields は duplicate text coordinates ではなく異なる role と見るべきである。
- [x] chosen `TCntV.01` range が overlap する `/DocumentText` entries を byte/UTF-16 unit intervals として要約する `rjtd text-position-count-range-preview <file>` を追加する。
- [x] 現在の local samples で `text-position-count-range-preview` を sweep する。61 checked、10 readable `TCntV.01` files、89 rows。`t0=0x0202` rows 36 件のうち 31/36 が chosen byte ranges で text と overlap する。major group は byte 25/28、unit 21/28 が text と overlap する。
- [x] chosen range edge alignment、first/last/previous/next `/DocumentText` entries、overlapped control-code counts を出す `rjtd text-position-count-range-boundaries <file>` を追加する。
- [x] 現在の local samples で `text-position-count-range-boundaries` を sweep する。major group の chosen byte ranges は map entries 535 件と overlap し、513 件が fully contained、22 件が partial。25/28 rows が controls を含み、`0x001c` が 281 回、`0x000e` が 38 回で優勢。`/DocumentText` control-delimited range structure が次の対象。
- [x] 現在の local samples で `text-control-context` を sweep する。61 checked、0 errors、60 files with controls。`0x001c` は 60 files で 51,971 回、`0x000e` は 41 files で 6,621 回。`0x001c` は text/text、text/control、control/text、control/control をよく分ける。`0x000e` は control/control、text/control、text/skipped-inline に多い。
- [x] duplicate `TCntV.01` ranges を group し raw-tail variants を表示する `rjtd text-position-count-clusters <file>` を追加する。
- [x] `be32@0/4` と shifted `be32@1/5` offset candidates を比較する `rjtd text-position-count-candidates <file>` を追加する。
- [x] 現在の `TCntV.01` samples のうち 9 件は `be32@0/4` に合い、`ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` は 32 件の `be32@0/4` plausible entries と 18 件の shifted `be32@1/5` plausible entries に分かれることを確認する。
- [x] `TCntV.01` entries を `be0` または `be1-shifted` に分類し、candidate offsets と raw tail を保存する `rjtd text-position-count-family <file>` を追加する。
- [x] 現在の `TCntV.01` samples で `text-position-count-family` を sweep する。10 files、89 records total、71 `be0`、18 `be1-shifted`。shifted entries はすべて `iwata_file` entries 32-49。
- [x] 各 `TCntV.01` record を chosen range と tail `u16be` fields に展開する `rjtd text-position-count-fields <file>` を追加する。
- [x] 現在の `TCntV.01` samples で `text-position-count-fields` を sweep する。`be0` records は 10 tail `u16be` fields plus extra byte `00`、`be1-shifted` records は exactly 10 tail `u16be` fields and no extra byte。
- [x] current tail-field invariants を記録する。shifted records は fixed `t3=0x0100`、`t4=0x0001`、`t5=0x0000`、`t6=0x0000`、`t7=0x0001`、`t8=0x0000`、`t9=0x0000`。`be0` records は複数の mostly fixed fields を共有するが variation が多い。
- [x] chosen `TCntV.01` range span と tail `t1..t2` span を signed deltas 付きで比較する `rjtd text-position-count-field-deltas <file>` を追加する。
- [x] 現在の local samples で `text-position-count-field-deltas` を sweep する。10 readable `TCntV.01` files、89 rows、すべて `t2 >= t1` だが、`t2 - t1` が chosen range span と等しい row はない。relation counts は `be0` `gt` 27 / `lt` 44、`be1-shifted` `gt` 15 / `lt` 3。
- [x] chosen `TCntV.01` ranges を `/LineMark` word/byte offsets、parsed `/PageMark`、`/PaperMark` row/byte offsets と比較する `rjtd text-position-count-layout-context <file>` を追加する。
- [x] 現在の local samples で `text-position-count-layout-context` を sweep する。10 readable `TCntV.01` files で 89 rows、chosen start/end values は word/row/byte interpretation のどれでも direct `/LineMark`、`/PageMark`、`/PaperMark` ranges 外。
- [x] CFB regular/mini stream placement を調べる `rjtd stream-meta <file> <stream-path>` を追加する。
- [x] empty `/DocumentTextPositionTables` payload 1 件は mini-sector start `224` で、observed 7680-byte mini stream の範囲外を指すことを確認する。
- [x] 初期 `/LineMark`、`/PageMark`、`/PaperMark` observations を `openjtd-spec` に記録する。
- [x] generic `rjtd stream-words <file> <stream-path>` と `rjtd stream-word-frequencies <file> <stream-path>` diagnostics を追加する。
- [x] `/LineMark` raw words を `/DocumentText` raw words と比較し、`0x1000`、`0x1001`、`0x1002` は LineMark-specific tags らしいと記録する。
- [x] `/LineMark` `0x1000`/`0x1001`/`0x1002` tag positions を surrounding word context 付きで出す `rjtd line-mark-tags <file>` を追加する。
- [x] 61 current local samples で `line-mark-tags` を sweep する。5 files が tags を含み、6 files は `/LineMark` なし、50 readable `/LineMark` streams は tags なし。tag totals は `0x1000` 915、`0x1001` 67、`0x1002` 554。
- [x] 各 `/LineMark` tag の直後の word は unique tag-family discriminator ではないことを確認する。high-frequency next words は `0x1000`、`0x1001`、`0x1002` 間で強く重なる。
- [x] `/LineMark` tag offsets と immediate next words を `/DocumentText` token map と比較する `rjtd line-mark-text-context <file>` を追加する。
- [x] 61 current local samples で `line-mark-text-context` を sweep する。55 files successful、6 lack `/LineMark`、known tag rows 1536 件を報告。direct LineMark byte/unit offsets は 587 rows で mapped `DocumentText` entries に hit し、tag-next words は 1511 rows で raw `/DocumentText` に現れる。
- [x] direct `/LineMark` word/byte offsets は proven `DocumentText` coordinates ではないが、tag-next words は通常どこかの raw `/DocumentText` に出ると記録する。
- [x] u32-oriented streams 用に generic `rjtd stream-dwords <file> <stream-path>` と `rjtd stream-dword-frequencies <file> <stream-path>` diagnostics を追加する。
- [x] 観察済み `/PaperMark` row shape が 12-byte header plus 8-byte `(index, flags)` rows であることを current large layout samples で証明する。
- [x] `/PaperMark` を document model に入れず、parser-backed `rjtd paper-marks <file>` diagnostics を追加する。
- [x] local samples で `paper-marks` を sweep し、55 件の `/PaperMark` streams のうち 52 件が observed row shape で parse することを記録する。
- [x] observed 84-byte `/PageMark` row family の parser-backed `rjtd page-marks <file>` diagnostics を追加し、各 row を raw bytes として保存する。
- [x] local samples で `page-marks` を sweep し、55 件の `/PageMark` streams のうち 20 件が observed 84-byte row family に合うことを記録する。
- [x] `/PageMark` stream length、declared CFB size、header values、row-shape candidates を分類する `rjtd page-mark-shape <file>` を追加する。
- [x] 初期 `/PageMark` reject families を特定する: count-plus-one variable rows、count-plus-one with 2-byte tail/trimming、declared-size mismatch regular streams、non-PageMark-looking payloads。
- [x] count-plus-one variable と count-plus-one-trim2 `/PageMark` families を raw-preserved row families として parser に昇格する。
- [x] count-plus-one family promotion 後、parser-backed `page-marks` が current `/PageMark` streams 55 件のうち 43 件を開くことを確認する。
- [x] count-variable `/PageMark` family を raw-preserved row family として parser に昇格する。
- [x] parser-backed `page-marks` が current `/PageMark` streams 55 件のうち 45 件を開くことを確認する。
- [x] fixed84-tail `/PageMark` family を trailing bytes preserved で parser に昇格する。
- [x] parser-backed `page-marks` が current `/PageMark` streams 55 件のうち 52 件を開くことを確認する。
- [x] suspicious raw streams で ASCII/UTF-16 string candidates を検出する `rjtd stream-text-probe <file> <stream-path>` を追加する。
- [x] 残り 3 件の unsupported `/PageMark` payloads は numeric PageMark rows ではなく stream/object names または legacy class/control metadata を含むことを確認する。
- [x] `/PaperMark` stream length、declared CFB size、header values、fixed-row candidates を分類する `rjtd paper-mark-shape <file>` を追加する。
- [x] `paper-mark-shape` が current `/PaperMark` streams 55 件すべてを開き、3 件の unsupported payloads を `non-paper-header` として分離することを確認する。
- [x] suspicious stream entries の FAT/miniFAT sector chains を調べる `rjtd stream-chain <file> <stream-path>` を追加する。
- [x] 残り 3 件の unsupported `/PageMark` と `/PaperMark` entries は complete miniFAT chains を持つが、payload bytes は layout rows ではなく CFB directory-entry fragments、OLE/ActiveX control metadata、または unrelated stream names と decode されることを確認する。
- [x] FAT、directory、mini FAT、root mini-stream sector chains を調べる `rjtd cfb-map <file>` を追加する。
- [x] `kaisya_annai` と `shanai_lan` の CFB directory-entry fragments を説明する。root mini-stream chains が CFB directory chains と重なるため、`/PageMark` と `/PaperMark` が structurally complete miniFAT chains 経由で directory-entry bytes を読む。
- [x] 同じ CFB file 内で exact duplicate stream payloads を探す `rjtd stream-find <file> <stream-path>` を追加する。
- [x] `kazoku_ryoko` `/PageMark` を `/EmbedItems/Embedding 1/JSFart2Contents` offset 1664 の exact slice まで追跡する。
- [x] raw CFB directory ids、sibling links、start sectors、resolved paths を調べる `rjtd cfb-dir <file>` を追加する。
- [x] `kazoku_ryoko` `/PaperMark` を root-level object/control stream sequence around `/EmbedFrame` and `/Figure` に追跡する。`SO` marker は `/Figure` と `\x01CompObj` にも現れ、最初の nonzero fields は `/EmbedItems/Embedding 2/JSFart2Contents` offset 192 と一致する。
- [x] arbitrary byte markers を all readable streams across CFB で検索する `rjtd stream-find-bytes <file> <hex>` を追加する。
- [x] `kazoku_ryoko` `/PaperMark` object/control evidence を `stream-find-bytes` で再現する。`534f0000` は `/PaperMark`、`/Figure`、`\x01CompObj` に現れ、20-byte coordinate-like suffix は `/PaperMark` と `/EmbedItems/Embedding 2/JSFart2Contents` に現れる。
- [x] all readable streams で `SO\0\0` object/control marker を scan し preserved little-endian fields と raw bytes を出す `rjtd so-records <file>` を追加する。
- [x] local samples で `so-records` を sweep する。4/61 samples が SO records を持ち、24 records total、`kazoku_ryoko` だけが `/PaperMark` 経由で SO record を expose する。
- [x] preserved raw bytes で SO records を group し repeated locations を報告する `rjtd so-record-clusters <file>` を追加する。
- [x] `JSFart2Contents` SO records が singleton geometry-like records と repeated default/control clusters に分かれること、`kazoku_ryoko` `/PaperMark` が older `kazoku_ryoko` `JSFart2Contents` sample の geometry-like cluster と一致することを確認する。
- [x] SO records を little-endian 32-bit fields plus signed and low/high 16-bit views に展開する `rjtd so-record-fields <file>` を追加する。
- [x] 現在の SO records は 9 little-endian dwords として扱うのがよいと確認する。field 0 は marker `0x00004f53`、repeated default/control records は `0x00000100` や `0x00000064` のような小さな constants、singleton geometry-like records は fields 1-4 に coordinate-like values を持つ。
- [x] SO records を `geometry-like`、`default-control`、packed subfamilies、`truncated`、`unknown` に分類する `rjtd so-record-geometry <file>` を追加する。
- [x] SO payload dwords を low/high 16-bit unsigned and signed halves として出す `rjtd so-record-halves <file>` を追加する。
- [x] local samples で `so-record-geometry` を sweep する。61 checked、0 failures、4 files with SO records、24 records total (`geometry-like` 9、`default-control` 8、`packed-jseq3-like` 4、`packed-ffff-preamble` 2、`truncated` 1)。
- [x] 以前の generic `packed` SO bucket を分割する。`packed-jseq3-like` は `JSEQ3Contents` records だけに現れ、`packed-ffff-preamble` は repeated geometry-like records 前の `JSFart2Contents` offset-324 preamble だけに現れる。
- [x] readable CFB streams の object/image/shape/table path evidence、`SO\0\0` marker、binary image signature、SVG text signature、payload prefix、`decoded=false` を classify する `rjtd object-stream-candidates <file>` を追加する。
- [x] current local samples で `object-stream-candidates` を sweep する。61 checked、candidates を持つ files 43、2,253 readable streams 中 candidate rows 933、object-path evidence files 27、shape-path evidence files 42、image-signature evidence files 17、SO-marker evidence files 4、SVG signature 0、table-path evidence 0、unreadable stream 0。
- [x] object/image stream inventory を `openjtd-spec` RFC 0008 に記録する。Embedded JPEG recovery は `/EmbedItems/Embedding */Contents` から始められ、`hyo` table sample は named table stream ではなく `/DocumentText` control/layout decoding が必要である可能性が高い。
- [x] non-text PDF rendering へ接続する前に、object/image/shape stream candidate evidence を model/export/app-core decoded-false `objectStreamCandidates` に promote する。
- [x] model-preserved `objectStreamCandidates` JSON export を sweep する。61 checked、0 failures、positive files 43、candidates total 933、image-signature files 17、SO-marker files 4、table-path files 0、SVG-signature files 0。
- [x] image-signature object candidates から complete embedded image payload spans を extract し、object geometry はまだ decoded せず decoded-false `objectStreamCandidates.imagePayloads` に bytes を保存する。
- [x] model-preserved `imagePayloads` JSON export を sweep する。61 checked、0 failures、payload files 12、strict complete payloads 67、dimensioned payloads 35、629,024 bytes、JPEG 35 rows/5 files、GIF89a 31 rows/9 files、GIF87a 1 row/1 file。
- [x] decoded-false image payload object envelopes を保存する。header/trailer slices と conservative `le32` declared payload-length candidates を含む。
- [x] image payload envelopes を sweep する。61 checked、0 failures、envelopes 67、`le32` declared length matches 20、現在はすべて `Embedding */Contents` rows に限られる。
- [x] decoded-false image envelope header field candidates を保存する。先頭 prefix `u16/u32` values と `Embedding */Contents` headers の source path candidates を含む。
- [x] image envelope header fields を sweep する。61 checked、0 failures、source path candidates 66、すべて `Contents` rows (`jpg` 34、`gif` 32)。first prefix word `9` は payload rows 59、`4` は 6 に現れる。
- [x] object stream candidates に decoded-false path-derived `ownershipCandidate` evidence を保存する。`Embedding N`、stream role、figure/frame/layout stream families を含む。
- [x] ownership candidates を sweep する。61 checked、0 failures、candidates 933、ownership 付き 474、strict image payload rows 67 はすべて ownership candidate に covered (`contents` 67)。
- [x] embedded image candidates の `Embedding N` byte pattern を `FigureData`、`/Figure`、`/Frame`、`/LayoutBox`、`/PageMark`、`/PaperMark` streams で検索し、decoded-false `ownershipReferences` evidence として保存する。
- [x] ownership references を sweep する。61 checked、0 failures、references 付き files 12、references 付き embedded image candidates 52、reference rows 604、total byte matches 9,949、strict image payload rows 67 のうち 67 が cross-stream reference candidates に covered。
- [x] `rjtd object-ownership-references <file>` を追加する。model-owned ownership reference match の source stream、target stream、encoding、offset、total match count、alignment、local context hex、match offset の le/be 16/32-bit values を出力する。
- [x] `object-ownership-references` を sweep する。61 checked、0 failures、rows 付き files 12、reported preview offsets 3,167。target families は `figure-data` 1,010、`frame` 897、`layout-box` 528、`page-mark` 498、`paper-mark` 151、`figure` 83。
- [x] reference-context alignment evidence を記録する。reported preview offsets は mod2 `0` 1,401 / `1` 1,872、mod4 `0` 728 / `1` 855 / `2` 673 / `3` 1,017 に分かれる。`u16-be`/`u16-le` offsets は embedding index を match offset の be/le16 value として直接 expose し、`u32-le` は low 16 bits に expose する。
- [x] `rjtd object-ownership-reference-fields <file>` を追加する。ownership reference offsets を candidate record strides に投影し、target、encoding、stride、field offset、row indexes、source count、embedding indexes、cross-row matches を summarize する。
- [x] `object-ownership-reference-fields` を sweep する。61 checked、0 failures、field groups 付き files 12、projected field groups 33,492。すべての reported offset を 20 個の candidate strides に投影する diagnostic surface であり、decoded geometry ではない。
- [x] cross-row-free かつ stride >= 12 の強い候補を記録する。`frame/u16-le/12/5` は weighted matches 102、`frame/u16-be/12/7` は 89、`frame/u16-be/20/15` は 70。
- [x] `rjtd object-frame-reference-records <file>` を追加する。最も強い `/Frame` reference projections を candidate row bytes として展開し、row hex、BE/LE 16-bit fields、BE/LE 32-bit fields を出力する。
- [x] `object-frame-reference-records` を sweep する。61 checked、0 failures、candidate records 付き files 12、expanded rows 261。Candidate counts は `u16-le/12/5` 102、`u16-be/12/7` 89、`u16-be/20/15` 70。
- [x] dominant expanded `/Frame` row families を記録する。`00010000000N000000020001` style の 12-byte rows と `00000000010200380000000N` rows が繰り返されるが、まだ decoded placement geometry ではなく row-family evidence である。
- [x] `rjtd object-frame-record-families <file>` を追加する。expanded `/Frame` candidate rows を decoded-false diagnostic family として group する。
- [x] `object-frame-record-families` を sweep する。61 checked、0 failures、family 付き files 12、records 261。Family counts は `frame-index-tail-coordinate-row12` 65、`frame-index-tail-window20` 65、`frame-index-mixed-row12` 61、`frame-index-flag-row12` 41、`frame-index-tail-zero-row12` 22、`frame-index-mixed-window20` 5、`frame-index-tail-mixed-row12` 2。
- [x] `rjtd object-frame-row-links <file>` を追加する。20-byte `/Frame` window が matching 12-byte frame row を suffix として含むか検証する。
- [x] `object-frame-row-links` を sweep する。61 checked、0 failures、20-byte rows 付き files 9、row20 windows 70、same-source suffix row に link されたもの 65、unlinked 5。Linked rows はすべて `frame-index-tail-window20 -> frame-index-tail-coordinate-row12` であり、20-byte context window より `u16-be/12/7` がより強い authoritative-row candidate である。
- [x] decoded-false `/Frame` reference rows と suffix links を model/export `objectStreamCandidates[].frameReferenceRows` に promote する。future image placement implementation は model-first のまま維持する。
- [x] model-preserved `frameReferenceRows` JSON export を sweep する。61 checked、0 failures、positive files 12、rows 261、suffix links 65。Family counts は CLI sweep と完全に一致する。
- [x] `rjtd object-image-frame-candidates <file>` を追加する。各 image payload source を model-owned `/Frame` row evidence、payload kind/dimensions、row family、row20 suffix link、coordinate-looking row12 pair、best coordinate/payload aspect delta で summarize する。
- [x] `object-image-frame-candidates` を sweep する。61 checked、0 failures、image payload source 付き files 12、image sources 52、frame-linked sources 52、missing-frame sources 0、frame rows 261、dimensioned payloads 35、aspect candidates 付き sources 13。Preferred diagnostic buckets は `row12-tail-coordinate` 25、`row12-tail-zero` 7、`u16-le-row12` 19、`none` 1。
- [x] `row12-tail-coordinate` は強い placement-analysis candidate だが、まだ rendering promotion には不十分であると記録する。best coordinate/payload aspect delta <= 250 permille の sources は `natsu.jtd` の 2 件のみであり、aspect evidence はまだ sparse である。
- [x] `rjtd object-fdm-index <file>` を追加し、`/FigureData/*/FDMIndex` rows を sibling `FDMVector` segments と照合する。vector offset、kind field、bbox-like fields、segment prefix、image signature hits を decoded-false evidence として保存する。
- [x] `object-fdm-index` を sweep する。61 checked、0 failures、indexes 付き files 31、index streams 39、parsed rows 417、image 付き rows 6、image hits 13、missing sibling vectors 2。これは FDMIndex/FDMVector が `Embedding N` `/Frame` rows とは別の image-placement evidence path であることを示す。
- [x] `rjtd object-fdm-index-shape <file>` を追加し、exact 22-byte FDMIndex tables、declared-count prefix tables と trailing auxiliary payload、mixed declared rows、unknown-header streams、missing vectors を分離する。
- [x] `object-fdm-index-shape` を sweep する。61 checked、0 failures、indexes 39、`fdm-index-v1` headers 35、unknown headers 4、plausible declared counts 34、raw stream rows 417、raw invalid offsets 252、declared-prefix rows 147、declared-prefix invalid offsets 43、declared-prefix image hits 13。Shape counts は `row22-count-prefix` 17、`row22-exact` 14、`row22-mixed-declared` 3、`unknown-header` 3、`missing-vector` 2。
- [x] `rjtd object-fdm-index-rows <file>` を追加し、FDMIndex analysis 用に row scope (`declared`, `post-declared`, `raw`)、row role (`vector-segment`, `coordinate-like-invalid`, `invalid-vector-offset`)、BE16/i16 field views、row bytes、segment image hits を出力する。
- [x] `object-fdm-index-rows` を sweep する。61 checked、0 failures、indexes 付き files 31、indexes 39、rows 417、declared rows 147、post-declared rows 253、raw rows 17、valid vector rows 165、invalid rows 252、image hits 13、missing vectors 2。Role counts は `vector-segment` 165、`coordinate-like-invalid` 231、`invalid-vector-offset` 21 で、declared invalid rows 43 はすべて `coordinate-like-invalid` である。
- [x] decoded-false `FDMIndex` row evidence を、対応する `FDMVector` candidate の model/export/app-core JSON `objectStreamCandidates[].fdmIndexEntries` に promote する。ただし auxiliary payload bytes を false rows として expose しないよう、有効な `fdm-index-v1` declared-count prefix rows に制限する。
- [x] model-preserved `fdmIndexEntries` JSON export を sweep する。61 checked、0 failures、entries 付き files 24、entries 付き candidates 30、rows 147、image-linked rows 付き files 3、image-linked rows 6、image hits 13、valid vector offsets 104、invalid/out-of-range vector offsets 43。
- [x] `fdmIndexEntries` は現在観測されている image-bearing FDMVector segments をすべて識別しつつ、false auxiliary rows を減らす。invalid declared-prefix rows 43 件は、ちょうど 3 files の coordinate-like diagnostic rows として分類でき、image-bearing vector segments ではないため、decoded-false のまま保持し、renderable page geometry や paint resources へ promote してはならない。
- [x] `rjtd object-fdm-image-candidates <file>` を追加し、model-owned `fdmIndexEntries` から image-bearing FDMVector rows を summarize する。normalized bbox diagnostics、complete payload coverage、page placement が証明されるまでの明示的な `renderable=false` を含む。
- [x] `object-fdm-image-candidates` を sweep する。61 checked、0 failures、candidates 付き files 3、FDM sources 3、image-bearing rows 6、image hits 13、strict complete payloads 0、plausible bbox rows 5、renderable rows 0。FDMVector segments 内で観測される `FFD8FF` hits は SOF/SOS structure を持つ valid JPEG payload ではなく、vector data 内の JPEG-like byte patterns であるため、signature-only decoded-false evidence のまま残す。
- [x] 同じ FDM image rows を app-core `getPageOverlayImages` の `unplacedDiagnostics` として expose する。`imageCount:0`、`placementProven:false`、`renderable:false` を維持し、rhwp-shaped overlay API は callable にしつつ page placement が decoded されたようには見せない。
- [x] `/Frame` fixed 60-byte records を decoded-false model/export/app-core `objectFrameRecords` として保存する。observed object id、record kind/type、geometry-like fields を含む。
- [x] `rjtd object-fdm-frame-links <file>` を追加し、image-bearing FDMIndex rows と `/Frame` records を `fdm row index == frame object id` で相関させる。
- [x] strict image payload dimension diagnostics を追加する。model/export JSON は optional `imagePayloads[].dimensions` を保持し、JPEG payload spans は SOF/SOS structure validation を通過した場合だけ promote され、`object-fdm-frame-links` は frame size、payload dimensions、dimensioned payload counts、best frame/payload aspect delta を報告する。
- [x] `object-fdm-frame-links` を sweep する。61 checked、0 failures、positive files 3、FDM image rows 6、image hits 13、frame-linked rows 6、missing-frame rows 0、strict complete payloads 0、dimensioned payloads 0、renderable rows 0。
- [ ] FDM または `Embedding N` image ownership candidates を page geometry や paint resources へ promote する前に、`/Frame` geometry units、page association、paint order、payload-to-image selection を decode する。
- [ ] unknown inline formatting/control records の背後に隠れた残り text を復元する。
- [ ] current token layer を超えて true `DocumentText` record boundaries と control semantics を decode する。
- [ ] `MarkV.01` delta candidates 9、29、30 が強く score する理由を説明する。現在の evidence は `unit + 29` を unique stable adjustment と扱うことを支持しない。
- [ ] varying MarkV.01 header value `0x0603`/`0x0610`/`0x061c` の意味を decode する。current summary、LineMark-context、exact-byte-search evidence は direct page-count、document-length、global page-style-code、direct LineMark-entry-offset interpretation を弱める。
- [ ] 29-byte `TCntV.01` records 内の remaining fields の semantic meaning を decode する。current `text-position-count-field-deltas` evidence は `t1/t2` が ordered range-like pair であるが chosen `start/end` range と同じ span ではないことを示す。
- [ ] `TCntV.01` が byte coordinates、UTF-16 unit coordinates、layout/object-local coordinates を混在させるのか、または current token map が intervening record structure を欠いているのかを判断する。tail `t1/t2` は現時点で UTF-16 unit coordinates に寄り、delta 29/30 で unit hits は改善する。一方 `0x0202` chosen ranges は strong byte-range text/control overlap を示すが同じ tail coordinate behavior とは一致しない。
- [ ] `/DocumentText` control boundaries `0x001c` と `0x000e` を decode する。current evidence では `0x001c` は high-frequency text/control delimiter、`0x000e` は adjacent control clusters または skipped-inline content の前に多い。
- [ ] `text-boundary-layout-map` で見えた file-specific shifts を説明できる row-local、section-local、または record-local base offset を探す。
- [ ] `iwata_file` strict boundary candidates のうち 10 件だけが line-word/page-field exact endpoint evidence を両方持ち、残りの strict candidates と selected finance spans が持たない理由を説明する。view-style group hits は strict non-paragraph rows にも現れるため、反証されるまでは default/flag-like evidence として扱い、real paragraph construction には使わない。
- [ ] shifted leading byte が flag、prefix、version、または preceding record boundary として説明できたら、`TCntV.01` `be0` と `be1-shifted` diagnostics を explicit raw-preserving record family types に昇格する。
- [ ] `TCntV.01` の actual coordinate target を特定する。current evidence は direct `/LineMark`、`/PageMark`、`/PaperMark` word/row/byte coordinates を rejected。
- [ ] empty `/DocumentTextPositionTables` sample の out-of-range mini-sector が stale directory entry、malformed ministream chain、または別の storage/object boundary から復元可能なものか判断する。
- [ ] `page-marks` がまだ support していない `/PageMark` variants の record layout を完全に証明する。
- [ ] `SO` object/control record family field semantics を decode する。current evidence は singleton records の fields 1-4 が geometry-like tuples、repeated records が default/control constants を持つことを示すが、`packed-jseq3-like` 16-bit halves の exact meaning は未証明。
- [ ] embedded image payload spans の前にある semantic object header fields を decode し、payload ownership を `/Figure`、`/Frame`、`/LayoutBox`、layout mark evidence に接続する。
- [ ] table semantics は named stream matching ではなく `/DocumentText` control ranges と layout/style streams から decode する。current inventory は `hyo` sample を含め named table stream を見つけていない。
- [ ] `/PaperMark` header count-like values と `0x00010000`/`0x00010010`/`0x00010011` flags の semantic meaning を decode する。
- [ ] parser shape を広げる前に、3 件の unsupported `/PaperMark` stride values を説明する。
- [ ] `/LineMark` 内の `0x1000`、`0x1001`、`0x1002` tag families を decode する。current evidence では immediate next word は family discriminator ではなく payload-like で、LineMark offsets は direct text coordinates ではない。
- [ ] MarkV.01 header の final `u16` が両 stream を持つ 3 samples で `/LineMark` tag clusters 付近に入る一方、MarkV.01 entry offsets は入らない理由を説明する。
- [ ] embedded fragment plausibility filtering を structured object/stream boundary parsing に置き換える。

Important constraint:

- Ichitaro filter license は decompilation と reverse engineering を制限している。binary は no-code reference artifact として扱う。disassembly から code を copy したり implementation を derive したりしない。

## M3: Document Model

Goal: document model layer 経由で `rjtd export <file> --format json` を実装する。

Status: minimal model path implemented.

Completed:

- [x] extracted `DocumentText` から minimal `Document` を構築する。
- [x] rhwp の parser trait に似た `DocumentParser` entry point を追加する。
- [x] raw `/DocumentText` bytes を `Document` model に保存する。
- [x] non-empty text lines を `TextRun` inlines を持つ `Paragraph` blocks として表現する。
- [x] `Document` から JSON export を実装する。
- [x] `Document` から Markdown と plain text export を実装する。
- [x] `rjtd export <file> --format json` を wire する。
- [x] `rjtd export <file> --format md` を wire する。
- [x] exporter が raw file または stream bytes ではなく `Document` を consume するように保つ。
- [x] observed ruby base plus phonetic annotation pair を structured `Inline::Ruby` model data として保存し、plain text/Markdown/PDF output は visible base text を使い続ける。
- [x] observed style/layout streams (`/DocumentEditStyles`, `/DocumentViewStyles`, `/TextLayoutStyle`, `/PageLayoutStyle`, `/PageLayoutStyleHeader`) を record semantics decode 前に named `UnknownStyle` model data として保存する。
- [x] decoded paragraph styles と偽らず、app-core document/style JSON (`getDocumentInfo`, `getStyleList`, `getStyleDetail`) から preserved JTD style stream sources を expose する。
- [x] future style record decoding が raw blobs だけでなく structured evidence から始められるよう、observed style stream families と big-endian header fields (`ssmg` vs table-like prefixes) を JSON に summarize する。
- [x] observed style stream record boundary candidates を JSON に expose する。`0x5555`/`0x4444` Ssmg slot records は `ssmg-slots`、DocumentViewStyles-style `u16 code + u16 payload_len + payload` records は `sequential` として報告する。
- [x] Ssmg style records から conservative UTF-16BE `label` candidates を抽出する。observed labels には `脚注(標準)`、`本文(ｵｰﾄｽﾀｲﾙ)`、page layout labels の `中扉(自動)` などが含まれる。
- [x] labeled `/TextLayoutStyle` records を rhwp-shaped app-core style candidates として promote し、`getStyleList` と `getStyleDetail` が observed JTD style names を `decoded:false` のまま expose できるようにする。
- [x] app-core `applyStyle` が JTD style candidate references を in-memory `Paragraph` model に保存し、`getStyleAt` と split paragraphs がその fallback style state を反映するようにする。
- [x] WASM `HwpDocument` wrapper が rhwp-shaped browser API surface 経由で JTD style candidates を expose/apply することを検証する。
- [x] apps が style panel を開く前に observed JTD style candidates を検出できるよう、app-core `getDocumentInfo` に `styleCandidateCount` と `styleCandidateNames` を追加する。
- [x] preserved style stream summaries、record offsets/codes、payload lengths、labeled `/TextLayoutStyle` candidates を観測する reverse-engineering diagnostics として `rjtd style-records <file>` と `rjtd style-candidates <file>` を追加する。
- [x] unlabeled records、payload digests、short payload previews、labeled candidate IDs を含む all `/TextLayoutStyle` records を inspect する `rjtd text-layout-style-records <file>` を追加する。
- [x] `/DocumentViewStyles` group records を decoded style references として扱う前に payload lengths、payload digests、short payload previews を inspect する `rjtd document-view-style-groups <file>` を追加する。
- [x] `TCntV.01` tail fields と observed text/page style candidate IDs・record indexes を相関させる `rjtd text-position-style-context <file>` を追加する。ただし、これらの hits を decoded style references として扱わない。
- [x] field-level style hit distributions を summarize し、`f1` のような variable fields を text style candidates、page style candidates、`/DocumentViewStyles` group records と比較しつつ、`f7` のような near-constant/default-like fields を分離する `rjtd text-position-style-summary <file>` を追加する。
- [x] `TCntV.01` tail fields と adjacent field pairs を document-text unit/text hits と比較し、`f1/f2` が pure style reference より range/coordinate pair に近く振る舞うことを示す `rjtd text-position-count-tail-field-roles <file>` を追加する。
- [x] valid `TCntV.01` text-count entries を decoded-false `textCountRanges` model/export/app-core metadata として保存し、parse 後に observed range/coordinate evidence を捨てないようにする。
- [x] parsed `TextRun` model data に `/DocumentText` byte/UTF-16 source span を保存し、decoded-false `textCountRanges` に byte/unit `documentTextOverlaps` evidence を attach する。
- [x] app-core `getPageLayerTree` から fallback `textRun` ops と rhwp-shaped `textSources`/`source` span を expose し、可能な場合は JTD byte/unit source range も含める。
- [x] `getPageLayerTree` に fallback `pageBackground` paint op を追加し、layer/replay order を rhwp の background-to-flow model に合わせる。
- [x] `schema`、`resourceTable`、`outputOptions`、`fontResources`、feature lists、fallback `textV2` diagnostics など rhwp-shaped layer tree envelope metadata を追加する。
- [x] model-owned fallback `WritingMode` path を追加し、`DocumentCore`、SVG page rendering、`getPageLayerTree` が exporter から raw style streams を読まずに `horizontal` と `vertical-rl` を project できるようにする。
- [x] conditional `rjtd-export` local-sample PDF smoke test を追加し、available local `.jtd`、`.jtt`、`.jttc` samples をすべて parse/export して PDF header、page marker、EOF marker、minimum size を確認する。
- [x] app-core `getValidationWarnings` が empty report 固定ではなく、rhwp-shaped JTD fallback/preservation diagnostics を report するようにする。
- [x] `/DocumentText` control boundary codes を decoded-false `textControlBoundaries` model/export/app-core metadata として保存し、可能な場合は byte/unit source span を attach する。
- [x] preserved `textControlBoundaries` を fallback paragraph character offsets に project し、rhwp-shaped `getControlTextPositions` と nearest-control navigation diagnostics で使えるようにする。
- [x] projected `textControlBoundaries` を app-core `getPageControlLayout` でも `type:"jtdControl"`、fallback bounding box、`decoded:false` diagnostics として expose する。
- [x] `/DocumentText` intervals を all controls または selected control delimiter で summarize する `rjtd text-control-ranges <file> [control-code]` を追加する。
- [x] chosen `TCntV.01` ranges と control-delimited `/DocumentText` intervals を比較する `rjtd text-position-count-control-ranges <file> [control-code]` を追加する。
- [x] candidate control-delimited overlap summaries を decoded-false `textCountRanges` model/export/app-core JSON の `controlRangeOverlaps` として expose する。
- [x] `controlRangeOverlaps` から derived decoded-false `textBoundaryCandidates` model/export/app-core JSON を expose し、parsed paragraph semantics を変えずに paragraph-boundary candidate evidence を保存する。
- [x] model-derived decoded-false boundary candidate の basis、delimiter、interval count、single/multi classification、source span、`decoded=false` を出力する `rjtd text-boundary-candidates <file>` を追加する。
- [x] `text-boundary-candidates` を current local samples 全体に sweep する。61 checked、candidates を持つ files 10、candidate rows 356、overlapped intervals 1,586、single-interval candidates 222、multi-interval candidates 134。最大 single candidate は `justsystems-20120223023609-jp-just-finance-j200403sc.jtd` の `0x001c`/unit 44 intervals であり、paragraph promotion にはより strict な rule が必要である。
- [x] decoded-false boundary candidate を `/DocumentText` visible text、line breaks、edge alignment と比較する `rjtd text-boundary-candidate-context <file>` を追加する。
- [x] `text-boundary-candidate-context` を current local samples 全体に sweep する。candidate rows 356、line break を 1 つ以上持つ rows 276、total line breaks 3,458、control gap 直後に始まり aligned text boundary で終わる rows 210。`0x001c` single-interval edge-good rows のうち byte basis は one-line-break 17 と zero-line-break 16、unit basis は one-line-break 22 と zero-line-break 13 である。`0x000e` は many line breaks を含む coarse cases が多く、direct paragraph promotion にはまだ粗すぎる。
- [x] byte/unit decoded-false boundary candidate を text-count range と delimiter で pair し、edge-good flags、line-break counts、visible text previews、match flags を報告する `rjtd text-boundary-candidate-agreement <file>` を追加する。
- [x] `text-boundary-candidate-agreement` を current local samples 全体に sweep する。10 files で byte/unit pairs 178。exact visible-text match は 1 row だけで、その row も empty であるため text equality は promotion rule として使えない。より strict な `0x001c` single/single set は 43 pairs で、unit-basis edge-good/non-empty/line-break<=1 は 33 rows、byte-basis は 28 rows を残す。
- [x] unit-basis `0x001c` single candidate を `/LineMark`、`/PageMark`、`/PaperMark` の direct index/byte context と比較する `rjtd text-boundary-candidate-layout-context <file>` を追加し、rule は diagnostic-only に保つ。
- [x] `text-boundary-candidate-layout-context` を current local samples 全体に sweep する。8 files で unit `0x001c` single candidates 52、strict rule-selected rows 35。ただし selected rows のうち `/LineMark`、`/PageMark`、`/PaperMark` start/end direct hits は 0 であり、candidate source units と layout mark rows は同じ coordinate space ではない。
- [x] unit-basis `0x001c` candidates を sparse `/LineMark` tag positions、`/PageMark` entry indexes/raw fields/byte boundaries、`/PaperMark` entry indexes/byte boundaries と複数の global unit transform hypotheses で score する `rjtd text-boundary-layout-map <file>` を追加する。
- [x] `text-boundary-layout-map` を current local samples 全体に sweep する。61 checked、0 failures、8 files に unit `0x001c` single candidates 52、4 files に strict selected candidates 35。non-boundary exact hits は存在するが、winning target/base/delta combinations は file-specific である。`iwata_file` は line-word-value/page-be32-field の unit-div2 shifts around -1140..-1192 を好み、finance samples は別々の page-be32-field shifts を好むため、paragraph promotion 用の single global layout-map transform はまだ reject する。
- [x] unit `0x001c` single candidate ごとに linked `TCntV.01` row、local delta、nearest start/end layout points、exact endpoint count を出力する `rjtd text-boundary-layout-map-rows <file>` を追加する。
- [x] `text-boundary-layout-map-rows` を current local samples 全体に sweep する。61 checked、0 failures、同じ unit `0x001c` single candidates 52 と strict selected candidates 35 が見つかる。`iwata_file` の strict selected candidates 32 のうち 10 件が `line-word-value` と `page-be32-field` の両方で row-local `exact=2` evidence を持ち、selected finance candidates 3 は row-local `exact=2` evidence を持たない。したがって strict candidates は引き続き diagnostic-only であり、次の rule は paragraph-like rows と non-paragraph large spans を分離する必要がある。
- [x] strict unit `0x001c` single candidate に加えて `line-word-value` と `page-be32-field` の両方に row-local exact endpoint evidence がある場合だけ `paragraph-like=true` を報告する diagnostic-only classifier `rjtd text-boundary-paragraph-like <file>` を追加する。
- [x] `text-boundary-paragraph-like` を current local samples 全体に sweep する。61 checked、0 failures、unit `0x001c` single candidates 52、strict selected candidates 35、paragraph-like candidates 10、strict selected but non-paragraph-like candidates 25。この rule では `iwata_file` だけが paragraph-like candidates を作る。
- [x] paragraph-like classifier と linked `TCntV.01` tail fields、text/page style hits、view-style group hits、byte/unit range preview を同時に出力する `rjtd text-boundary-paragraph-like-style-context <file>` を追加する。
- [x] `text-boundary-paragraph-like-style-context` を current local samples 全体に sweep する。61 checked、0 failures、同じ unit candidates 52、strict selected candidates 35、paragraph-like candidates 10、selected non-paragraph-like candidates 25。paragraph-like rows 10 件は text/page style candidate hits を持たないが、すべて `iwata_file` の `/DocumentViewStyles` group evidence を持つ。strict non-paragraph rows も 25/25 で view-group hit を持つため、これは paragraph discriminator ではなく、paragraph style attachment を証明しない。
- [x] paragraph-like、strict-non-paragraph、non-strict bucket を exact layout evidence、`TCntV.01` family/span、tail field counts、style/view-style hits で要約する `rjtd text-boundary-paragraph-like-discriminators <file>` を追加する。
- [x] `text-boundary-paragraph-like-discriminators` を current local samples 全体に sweep する。61 checked、0 failures。dual exact layout evidence は paragraph-like rows だけに現れる (10/10)。strict-non-paragraph と non-strict はそれぞれ 0/25、0/17。`iwata_file` では paragraph-like rows は `be0` で `range-spans=2..8` だが、strict-non-paragraph と non-strict rows は `range-spans=0..0` である。したがって nonzero chosen `TCntV.01` span と dual row-local layout exactness を次の discriminator としてテストしつつ、まだ decoded-false に保つ。
- [x] この stricter discriminator を real paragraph として再構築せず、decoded-false `textParagraphBoundaryCandidates` として model/export/app-core JSON に保存する。61-sample JSON export sweep は 0 failures で、合計 10 candidates を保存し、すべて `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` 由来である。
- [x] `rjtd text-paragraph-boundary-targets <file>` を追加し、preserved `textParagraphBoundaryCandidates` を concrete `/LineMark` word indexes と `/PageMark` raw row fields へ trace する。
- [x] `text-paragraph-boundary-targets` を current local samples 全体に sweep する。61 checked、0 failures、candidates を持つ file は 1、total candidates は 10。現在の hit formatter では 20 endpoints のうち line endpoints 6 件と page endpoints 4 件が non-unique または missing であるため、exactness だけで real paragraph construction を開始してはならない。
- [x] app-core `getCanvasKitReplayPlan` が rhwp mode policy (`default`/`compat`) に従い、fallback `pageBackground`/`textRun` ops を empty plan ではなく direct replay items として report するようにする。

Remaining:

- [ ] extracted newline text から導出するのではなく、real paragraph boundaries を decode する。
- [ ] parse 時に decoded `TextLayoutStyle`、`DocumentEditStyles`、`DocumentViewStyles`、または関連 streams から paragraphs と text runs に style IDs を infer/attach する。
- [ ] 対応する stream mutation format が証明されたら、style edits を decoded JTD style/body streams に保存する。
- [ ] redistributable samples が利用可能になったら fixture-based expected JSON outputs を追加する。
- [ ] inner `DocumentText` structure が理解できたら、decompressed `.jttc` template text/control placeholders を meaningful model blocks に変換する。

## M4: Markdown Export

Goal: document model layer 経由で `rjtd export <file> --format md` を実装する。

Status: minimal model-based Markdown export implemented.

Completed:

- [x] `Document` を consume する Markdown exporter を実装する。
- [x] `rjtd export <file> --format md` を wire する。
- [x] local `a5.jtd` が `一、午后の授業` のような recovered section titles を生成することを確認する。

Remaining:

- [ ] record parser がそれらを expose した後、headings、lists、tables、ruby、layout semantics を保存する。
- [ ] model が stable block/inline semantics を持った後に HTML export を追加する。
