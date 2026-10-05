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

### JTD 固有のエンジン

OpenJTD の model、描画、編集、原本保存は JTD の原本根拠に従う。他のエンジンから
適用できる layer 分離・保存・検証の pattern を参照しても、その形式固有の model を
前提にはしない。

## Relationship with rhwp

rhwp は HWP/HWPX 文書向けの独立した Rust エンジンである。OpenJTD は JTD/JTT/JTTC
エンジンを開発する。rhwp は設計参考と任意の連携先であり、必須の document model や
エディタ API 全体の契約ではない。

```text
rhwp
 ├─ HWP
 ├─ HWPX
 └─ Hancom ecosystem

OpenJTD
 ├─ JTD
 ├─ JTT / JTTC
 └─ Ichitaro ecosystem
```

外部エディタ UI、ビューア、exporter、検索用ツールは、具体的な利用者が必要とする時に
小さな adapter を通じて対応済みの JTD 機能を使える。JTD の編集・保存・再読込は
エンジンの責任として残る。共通 IR、HWP/HWPX 変換、完全な Studio parity は忠実な JTD
エンジンの前提ではない。[rhwp の連携範囲](RHWP-COMPATIBILITY.ja.md) が参照と adapter の
境界を定義する。

## Architecture Policy

`rjtd` engine は解析、model の所有、出力を次の階層に分離する。この境界は、外部エディタの
API 形状にかかわらず JTD の原本保存を守る。

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

最上位の `docs` は、プロジェクト憲章、architecture、rhwp の参照・連携方針、長期 roadmap を含む。

`rhwp` は、`rjtd` の構造、API 思想、test strategy を比較するための local external
reference clone である。

`rjtd` 以下には Rust ツール群とエンジン実装を置く。

## rjtd Engine Repository Structure

Rust workspace は parser、model、exporter、CLI、browser binding の責任を分離し、
JTD の実装要件を支える。

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

実装した責任に独立した境界が必要になった時だけ新しい crate を追加する。
参照プロジェクトの将来の module 配置を再現するためには追加しない。

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

以下は創設時のマイルストーンである。現在の状態、追加の WASM ビューア milestone、
次の完了条件は [ROADMAP.ja.md](ROADMAP.ja.md) で管理する。

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

OpenJTD は共同研究の名称である。共同で検証できる知識と独立した実装を、次の
リポジトリに分けて管理する。

```text
OpenJTD/
├── community       # 共同方針と議論。初期は非公開
├── spec            # 公開 RFC・観察・検証基準
├── corpus          # 再配布可能な入力と公開 manifest
├── corpus-private  # 共同研究者への共有が許可された入力
└── rjtd            # 独立した Rust 実装
```

rjtd と Tika JTD+ は、それぞれ独立した実装目標を維持する。共有する形式知識は、
どちらの engine にも依存せず役立つものとする。共同で著作した資料は Apache-2.0 を
既定とし、外部文書は元の条件を維持して個別に出所と共有根拠を記録する。

初期試行として RFC 0001 と RFC 0003、および日本語訳を
[spec](https://github.com/OpenJTD/spec) に移入する。共同研究方針・根拠ラベル・corpus
形式は共同レビュー用の草案である。ローカル研究記録・fixtures・viewer source は
本実装リポジトリに残し、許可のないローカル入力を共有 corpus に移さない。
