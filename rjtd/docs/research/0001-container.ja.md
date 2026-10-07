# rjtd 研究記録 0001: Container readers and recovery

Status: 実装記録。共同形式 RFC ではない

English source: [0001-container.md](0001-container.md)

## 実装の範囲

container reader と回復処理。標準 CFB と限定した lenient fallback、CLI 表示、open 成功数は実装動作であり形式要件ではない。

[形式向け記録](../../../openjtd-spec/rfc/0001-container.ja.md) は原本観測と未確定解釈を記録する。
Rust type、candidate 条件、JSON、CLI、paint 近似、test、資源方針は実装動作であり、
native semantics の証明ではない。

## 再現の入口

repository root で `cargo build --manifest-path rjtd/Cargo.toml -p rjtd-cli` を実行する。
解析・開示を許可された入力を使い、command の必須引数は `rjtd/target/debug/rjtd --help` を確認する。

```sh
rjtd/target/debug/rjtd streams path/to/document.jtd
rjtd/target/debug/rjtd info path/to/document.jtd
```

page number、stream、format などを要する command がある。結果には実際の全 command、
入力 ID/hash、作成 version、実装 commit、実行/skip、制限を記録する。private corpus の
検証を列挙しても public 再現性の証明にはならない。

## 過去の根拠

[分離前の完全な記録](https://github.com/OpenJTD/rjtd/blob/cce9cb20611c8c809a7376c8df6130aed14a0437/openjtd-spec/rfc/0001-container.ja.md) に raw example、command、local sweep、棄却仮説、実装履歴を
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

初期 local JTD samples は Compound File Binary (CFB) documents である。

M1 は container inventory だけを実装する。stream payloads は解釈しない。

container layer はまず `cfb` crate を使う。これは OLE/CFB files に対する rhwp の dependency choice と一致する。standard reader が reject する malformed FAT data を CFB file が持つ場合、rjtd は rhwp の `LenientCfbReader` approach を model にした narrow lenient reader に fallback する。

## Command

```sh
cd rjtd
cargo run -p rjtd-cli -- streams ../rjtd-testdata/local-samples/a5.jtd
```

output format は tab-separated:

```text
<entry-kind>    <byte-size>    <path>
```

`entry-kind` は `storage` または `stream`。

stream names 内の ASCII control characters は表示用に escape される。例: `\x04JSRV_SegmentInformation`。

## Lenient FAT Fallback

一部の local `.jtd` samples には duplicate sector pointers のような FAT inconsistencies が含まれる。rhwp は同等の HWP files を、別 dependency 追加ではなく direct `LenientCfbReader` implementation で処理する。

rjtd はその pattern に従う。

- standard `cfb` parsing を最初に試す。
- lenient parsing は、standard parsing が CFB らしい file で失敗した後だけ使う。
- stream inventory と stream reads は同じ fallback を共有する。
- successful lenient open 後も、missing streams は `not found` errors のままにする。

Current local sweep:

| Command | Local samples checked | Result |
| --- | ---: | --- |
| `rjtd info` | 61 | 61 opened |
| `rjtd cat` | 61 | 61 opened; 2 use embedded `SsmgV.01` fragments instead of named `/DocumentText` |

## Local Samples

以下の sample names は、初期 container inventory work で使った local files を指す。
観察済み stream layouts を比較するための examples である。

| Sample | Entry count | Notes |
| --- | ---: | --- |
| `a5.jtd` | 32 | `LineMark`, `PageMark`, `PaperMark`, `PageLayoutStyleHeader` を持つ |
| `46.jtd` | 31 | `LineMark`, `PageMark`, `PaperMark` を持つ |
| `b6.jtd` | 31 | `LineMark`, `PageMark`, `PaperMark` を持つ |
| `shinsyo.jtd` | 28 | initial inventory では `LineMark`, `PageMark`, `PaperMark` を持たない |
| `a6.jtd` | 28 | initial inventory では `LineMark`, `PageMark`, `PaperMark` を持たない |

## Common Top-Level Streams

これらの top-level streams は five local samples すべてに現れる。

- `/\x04JSRV_SegmentInformation`
- `/\x04JSRV_SummaryInformation`
- `/\x05SummaryInformation`
- `/AutoTextInfo`
- `/DocumentEditStyles`
- `/DocumentPeripheralThree`
- `/DocumentPeripheralTwo`
- `/DocumentText`
- `/DocumentTextPositionTables`
- `/DocumentViewStyles`
- `/Font`
- `/Footnote`
- `/Header`
- `/MarkTag`
- `/PageLayoutStyle`
- `/ReferenceInfo`
- `/RelatedDocuments`
- `/TextLayoutStyle`
- `/ThinkingTemplate`

## Common Storage Tree

five samples はすべて次の macro storage tree を含む。

```text
/DocumentMacro
/DocumentMacro/\x04JSRV_SegmentInformation
/DocumentMacro/Macros
/DocumentMacro/Macros/\x04JSRV_SegmentInformation
/DocumentMacro/Macros/BaseStorage0
/DocumentMacro/Macros/BaseStorage0/\x04JSRV_SegmentInformation
/DocumentMacro/Macros/BaseStorage0/InfoStream
/DocumentMacro/Macros/BaseStorage0/MacrosStream
/DocumentMacro/Macros/BaseStorage0/MacrosStreamStyle3
```

## Early Hypotheses

- `/DocumentText` は圧倒的に大きな common stream なので、primary body text candidate である。
- `/DocumentTextPositionTables` は text positions を index または map している可能性が高い。
- `/DocumentViewStyles`、`/DocumentEditStyles`、`/TextLayoutStyle`、`/PageLayoutStyle`、`/Font` は style/model candidates である。
- `JSRV_*` streams と `SummaryInformation` streams は、理解される前でも保存すべきである。

## Next Questions

- `/DocumentText` が compressed、encoded、segmented、encrypted のいずれかを判定する。
- `/DocumentText` size と content を、known text を持つ trivial documents と比較する。
- page size variants が `a5`、`a6`、`b6`、`46` の違いを説明するか確認する。
- line/page mark streams が optional layout caches なのか required model data なのか判断する。
- named `/DocumentText` ではなく embedded `SsmgV.01` fragments を使う samples の proper object/stream boundaries を特定する。
