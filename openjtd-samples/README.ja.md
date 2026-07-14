# openjtd-samples

OpenJTD の公開可能な JTD sample documents と output artifacts を管理する場所である。

OpenJTD の一部として公開できる files だけを含める。

## 権利の境界

sample またはそこから生成した artifact を commit または公開する前に、出所と再配布の
許可を確認し、該当する notice を保持する。Apache-2.0 は、sample の content や、その
content を表現する生成 output に対する権利を単独では許諾しない。

## PDF output artifacts

`pdf-output/` には local sample set から生成した PDF を置く。repository root で次の
command を実行すると再生成できる。

```sh
scripts/regenerate-pdf-output.sh
```

この script は `rjtd-testdata/local-samples/` の `.jtd`、`.jtt`、`.jttc` files を読み、
同じ stem の PDF を `pdf-output/` に書き出す。
