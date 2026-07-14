# rjtd-cli

Command-line inspection and export tools for Ichitaro JTD documents.

`rjtd-cli` installs the `rjtd` executable from the
[OpenJTD](https://github.com/KimEJ/OpenJTD) Rust workspace. It combines
`rjtd-core`, `rjtd-model`, and `rjtd-export` for end-user inspection and
developer-focused format diagnostics.

## Developer preview

Version 0.0.1 is an experimental developer preview. Command names, output
schemas, and library behavior may change in any later 0.0.x release.

## Role in OpenJTD

`rjtd-cli` is the end-user and investigator boundary: it reads files through
`rjtd-core`, builds document views with `rjtd-model`, and writes output through
`rjtd-export`. Prefer the library crates when embedding OpenJTD; this binary's
diagnostic commands and tabular output are intentionally not a stable API.

## Install

```sh
cargo install rjtd-cli --version 0.0.1
```

## Common commands

```sh
rjtd --help
rjtd info document.jtd
rjtd cat document.jtd
rjtd export document.jtd --format json
rjtd export document.jtd --format md
rjtd export document.jtd --format text
rjtd export document.jtd --format html
rjtd export document.jtd --format pdf -o document.pdf
```

The CLI also exposes low-level stream, sector-chain, text-position, style,
layout, and object probes. Those commands are reverse-engineering diagnostics;
their `Candidate`, `Unknown`, and `Diagnostic` classifications are not stable
document semantics. JSON fields marked `decoded: false` retain evidence but do
not assert a completed interpretation.

## Format and output limits

The workspace has exercised the CLI with observed `.jtd`, `.jtt`, and `.jttc`
files. Input larger than 64 MiB is rejected before a full read. JSON fields
marked `decoded: false` preserve evidence without claiming that the associated
layout, style, or object meaning is known. Text and export output can therefore
be partial or use conservative fallbacks.

## License

Apache-2.0.
