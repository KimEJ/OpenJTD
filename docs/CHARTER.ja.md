# OpenJTD プロジェクト憲章

## 共同研究と独立実装

OpenJTD は一太郎の JTD/JTT/JTTC について、仕様、観測、検証資料を共有する公開研究
プロジェクトである。知識は特定の実装に依存せず、独立に検証できる形で残す。

本 repository の `rjtd` は独立した Rust エンジンであり、忠実な読取・描画・編集・保存を
長期目標にする。Tika JTD+ は抽出と統合の目標を独立に維持する。どちらの API、model、
出力形式、機能一覧も共同形式の定義ではない。

## JTD 原本を基準にする

解釈と model は原本と再現可能な観測で決める。本文流路、境界宣言、record、source range、
object 関係、未知データを保持する。罫線領域を解釈コアで固定の表 container にする必要はない。

他の office 実装は具体的な問題に役立ち、参照が許される場合に利用する。構造、address、依存、
API coverage を開発義務や完了条件にしない。既存 utility を優先し、新しい依存には明示承認が必要である。

## 解釈と描画

[architecture](ARCHITECTURE.ja.md) は原本解釈、描画、application state の境界を定める。

```text
Document bytes -> container / streams / records -> owned document model
                                                    |-> extraction / inspection
                                                    |-> rendering -> SVG / PDF / browser
                                                    `-> supported editing / save
```

根拠のある原本座標・単位・書字方向・書式・行/page 範囲・anchor・paint order は解釈に属する。
glyph 計測、代替 font、fallback layout、DPI 変換、paint は描画に属する。consumer は model を使い、
exporter/renderer は原本 container を開き直さない。

これは目標境界である。現在の `rjtd-model` は解析/model 統合、描画、app state を含む。
実コードの分離には動作を保持する移行と回帰検証が必要である。

## 保存と根拠

未知 record/block/style/object と raw data を保持する。`Candidate`、`Unknown`、`Diagnostic`、
`decoded:false`、reference-backed projection は表示成功でも根拠の制限を失わない。
parser 成功や一つの画像一致だけで形式意味を確定しない。

一太郎 2026 の native 文書で一条件ずつ変え、baseline を作る。旧 version の適用範囲は規則・version
ごとに別途記録する。対照入力と解析を許可された実文書は検証目的が異なる。観測、仮説、反例、
実装動作を区別する。

## 研究と権利

現行実装の研究規則は [CONTRIBUTING.md](../CONTRIBUTING.md) に従う。公開 metadata、許可された
文書解析、対照実験、通常操作で観測できる動作を根拠にする。参照 artifact と外部 code の条件を守る。
この憲章は proprietary code の複製、private SDK、decompiler からの実装復元、権利のない入力・
派生出力の配布を許可しない。

共同方針と data format は記録された共同レビューを要する。local 文書だけでは組織合意にならない。
provenance、sharing basis、許可された相手を記録し、private storage を共有許可と見なさない。

## repository の責務

| Repository | 責務 |
| --- | --- |
| `OpenJTD/community` | 共同方針の議論、決定、合意記録。初期設定中は private |
| `OpenJTD/spec` | 形式 RFC、観測、再現手順、検証基準 |
| `OpenJTD/corpus` | 再配布可能入力と公開を許可された manifest |
| `OpenJTD/corpus-private` | 指定された共同研究者へ共有を許可された入力 |
| `OpenJTD/rjtd` | 独立 Rust 実装、release 手順、local 研究記録 |

RFC 0001/0003 の共有 import は historical status を維持する。形式主張は
[rjtd 実装記録](../rjtd/docs/research/README.ja.md) と分離する。共同著作物は Apache-2.0 を基本とし、
外部文書は個別の元条件と provenance に従う。

## workspace と milestone

`rjtd/` は core/model/export/CLI/WASM/testkit を持つ。root `docs/` は実装方向と境界、
`openjtd-spec/` は local 形式 draft を管理する。samples/testdata/viewer は別の用途を持ち、任意の
外部 checkout は architecture/runtime の必須要素ではない。

現在の範囲は [機能状況](FEATURE-STATUS.ja.md)、根拠は [検証](VALIDATION.ja.md)、将来の順序と完了条件は
[roadmap](ROADMAP.ja.md) が管理する。container、抽出、model/保存、export、
公開形式研究、WASM viewer を扱い、原本根拠、再現検証、明示範囲で完了を判断する。
構造を保持する編集には edit/save/reopen が必要であり、外部連携は consumer と操作が決まってから範囲を定める。
