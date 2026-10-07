# 実装開発の過去記録

workspace README から移した過去の実装説明。現在の計画や新しい検証ではない。
[機能状況](../../../docs/FEATURE-STATUS.ja.md) と [検証](../../../docs/VALIDATION.ja.md) を参照する。
元の相対 link は調整した。

限定的な文字範囲の太字・斜体・一本下線候補、半サイズの上付き・下付き、色・サイズ変更、font ID と既定値復帰を保持する。SVG/PDF と viewer は backend の文字送りを run 間で再利用する。flag、font mapping、synthetic paint、添字位置、折返し、native 空白の candidate/fallback 制限は [RFC 0003](../../../openjtd-spec/rfc/0003-document-text.ja.md) を参照。 PDF の synthetic bold は選択可能な文字を一度だけ記録し、同じ解決済み font の glyph outline で stroke を描く。SVG/viewer の paint と model evidence は維持する。候補の太字強度を変えず、PDF text extraction の重複を防ぐ。 PDF の generic family は実在する font を選び、macOS の優先 font と利用可能な Linux fallback を使う。

補助の脚注・リンク・bookmark tag stream を raw のまま保持する。限定した一 link の脚注 profile は、source に対応した note text 候補を JSON にも公開する。本文、脚注配置、field 評価は分離して扱う。

named text の明示改ページ後も本文を保持し、model inline span は表示 UTF-16 unit を指す。raw wrapper は保全する。限定した見出し・一覧 cache は物理 source 行と fixed84 page 範囲を再利用する。別の動的 field profile と一般 native typography は未解決。

限定した保存済み目次領域の title/page label を保全し、設定 record に本文行高を割り当てず物理 page 範囲を再利用する。一般 tab stop、生成と編集可能な目次 semantics は未解読。統制した保存済み leader profile は後述する。

限定した印刷日・page・link cache と source 対応を保持する。印刷日は明示 render context（browser/Unix PDF の local date を既定で使用）で描画し、raw cache を変更しない。外部 link の色・下線と HTTP(S) SVG target は対応 record に従う。一般的な再番号付け、bookmark 位置、別 field profile は未解決。

JTTC model は解凍した inner CFB を補助 layout mark・note/bookmark・object/frame stream に再利用し、圧縮 source と共有 resource limit を保全する。JTT/JTTC の source-page 配置は同じ model 経路に従う。

統制した単一書式の縦長・横長・縦長 profile は、SVG・page/layer info・PDF・canvas 寸法で page ごとの layout を選ぶ。未知の書式対応や異なる余白・紙 profile は fallback を保ち、他 page typography・一般 section 編集は未解決。

plain `/Header` slot を raw のまま保存し、source に対応する text 候補を公開する。限定した global 横書き profile の header・footer・見開きの切替・表紙非表示・有効な中央 page-number pattern を SVG/PDF と layer info に描画する。slot role、名目 anchor、font metrics、一般的な番号付けは候補のまま。[RFC 0007](../../../openjtd-spec/rfc/0007-layout-mark-streams.ja.md) を参照。

統制した modern view profile で横書き・縦書きと source の30mm余白を選ぶ。plain 縦書きは source の列と PageMark pitch を再利用し、既知の字間60% profile と二桁・幅に収めない縦中横 cache を SVG/PDF と layer の共通投影で描画する。数字を本文に復元し、raw control と unknown cache を保全する。他 profile、native font metrics、英字・空白の送り、縦用 glyph 置換は candidate/fallback の制限を残す。[RFC 0003](../../../openjtd-spec/rfc/0003-document-text.ja.md) を参照。

統制した単一 PNG profile の frame/cache/image を関連付け、source 位置・寸法、inline 挿入、両側回避、前面描画を SVG/PDF と layer に反映する。raw data と隠れた本文を保全する。inline baseline と代替 font の回避に差が残り、複数画像・rich/縦書き・別 profile は diagnostic のまま。[RFC 0008](../../../openjtd-spec/rfc/0008-object-stream-candidates.ja.md) を参照。

統制した四角形・楕円・直線 profile は source 空白行 anchor、frame geometry、FDM command、fill color、独立に照合した paint order を関連付ける。SVG/PDF と layer は投影を共有し、raw 図形 stream と decoded-false evidence を保全する。一般単位、透明・connector profile、編集可能な図形 semantics は未解決。[RFC 0008](../../../openjtd-spec/rfc/0008-object-stream-candidates.ja.md) を参照。

