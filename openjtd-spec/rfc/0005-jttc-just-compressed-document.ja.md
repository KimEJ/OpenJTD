# RFC 0005: JTTC JustCompressedDocument Container

Status: draft

Observed: 2026-06-18

## Summary

観察済み `.jttc` files は CFB containers であり、document body は `/JSCompDocument` に保存される。

その stream は別の CFB document を wrap している。

```text
outer CFB
  -> /JSCompDocument
  -> JustCompressedDocument marker
  -> LHA -lh5- member
  -> inner CFB
  -> /DocumentText
```

current rjtd implementation は、この observed profile を新しい LHA/LZH dependency なしに直接 decode する。

## Relationship To rhwp Policy

rhwp には LHA/LZH/LH5 dependency がない。rjtd dependency policy の下では、便利さだけのために rjtd がそれを導入すべきではない。

したがって current support は observed `JustCompressedDocument` profile のための narrow direct implementation である。これは project rule と一致する。rhwp が dependencies を使うところでは rhwp dependencies を使い、rhwp が comparable low-level parsing を直接実装しているところでは direct implementation を使う。

## Outer CFB

観察済み template samples は小さな outer stream inventory を expose する。

`setsuden_05.jttc`:

```text
stream      336  /\x04JSRV_SegmentInformation
stream     2294  /\x04JSRV_SummaryInformation
stream      416  /\x05SummaryInformation
stream   989412  /JSCompDocument
```

`rjtd info` は outer file を次のように報告する。

```text
format                       cfb-just-compressed-document
document_text_bytes          -
compressed_document_bytes    989412
```

outer CFB は `/DocumentText` を直接 expose しない。

## JSCompDocument Layout

観察済み `/JSCompDocument` streams は次で始まる。

```text
2600 4a75 7374 436f 6d70 7265 7373 6564 446f 6375 6d65 6e74
```

これは `JustCompressedDocument` marker として解釈される。observed samples では、method `-lh5-` の LHA member が offset 38 から始まる。

Observed member metadata:

| Sample | `/JSCompDocument` bytes | LHA method | packed bytes | original bytes |
| --- | ---: | --- | ---: | ---: |
| `setsuden_05.jttc` | 989412 | `-lh5-` | 989292 | 1598976 |
| `setsuden_06.jttc` | 1182497 | `-lh5-` | 1182377 | 1913856 |

decompressed bytes は CFB magic で始まる。

```text
d0 cf 11 e0 a1 b1 1a e1
```

## Inner CFB

decompressed inner CFB は `/DocumentText` を含む。observed `setsuden_05.jttc` sample では、inner inventory は 65 streams を含み、`/DocumentText` は 564 bytes である。

current text extraction はその inner `/DocumentText` を読めるが、template samples は blank/control-heavy で non-empty model blocks を生成しない。

## Implemented Commands

```sh
cargo run -p rjtd-cli -- cat ../rjtd-testdata/local-samples/setsuden_05.jttc
cargo run -p rjtd-cli -- export ../rjtd-testdata/local-samples/setsuden_05.jttc --format json
```

JSON inner raw stream summary の抜粋。現在の export は圧縮 wrapper と補助 stream も保持する。

```json
{
  "blocks": [],
  "rawStreams": [
    { "name": "/DocumentText", "size": 564 }
  ]
}
```

## Known Gaps

- observed single-member `-lh5-` profile だけを support する。
- LHA header checksums と CRC values はまだ validate しない。
- 他の LHA methods は reject する。
- Multi-member archives は interpret しない。
- Inner CFB parsing は lenient FAT fallback を含む shared container reader を使う。

## Next Steps

- synthetic data を使い、minimal LH5 decoder の regression fixtures を追加する。
- metadata boundary が明確になったら、より多くの `JSCompDocument` metadata を document model に保存する。
- template/control-heavy content を blank text として扱わず、inner `DocumentText` stream の解釈を続ける。

## Model 読み込み用の共有 inner container

`DocumentTextPayload` は `/DocumentText` byte と分離して、すでに解凍した inner CFB
を公開する。model は line/page/paper mark、layout box、脚注、bookmark tag、auto text、
position table、object/frame にこの container を使う。従来は outer wrapper だけを探し、
inner mark が存在しても source-page 配置が fallback になった。outer named text の
既存優先順位は維持する。

元の `/JSCompDocument` byte を model raw stream に保持する。inner stream も outer と
同じ累積 stream count/byte budget に含め、元 input と LH5 output の制限は分離する。
補助読み取りのための追加解凍は行わない。既存の standalone style/font reader は自身の
shared-budget visit を引き続き計上し、現在の model の三 visit を reset しない。
生成した literal-LH5 fixture で exact limit と最初の超過 byte/count の拒否を検証する。

exporter は引き続き model を読む。inner container は解析 source であり、すべての
layout/object field の解読の証明ではない。
