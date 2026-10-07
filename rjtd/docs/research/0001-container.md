# rjtd research 0001: Container readers and recovery

Status: implementation record; not a shared format RFC

Japanese translation: [0001-container.ja.md](0001-container.ja.md)

## Implementation Scope

`rjtd-core/src/container.rs`; standard CFB access and the bounded lenient fallback. CLI inventory formatting and successful-open sweep counts describe this implementation, not normative JTD requirements.

The [format-facing record](../../../openjtd-spec/rfc/0001-container.md) retains source
observations and unresolved interpretations. Rust types, candidate admission,
JSON fields, CLI output, paint approximations, test coverage, and resource
policies are implementation behavior. They do not establish native semantics.

## Reproduction Entry Points

From the repository root, build the CLI with `cargo build --manifest-path rjtd/Cargo.toml -p rjtd-cli`. Use an input whose analysis and disclosure are
permitted. Inspect `rjtd/target/debug/rjtd --help` for each command's required arguments.

```sh
rjtd/target/debug/rjtd streams path/to/document.jtd
rjtd/target/debug/rjtd info path/to/document.jtd
```

Some commands require a page number, stream, format, or other argument.
Record the actual full command, input ID/hash, authoring version, implementation
commit, executed/skipped checks, and observed limitations with any result.
Private-corpus checks do not become public reproducibility by being listed here.

## Historical Evidence

The [complete pre-separation record](https://github.com/OpenJTD/rjtd/blob/cce9cb20611c8c809a7376c8df6130aed14a0437/openjtd-spec/rfc/0001-container.md) preserves the original raw
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

Japanese translation: [0001-container.ja.md](0001-container.ja.md)

## Summary

The initial local JTD samples are Compound File Binary (CFB) documents.

M1 implements only container inventory. It does not interpret stream payloads.

The container layer first uses the `cfb` crate, matching rhwp's dependency choice for OLE/CFB files. If a CFB file has malformed FAT data that the standard reader rejects, rjtd falls back to a narrow lenient reader modeled after rhwp's `LenientCfbReader` approach.

## Command

```sh
cd rjtd
cargo run -p rjtd-cli -- streams ../rjtd-testdata/local-samples/a5.jtd
```

The output format is tab-separated:

```text
<entry-kind>    <byte-size>    <path>
```

`entry-kind` is `storage` or `stream`.

ASCII control characters in stream names are escaped for display, for example `\x04JSRV_SegmentInformation`.

## Lenient FAT Fallback

Some local `.jtd` samples contain FAT inconsistencies such as duplicate sector pointers. rhwp handles comparable HWP files with a direct `LenientCfbReader` implementation rather than adding another dependency.

rjtd follows that pattern:

- standard `cfb` parsing is attempted first;
- lenient parsing is used only after standard parsing fails on a CFB-looking file;
- stream inventory and stream reads share the same fallback;
- missing streams remain `not found` errors after a successful lenient open.

Current local sweep:

| Command | Local samples checked | Result |
| --- | ---: | --- |
| `rjtd info` | 61 | 61 opened |
| `rjtd cat` | 61 | 61 opened; 2 use embedded `SsmgV.01` fragments instead of named `/DocumentText` |

## Local Samples

The sample names below refer to local files used during early container
inventory work. They are examples for comparing observed stream layouts.

| Sample | Entry count | Notes |
| --- | ---: | --- |
| `a5.jtd` | 32 | Has `LineMark`, `PageMark`, `PaperMark`, and `PageLayoutStyleHeader` |
| `46.jtd` | 31 | Has `LineMark`, `PageMark`, and `PaperMark` |
| `b6.jtd` | 31 | Has `LineMark`, `PageMark`, and `PaperMark` |
| `shinsyo.jtd` | 28 | Does not have `LineMark`, `PageMark`, or `PaperMark` in the initial inventory |
| `a6.jtd` | 28 | Does not have `LineMark`, `PageMark`, or `PaperMark` in the initial inventory |

## Common Top-Level Streams

These top-level streams appear in all five local samples:

- `/\x04JSRV_SegmentInformation`
- `/\x04JSRV_SummaryInformation`
- `/\x05SummaryInformation`
- `/AutoTextInfo`
- `/DocumentEditStyles`
- `/DocumentPeripheralThree`
- `/DocumentPeripheralTwo`
- `/DocumentText`
- `/DocumentTextPositionTables`
- `/DocumentViewStyles`
- `/Font`
- `/Footnote`
- `/Header`
- `/MarkTag`
- `/PageLayoutStyle`
- `/ReferenceInfo`
- `/RelatedDocuments`
- `/TextLayoutStyle`
- `/ThinkingTemplate`

## Common Storage Tree

All five samples include the following macro storage tree:

```text
/DocumentMacro
/DocumentMacro/\x04JSRV_SegmentInformation
/DocumentMacro/Macros
/DocumentMacro/Macros/\x04JSRV_SegmentInformation
/DocumentMacro/Macros/BaseStorage0
/DocumentMacro/Macros/BaseStorage0/\x04JSRV_SegmentInformation
/DocumentMacro/Macros/BaseStorage0/InfoStream
/DocumentMacro/Macros/BaseStorage0/MacrosStream
/DocumentMacro/Macros/BaseStorage0/MacrosStreamStyle3
```

## Early Hypotheses

- `/DocumentText` is the primary body text candidate because it is by far the largest common stream.
- `/DocumentTextPositionTables` likely indexes or maps text positions.
- `/DocumentViewStyles`, `/DocumentEditStyles`, `/TextLayoutStyle`, `/PageLayoutStyle`, and `/Font` are style/model candidates.
- `JSRV_*` streams and `SummaryInformation` streams should be preserved even before they are understood.

## Next Questions

- Determine whether `/DocumentText` is compressed, encoded, segmented, or encrypted.
- Compare `/DocumentText` size and content against trivial documents with known text.
- Identify whether page size variants explain the differences between `a5`, `a6`, `b6`, and `46`.
- Decide whether line/page mark streams are optional layout caches or required model data.
- Identify the proper object/stream boundaries for samples that use embedded `SsmgV.01` fragments instead of named `/DocumentText`.
