# RFC 0003: DocumentText Content and Associated Records

Status: draft for publication preparation; joint review pending

Japanese translation: [0003-document-text.ja.md](0003-document-text.ja.md)

## Scope and Evidence

This draft records observed wire forms and bounded interpretations. Early
local observations and controlled Ichitaro 2026 cases have different coverage;
neither establishes support for all historical versions. Public input IDs,
hashes, authoring steps, and independent results must be attached to each
claim before shared acceptance. A renderer's successful output is not that
independent verification.

## Named Text and Inline Framing

Observed named streams start with `SsmgV.01` at byte 0 and `TextV.01` at byte 20.
A BE-u32 at byte 28 gives a UTF-16-unit content length. Content starts at byte
32 and ends at `32 + 2 * length`, before the byte-oriented style events.
Visible text may start immediately at byte 32 without a `0x001f` marker.
The complete 32-bit count matters; a low header word is not a proven format selector.

Marker-based fragments also occur, including outside a named stream.
Their physical-file offsets must not be equated with logical-stream offsets.
UTF-16BE text and `0x001d ... 0x001e` inline caches are observed. Visible base
text, phonetic annotations, cached field values, and template instructions
must remain distinguishable. Ruby examples associate selector `3` with base
text and `0x0082` with kana annotation in the controlled grouped profile;
other inline forms require their own framing and interpretation.

`/LayoutBoxText` also contains bounded `TextV.01` content. Printable words
inside a control-only payload are not evidence of visible text or ownership.

## Character Property Associations

The style-event wire grammar is in RFC 0009. Controlled source ranges support
these associations; unlisted combinations and control-only ranges remain unknown:

| Property | Observed association and scope |
| --- | --- |
| `2` | BE-u16 font size in 0.01 mm: 370 approximately 10.5 pt, 494 approximately 14 pt; zero restores the default in the observed profile |
| `3` | Font-directory ID; `0xffff` restores the observed default; ID zero is valid |
| `15` | Text-range `0x00BBGGRR` color, including `0xffffffff` automatic/default; not a universal role over table controls |
| `20 = 0x84000000` | Bold selection |
| `20 = 0x90000000` | Italic selection |
| `20 = 0x80000010`, `13 = 1` | Single underline selection |
| `20 = 0x80000c00` / `0x80000400`, `4 = 5 = 50` | Upper/lower quarter menu profiles with half-size character dimensions |

The sequential `/DocumentViewStyles` `0x1006` payload has the observed default
size at bytes 3–4 and default font ID at bytes 5–6. This association requires
its known complete payload form. Duplicate font IDs, reserved values, mixed
ranges, and unknown property widths do not establish a selection.
Selected font names are source data; substitute fonts, synthetic bold/shear,
script baselines, and PDF text painting are implementation behavior.

## Saved Fields, Notes, and TOC

Controlled class-zero field records have lengths/tags 28/`0x0033` for printing
date, 15/`0x0035` for page number, and 12/`0x0048` for external link. Complete
contiguous argument/value wrappers distinguish cached text from expressions.
The observed arguments include `DATE`, `PAGENUMBER`, and an HTTP(S) target.
Saved text does not define a general field evaluator or renumbering rule.

`/Footnote` and `/FootnoteLink` associate a note body and marker with a body
marker in the single-link case. Property-1 references 1/2 and matching
`TextLayoutStyle` slots corroborate marker typography for that case.
They do not establish a general inheritance decoder, note-area print enable,
page-end/document-end choice, separator geometry, or note-area baseline.

Saved TOC regions have paired class-`0x0020` setting records. The observed
17-unit title and 18-unit leader contexts have distinct following empty-cache
variants: 5 after title, 3 after leader. Raw leader values 1/100 correspond to
solid/dotted selection; a third variant has no leader record. Stored title,
label, and source ranges do not decode heading levels or TOC regeneration.

## Direction, Spacing, and Bookmarks

Controlled sequential view margin/direction profiles differ at `0x1002`:
a 32-byte horizontal payload has `0x40` at byte 10; the 33-byte vertical
payload has `0x50,0x01` at bytes 10/11. Other fields and the complete profile
must agree. Edit/caret state is not a direction discriminator.
The `0x1006` tail `0x0266` paired with its observed `0x100b` form corresponds
to the UI's 60% character spacing. A general numeric formula is not decoded.
The no-fit two-digit tatechuyoko record uses class zero/tag `0x0047` and
complete paired caches; ordinary three-digit content is a counterexample to
promoting arbitrary digits to that form.

A controlled 16-word paragraph header carries pitch value 1000, associated
with 10 mm. It is distinct from the repeated-pitch table/body profile.
Source font properties can cover leading cell spaces differently from the
visible label. Their glyph advance is not determined by the font value alone.

`/MarkTag` names and unresolved position tables are described in RFC 0006.

## Remaining Questions

Resolve general Japanese tracking/line distribution, font/style inheritance,
field expression types, note placement, ruby units, and bookmark coordinates.
Keep source data distinct from fallback measurements and output approximations.

## Implementation Record and History

[Commands, APIs, output behavior, and detailed historical research](../../rjtd/docs/research/0003-document-text.md).
