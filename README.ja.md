# OpenJTD

一太郎文書（`.jtd`、`.jtt`、`.jttc`）向けのオープンソース JTD
レンダリングエンジン兼エディタプロジェクトです。

OpenJTD は、オープンソースの JTD レンダリングエンジン兼エディタになることを
目指しています。現在は `rjtd` という Rust ツール群を中心に、コンテナ調査、
テキスト抽出、文書モデル化、エクスポート、ビューア統合に必要な構成要素を作って
います。長期的な技術マイルストーンは、忠実なレイアウト描画と編集機能を支えられる
実用的な JTD エンジンを作ることです。

## 現在の rjtd コンポーネント

- `.jtd`、`.jtt`、`.jttc` ファイルの CFB/OLE コンテナ一覧化と、壊れた
  ファイルに対する緩いフォールバック処理。
- 観測済みの `/DocumentText` ストリームからのテキスト抽出。
- 観測済み `.jttc` の `JustCompressedDocument` と `-lh5-` ペイロード対応。
- 名前付き `/DocumentText` ストリームを持たないファイルに対する埋め込み
  `SsmgV.01` / `TextV.01` フラグメント復元。
- 最小限の Document Model から、プレーンテキスト、Markdown、JSON、
  テキスト指向 PDF を出力。
- `/DocumentTextPositionTables`、`/LineMark`、`/PageMark`、`/PaperMark`、
  オブジェクト/制御マーカー調査用の診断パーサー。
- 初期ビューア統合実験で使う WASM ラッパー。

## rjtd クイックスタート

```sh
cd rjtd
cargo test --workspace

cargo run -p rjtd-cli -- info path/to/document.jtd
cargo run -p rjtd-cli -- cat path/to/document.jtd
cargo run -p rjtd-cli -- export path/to/document.jtd --format md
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
- [`openjtd.github.io/`](openjtd.github.io/) - 将来のプロジェクトサイト。

## ドキュメント

- [`rjtd/README.md`](rjtd/README.md) は `rjtd` Rust ワークスペース、CLI、
  エクスポータ、診断コマンド群を説明します。
- [`openjtd-spec/README.md`](openjtd-spec/README.md) は仕様作業と RFC プロセスの
  索引です。
- [`docs/CHARTER.md`](docs/CHARTER.md)、[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)、
  [`docs/ROADMAP.md`](docs/ROADMAP.md) はプロジェクト方針を説明します。

## 設計上の参照

OpenJTD のリポジトリ構成とエンジン境界は、JTD 向けに調整しつつ `rhwp` の構造を
参考にしています。

## プロジェクト状況

OpenJTD は、リバースエンジニアリングと構成要素の整備段階です。まだ完全な JTD
レンダリングエンジンでもエディタでもなく、`rjtd` の API、データモデル、診断
コマンドは今後も変わる可能性があります。

観測済みファイルではテキスト抽出が動作しますが、段落セマンティクス、レイアウト
再現性、スタイル、表、ルビ、画像、ネイティブ編集挙動は未完成です。PDF と SVG
出力は、ネイティブレイアウトの再現ではなく、テキスト指向のフォールバック出力
として扱ってください。

## 翻訳

英語を既定のドキュメント言語とします。日本語訳は `*.ja.md` を使います。

## コントリビューションとセキュリティ

Apache-2.0 と DCO に基づくコントリビューション条件、クリーンルーム調査規則、
pull request の流れ、サンプルの来歴要件については
[CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。脆弱性の可能性は
[SECURITY.md](SECURITY.md) に従って非公開で報告し、公開 issue や pull request に
詳細を記載しないでください。

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
