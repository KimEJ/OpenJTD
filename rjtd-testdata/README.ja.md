# rjtd-testdata

rjtd implementation verification のための fixture、expected output、regression test data を管理する場所である。

original sample documents と派生した expected results は分離して追跡する。

`local-samples/` は local manual checks に使う個人用ファイルを置くための場所である。
commit する対象は、再配布可能な fixtures と派生した expected results に限定する。

## 権利の境界

fixture または expected result を commit する前に、出所と再配布の許可を確認し、該当する
notice を保持する。root の Apache-2.0 は、test-input の content や、それを表現する生成
result に対する権利を単独では許諾しない。local samples は、公開を許す権利が確認できる
まで local に留める。
