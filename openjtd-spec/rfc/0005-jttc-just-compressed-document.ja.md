# RFC 0005: JTTC JustCompressedDocument Container

Status: draft for publication preparation; joint review pending

English source: [0005-jttc-just-compressed-document.md](0005-jttc-just-compressed-document.md)

## 観測した container profile

調査した JTTC は outer CFB の `/JSCompDocument` に本文 container を持ち、outer に
`/DocumentText` は直接存在しない。

```text
outer CFB -> /JSCompDocument -> JustCompressedDocument
          -> one LHA -lh5- member -> inner CFB -> /DocumentText
```

wrapper prefix は以下である。

```text
2600 4a75 7374 436f 6d70 7265 7373 6564 446f 6375 6d65 6e74
```

観測 profile の LHA member は stream byte 38 で始まり、
展開後は CFB signature `d0 cf 11 e0 a1 b1 1a e1` になる。記録例の packed/original 長は
989292/1598976 と 1182377/1913856 bytes。これらを固定値として一般化しない。

## 内側の文書

logical document stream は inner CFB にある。本文、style、font、layout mark、field、object を
その範囲で調べ、outer にないことを文書中の欠落と見なさない。wrapper、展開 CFB、logical stream
の座標を区別する。blank/control-heavy 本文も補助 stream の欠落を証明しない。

## 未確認の範囲と検証

他の LHA method、複数 member、header/checksum/CRC 全般、全 JTTC version は未確認である。
native JTT/JTTC save pair の inner inventory、正確な stream 内容、独立した展開結果を比較する。
作成 version、hash、権利、失敗、制限を記録する。decoder algorithm、資源 budget、回復方針、
reader の訪問回数は実装記録に置く。

## 実装記録と履歴

[実装の command・API・出力・過去の詳細記録](../../rjtd/docs/research/0005-jttc-just-compressed-document.ja.md).
