# Architecture

`rjtd` は JTD 原本の解釈と描画を分離する。OpenJTD の共同研究は原本データとその意味を
記録し、出力方法と近似は各実装が管理する。他の office code は役立ち、参照が許される場合の
任意資料である。その構造、address、依存、API parity を JTD の要件にしない。

```text
Document File
      │
      ▼
Container Layer
      │
      ▼
Stream Layer
      │
      ▼
Record Layer
      │
      ▼
Document Model
      │
      ├──── Text / Markdown / semantic HTML / diagnostic JSON
      └──── Rendering
                ├──── font measurement and layout
                ├──── SVG / page layers
                └──── PDF / browser output
```

## Layer Rules

- Container code は logical streams を発見して開く。
- Stream code は byte-level の stream access を扱う。
- Record code は typed record と unknown record boundaries を decode する。
- Model code は semantic document structures を所有する。
- model の意味は JTD の原本根拠で決める。利用者固有の編集 address は原本解釈ではなく
  adapter 境界で扱う。
- Export code は document model だけを consume する。

Exporter は raw container、stream、record data を直接読んではならない。

## Unknown Preservation

リバースエンジニアリングは段階的に進む。まだ理解されていない data は破棄せず、unknown model shape のいずれかとして次の層へ運ぶ。

- `UnknownRecord`
- `UnknownBlock`
- `UnknownStyle`
- `UnknownObject`

## Current Implementation Boundary

この分離は設計目標である。今回の文書変更は Rust module を移動せず、描画 component の
独立 build が成立したことも意味しない。

`rjtd-core` は低水準の container・stream・record parser を持つ。`rjtd-model` は
`Document` と parser 統合に加え、`DocumentCore`、pagination、SVG、page-layer の
直列化、編集状態も持つ。`rjtd-export` は出力の直列化と native PDF backend を持つ。
したがって現在の crate 名だけでは、目標とする境界を強制できていない。

現在の core block model は `Paragraph` と `Unknown`、inline は text、ruby、unknown
object を公開する。表、スタイル、レイアウト、オブジェクトの候補は、意味が証明されるまで
根拠として保持する。SVG/PDF 描画はこのモデルに fallback layout と限定的な診断投影を
組み合わせる。

`DocumentCore` は read/render API と基本的な本文編集を提供する。WASM の `HwpDocument`
wrapper は既存の browser API を維持するが、多くの高度な呼出しは既定値や no-op を返す。
API の存在は JTD 編集対応や round-trip preservation の証明にはならない。

基本的な文書 HTML 出力は `rjtd-export` に属し、app-core の HTML clipboard methods は
別の互換 surface である。現在の範囲は [機能状況](FEATURE-STATUS.ja.md)、根拠は
[検証](VALIDATION.ja.md)、次の完了条件は [roadmap](ROADMAP.ja.md) を参照する。

## 解釈コア

解釈は原本を読み、流路、record、source range、関係、未知データを保持する。field の意味は
根拠で裏付けられる範囲だけ確定する。抽出、調査、描画、編集が利用できる所有モデルを出力する。

原本の座標と単位、書字方向、文字属性、保存済み行・ページ範囲、罫線境界、object anchor、
描画順序は、原本の意味を表す限りここに属する。見た目に影響する layout 指示も同様である。
未確定の解釈は source evidence を伴う candidate のまま保持する。

コアは SVG/PDF、browser API、インストール済み font、glyph 計測、DPI 変換、fallback
layout に依存しない。renderer を初期化せず文書を調査できなければならない。出力側の計測で
原文、source span、field 値、根拠の確度を変更しない。

罫線は原本の text flow に付随する。出力や編集のための `Table -> Row -> Cell` 投影は
原本の流路を置き換えず、解析の前提条件にもならない。

## 描画

描画は model と明示的な出力 context を受け取る。context は利用可能 font、その計測値、
出力倍率、印刷日などの動的値を持つ。glyph advance、行配置、fallback pagination、ruby・
script 配置、clipping、paint を計算し、SVG、page layer、PDF、browser 出力を作る。

解釈済みの原本指示を利用し、近似は別に記録する。見た目が一致しても field の意味の証明には
ならない。合成 bold、代替 font metrics、推定 baseline、backend 固有 leader は描画側の選択である。

renderer は input container を開かず、model の raw stream も decode しない。raw field の
認識と配置を混在させる既存関数は、先に認識結果を解釈モデルへ分離する。その後に配置と paint を
描画境界へ移す。依存は model から描画への一方向であり、コアが描画結果を使って原本解釈を
完成させることはない。

`DocumentCore` は移行中の application facade として維持できる。load、search、editing、
selection、snapshot は application の責務であり、selection geometry と hit testing は描画結果を
使う。facade の名称や互換 method は解釈コアの境界を決めない。

## 移行と回帰検証

まず既存 workspace 内で境界を作る。描画 crate の追加は責務と一方向の依存が明確になってから
判断する。この計画で crate や依存を追加しない。

初期の責務対応は file 名だけで決めない。

| 現在の surface | 目標の責務 | 分割条件 |
| --- | --- | --- |
| `rjtd-core` と model `parse` | container/record 解析と model load | 資源上限と raw 保持を描画に依存させない |
| `document_text/flow`、field/note、文字 style 解釈 | 流路・関係・原本属性 | 配置 helper を認識結果から分ける |
| `document_text/native_*`、`table_grid`、`object_media/native_*` | 原本解釈と描画が混在 | raw field 認識を先に model candidate へ分け、配置・advance・paint は描画へ置く |
| `document_text/svg`、page-layer 構築、`table_grid_render_projection` | 描画 | model の原本指示を利用し、残る raw 認識を分離してから移す |
| `rjtd-export` PDF と browser font 計測 | 出力 backend | 描画/model 結果に依存し、コアの依存先にはしない |
| `DocumentCore`、`search_render_editing`、WASM facade | application 調整 | public call を維持し、描画へ委譲して geometry を使う |

保存済み行・page 範囲と座標は renderer が使っても原本指示であり、fallback 改行・pagination は
計算する出力である。同じ `layout` / `native` module にあるという理由で一括移動しない。
model 診断 JSON と描画 page-layer JSON tree も区別する。

1. 関数を原本解釈、描画、app state に分類する。混在関数を source evidence の結果で分け、
   unknown bytes と candidate status を保持する。
2. font 計測、出力単位変換、SVG paint、page-layer 構築を描画境界へ移す。既存 facade API は
   委譲する互換入口として残す。
3. 保存済み原本行・ページ指示と fallback layout を分ける。解釈済み指示は抽出と描画で共有し、
   相互の出力を解析入力にしない。
4. renderer なしで解析と model 調査が build できることを検証し、その後に crate 分離を判断する。

各コード移動で public parser/model 回帰、formatting、Clippy、workspace tests、WASM、MSRV と、
本文、source span、unknown data、ページ寸法、描画出力を検証する。既存 native pair は local で使い、
private・利用不可の入力は別に報告する。既存の原本意味と描画動作を変えない。

## 形式 RFC と実装記録

形式 RFC は bytes、record 境界、関係、観測動作、仮説、反例、適用する作成 version を記録する。
主張ごとに根拠と検証範囲が必要である。再現用 tool の command や parser output は示せるが、
その data model や fallback behavior は形式の契約ではない。

CLI、Rust API、JSON 形状、描画近似、test sweep、実装優先順位は
[rjtd の研究記録](../rjtd/docs/research/README.ja.md) に置く。形式向け draft は
[openjtd-spec](../openjtd-spec/README.ja.md) で索引する。この local 分離は共有リポジトリを
変更せず、draft の共同承認も意味しない。
