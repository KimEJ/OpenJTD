# RFC 0008: Object Streams, Anchors, and Paint Candidates

Status: draft for publication preparation; joint review pending

Japanese translation: [0008-object-stream-candidates.ja.md](0008-object-stream-candidates.ja.md)

## Evidence Scope

Paths, image signatures, and `SO\0\0` markers identify candidate data; they
do not prove ownership, geometry, or paint order. Names include `/Frame`,
`/Figure`, `/LayoutBox`, `/EmbedItems`, `EmbeddingInfo`, `Contents`,
`EmbeddedPress`, `FDMIndex`, and `FDMVector`. Some ruled-line documents have
no named table object; their flow is investigated in RFC 0009.
The following associations are limited to controlled modern profiles.

## Single PNG Frame

One 76-byte Frame with a 60-byte record (ID 0/type 1) is associated with
`/EmbedItems/Embedding 1/Contents`, a complete PNG, and a complete source
object/cache group. Record offsets are relative to the 60-byte row:

| Offset | Controlled association |
| --- | --- |
| 16, BE-u16 | 0 inline, 1 floating |
| 28/32/36/40 | integer x/y/width/height fields, with zero adjacent low words |
| 44 | 200 corresponds to 2 mm text clearance in the observed case |
| 46 | 1 wrap, 4 front |

The frame dimensions, image aspect, and contiguous source context corroborate
this link. The class-zero/tag-`0x0030` source record uses `0x0107,0x0010`
inline or `0x0507,0x0012` floating contexts. A generic equal integer is not
sufficient object ownership evidence. Inline baseline, arbitrary alignment,
cropping, and other image sets remain unproven.

## Figure Order and Fill

In the controlled three-figure profile, Frame kind 4 at row offset 8 and the
one-based slot at offset 12 correspond to separate Figure ID ordering.
A rectangle/ellipse order swap corroborates that order independently of their
bounds. FDMIndex coordinates are axis pairs x1,x2,y1,y2 in this profile.

Observed outline markers include `01000660` rectangle, `01000460` ellipse,
and `01000160` line. A filled `01000a60` parent has a 46-byte prefix and a
nested outline child; BE-u32 at parent offset 36 has BGR fill color.
Known opaque AlphaBlend profiles accompany the no-transparency cases.
These observations do not establish arbitrary paths, global FDM world units,
all alpha modes, or editable figure semantics.

## Saved Equation Data

A `JSEQ.Document.3` embedding associates frame reference zero with matching
primary/trailing dimensions in the observed first-frame case. Other classes
need their own evidence. Its GCI snapshot contains font `0x94`, select/restore
`0x60`, background `0x40`, one-character TextOut `0xc8`, and release `0x65`
packets. Font values 370/240 in the observed size field correspond to normal
and smaller characters; TextOut supplies a saved top reference, not a decoded
font baseline. Ordered characters agree with editable JSEQ3 character/color
packets in the controlled equation.

This is evidence for saved character/style/coordinate relationships, not a
general equation AST or an extraction algorithm for every embedded format.

## Validation and Limits

Compare isolated anchor, wrapping, color, order, and equation edits with the
saved streams and native output. Record offsets relative to their owning
record, input versions, hashes, and permissions. Multiple images, rich/vertical
anchors, font substitution, generalized units, and equation editing remain
outside these bounded observations. Rendering algorithms and raster/vector
backend differences belong to implementation records.

## Implementation Record and History

[Commands, APIs, output behavior, and detailed historical research](../../rjtd/docs/research/0008-object-stream-candidates.md).
