# rjtd Roadmap

rjtd の読取・描画・編集・保存に向けた実行順を定める。現在の能力は
[機能状況](FEATURE-STATUS.ja.md)、計測と限界は [検証](VALIDATION.ja.md)、具体作業は
[TODO](../TODO.ja.md) に分ける。[憲章](CHARTER.ja.md) と [architecture](ARCHITECTURE.ja.md) が範囲を定める。

## 範囲

原本 flow、range、関係、unknown を保持する。原本指示の意味は解釈、font 計測・fallback layout・paint は
描画の責任である。根拠のある座標・単位・方向・書式・object order は解釈に含める。

この local 計画で共同規定を変えたり他 repository へ移入したりしない。将来の共同レビューは独立再現可能な
local 観測を利用できるが、現行規定内の作業を止める前提にはしない。外部 API parity、office 共通 model、
他形式変換は完了条件ではない。

## Next Priorities

### 1. 解釈コアの分離

source 認識と描画が混在する関数を evidence result で分ける。本文・record・関係・unknown・source range を
model に保持する。移行中の caller を維持し、依存が明確になってから crate 境界を判断する。

**完了条件:** renderer/font 初期化なしで解析・model 調査が build/run でき、source/model/resource-limit
回帰を保持する。module の改名だけでは完了しない。

### 2. local 解釈検証の再現

既存 source span・診断・test を使い、入力・command・revision・期待観測・反例を記録する。
native/import/synthetic を区別し、抽出と低水準 flow から始める。許可された最小 fixture を別 checkout で
再現できるようにし、private 検証は別 tier で報告する。共同 observation schema は定義しない。

**完了条件:** 対象検証が未追跡 private 入力なしで再現し、失敗・未対応が見え、規則ごとの入力/version 範囲を
追跡できる。入力を skip した green test は独立根拠ではない。

### 3. 描画境界の確立

font 計測、出力単位、fallback 行/page layout、SVG/page layer、backend paint を model の外側に分ける。
保存済み原本行/page 指示と計算 layout を区別する。app facade は load・selection・editing・描画結果を調整する。
wrapper 名と不要な互換 method は実際の consumer に照らして整理する。

**完了条件:** コアが描画に依存せず renderer が raw input を decode しない。既存 text/source/model と
SVG/layer/native PDF、WASM/browser contract の回帰で移動した動作を保護する。

### 4. 検証済み profile の外へ拡張

識別力のある観測で残る解釈を解決し、独立文書と作成 version に広げる。一太郎 2026 baseline と歴史的適用を
分ける。flow・style・page 指示・note・object を拡張し、PDF 座標への fitting や見た目だけの意味確定を避ける。

**規則ごとの完了条件:** source 根拠、独立対照、棄却仮説、限界、回帰が適用範囲を説明する。
抽出・flow・描画精度は別に判定する。robustness と実行可能な malformed-input fuzzing も拡張に伴う。

### 5. 構造を保持する編集と保存

対応する JTD 構造を変更し unknown と関係を保持する。証明した source semantics に基づき、外部 adapter は
consumer と必要操作が決まってから加える。

**操作ごとの完了条件:** edit/save/reopen で意図した変更と無関係な source/unknown を検証する。
成功応答、fallback 段落変更、method の存在だけでは足りない。

## 依存と進捗

解釈分離後に最終描画境界を決める。local 再現検証は移行中にも進められる。根拠のある修正は全 refactor を
待たず進められるが同じ境界を守る。編集は外部 UI でなく証明した構造に続く。

実行/利用不可の検証、残る risk、revision を各段階で記録し、[移行 gate](ARCHITECTURE.ja.md#移行と回帰検証)
に従う。旧 M1–M6 は機能表と過去 backlog に保持し、この順序へ改番しない。
