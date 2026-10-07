# rjtd 機能状況

開発 source tree の範囲を示し、全入力や公開済み 0.0.1 package の能力を保証しない。
release は [changelog](../rjtd/CHANGELOG.md)、実行した検証は [検証報告](VALIDATION.ja.md)、
将来の順序は [roadmap](ROADMAP.ja.md) と [TODO](../TODO.ja.md) を参照する。

## 状態の意味

実装済みは対象 profile の code path があること。検証済みは入力集合と実行検査を要する。
candidate/diagnostic/fallback、`decoded:false`、reference-backed の限界は維持する。
method の存在、生成 PDF、入力を skip した test は忠実度の証拠ではない。

## 元の milestone

| ID | 領域 | 現在の範囲 | 残る境界 |
| --- | --- | --- | --- |
| M1 | container 調査 | 観測 JTD/JTT/JTTC CFB と限定 FAT 回復 | 全 version/container ではない |
| M2 | 抽出 | named/embedded text、ruby base、観測 LH5 inner data | 一般 control/paragraph/style は未完成 |
| M3 | model | source flow/span、raw/unknown、candidate と app facade | 描画/app が混在。構造保持編集/save は未完成 |
| M4 | export | text/Markdown/JSON、基本 HTML、native PDF | HTML は full layout でなく PDF は限定 source/fallback |
| M5 | local 形式研究 | 形式と実装記録を分離し RFC 0004 番号を保持 | 主張別根拠・独立 review が必要。共同文書は未変更 |
| M6 | WASM viewer | browser-local 選択/drop、page、SVG/text | full editor でなく runtime/deploy は別検証 |

## 限定した開発 profile

段落 indent/余白/pitch、font size/文字 style/font 選択、罫線本文/border/空・merge span、保存 field/TOC/leader、
running region/用紙方向/cover、縦書き/縦中横、単一 PNG、単純図形の色/順序、保存 JSEQ/GCI 文字、
JTT/JTTC inner 補助 stream を扱う候補がある。

一般解読ではない。保存番号/field/TOC は生成/編集の証明ではなく、正確な native font/glyph、任意 object/style、
歴史的 version は限界がある。[既知の差異](VALIDATION.ja.md#既知の差異) を参照する。

## 実装境界

core は低水準根拠を持つ。model は `--no-default-features` で解析・原本調査のみを
build でき、optional `rendering` は `DocumentCore` と app/描画状態を提供する。
default は bitmap 描画を維持する。原本候補 API は renderer 初期化なしで原本単位、
font identity、section/running policy、logical row range を公開する。export は出力/PDF、
WASM は既存 `HwpDocument` を維持する。wrapper/API 整理は別の作業である。
[model build 構成](../rjtd/crates/rjtd-model/README.md#build-configurations) を参照する。
