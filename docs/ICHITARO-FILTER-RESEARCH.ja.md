# 過去の一太郎フィルター調査

OpenOffice 一太郎 import filter は、個別の相互運用問題に役立つ場合の任意の歴史資料である。
現在の baseline 研究は一太郎 2026 で作った文書の対照実験を使う。外部実装は JTD の意味や
必須の設計先例を定めない。

local `.oxt` は `third-party/ichitaro-filter/` にある。artifact の識別と metadata は
[RFC 0002](../openjtd-spec/rfc/0002-ichitaro-openoffice-filter.ja.md) に記録する。

## 現行実装の境界

[CONTRIBUTING.md](../CONTRIBUTING.md) と artifact の元条件に従う。既存の参照範囲は metadata、
filter/type 登録、package inventory、可視文字列、独立に観測できる application 動作である。
DLL の実装 logic を copy/reconstruct しない。深い解析には許される範囲の別途確定が必要であり、
今回の文書整理は範囲を拡張しない。共同規定 draft や議論は記録した承認と個別条件の代わりにならない。

hint、仮説、独立再現した JTD 観測、出力近似を区別する。過去の実装は形式の権威ではない。
