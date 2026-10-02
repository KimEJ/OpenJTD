# rjtd-testdata

rjtd implementation verification のための fixture、expected output、regression test data を管理する場所である。

original sample documents と派生した expected results は分離して追跡する。

`local-samples/` は local manual checks に使う個人用ファイルを置くための場所である。
commit する対象は、再配布可能な fixtures と派生した expected results に限定する。

## Verification Tiers

以下は `rjtd/` から実行する。

```sh
# 入力がある場合のローカルテストも含めた workspace 検証。
cargo test --workspace --locked --no-fail-fast

# 名前に local_ を含むテストを除いた subset。
cargo test --workspace --locked --no-fail-fast -- --skip local_

# 統制実験の明示的な検証。ローカル source-y corpus が必要。
cargo test -p rjtd-cli --test source_y_probe --locked -- --ignored
```

合成 fixture は Rust テスト内で構築する。多くの実文書テストは未追跡の
`local-samples/` に依存し、原本や参照 PDF がない場合は早期 return する。
明示的に ignore されたテストもある。そのため `ok` は必ずしもサンプルの assertion が
実行されたことを意味しない。結果を報告するときはコマンド、利用できた入力集合、
スキップしたケースを記録する。クラウドの placeholder は実行前にダウンロードする。

source-y probe テストは `local-samples/ichitaro-source-y-probe/` の manifest、
`corpus/baseline-sweep/`、`corpus/page01-grid/` を必要とする。ローカルの来歴メモでは
native 作成文書と RTF-import surrogate を区別している。解読規則を確定するときも
この区別を維持する。

PDF 生成検査は構造上の妥当性を確認する。再現精度の検証には信頼できる参照 PDF が必要で、
artifact テストには `openjtd-samples/pdf-output/` の生成物も必要である。exporter テストは
既知のページ数・向きの不一致を根拠として保持する。その差を許容してもレイアウトの同等性を
意味しない。再配布可能な最小 corpus と実行済み・スキップの明示的な報告は
[計画中の作業](../docs/ROADMAP.ja.md#next-priorities) である。

## 権利の境界

fixture または expected result を commit する前に、出所と再配布の許可を確認し、該当する
notice を保持する。root の Apache-2.0 は、test-input の content や、それを表現する生成
result に対する権利を単独では許諾しない。local samples は、公開を許す権利が確認できる
まで local に留める。
