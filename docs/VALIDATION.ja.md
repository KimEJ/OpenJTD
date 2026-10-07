# rjtd 検証記録

記録日: 2026-10-07。実装検証と限界を記録し、将来の順序や全 JTD 忠実度認証ではない。
[機能状況](FEATURE-STATUS.ja.md)、[roadmap](ROADMAP.ja.md)、[fixture guide](../rjtd-testdata/README.ja.md) を参照する。

## 入力と revision

local native batch は原本/PDF 67 対・85 page、checklist 69 項目中 T04/O07 は保留である。
原本と詳細出力は private のままで public corpus へ移していない。実装監査は checkout `3869c48`、
同一 engine source の dev `cce9cb2` を対象にした。当時の文書整理は新実装・releaseではない。後続の整理検証は下記に別記する。

67 ケースの SVG/layer は既に調査した artifact と同一性を確認した。PDF 本文・page 寸法/数、限定 geometry/paint、
source span、対象 source-profile 回帰は各検証範囲で比較した。bold、plain fixed pitch、cell padding は修正後 artifact を
使う。抽出順と文字欠落を区別し、printer tile/primitive 数を model object 数と同一視しない。

## 結果の範囲

当該入力集合で source 根拠を確立した作業は実装・検証済みで dev にある。全 glyph 位置、一般 editing、任意 version、
全形式意味の証明ではない。[CI 37505779347](https://github.com/OpenJTD/rjtd/actions/runs/37505779347) は
`cce9cb2` の quality/MSRV を通過し、508 passed・0 failed・31 ignored を記録した。
未 commit 文書の CI 結果や private assertion の実行証明ではない。

## 既知の差異

| 範囲 | 検証/保持した情報 | 残る限界 |
| --- | --- | --- |
| P09 note/ruby | 本文/link、marker span、限定 typography | 下部脚注/区切線未描画。print/end-mode/gap/baseline 未解読 |
| 日本語 plain flow | 原本行/余白/indent/font/fixed pitch | 一般字間と wrapped-line 分散未解決 |
| bookmark | 名前と元 directory/position bytes | address 単位/normalization/navigation 未解読。固定 +29 は反証あり |
| O01/O02 image | image/frame 寸法、mode、clearance | inline baseline と代替 font wrap split に差異 |
| 一般描画 | 限定 source profile と根拠保持 | 正確な native font/printer と任意 variant は未証明 |
| 編集 | 基本本文操作と facade | 原本保持 mutation/save/reopen・一般再生成は未完成 |

現行版対照は歴史的実文書の pagination/orientation 反例を解決しない。規則/version ごとに適用を検証する。
独立 contrast の不足は field 役割の証明を制限するが、形式に機能がないという証明ではない。

## 再現と報告

[fixture tier](../rjtd-testdata/README.ja.md#verification-tiers) に従い、入力識別/権利、version、command、revision、
実行/skip、差異を記録する。synthetic parser 契約、import 経路、native 出力比較を区別する。
詳細 local audit は ignored workspace にあり、public checkout に含まれると主張しない。
品質 workflow の範囲と忠実度/browser runtime の検証を分ける。今回の文書整理で新 native 入力を作成していない。

## 解釈/描画整理の検証 (2026-10-08)

model の optional `rendering` 境界は上記67対の形式監査と別に検証する。local workspace
回帰は `local_` 除外で483 passed/0 failed/17 ignored/52 filtered、source-only は43 passed、
bitmap なしの描画は228 passed/11 ignored/24 filtered。両 Clippy、workspace/source-only MSRV、
WASM、format、警告なし doc を通過した。stream/record/image 予算は描画なしでも検証し、
app page 予算は描画構成で保持する。

独立 consumer で source query、`DocumentCore` と bitmap 依存の不在、bitmap なし描画、
default 描画を確認した。private native31ケースの model 調査は renderer 有無で同一だった。
default 出力比較は31ケース49 pageの整理前rjtd出力が基準で、原本officeとの完全一致ではない。
日付境界をまたぐ PDF 比較では既存 exporter API で printing date を固定する。

model/parser、資源予算、app state、snapshot、移動した調整処理の本体を照合した。
外部依存、共同 repository 変更、Windows 入力は追加していない。候補の限定profileと
raw/unknownを維持する。WASM名称/APIと後続release preflightは別作業として残る。
