# rjtd

OpenJTD の Rust ツール群と document-engine workspace

## Role

`rjtd` は OpenJTD の Rust ツール群である。日本のワードプロセッサ「一太郎 (Ichitaro)」
で使われる JTD 文書形式を解析・処理し、現在の parser、model、export、CLI、WASM、
app-core integration components を提供する。

このフォルダは `OpenJTD/rjtd` の実装 workspace である。OpenJTD は共同研究の名称であり、
rjtd と Tika JTD+ は独立した実装を維持する。共同 RFC review は
[OpenJTD/spec](https://github.com/OpenJTD/spec) で行う。

プロジェクト全体の憲章とエコシステム計画は、上位の [docs/CHARTER.ja.md](../docs/CHARTER.ja.md) に従う。

## 0.0.1 Developer Preview

日付付き 0.0.1 release は experimental developer preview である。公開 package は `rjtd-core`、`rjtd-model`、`rjtd-export`、`rjtd-cli`、
`rjtd-wasm` で、`rjtd-testkit` は内部専用である。現在の checkout は後続の未 release 開発を含み得る。
release scope は [CHANGELOG.md](CHANGELOG.md)、必須の公開順序は
[RELEASING.md](RELEASING.md) を参照する。

観察済み `.jtd`、`.jtt`、`.jttc` files は異なる範囲で対応しているが、
実装は完全な Ichitaro format specification ではない。`Candidate`、
`Unknown`、`Diagnostic` と名付けた値、および `decoded: false` の JSON
fields は、final semantics を主張せず reverse-engineering evidence を保持する。
public API と command output schema は後続の 0.0.x release で変更され得る。

## JTD 固有のエンジンと設計参考

解釈と model は JTD 原本と再現可能な観測で決める。他の office 実装は、具体的な問題に
役立ち、参照が許される場合の任意資料である。外部の model、依存選択、API parity を完了条件にしない。
[architecture](../docs/ARCHITECTURE.ja.md) を参照する。

## Architecture Policy

`rjtd` は原本解析、model の所有、出力を次の階層に分離する。

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
      ├──── Plain Text / Markdown Export
      ├──── HTML Export
      ├──── JSON Export
      └──── App Core / SVG / PDF Export
```

すべての機能はこの階層を通じて実装する。特定の Exporter が元データを直接読んではならない。必ず Document Model を経由する。

`rjtd-model::DocumentCore` は読込、ページ照会、SVG/HTML 描画、layer diagnostics、
基本的な本文編集・検索・選択・clipboard・snapshot を提供する。解析できた箇所の
source byte/unit span を保持する。`rjtd-wasm::JtdDocument` は既存 viewer の browser API を
公開する。名称変更や不要な互換 method の整理は、consumer と generated binding を確認する
別のコード作業である。API の数は JTD 機能の完了条件ではない。

高度な書式、表・セル、オブジェクト、header/footer、note の多くのメソッドは、既定値、
no-hit、no-op を返す。設定メソッドの一部は decoded JTD settings を保存せず成功を返す。
これらの互換 surface は native 編集や保存への対応を意味しない。HWP/HWPX 出力は空の
byte 列を返し、`exportHwpVerify` は変換が未実装であることを報告する。

基本的な文書 HTML 出力は `rjtd-export` と CLI に実装され、段落テキストとルビ markup
を含む。app-core の rich HTML clipboard は限定的で、本文選択は escape 済みの段落
テキストを出力できる。現在のマイルストーンと完了条件は [roadmap](../docs/ROADMAP.ja.md)
を参照する。

## Document Model First

rjtd の中核は Parser ではない。Document Model である。

すべての Parser は Document Model を生成しなければならない。すべての Exporter は Document Model を consume しなければならない。

## Unknown Preservation Rule

解析されていない data は絶対に破棄しない。

```text
UnknownRecord
UnknownBlock
UnknownStyle
UnknownObject
```

リバースエンジニアリング中の data loss を防ぐ。

## Default Resource Limits

public parser surface は 64 MiB を超える source input を拒否する。LH5 member
は 256 MiB を超える declared output を拒否し、1 MiB の allowance を超える
output は packed member size の 256 倍以内でなければならない。browser canvas
rendering は各辺 16,384 pixels、合計 64 MiPixels に制限される。

これらは pre-stable safety ceiling であり compatibility guarantee ではない。
limits-aware な文書読込では、stream、record、embedded image、page の構築で同じ
`ParseLimits` resource budget を共有する。既定値は CFB の異なる stream path 1,024 件と
合計計上サイズ 64 MiB、保持する frame/embedding record 65,536 件と 64 MiB、画像
1,024 件と payload/envelope 合計 64 MiB、ページ 65,536 件と保持する page line 1 Mi 件。
画像 header 由来の幅・高さは各 16,384、累積 pixel 数は 64 MiPixels に制限する。

Strict CFB は宣言サイズ、lenient recovery は sector chain から到達可能な byte 数を
計上する。同じ path は 1 回だけ数え、実際の読込量を計上値と照合する。画像の寸法は
payload/envelope を clone する前に PNG、GIF、BMP、JPEG の header から取得する。
このモデル読込経路では bitmap を decode・保持しないため、後段の decoded-image
allocation を制限するという主張ではない。信頼できない文書は適切に制限した環境で処理し、
bypass の可能性は上位の [security policy](../SECURITY.md) から報告する。

## Workspace Layout

```text
rjtd/
├── crates
│   ├── rjtd-core
│   ├── rjtd-model
│   ├── rjtd-export
│   ├── rjtd-cli
│   ├── rjtd-wasm
│   └── rjtd-testkit
├── docs
├── samples
├── fuzz
├── tests
└── tools
```

6 つの workspace crate はそれぞれ実装または fixture support の役割を持つ。
crate test は `crates/`、CLI と研究文書は `docs/` にある。他の directory の存在だけで
実行可能 fuzz/test program が実装済みとは判断しない。

## Commands

```sh
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## CLI と開発文書

command の説明は [CLI 診断](docs/CLI-DIAGNOSTICS.ja.md) に分けた。この directory からの主な入口:

```sh
cargo run -p rjtd-cli -- --help
cargo run -p rjtd-cli -- info path/to/document.jtd
cargo run -p rjtd-cli -- cat path/to/document.jtd
cargo run -p rjtd-cli -- export path/to/document.jtd --format json
cargo run -p rjtd-cli -- export path/to/document.jtd --format pdf -o output.pdf
```

現在の範囲は [機能状況](../docs/FEATURE-STATUS.ja.md)、実行検証は [検証](../docs/VALIDATION.ja.md)、
次の順序は [roadmap](../docs/ROADMAP.ja.md)、具体作業は [TODO](../TODO.ja.md)、
詳細研究は [研究記録](docs/research/README.ja.md) を参照する。
