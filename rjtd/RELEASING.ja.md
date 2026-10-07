# rjtd release 準備

[変更履歴](CHANGELOG.md) の2026-07-14版0.0.1とtag `rjtd-v0.0.1` を保持する。
開発変更は同じ0.0.1の再公開ではない。[最初のrelease手順](docs/research/legacy-release-0.0.1.md) は履歴として残す。

## version-aware preflight

[preflight](../scripts/release-preflight.sh) はlocked Cargo package IDから対象versionと各内部依存versionを読む。
crates.ioの対象version不存在と各正確依存versionのindex公開を確認する。既存package名の新versionは許可し、
既に公開済みの対象versionは拒否する。時点確認であり名前予約、owner確認、公開承認ではない。

archive内容を検査し、tokenなしの新規一時Cargo homeで `cargo publish --dry-run` だけを
`crates-io` に実行する。uploadは行わず `rjtd-testkit` は拒否する。dirty checkoutは拒否し、
`--allow-dirty` はlocal candidate調査だけに使う。一時home/targetは終了時に削除する。

repository rootから実行する。

```sh
bash scripts/release-preflight.sh --package rjtd-core
python3 scripts/ci-test-release-preflight.py
```

offline testはCargo/curl/gitを置換して新旧version、正確依存version、credential隔離、失敗時upload禁止を検証する。
後続releaseではscope/version、manifest、lockfileを合わせて実候補を検証する。ownerと最終公開は別操作である。

## branch と source

`dev` は統合、releaseは品質検査を通過したclean reviewed `main` commitから行う。
無関係checkout/ユーザーcommitを保持する。tag、日付付きchangelog、manifest、lockfile、upload内容を一致させる。
公開済みversionを上書きせずsource tagを移動しない。

## package 順序

公開packageはcore、model、export、WASM、CLI。`rjtd-testkit` は `publish = false` の内部package。
各依存の正確versionがindexedになるまでdependentの検証/uploadへ進まない。crate境界変更時はgraphを再確認する。

## candidate 検証

`rjtd/` からfmt/check/test/Clippy/doc/WASMをlockedで実行する。
[英文手順](RELEASING.md#candidate-verification) のcommandと品質workflow、LICENSE/package/notices/auditを確認する。
private/skipped入力を別記し、品質成功をnative layout完全一致の証明としない。dirty調査はclean release gateではない。

## 公開記録

最初のupload前にreview済みchangelogの日付とimmutable source tagを作り、全packageを同じcommitからuploadする。
曖昧な応答はregistry結果を確認してから再試行する。部分成功ではtagと成功範囲/blockerを保持し、修正は新versionにする。
credentialをcommand/file/logに残さない。owner変更、公開、tag、viewer deployは別の承認されたrelease操作である。
package page/doc/notices/deploy結果を実releaseで確認し、source fileだけから推測しない。
