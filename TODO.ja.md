# 開発作業

実行順は [roadmap](docs/ROADMAP.ja.md)、現在の能力と M1–M6 は [機能状況](docs/FEATURE-STATUS.ja.md)、
計測は [検証](docs/VALIDATION.ja.md) を参照する。[以前の全 backlog](rjtd/docs/research/legacy-backlog.ja.md) に
完了実験・未解決調査・反例を保持した。元の open 項目は固有の根拠で完了するまで未解決である。

## 解釈コア

- [x] 混在した source 認識、描画、app state を分類する。
- [x] 認識結果を model に置き、flow/span/raw/unknown/資源上限を保持する。
- [x] font/renderer 初期化なしの解析・model 調査を検証する。

実装整理は model/source、optional rendering、app 内部 module の境界であり独立 renderer crate は作成していない。
source-only consumerと生成SDK契約を検証し、公開互換methodはconsumer review後も保持する。
未確定の形式/編集研究は未完了として残す。

## 再現可能な解釈検証

- [ ] 既存 test/診断を使う許可済み最小 fixture を選ぶ。
- [ ] 作成経路/version、識別、command、revision、期待観測、反例を記録する。
- [ ] 別 checkout で検証し private/欠落/skip/未対応を分けて報告する。

## 描画と app 境界

- [x] 保存原本行/page 指示と fallback 計算を分ける。
- [x] font 計測、出力単位、SVG/page layer、paint を model の外側へ移す。
- [x] HwpDocument 名、generated binding、viewer caller、未使用互換 method を一緒に確認する。
- [x] 移動中の source/model、出力回帰、WASM/browser contract を保持する。

## 根拠と coverage

- [ ] 脚注 print/placement/separator field を識別できる根拠で解読する。
- [ ] 日本語 tracking/distribution、bookmark normalization、inline baseline を解決する。
- [ ] 独立文書/version へ広げ、過去 pagination/orientation 反例を保持する。
- [ ] 実行可能 fuzz target と必要な resource-boundary 回帰を追加する。

## 編集と release 準備

- [ ] 構造保持 edit/save/reopen と unknown 保持を操作ごとに検証する。
- [ ] rich clipboard/selection/hit test を確立した JTD 意味と対応付ける。
- [x] 次 version の前に 0.0.1/未割当名前提の release tool を更新する。

文書整理とコード完了は別である。共同規定/schema review と他 repository への移入は今回の local 作業外とする。
