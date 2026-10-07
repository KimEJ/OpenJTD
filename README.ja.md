# rjtd

一太郎文書（`.jtd`、`.jtt`、`.jttc`）向けの独立した Rust JTD
レンダリングエンジン兼エディタプロジェクトです。

`rjtd` は OpenJTD の共同研究エコシステムに属する独立した実装です。
OpenJTD は JTD の研究・仕様・検証資料を共有する名称であり、rjtd と Tika JTD+ は
それぞれの実装目標を維持します。現在は Rust ツール群を中心に、コンテナ調査、
テキスト抽出、文書モデル化、エクスポート、ビューア統合に必要な構成要素を作って
います。長期的な技術マイルストーンは、忠実なレイアウト描画と編集機能を支えられる
実用的な JTD エンジンを作ることです。

共同研究は [OpenJTD/spec](https://github.com/OpenJTD/spec) と
[OpenJTD/corpus](https://github.com/OpenJTD/corpus) で管理します。
組織方針の議論に使う `community` は初期設定中は非公開とし、`corpus-private` には
共同研究者への共有が許可された資料のみを置きます。共同方針と manifest 形式は
共同レビュー用の草案です。本リポジトリの実装・リリース手順・ローカル研究記録は
独立して維持します。

## 現在の rjtd コンポーネント

- `.jtd`、`.jtt`、`.jttc` ファイルの CFB/OLE コンテナ一覧化と、壊れた
  ファイルに対する緩いフォールバック処理。
- 観測済みの `/DocumentText` ストリームからのテキスト抽出。
- 観測済み `.jttc` の `JustCompressedDocument` と `-lh5-` ペイロード対応。
- 名前付き `/DocumentText` ストリームを持たないファイルに対する埋め込み
  `SsmgV.01` / `TextV.01` フラグメント復元。
- テキスト中心の Document Model から、プレーンテキスト、Markdown、JSON、基本 HTML、
  限定的な診断用レイアウト投影を含む PDF を出力。
- `/DocumentTextPositionTables`、`/LineMark`、`/PageMark`、`/PaperMark`、
  オブジェクト/制御マーカー調査用の診断パーサー。
- WASM bindings と、ページ移動・テキストタブを持つ静的ブラウザビューア。

## OpenJTD が重要な理由

一太郎の独自形式である JTD、JTT、JTTC で作られた文書には、作成元の
ソフトウェアがなくても読み続ける必要があるものがあります。OpenJTD は
Apache-2.0 の Rust 実装（`rjtd`）と公開仕様メモを組み合わせ、形式調査と
互換性の作業を検証可能かつ再利用可能な形にします。これはデジタル保存、
アクセシビリティ、相互運用性に役立ちます。

このプロジェクトは、信頼できない文書に対して意図的に保守的です。`rjtd` は
観測済み/decoded の挙動と experimental research を区別し、可能な範囲で
unknown structures を保持します。また、パーサーのクラッシュ、ハング、壊れた
出力、過剰なリソース使用をセキュリティ上の懸念として扱います。まだ完全な
レンダリングエンジンやエディタではありません。現在の制限については
[プロジェクト状況](#プロジェクト状況) と [roadmap](docs/ROADMAP.ja.md) を参照してください。

## rjtd クイックスタート

```sh
cd rjtd
cargo test --workspace

cargo run -p rjtd-cli -- info path/to/document.jtd
cargo run -p rjtd-cli -- cat path/to/document.jtd
cargo run -p rjtd-cli -- export path/to/document.jtd --format md
cargo run -p rjtd-cli -- export path/to/document.jtd --format html
cargo run -p rjtd-cli -- export path/to/document.jtd --format json
cargo run -p rjtd-cli -- export path/to/document.jtd --format pdf -o output.pdf
```

visual regression checks に使う local sample PDF artifacts を更新するには、repository
root で次を実行します。

```sh
scripts/regenerate-pdf-output.sh
```

## リポジトリ構成

- [`rjtd/`](rjtd/) - 現在の OpenJTD 構成要素を作る Rust ツール群とワークスペース。
  コアエンジン、CLI、エクスポータ、WASM ラッパー、テスト補助を含みます。
- [`openjtd-spec/`](openjtd-spec/) - 公開仕様メモと RFC 記録。
- [`docs/`](docs/) - 憲章、アーキテクチャ、ロードマップ、調査ポリシー。
- [`openjtd-samples/`](openjtd-samples/) - 再配布可能なサンプル/出力成果物。
- [`rjtd-testdata/`](rjtd-testdata/) - テストフィクスチャ。
- [`openjtd.github.io/`](openjtd.github.io/) - 静的 WASM ビューアと GitHub Pages 用資材。

## ドキュメント

- [`rjtd/README.ja.md`](rjtd/README.ja.md) は `rjtd` Rust ワークスペース、CLI、
  エクスポータ、診断コマンド群を説明します。
- [`openjtd-spec/README.ja.md`](openjtd-spec/README.ja.md) は仕様作業と RFC プロセスの
  索引です。
- [`docs/CHARTER.ja.md`](docs/CHARTER.ja.md) は長期ビジョンと研究方針を定義します。
- [`docs/ARCHITECTURE.ja.md`](docs/ARCHITECTURE.ja.md) はエンジン層とモデルの境界を定義します。
- [`docs/ROADMAP.ja.md`](docs/ROADMAP.ja.md) は将来の順序と完了条件を管理します。
- [`機能状況`](docs/FEATURE-STATUS.ja.md) は現在の能力と M1–M6 を記録します。
- [`検証`](docs/VALIDATION.ja.md) は実行根拠と限界を記録します。
- [`TODO.ja.md`](TODO.ja.md) は実行する作業と全過去 backlog の参照を持ちます。
  診断完了は形式解読の証明ではありません。
- [`rjtd-testdata/README.ja.md`](rjtd-testdata/README.ja.md) は fixture の来歴と、
  portable な検査・ローカル参照検証の違いを説明します。

## 設計上の参照

model と動作は JTD 原本と再現可能な観測で決める。他の office 実装は問題解決に役立ち、
参照が許される場合に利用する。その構造、address、依存、API coverage を開発義務にしない。
解釈・描画・application の境界は [architecture](docs/ARCHITECTURE.ja.md) を参照する。

## プロジェクト状況

開発 tree は限定読取/描画と基本本文編集を持つが、一般 fidelity と構造保持 editing/save は未完成です。
公開済み release と開発 checkout は異なり得ます。[機能状況](docs/FEATURE-STATUS.ja.md) と
[検証](docs/VALIDATION.ja.md) で能力・根拠・差異を確認します。

次の順序は解釈コア分離、local 再現検証、描画境界、根拠に基づく規則拡張、構造保持編集/save です。
[roadmap](docs/ROADMAP.ja.md) を参照してください。

## 翻訳

英語を既定のドキュメント言語とします。日本語訳は `*.ja.md` を使います。韓国語作業版は既存規則に従う local ignored の `*.ko.md` であり、
local 更新は公開を意味しません。

## コントリビューションとセキュリティ

Apache-2.0 と DCO に基づくコントリビューション条件、クリーンルーム調査規則、
pull request の流れ、サンプルの来歴要件については
[CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。脆弱性の可能性は
[SECURITY.md](SECURITY.md) に従って非公開で報告し、公開 issue や pull request に
詳細を記載しないでください。

## サポート

コミュニティでの支援はベストエフォートです。個別にスコープを定める有償
エンジニアリングの相談は、[Upwork のメンテナー](https://www.upwork.com/freelancers/eojinkim)
に連絡してください。境界は [SUPPORT.md](SUPPORT.md) を参照し、脆弱性の報告は
[SECURITY.md](SECURITY.md) に従ってください。

## ライセンス

OpenJTD が著作したソースコードとドキュメントは
[Apache License, Version 2.0](LICENSE) の下で提供されます。

同梱のサンプル・テスト入力文書および第三者素材には、別個の権利または条件が
適用される場合があります。本ライセンスの案内は、それらの素材に対する権利を
許諾するものではありません。

生成物を配布できるのは、元となる入力資料の権利が許す場合に限られます。Apache-2.0
は、その生成物に表現された入力コンテンツに対する権利を許諾しません。「Ichitaro」、
「一太郎」、「JustSystems」などの第三者名は、文書形式または互換対象を特定するための
説明的な使用であり、各権利者との提携または支持を意味しません。ローカル参照資料との
境界については [THIRD_PARTY.md](THIRD_PARTY.md) を参照してください。
