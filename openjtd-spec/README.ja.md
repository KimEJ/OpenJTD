# openjtd-spec

JTD public specification と RFC 形式の reverse engineering records を管理する場所である。

共同仕様の作業は [OpenJTD/spec](https://github.com/OpenJTD/spec) で管理する。
初期試行として RFC 0001 と RFC 0003、および日本語訳を、内容を変更せず過去の草案として
移入した。[移入記録](https://github.com/OpenJTD/spec/blob/main/IMPORTS.md) を参照する。

このディレクトリには実装に付随する研究記録と未移入の RFC を残す。移入済み RFC の新しい
共同レビューは共有リポジトリで行う。過去のコピーを残すことは、その主張の共同承認や
独立再現を意味しない。

## ライセンス境界

別の条件が明記されない限り、このディレクトリ内で OpenJTD が著作した文章は、root の
[Apache License, Version 2.0](../LICENSE) の対象です。参照するソフトウェア、文書、
引用物、製品名は、それぞれの権利および条件に従います。このドキュメントはそれらを
ライセンスするものではありません。

## RFCs

英語原文と日本語訳を併せて管理する。各 RFC の status と根拠が適用範囲を示し、
RFC の存在は形式全体への対応を意味しない。現在の実装優先順位は
[roadmap](../docs/ROADMAP.ja.md) を参照する。

| RFC | English | 日本語 |
| --- | --- | --- |
| 0001 | [JTD Container Inventory](rfc/0001-container.md) | [JTD コンテナインベントリ](rfc/0001-container.ja.md) |
| 0002 | [Ichitaro OpenOffice Filter Reference](rfc/0002-ichitaro-openoffice-filter.md) | [一太郎 OpenOffice フィルター参照](rfc/0002-ichitaro-openoffice-filter.ja.md) |
| 0003 | [DocumentText Initial Text Extraction](rfc/0003-document-text.md) | [DocumentText 初期テキスト抽出](rfc/0003-document-text.ja.md) |
| 0004 | [Initial Document Model Export](rfc/0004-document-model-export.md) | [初期 Document Model Export](rfc/0004-document-model-export.ja.md) |
| 0005 | [JTTC JustCompressedDocument Container](rfc/0005-jttc-just-compressed-document.md) | [JTTC JustCompressedDocument コンテナ](rfc/0005-jttc-just-compressed-document.ja.md) |
| 0006 | [DocumentTextPositionTables Initial Mark Offsets](rfc/0006-document-text-position-tables.md) | [DocumentTextPositionTables 初期 Mark offset](rfc/0006-document-text-position-tables.ja.md) |
| 0007 | [Layout Mark Streams Initial Inventory](rfc/0007-layout-mark-streams.md) | [Layout Mark Streams 初期インベントリ](rfc/0007-layout-mark-streams.ja.md) |
| 0008 | [Object and Embedded Image Stream Candidates](rfc/0008-object-stream-candidates.md) | [Object and Embedded Image Stream Candidates](rfc/0008-object-stream-candidates.ja.md) |
| 0009 | [DocumentText Paragraph Record Structure](rfc/0009-document-text-paragraph-record.md) | [DocumentText 段落レコード構造](rfc/0009-document-text-paragraph-record.ja.md) |
