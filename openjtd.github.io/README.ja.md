# openjtd.github.io

このディレクトリは `.jtd`、`.jtt`、`.jttc` 向けの OpenJTD 静的 WASM ビューアを
含む。ローカルファイルの選択・ドロップ、SVG ページ移動、plain-text タブに対応する。
文書の内容はブラウザ内で処理する。

ビューアは `rjtd-wasm::HwpDocument` を使い、モデルの fallback・診断用描画の制限を
引き継ぐ。完全なエディタではなく、一太郎の native layout の再現精度も保証しない。
[機能状況](../docs/FEATURE-STATUS.ja.md)、[検証](../docs/VALIDATION.ja.md)、
[roadmap](../docs/ROADMAP.ja.md) を参照する。

## Build and Deployment

[deployment workflow](../.github/workflows/deploy-viewer.yml) をビルド・資材コピー手順の
基準とする。`wasm-pack build --target web rjtd/crates/rjtd-wasm` でビルドし、生成した
package で `pkg/` を置き換え、distribution notices を含めて `main` から
<https://openjtd.github.io/rjtd/> へ公開する。workflow dispatch でも build は `main` に
限定される。GitHub Pages URL はリポジトリ移転後に redirect されないため、以前の
個人アカウントの viewer URL は使用しない。

`pkg/` は生成物である。ソースを checkout しただけでは、WASM package のビルド・コピー
が済むまでビューアを起動できない場合がある。過去の Cloudflare デプロイ実験は履歴であり、
現在の workflow は GitHub Pages を対象とする。公開先の稼働確認は deployment run を
別途確認する。

## Verification

Rust quality workflow は WASM target と viewer source contract を検査するが、実際の
ブラウザセッションは実行しない。実行時の回帰検証と描画制限表示は [TODO](../TODO.ja.md#描画と-app-境界) の未完了作業である。
元の M6 範囲は機能状況に残す。
