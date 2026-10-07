# openjtd-spec

JTD 原本の bytes、record、関係、観測、仮説、検証範囲を記録する形式向け draft の索引である。
CLI、Rust model、JSON、描画近似、実装の test sweep は
[rjtd の研究記録](../rjtd/docs/research/README.ja.md) に分離した。
[解釈と描画の設計](../docs/ARCHITECTURE.ja.md) に境界と移行条件を記録する。

## 共同研究と local draft

共同仕様のレビューは [OpenJTD/spec](https://github.com/OpenJTD/spec) で行う。
RFC 0001/0003 の英日版は変更しない historical draft として移入済みであり、
[移入記録](https://github.com/OpenJTD/spec/blob/main/IMPORTS.md) が元 commit と hash を保持する。
今回の local 分離は、その共有コピーや合意 status を変更しない。

ここにある文書は公開準備 draft である。原本形式の主張、仮説、反例と実装動作を分けたが、
各主張の公開可能な入力 ID/hash、権利、作成 version、再現手順、独立結果を揃えてから共同承認を受ける。
local native 比較、synthetic 回帰、parser 成功、見た目の一致は互いの代用にならない。
本文中の根拠の限界を維持し、全形式・全 version の解読完了と見なさない。

## RFC 索引

RFC 0004 は実装 model/export の記録であり、形式向け RFC から外して番号を予約した。
他の番号は維持する。詳細な過去の観測・byte 例・反例・実装履歴は対応する研究記録に保存した。

| RFC | English | 日本語 | 状態 |
| --- | --- | --- | --- |
| 0001 | [JTD Container Inventory](rfc/0001-container.md) | [JTD Container Inventory](rfc/0001-container.ja.md) | 公開準備 draft |
| 0002 | [Historical Ichitaro Filter Metadata](rfc/0002-ichitaro-openoffice-filter.md) | [Historical Ichitaro Filter Metadata](rfc/0002-ichitaro-openoffice-filter.ja.md) | 公開準備 draft |
| 0003 | [DocumentText Content and Associated Records](rfc/0003-document-text.md) | [DocumentText Content and Associated Records](rfc/0003-document-text.ja.md) | 公開準備 draft |
| 0004 | [Implementation Model Export Record](rfc/0004-document-model-export.md) | [Implementation Model Export Record](rfc/0004-document-model-export.ja.md) | 実装記録へ移動・番号予約 |
| 0005 | [JTTC JustCompressedDocument Container](rfc/0005-jttc-just-compressed-document.md) | [JTTC JustCompressedDocument Container](rfc/0005-jttc-just-compressed-document.ja.md) | 公開準備 draft |
| 0006 | [DocumentText Position Tables and Bookmark Names](rfc/0006-document-text-position-tables.md) | [DocumentText Position Tables and Bookmark Names](rfc/0006-document-text-position-tables.ja.md) | 公開準備 draft |
| 0007 | [Layout Marks and Page Instructions](rfc/0007-layout-mark-streams.md) | [Layout Marks and Page Instructions](rfc/0007-layout-mark-streams.ja.md) | 公開準備 draft |
| 0008 | [Object Streams, Anchors, and Paint Candidates](rfc/0008-object-stream-candidates.md) | [Object Streams, Anchors, and Paint Candidates](rfc/0008-object-stream-candidates.ja.md) | 公開準備 draft |
| 0009 | [DocumentText Record Framing and Ruled-Line Flow](rfc/0009-document-text-paragraph-record.md) | [DocumentText Record Framing and Ruled-Line Flow](rfc/0009-document-text-paragraph-record.ja.md) | 公開準備 draft |

## ライセンス境界

別記がなければ著作した本文は root の [Apache License 2.0](../LICENSE) に従う。
外部 software、入力文書、引用、製品名にはそれぞれの条件があり、この文書で再ライセンスしない。
実装優先順位は [roadmap](../docs/ROADMAP.ja.md) に置く。
