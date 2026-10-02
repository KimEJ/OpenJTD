# OpenJTD Roadmap

OpenJTD の長期目標は、忠実な JTD レンダリングエンジンとエディタである。
現在はリバースエンジニアリングと構成要素の開発段階にある。次の成果目標は、
横書き本文と単純な表を持つ 1 ページ文書という限定した範囲で、読み取りと描画を
再現可能にすることである。

この文書はマイルストーン概要と次の優先順位を管理する。
[憲章](CHARTER.ja.md) は長期方針、[TODO.md](../TODO.ja.md) は詳細タスクと研究履歴、
[openjtd-spec](../openjtd-spec/README.ja.md) は形式の根拠を記録する。

## Reading Status

- **Implemented（実装済み）** は記載した範囲のコード経路が存在することを示す。
  すべての一太郎バージョンや native layout への対応を意味しない。
- **Diagnostic / fallback（診断・代替表示）** は根拠や近似表示があることを示す。
  ビューアに表示されても、`decoded:false`、`Candidate`、`reference-backed` は
  意味の確定を制限する条件として維持する。
- **Verified（検証済み）** の結果には入力集合と実施した検査を明記する。
  ローカルサンプルテストのスキップや PDF 生成だけでは描画精度の証明にならない。
- **Planned（計画）** は未完了の作業であり、以下の受け入れ条件で完了を判断する。

## M1: Container Explorer

観測済み CFB/OLE `.jtd`、`.jtt`、`.jttc` に対して `rjtd streams` と `rjtd info`
を実装済み。壊れた FAT の lenient fallback も含む。
これはコンテナアクセスの実装であり、文書形式全体への対応を意味しない。

## M2: Text Extraction

観測済み `/DocumentText`、ルビの親文字、埋め込み `SsmgV.01` / `TextV.01`
フラグメント、JTTC の `JustCompressedDocument` / `-lh5-` profile に対応済み。
`rjtd cat` がこの経路を利用する。完全な段落・制御コード・スタイルの意味には
未解読部分が残る。

## M3: Document Model

段落、text run、ルビ、source span、未知データの保存を持つテキスト中心のモデルを
実装済み。表、スタイル、画像、レイアウトの候補は研究根拠を保持するが、完全な
意味モデルではない。

SVG と native PDF 出力、基本的な本文編集、検索、選択、snapshot は存在する。
描画は fallback text layout と限定的な source/reference-backed projection を
組み合わせている。一般的なレイアウト再現、構造を保持した編集、JTD への保存は未完成。

## M4: Markdown Export

最小限のモデルベース Markdown と、plain text、JSON、基本 HTML 出力を実装済み。
HTML は段落テキストとルビのマークアップを保持するが、完全なレイアウトは再現しない。
見出し、リスト、表、スタイルの意味には追加のモデル解読が必要である。
rich HTML clipboard API は別の未完成な app-core surface である。

## M5: Public Specification

継続中。[RFC 0001–0009](../openjtd-spec/README.ja.md) はコンテナ、テキスト、圧縮、
レイアウト、オブジェクト、段落レコードの観測を英語・日本語で記録する。
これらは段階的な研究記録であり、完全な仕様ではない。
新しい解読の主張には裏付けとなるサンプルと保存された反例が必要である。

## M6: WASM Viewer

[静的ビューア](../openjtd.github.io/README.ja.md) はファイル選択・ドロップ、ページ移動、
SVG 表示、plain-text タブを実装済み。文書はブラウザ内で処理する。
リポジトリのデプロイ workflow は `main` からビルドし、GitHub Pages を対象とする。
過去の Cloudflare 実験は現在の構成ではない。

ブラウザ binding の広さは完全な編集対応を意味しない。ビューアはモデルと描画の制限を
引き継ぐ。ブラウザ実行時の回帰検証と再現精度の制限を示す表示は次の作業である。
公開先の稼働状況は workflow の存在とは別に確認する必要がある。

## Next Priorities

### 再現精度の到達点: 2026-10-02

現在の実装は `TextV.01` 先頭のテキストを保持し、名前付き content を style tail の前で
区切り、補助文字の UTF-16 source range を保持する。対応する source style から font size と
各辺の page margin を読み、明示的な page-layout margin を document-view default より優先する。

先頭ページの横書き control table には、source 由来の cell-text 配置と基本的な SVG/PDF border
projection がある。ローカル根拠は単一 table の 20 variants と、interstitial text・末尾の空行を
含む 2-table 文書 1 件を対象とする。別の完全な Frame/LayoutBox grid profile は 6 個の cell label を
保持する。これらは `decoded:false` のままで、厳密な font metrics、border paint style、一般的な
wrap、merged/sparse cell、一般的な page assignment は未完成である。詳細と除外例は
[RFC 0008](../openjtd-spec/rfc/0008-object-stream-candidates.ja.md) と
[RFC 0009](../openjtd-spec/rfc/0009-document-text-paragraph-record.ja.md) に記録する。

大きな table と pagination は、既存の reference pair を使って実装を続ける課題である。
例えばローカルの食品分類表は、1-page reference に対してまだ 26 fallback pages を生成する。
縦書き・JTT/JTTC の reference pair、元の font 環境、統制した画像・図形 sample は検証範囲を
広げるために有用だが、不足していても既存の再現失敗への作業は続けられる。
どの形式についても完全な再現精度は主張しない。

### 完了条件に向けた作業

1. **精度検証を再現可能にする。** 来歴を確認した fixture manifest と再配布可能な
   最小 corpus を整備する。実行済み、スキップ、既知の不一致を区別し、テキスト、
   ページ数・向き、表構造、座標の期待値を記録する。既存の統制実験では native 作成と
   import 経由の代替サンプルを区別する。別の checkout でも未追跡の非公開ファイルなしに
   検証を再現できることを完了条件とする。
2. **限定した描画範囲を完成させる。** 単純な横書き 1 ページ文書について、段落・行境界、
   ページ原点、列幅、行高、テキスト位置を解読する。reference PDF の座標や sample 固有の
   補正なしに独立した文書にも同じ規則が適用できることを完了条件とする。
   棄却した仮説と未確定の候補は診断専用のまま保持する。
3. **証明した規則を拡張する。** 複数ページの流れとスタイル、続いて縦書き、画像・
   オブジェクトの所有関係と描画順序を追加する。各拡張には複数サンプルの回帰根拠が必要。
   不正入力向けの実行可能な fuzz target も追加する。現在の `rjtd/fuzz/` は placeholder
   だけである。
4. **構造を保持した編集と保存を実装する。** 対応する編集能力を明示し、変更時にも未知
   データを保持する。編集・保存・再読込の検証を完了条件とする。完全な rhwp Studio parity、
   共通形式 IR、HWP/HWPX 出力は長期作業に位置づける。

## Verification Boundary

[Rust quality workflow](../.github/workflows/rust-quality.yml) は formatting、check、
test、Clippy、documentation、WASM、package、notice、dependency audit、MSRV を検査する。
これらは各検査範囲の品質を示すもので、native layout との同等性を示すものではない。

一部のローカルサンプルテストは原本・参照ファイルがなければ早期 return し、別のテストは
`--ignored` が必要である。生成 PDF もローカル成果物である。
[fixture guide](../rjtd-testdata/README.ja.md) に検証の区分を記載する。
既知のページ数・向きの不一致は、解読した規則が fallback を置き換えるまで明示する。
