# Contributing to OpenJTD

OpenJTD is a pre-stable clean-room implementation of JTD, JTT, and JTTC document
handling. Contributions should preserve unknown evidence, keep parser and model
boundaries explicit, and avoid claiming semantics that the available evidence
does not establish.

## Development Setup

Install the current stable Rust toolchain with `rustfmt` and `clippy`, then run:

```sh
cd rjtd
cargo fmt --all --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Some ignored tests and local regression scripts require private or
redistribution-restricted samples. A normal pull request is not expected to
have those files, and a green public test run must not be described as coverage
of the private corpus.

## Branch and Pull Request Flow

1. Create a focused branch in your fork or local clone.
2. Open development pull requests against `dev`.
3. Reserve `dev` to `main` pull requests for project releases.
4. Keep changes small, preserve unrelated work, and explain any unverified
   format assumption.
5. Run the quality commands above and record the results in the pull request.

## License and Developer Certificate of Origin

OpenJTD-authored code and documentation are licensed under Apache-2.0. Unless a
separate written agreement applies, intentionally submitted contributions are
handled under the contribution terms in Section 5 of that license.

The project uses the [Developer Certificate of Origin 1.1](https://developercertificate.org/).
Sign off every commit with:

```sh
git commit -s
```

The resulting `Signed-off-by` line certifies that you have the right to submit
the contribution under the project's license. OpenJTD does not currently
require a separate contributor license agreement or copyright assignment.

### Generated Merge Commits

DCO remains required on every human-authored contribution commit. A two-parent
GitHub/web-flow generated merge commit that adds no independent authored content
is exempt from its own `Signed-off-by` line only when every non-generated
authored contribution commit it combines has a valid `Signed-off-by` line.

There is no exemption for a squash or rebase commit that replaces authored
identity, or for a generated commit that contains independent content.

Lore trailers remain required on agent-authored project commits. Content-free
platform-generated merge commits are exempt from Lore trailers.

## Clean-Room Research Rules

Allowed evidence includes:

- public documentation and public metadata;
- independent observation of files, containers, streams, and records;
- comparison of samples that you are authorized to use;
- behavior observable through normal use of an application;
- RFC records that separate observation, hypothesis, and unresolved evidence.

Do not use or submit:

- copied Ichitaro or proprietary filter source code;
- private SDKs, headers, type libraries, or internal documentation;
- implementation logic reconstructed from decompiler or disassembler output;
- third-party code, samples, fonts, images, or documents without permission;
- code copied from the local `rhwp` reference project.

Preserve unproven information as unknown, diagnostic, fallback, or
`decoded:false` evidence. Do not promote a single-sample heuristic to decoded
semantics.

## Samples and Fixtures

Public fixtures must include enough provenance to verify redistribution:

- original filename and SHA-256;
- creator and copyright owner;
- the submitter's authority to redistribute the file;
- license or written permission and its scope;
- whether derived PDF, JSON, image, or text output may be published;
- third-party images, fonts, personal data, or confidential content;
- how the sample was created, such as synthetic, native, or imported.

If any right is unclear, keep the file under the ignored
`rjtd-testdata/local-samples/` tree and do not attach it to an issue or pull
request. DCO sign-off does not replace permission from a sample's rights holder.

## RFC and Documentation Changes

Format discoveries belong in `openjtd-spec/rfc/`. A proposal begins as a draft
pull request and should identify sources, sample provenance, observed facts,
hypotheses, and unresolved fields. Acceptance requires maintainer review; later
evidence may supersede an accepted record. English is the source language, and
the matching Japanese translation should be updated when an existing paired
RFC changes.

## Security Reports

Do not open public issues for possible vulnerabilities. Follow
[`SECURITY.md`](SECURITY.md) instead.
