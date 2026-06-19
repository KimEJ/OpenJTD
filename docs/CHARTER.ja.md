# OpenJTD Project Charter

Open-source JTD Rendering Engine and Editor Project for Ichitaro Documents

Open Infrastructure for Ichitaro Documents

## Vision

OpenJTD は、日本のワードプロセッサ「一太郎 (Ichitaro)」で使われる JTD 文書形式の
オープンソース JTD レンダリングエンジン兼エディタプロジェクトである。

このプロジェクトは単なるファイル変換器ではない。

最終目標は、オープンソースの JTD レンダリングエンジン兼エディタを作ることである。
現在の実装の中心は、解析、parser、document modeling、export、viewer integration を
担う Rust ツール群 `rjtd` である。長期的な技術目標は、忠実なレイアウト描画と編集
機能を支えられる実用的な JTD エンジンを作ることである。

## Foundational Principle

### Follow rhwp

OpenJTD は可能な限り rhwp プロジェクトの構造と思想を参考にする。

rhwp は HWP/HWPX 文書のための現代的な Rust ベース文書エンジンである。

`rjtd` は rhwp の構造を JTD 領域の参考にする。

したがって、プロジェクト構造、layer 分離方式、data model 設計方式、test strategy はまず rhwp を参照する。

新しい構造を設計するより、検証済みの構造を再利用する。

## Relationship with rhwp

OpenJTD は rhwp の競合プロジェクトではない。

むしろ同じ思想を共有する姉妹プロジェクト (sister project) である。

```text
rhwp
 ├─ HWP
 ├─ HWPX
 └─ Hancom ecosystem

OpenJTD
 ├─ JTD
 ├─ JTDC
 └─ Ichitaro ecosystem
```

長期的には次のような共通エコシステムを検討する。

```text
Document Ecosystem
 ├─ rhwp
 ├─ OpenJTD
 ├─ common-document-model
 ├─ common-renderer
 ├─ common-exporter
 └─ common-viewer
```

今後の目標は次の通り。

- HWP ↔ JTD 共通 API
- 文書フォーマット共通 IR
- 共通 Viewer
- 共通 Exporter
- Apache Tika plugin

## Architecture Policy

`rjtd` engine は rhwp と同じく次の階層を維持する。

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
      ├──── Markdown Export
      ├──── HTML Export
      ├──── JSON Export
      └──── Future Renderer
```

すべての機能はこの階層を通して実装する。

特定の Exporter が元データを直接読んではならない。

必ず Document Model を経由する。

## Workspace Structure

最上位ワークスペースには、プロジェクト全体の計画と各サブプロジェクトをまとめて置く。

```text
openjtd-workspace/
├── docs
│   ├── CHARTER.md
│   ├── ARCHITECTURE.md
│   ├── ROADMAP.md
│   └── RHWP-COMPATIBILITY.md
├── rhwp
├── rjtd
├── openjtd-spec
├── openjtd-samples
├── rjtd-testdata
└── openjtd.github.io
```

最上位の `docs` は、プロジェクト憲章、エコシステム計画、rhwp 継承ポリシー、長期ロードマップを含む。

`rhwp` は、`rjtd` の構造、API 思想、test strategy を比較するための local external
reference clone である。

`rjtd` 以下には Rust ツール群とエンジン実装を置く。

## rjtd Engine Repository Structure

Rust エンジンリポジトリは rhwp の構造をできるだけ維持する。

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

現在使っていない crate もあらかじめ作成する。

これはプロジェクトの成長方向を固定するためである。

## Document Model First

rjtd の中核は Parser ではない。

Document Model である。

すべての Parser は Document Model を生成しなければならない。

すべての Exporter は Document Model を consume しなければならない。

```text
JTD
  ↓
Parser
  ↓
Document Model
  ↓
Exporter
```

## Unknown Preservation Rule

解析されていない data は絶対に破棄しない。

```text
UnknownRecord
UnknownBlock
UnknownStyle
UnknownObject
```

という形で保存する。

リバースエンジニアリング中の data loss を防ぐ。

## Reverse Engineering Policy

rjtd は clean-room reverse engineering を原則とする。

許可:

- ファイル分析
- binary structure analysis
- sample comparison
- 文書化

禁止:

- Ichitaro code のコピー
- 非公開 SDK の使用
- 著作権侵害

## Initial Milestones

### M1: Container Explorer

```text
rjtd streams sample.jtd
```

目標:

- CFB analysis
- Stream list の取得

### M2: Text Extraction

```text
rjtd cat sample.jtd
```

目標:

- text extraction

### M3: Document Model

```text
rjtd export sample.jtd --format json
```

目標:

- Paragraph
- TextRun
- Style

構造を生成する。

### M4: Markdown Export

```text
rjtd export sample.jtd --format md
```

### M5: Public Specification

別リポジトリを運用する。

```text
openjtd-spec
```

リバースエンジニアリング結果を RFC 形式で記録する。

`openjtd-spec` は `rjtd` code と同格のプロジェクトとして扱う。JTD のような closed
format では、後に仕様リポジトリが code より大きな資産になる可能性が高い。

## Long-Term Vision

OpenJTD は最終的に次の三つを提供する。

1. JTD Editor
2. JTD Engine and Rust Toolset
3. JTD Specification and Document Ecosystem

目標は「JTD を読めるライブラリ」ではなく、「JTD を理解できるオープンなエコシステム」を作ることである。

## GitHub Organization Model

初期 GitHub organization には次の構造を推奨する。現在のワークスペース最上位ディレクトリも、この構成をあらかじめ反映している。

```text
openjtd/
├── docs
├── rjtd
├── openjtd-spec
├── openjtd-samples
├── rjtd-testdata
└── openjtd.github.io
```

特に `openjtd-spec` を `rjtd` code と同格のプロジェクトとして扱う原則を organization
structure にも反映する。
