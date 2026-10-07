# rjtd の研究・実装記録

CLI、Rust API、model、JSON、描画近似、回帰、資源制限を実装側で管理する。
各記録の先頭は現在の範囲、後半は分離前の詳細な混合記録である。過去の能力・規定・仮説を
現行仕様と見なさない。原本観測と反例は削除せず、固定 Git commit への参照も保持する。
[形式向け draft](../../../openjtd-spec/README.ja.md) と
[解釈・描画の設計](../../../docs/ARCHITECTURE.ja.md) を参照する。

共有 RFC 0001/0003 の import 履歴は変更しない。新しい共有主張には別途根拠とレビューが必要である。

| 元の番号 | English record | 日本語記録 |
| --- | --- | --- |
| 0001 | [JTD Container Inventory](0001-container.md) | [JTD Container Inventory](0001-container.ja.md) |
| 0002 | [Historical Ichitaro Filter Metadata](0002-ichitaro-openoffice-filter.md) | [Historical Ichitaro Filter Metadata](0002-ichitaro-openoffice-filter.ja.md) |
| 0003 | [DocumentText Content and Associated Records](0003-document-text.md) | [DocumentText Content and Associated Records](0003-document-text.ja.md) |
| 0004 | [Implementation Model Export Record](0004-document-model-export.md) | [Implementation Model Export Record](0004-document-model-export.ja.md) |
| 0005 | [JTTC JustCompressedDocument Container](0005-jttc-just-compressed-document.md) | [JTTC JustCompressedDocument Container](0005-jttc-just-compressed-document.ja.md) |
| 0006 | [DocumentText Position Tables and Bookmark Names](0006-document-text-position-tables.md) | [DocumentText Position Tables and Bookmark Names](0006-document-text-position-tables.ja.md) |
| 0007 | [Layout Marks and Page Instructions](0007-layout-mark-streams.md) | [Layout Marks and Page Instructions](0007-layout-mark-streams.ja.md) |
| 0008 | [Object Streams, Anchors, and Paint Candidates](0008-object-stream-candidates.md) | [Object Streams, Anchors, and Paint Candidates](0008-object-stream-candidates.ja.md) |
| 0009 | [DocumentText Record Framing and Ruled-Line Flow](0009-document-text-paragraph-record.md) | [DocumentText Record Framing and Ruled-Line Flow](0009-document-text-paragraph-record.ja.md) |

公開比較には利用許可のある入力と十分な再現手順が必要である。local 個人資料の filename や過去の
sweep 数は public fixture grant ではない。元の混合記録は調査の履歴として読み、公開用の形式主張には
必要な根拠だけを権利と適用範囲付きで抽出する。

## 過去の参照記録

[過去の外部参照記録](legacy-office-reference.ja.md). 旧参照・互換方針と当時の観測を保持する。過去資料であり現行開発指示ではない。

## 開発案内と過去記録

- [CLI 診断](../CLI-DIAGNOSTICS.ja.md): 現行調査 command と出力の限界。
- [全過去 backlog](legacy-backlog.ja.md): M1–M6 の実験、未解決項目、反例。現在の順序は root TODO。
- [実装開発記録](legacy-implementation-overview.ja.md): workspace README から移した詳細な経緯。
- [初回 release 手順](legacy-release-0.0.1.md): 現行 release 準備とは別の歴史資料。

現在の能力は [機能状況](../../../docs/FEATURE-STATUS.ja.md)、根拠は [検証](../../../docs/VALIDATION.ja.md)。
文書移動は過去の数値・指示を最新結果へ更新しない。
