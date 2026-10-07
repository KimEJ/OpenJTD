# rjtd research 0005: Compressed-container loading

Status: implementation record; not a shared format RFC

Japanese translation: [0005-jttc-just-compressed-document.ja.md](0005-jttc-just-compressed-document.ja.md)

## Implementation Scope

The observed LH5 decoder and inner-container reuse live in parsing. Auxiliary streams share resource accounting; original wrapper preservation and input/decompression limits remain required. Decoder implementation, unsupported archive methods, and generated literal-LH5 regressions are implementation records. Exporters consume the loaded model.

The [format-facing record](../../../openjtd-spec/rfc/0005-jttc-just-compressed-document.md) retains source
observations and unresolved interpretations. Rust types, candidate admission,
JSON fields, CLI output, paint approximations, test coverage, and resource
policies are implementation behavior. They do not establish native semantics.

## Reproduction Entry Points

From the repository root, build the CLI with `cargo build --manifest-path rjtd/Cargo.toml -p rjtd-cli`. Use an input whose analysis and disclosure are
permitted. Inspect `rjtd/target/debug/rjtd --help` for each command's required arguments.

```sh
rjtd/target/debug/rjtd streams path/to/document.jtd
rjtd/target/debug/rjtd document-info path/to/document.jtd
```

Some commands require a page number, stream, format, or other argument.
Record the actual full command, input ID/hash, authoring version, implementation
commit, executed/skipped checks, and observed limitations with any result.
Private-corpus checks do not become public reproducibility by being listed here.

## Historical Evidence

The [complete pre-separation record](https://github.com/OpenJTD/rjtd/blob/cce9cb20611c8c809a7376c8df6130aed14a0437/openjtd-spec/rfc/0005-jttc-just-compressed-document.md) preserves the original raw
examples, commands, local sweeps, rejected hypotheses, and implementation
history at a fixed commit. It mixes source observations with implementation
behavior and includes superseded capability statements; it is a historical
source, not the current format contract or an executable public fixture pack.
The public shared imports of RFC 0001/0003 retain their own import commits and
status; this local edit does not change them.

## Separation Work

Apply the [interpretation/rendering boundary](../../../docs/ARCHITECTURE.md)
before moving mixed functions. Preserve text, source spans, unknown data,
candidate labels, resource limits, and output behavior in focused regressions.
No Rust modules were moved by this documentation change.

## Preserved Pre-Separation Record

The following original mixed research record retains its detailed observations,
raw examples, experiments, counterexamples, and implementation history. Its
capability, policy, and hypothesis statements describe their historical stage;
read them together with the current scope above. Relative links were rebased.

Status: draft

Observed: 2026-06-18

Japanese translation: [0005-jttc-just-compressed-document.ja.md](0005-jttc-just-compressed-document.ja.md)

## Summary

Observed `.jttc` files are CFB containers whose document body is stored in `/JSCompDocument`.

That stream wraps another CFB document:

```text
outer CFB
  -> /JSCompDocument
  -> JustCompressedDocument marker
  -> LHA -lh5- member
  -> inner CFB
  -> /DocumentText
```

The current rjtd implementation decodes this observed profile directly, without adding a new LHA/LZH dependency.

## Relationship To rhwp Policy

rhwp has no LHA/LZH/LH5 dependency. Under the rjtd dependency policy, that means rjtd should not introduce one only for convenience.

The current support is therefore a narrow direct implementation for the observed `JustCompressedDocument` profile, matching the project rule: use rhwp dependencies where rhwp uses dependencies, and direct implementation where rhwp directly implements comparable low-level parsing.

## Outer CFB

Observed template samples expose a small outer stream inventory.

`setsuden_05.jttc`:

```text
stream      336  /\x04JSRV_SegmentInformation
stream     2294  /\x04JSRV_SummaryInformation
stream      416  /\x05SummaryInformation
stream   989412  /JSCompDocument
```

`rjtd info` reports the outer file as:

```text
format                       cfb-just-compressed-document
document_text_bytes          -
compressed_document_bytes    989412
```

The outer CFB does not expose `/DocumentText` directly.

## JSCompDocument Layout

Observed `/JSCompDocument` streams begin with:

```text
2600 4a75 7374 436f 6d70 7265 7373 6564 446f 6375 6d65 6e74
```

This is interpreted as a `JustCompressedDocument` marker. In the observed samples, an LHA member with method `-lh5-` starts at offset 38.

Observed member metadata:

| Sample | `/JSCompDocument` bytes | LHA method | packed bytes | original bytes |
| --- | ---: | --- | ---: | ---: |
| `setsuden_05.jttc` | 989412 | `-lh5-` | 989292 | 1598976 |
| `setsuden_06.jttc` | 1182497 | `-lh5-` | 1182377 | 1913856 |

The decompressed bytes start with the CFB magic:

```text
d0 cf 11 e0 a1 b1 1a e1
```

## Inner CFB

The decompressed inner CFB contains `/DocumentText`. In the observed `setsuden_05.jttc` sample, the inner inventory contains 65 streams and `/DocumentText` is 564 bytes.

Current text extraction can read that inner `/DocumentText`, but the template samples are blank/control-heavy and produce no non-empty model blocks.

## Implemented Commands

```sh
cargo run -p rjtd-cli -- cat ../rjtd-testdata/local-samples/setsuden_05.jttc
cargo run -p rjtd-cli -- export ../rjtd-testdata/local-samples/setsuden_05.jttc --format json
```

An excerpt of the JSON inner raw stream summary (the current export also retains the compressed wrapper and auxiliary streams):

```json
{
  "blocks": [],
  "rawStreams": [
    { "name": "/DocumentText", "size": 564 }
  ]
}
```

## Known Gaps

- Only the observed single-member `-lh5-` profile is supported.
- LHA header checksums and CRC values are not validated yet.
- Other LHA methods are rejected.
- Multi-member archives are not interpreted.
- Inner CFB parsing uses the shared container reader, including the lenient FAT fallback.

## Next Steps

- Generated literal-LH5 decoder/container regression fixtures are now implemented; broader archive methods remain unsupported.
- Preserve more `JSCompDocument` metadata in the document model once the metadata boundary is clearer.
- Continue interpreting the inner `DocumentText` stream instead of treating template/control-heavy content as blank text.

## Shared Inner Container for Model Loading

`DocumentTextPayload` now exposes the already-decoded inner CFB separately from
its `/DocumentText` bytes. The model reuses this container for line/page/paper
marks, layout boxes, footnotes, bookmark tags, auto text, position tables, and
object/frame data. These streams previously looked only in the outer wrapper,
causing source-page placement to fall back even when the inner marks existed.
Named outer text retains its existing priority.

The original `/JSCompDocument` bytes are retained in model raw streams. Inner
streams join the same cumulative stream-count and stream-byte budgets as outer
streams; original-input and LH5 output limits remain separate. This adds no
new decompression pass for auxiliary loading. Existing standalone style/font
readers still charge their own shared-budget visits, so the model's current
three visits are preserved rather than silently reset. Generated literal-LH5
fixtures verify exact limits and rejection of their first exceeded byte/count.

Exporters continue to consume model data. The inner container is a parsing
source, not decoded proof of every layout or object field.
