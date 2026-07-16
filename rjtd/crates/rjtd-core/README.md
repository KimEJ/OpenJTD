# rjtd-core

Low-level parsers and diagnostics for Ichitaro JTD compound documents.

`rjtd-core` is the container and stream layer of
[OpenJTD](https://github.com/KimEJ/OpenJTD). Higher-level document semantics
live in `rjtd-model`; end-user exports live in `rjtd-export` and `rjtd-cli`.

## Developer preview

Version 0.0.1 is an experimental developer preview. The implementation is
based on observed files and is not a complete specification of the JTD family.
All public APIs may change in any later 0.0.x release.

## What it provides

- Compound File Binary (CFB) entry, stream, directory, and sector-chain access.
- Parsers for observed document text, position, layout, font, style, auto-text,
  and compressed-document structures.
- Conservative record types that preserve raw or unknown data while the format
  is being reverse engineered.
- Low-level diagnostics used by the `rjtd` command-line tool.

## Role in OpenJTD

This crate stops at container, stream, and record evidence. It does not expose
the application-facing `Document` or `DocumentCore` model, and it does not
export documents: use `rjtd-model` for those model APIs and `rjtd-export` for
end-user output. That separation lets consumers choose whether they need raw
format inspection or a higher-level document view.

The workspace has exercised these building blocks with observed `.jtd`, `.jtt`,
and `.jttc` samples. Malformed or previously unseen files may be rejected or
represented only as raw diagnostic data.

## Example

```rust,no_run
let bytes = std::fs::read("document.jtd")?;
let entries = rjtd_core::container::inspect_cfb_entries(&bytes)?;
for entry in entries {
    println!("{}", entry.path());
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Resource and interpretation limits

`ParseLimits::DEFAULT` caps bounded input at 64 MiB, each individual LH5
output at 256 MiB, and the aggregate LH5 output accumulated while parsing one
document at 256 MiB. It also applies a 256x expansion ceiling above a 1 MiB
allowance. The input check runs after the caller has already allocated its
`&[u8]`, so it cannot reclaim that allocation. The same limit set creates one
`ResourceBudget` for a limits-aware document: declared CFB stream count and
bytes, retained record count and bytes, retained image count and bytes,
header-derived image width/height/pixels, and constructed page/page-line
output are charged with checked cumulative arithmetic. Low-level helpers that
accept caller-owned slices remain compositional APIs; use the higher-level
`*_with_limits` or `*_with_budget` entry points when parsing untrusted input.

Types and fields named `Unknown`, `Candidate`, `Diagnostic`, or otherwise
marked as undecoded are evidence-preserving surfaces, not stable semantic
claims. A `Candidate` is a hypothesis inferred from observed bytes, `Unknown`
retains data that has not been classified, and a `Diagnostic` explains the
observed limitation. A `decoded: false` value means the source evidence is
retained but the field must not be treated as authoritative layout or document
meaning.

## License

Apache-2.0.
