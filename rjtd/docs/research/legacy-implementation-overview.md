# Recorded Implementation Development

Historical development descriptions moved from the workspace README. They are
not the current roadmap or a fresh validation run. See [feature status](../../../docs/FEATURE-STATUS.md)
and [validation](../../../docs/VALIDATION.md). Original relative links were rebased.

Bounded character runs now retain bold/italic/single-underline candidates, half-size upper/lower scripts, color/size changes, and source font-ID selections with default resets. SVG/PDF and the viewer reuse backend font advances between runs. Flags, font mapping, synthetic paint, script placement, wrapping, and native whitespace retain their documented candidate/fallback limits; see [implementation record 0003](0003-document-text.md#bounded-character-style-and-font-candidates). PDF synthetic bold retains one selectable text copy; its stroke uses glyph outlines from the same resolved font. SVG/viewer paint and model evidence are unchanged. This avoids duplicate PDF text extraction without changing candidate bold strength. Generic PDF families select installed faces, preserving macOS preferences and available Linux fallbacks.

Auxiliary footnote/link/bookmark-tag streams are preserved raw. The bounded single-link footnote profile also exposes source-linked note text candidates in JSON; body text, note placement, and field evaluation remain separate.

Named text now survives explicit page breaks, and model inline spans cover visible UTF-16 units while raw wrappers remain preserved. Bounded heading/list caches reuse physical source lines and fixed84 page ranges. Other dynamic field profiles and general native typography remain unresolved.

Bounded saved TOC regions now preserve title/page-label metadata and use physical page ranges without consuming text height for their setting records. General tab stops, generation, and editable TOC semantics remain undecoded; the controlled saved leader profile is described below.

Bounded printing-date/page/link caches are now modeled with source bindings. Printing dates use an explicit render context (local browser/Unix PDF date by default); raw caches remain unchanged. External-link color/underline and HTTP(S) SVG targets follow the bound record. General renumbering, bookmark positions, and other field profiles remain unresolved.

The JTTC model now reuses the decoded inner CFB for auxiliary layout marks, note/bookmark data, and object/frame streams while retaining the compressed source and shared resource limits. JTT/JTTC source-page placement follows the same model path.

The controlled single-style portrait/landscape/portrait profile now selects layout per page across SVG, page/layer information, PDF and canvas dimensions. Unknown style associations and differing margin/paper profiles keep fallback; other page typography and general section editing remain unresolved.

Plain `/Header` slots are preserved raw and exposed as source-linked text candidates. The bounded global horizontal profile now renders headers, footers, facing-page variants, cover suppression, and the enabled centered page-number pattern in SVG/PDF and layer information. Slot roles, nominal anchors, font metrics, and general numbering remain candidates; see [implementation record 0007](0007-layout-mark-streams.md#bounded-plain-running-regions).

The controlled modern view profile now selects horizontal/vertical writing and its 30mm source margins. Plain vertical text reuses source columns and PageMark pitch; the known 60% spacing profile and two-digit no-fit tatechuyoko caches render through the same SVG/PDF and layer projection. Visible digits are recovered while raw controls and unknown caches remain preserved. Other profiles, native font metrics, Latin/space advances, and vertical glyph substitutions retain their candidate/fallback limits; see [implementation record 0003](0003-document-text.md#bounded-modern-vertical-text).

The controlled single-PNG profile now binds frame/cache/image data and renders source position/size, inline insertion, two-sided clearance and front paint in SVG/PDF and layer output. Raw data and occluded text remain preserved. Inline baseline and fallback-font wrapping still differ; multiple-image, rich/vertical and other profiles remain diagnostic. See [implementation record 0008](0008-object-stream-candidates.md#bounded-single-png-placement).

The controlled rectangle/ellipse/line profile now binds source blank-line anchors, frame geometry, FDM commands, fill colors and independently corroborated paint order. SVG/PDF and layer output share that projection while raw figure streams and decoded-false evidence remain preserved. General units, transparency/connector profiles and editable figure semantics remain unresolved; see [implementation record 0008](0008-object-stream-candidates.md#bounded-native-figure-paint).

The controlled JSEQ/GCI text snapshot now retains the zero-based first frame association and renders source-bound equation characters, italic/script sizes and the reserved body row through SVG/PDF and layer output. The editable formula and snapshot character packets must agree. Font baselines and general equation/editing semantics remain candidates; see [implementation record 0008](0008-object-stream-candidates.md#bounded-gci-equation-text).

The controlled global landscape 60% character-spacing profile now uses physical source rows and spaced Japanese runs while keeping Latin advances backend-owned. SVG/PDF and layer output retain source spans and decoded-false geometry. Other tracking profiles and exact whitespace metrics remain unresolved.

The controlled saved TOC profile now distinguishes solid/dotted leaders from adjacent page labels. Complete title/leader records and empty caches retain one separator cell, body-right labels, physical source rows and backend text advances in SVG/PDF and layer data. Leader metrics and general tab/navigation/editing semantics remain decoded-false candidates; see [implementation record 0003](0003-document-text.md#bounded-saved-toc-leaders).

Linked standard footnote markers now retain their separate body/note source spans and the corroborated parent-style references. The bounded horizontal body marker uses half-size upper placement; its literal two-paragraph profile keeps one source line gap. Parsed ruby bases retain source spans and font advances, with bounded grouped-kana spacing. SVG/PDF/layer geometry remains candidate data. Note-area placement and general style/ruby inheritance remain unresolved; see [implementation record 0003](0003-document-text.md#bounded-linked-footnote-marker-and-ruby-source).

The controlled mixed-orientation middle page now inherits its corroborated 60% Japanese spacing from the selected page style. The known 4006/400a profile requires the same font size, 100% scales and validated style/page/margin association. Portrait neighbors retain their prior rendering, while SVG/PDF/layer data names the spacing basis. Other page typography and exact font/whitespace metrics remain unresolved.

The controlled plain-paragraph fixed 10mm profile now keeps its physical wrapped rows and source-bound advance through SVG/PDF and layer output. Its single saved pitch field applies only to that paragraph; unknown profiles, lost bindings and vertical/grid mixtures retain fallback. Raw text and PageMark data stay unchanged; glyph metrics remain candidates.

The admitted control-grid padding now follows the leading spaces’ own source font size, independently of each visible label. SVG/PDF and layer positions share the bounded relative-size adjustment; raw spaces/source ranges stay preserved, with separate padding provenance. Mixed or unsupported scaling profiles retain fallback.

Bounded bookmark names from `/MarkTag` are now exposed as model/JSON source candidates, preserving directory values and byte ranges. The original position table is retained through plain and compressed container paths. Positions, navigation and editing remain undecoded; no heuristic offset is presented as a bookmark coordinate.
