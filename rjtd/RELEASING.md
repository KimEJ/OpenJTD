# rjtd Release Preparation

The [changelog](CHANGELOG.md) records 0.0.1 on 2026-07-14, and the repository
contains tag `rjtd-v0.0.1`. Preserve that release record. Development changes
and documentation cleanup do not create another 0.0.1 release.
The [historical first-release procedure](docs/research/legacy-release-0.0.1.md)
is retained for provenance, not as the next-release runbook.

## Current Tooling Limitation

The repository [preflight](../scripts/release-preflight.sh) is still fixed to
`release_version="0.0.1"` and requires the selected crate name to be unallocated.
Those are first-release assumptions. The script is not a gate for a new version
of an existing crate. Its update is an open [code task](../TODO.md#editing-and-release-preparation),
not a documentation change or a reason to bypass package verification.

Before a later release, choose its scope/version and update the tooling and
version manifests together. Verify current ownership, target version absence,
internal dependency versions, package contents, dry-run behavior, and failure
handling for that release. Do not use an old name-availability check as proof
that a new version is publishable.

## Branch and Source Invariants

`dev` is integration; a release comes from a clean reviewed `main` commit after
its required quality checks. Preserve unrelated checkouts and user commits.
The source tag, dated changelog, manifests, lockfile, and uploaded package
contents must identify the same release. Once a version is published, do not
move its source tag or try to overwrite the version.

## Package Boundary and Order

The public packages are `rjtd-core`, `rjtd-model`, `rjtd-export`, `rjtd-wasm`,
and `rjtd-cli`. `rjtd-testkit` remains internal with `publish = false`.
Use the dependency order core, model, export, WASM, CLI and confirm each exact
predecessor version is indexed before verifying/uploading a dependent.
Confirm the graph again if the implementation split changes crate boundaries.

## Candidate Verification

From `rjtd/`, run against the chosen candidate:

```sh
cargo fmt --all --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo check -p rjtd-wasm --target wasm32-unknown-unknown --locked
```

Review LICENSE copies, package lists, distribution notices, and locked
dependencies using the quality workflow and repository verification scripts.
Then run Cargo package/build and publish dry-runs per package with tooling
updated for the selected version. Record private/skipped test inputs separately;
quality success does not certify native layout fidelity. A dirty candidate
inspection is not the final clean release gate.

## Publication Record

Date the reviewed changelog and create the immutable source tag before the
first upload. Keep all package uploads on that commit. Confirm the registry
outcome before retrying an ambiguous response or proceeding to a dependent.
If only a subset publishes, preserve the exact source tag and record the subset
and blocker; a correction requires a new version rather than a replacement tag.

Use credentials through supported secret/interactive mechanisms and keep them
out of commands, files, and logs. Changes to owners, publication, tags, or hosted
viewer deployment are separate release operations. This document cleanup did
not perform them. Verify package pages, docs builds, notices, and deployment
results for the actual release rather than inferring them from source files.