統制した JSEQ/GCI text snapshot は zero-based の最初の frame を関連付け、source 数式文字・italic/添字サイズ・確保した本文行を SVG/PDF と layer に描画する。editable formula と snapshot の文字 packet 一致を要求する。font baseline と一般数式・編集 semantics は候補のまま。[RFC 0008](../../../openjtd-spec/rfc/0008-object-stream-candidates.ja.md) を参照。

統制した global 横長・字間60% profile は、物理 source 行と和文の字間を再利用し、英字 advance は backend に委ねる。SVG/PDF と layer は source span と decoded-false geometry を保つ。他 tracking profile と正確な空白 metrics は未解決。

統制した保存済み目次 profile は、実線・点線 leader と題名に隣接する page label を区別する。完全な title/leader record と空 cache に限定し、一つの区切り cell、本文右端の label、物理 source 行、backend の文字送りを SVG/PDF と layer に反映する。leader metrics と一般 tab/navigation/edit semantics は decoded-false 候補のまま。[RFC 0003](../../../openjtd-spec/rfc/0003-document-text.ja.md) を参照。

標準脚注 marker は本文・脚注の個別 source span と照合済み parent-style 参照を保持する。限定した横書き本文 marker を半サイズの上付きに置き、literal な二段落 profile は一つの source 行間を維持する。parsed ruby base の source span・font advance を保持し、限定した grouped kana 間隔を描画する。SVG/PDF/layer geometry は候補のまま。脚注領域配置と一般 style/ruby 継承は未解明。[RFC 0003](../../../openjtd-spec/rfc/0003-document-text.ja.md) を参照。

統制した方向混在の中間 page は、選択した page style の照合済み字間60%を和文に反映する。既知4006/400a profile で同じ font size・100% scale・検証したstyle/page/margin対応を要求する。前後の縦長pageは従来描画を維持し、SVG/PDF/layer は字間basisを公開する。他page typographyと正確なfont/空白metricsは未解明。

統制した plain 段落の固定10mm profile は、物理的な折返し行と source に対応する改行幅を SVG/PDF・layer に反映する。単独の保存済み pitch はその段落だけに適用し、未知 profile、source 対応の喪失、縦書き・grid 混在は fallback を維持する。raw 本文・PageMark を変更せず、glyph metrics は候補のまま。

採用済み control grid の padding は、表示 label と独立に先頭空白自身の source font size を使う。SVG/PDF・layer の位置は限定した相対 size 調整を共有し、raw 空白・source 範囲と個別の padding provenance を保持する。混在・未対応の scale profile は fallback を保つ。

限定した `/MarkTag` の bookmark 名を model/JSON の source 候補として公開し、directory 値と byte 範囲を保持する。元の position table は通常・圧縮 container のどちらでも raw を保存する。位置・navigation・編集は未解明のままで、heuristic offset を bookmark 座標として示さない。

## 現在の限定 profile

67 native pair・85 page の local 検証範囲と残る問題は [roadmap](../../../docs/ROADMAP.ja.md) にまとめる。
実装の詳細は [研究記録](README.ja.md) を参照する。

- 文字 style/font：bold、italic、underline、半分の上/下付き、色/size、font ID と default 復帰。
  合成 bold の PDF searchable text は一つに保ち、generic family は利用可能 font を選ぶ。
- 原本 flow/paragraph：保存 source 範囲、物理行/page、余白、indent、固定 pitch、空白の独立した font 範囲。
- 保存 field/TOC：印刷日 context、page/link cache、title、solid/dotted/leader なし label。一般生成/編集ではない。
- running region/page：plain header/footer、odd/even、cover 抑制、既知 page-number ON、混在方向。
- 縦書き/ruby：原本 column と既知 60% spacing、二桁 no-fit 縦中横、source span と group kana 候補。
- object：単一 PNG frame、wrap/front、rectangle/ellipse/line と色・順序、保存 JSEQ/GCI 数式文字。
- JTT/JTTC：inner container の補助 stream を同じ model 経路で利用する。

全 profile の geometry と未確定の意味は candidate のままである。脚注領域、日本語一般字間、bookmark
座標、inline baseline、正確な font metrics、一般 editing/save は残る限界として扱う。
