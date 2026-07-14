# Releasing the OpenJTD Rust crates

Publishing a crates.io version is permanent: it cannot be overwritten or
deleted. `0.0.1` is therefore a staged, first-public-release procedure, not a
single workspace publish. Follow the [Cargo publishing reference](https://doc.rust-lang.org/cargo/reference/publishing.html)
from one clean, reviewed release commit on `main`.

## Release branch invariant

`dev` is the integration branch; it is never a release source. Merge the
reviewed `dev` state into `main`, wait for the `main` quality and deployment
workflows to pass, and only then date the changelog, create the release commit,
tag it, or run a real `cargo publish`. Immediately before the release gate,
confirm that the local checkout is the exact remote `main` tip:

```sh
git switch main
git pull --ff-only origin main
test "$(git branch --show-current)" = main
test "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)"
```

## Publication boundary and dependency order

| Package | crates.io | Required OpenJTD `0.0.1` versions |
| --- | --- | --- |
| `rjtd-core` | yes | none |
| `rjtd-model` | yes | `rjtd-core` |
| `rjtd-export` | yes | `rjtd-core`, `rjtd-model` |
| `rjtd-wasm` | yes | `rjtd-core`, `rjtd-model` |
| `rjtd-cli` | yes | `rjtd-core`, `rjtd-model`, `rjtd-export` |
| `rjtd-testkit` | **never** | internal only; manifest must retain `publish = false` |

Publish in this exact order: `rjtd-core`, `rjtd-model`, `rjtd-export`,
`rjtd-wasm`, then `rjtd-cli`. Wait for every uploaded predecessor to appear in
the crates.io index before starting its dependent's gate. `rjtd-testkit` is not
part of any package, dry-run, owner, or publish command.

## One-time source gate

From the clean release commit whose `CHANGELOG.md` already bears the final UTC
release date, review the Apache-2.0 material and the package lists, then run
the workspace checks once:

```sh
cd rjtd
cargo fmt --all --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
```

`--locked` is intentional: Cargo exits rather than changing or regenerating
`Cargo.lock`, which makes the gate reproducible. A product-specific `wasm-pack`
build may be run separately, but the crates.io gate is Cargo's package and
publish verification.

## Per-package release preflight

Run the repository gate immediately before each eventual upload, substituting
the package due at that stage:

```sh
./scripts/release-preflight.sh --package rjtd-core
```

The gate is deliberately publish-safe. It creates disposable `CARGO_HOME` and
`CARGO_TARGET_DIR` directories, clears registry-token environment variables,
uses `--locked`, and invokes `cargo publish` only with `--dry-run`. It never
calls `cargo login`, `cargo owner`, or a real publish command, and it prints no
credentials. Under Bash, its external command surface is exactly `cargo`,
`curl`, `git`, `grep`, `mktemp`, `mkdir`, `rm`, `tee`, `dirname`, and `cat`; it
checks their availability before normal execution. In particular, `tee` copies
the authoritative package list to a disposable file while preserving it for
operator review. The gate verifies all of the following before the dry-run:

- the selected package is one of the five public `0.0.1` crates and explicitly
  permits only `crates-io`;
- `rjtd-testkit` still has `publish = false`;
- the repository is clean (or, only for local candidate inspection,
  `--allow-dirty` was explicitly chosen);
- the selected exact crates.io name is currently unallocated; a `404` check is
  only a point-in-time observation, never a name reservation;
- every internal dependency required by that package is already visible at
  `0.0.1` in crates.io; and
- `cargo package --list` contains `Cargo.toml`, `LICENSE`, `README.md`, and Rust
  source before Cargo performs its normal package/build verification.

Cargo's [package command](https://doc.rust-lang.org/cargo/commands/cargo-package.html)
lists the archive contents and rebuilds a clean extracted package; Cargo's
[`publish --dry-run`](https://doc.rust-lang.org/cargo/commands/cargo-publish.html)
performs the upload checks without uploading. A dependent package must fail its
preflight until its exact internal `0.0.1` dependencies are indexed. That is an
expected release-order block, not a reason to use `--no-verify` or to omit the
dry-run.

For this intentionally dirty integration worktree only, this non-approval
candidate check is allowed:

```sh
./scripts/release-preflight.sh --allow-dirty --package rjtd-core
```

It does not make a release ready. The default clean-tree check must pass from
the tagged release commit.

## Irreversible publication procedure

1. Immediately before the first real upload, replace `Unreleased` in the
   `0.0.1` changelog header with that day's actual UTC date. Review and commit
   that change, confirm the repository is clean, run the one-time source gate,
   and create an annotated local tag on this exact release commit. The tag must
   therefore permanently contain the dated changelog that describes the source
   of every published crate.

   ```sh
   git tag -a rjtd-v0.0.1 -m "OpenJTD Rust crates 0.0.1"
   test "$(git rev-parse rjtd-v0.0.1^{commit})" = "$(git rev-parse HEAD)"
   ```

   Do not edit, commit, rebase, or switch commits between this check and the
   five publication stages. All five `cargo publish` commands below must run
   from the tagged release commit.

2. The human publisher must have a crates.io account, verified email, and an
   API token with permission to create crates. Supply the token only through
   an interactive `cargo login` or a secret `CARGO_REGISTRY_TOKEN` environment
   variable. Never put it in a command line, manifest, repository file, log,
   or this script.

3. For each package in the required order, run its preflight, then and only
   then run the corresponding real upload. These commands are the operator's
   production procedure; this task does not authorize them.

   ```sh
   cargo publish --locked -p rjtd-core
   cargo publish --locked -p rjtd-model
   cargo publish --locked -p rjtd-export
   cargo publish --locked -p rjtd-wasm
   cargo publish --locked -p rjtd-cli
   ```

   Wait for index visibility after each command. Cargo can time out while a
   successful upload is still propagating, so confirm the crate/version in the
   registry before continuing rather than retrying an upload blindly. Once
   `rjtd-core` is confirmed visible, push the already-verified annotated tag:

   ```sh
   git push origin rjtd-v0.0.1
   ```

   This push point preserves the source record as soon as the release becomes
   irreversible, without advertising a tag for an attempt that never uploads.

   - Before invoking the first real `cargo publish`, an operator may stop: no
     crate or tag has been published, so the local tag and the dated release
     commit may be discarded or revised and the changelog can remain
     `Unreleased`.
   - After any registry upload succeeds, retain the date and never move or
     delete the release tag. If a client response is ambiguous, first inspect
     crates.io; treat any visible `0.0.1` package as a partial release.
     Immediately push the exact tag if it was not pushed already, halt later
     publication stages, and record the published subset and blocker in a
     follow-up commit or issue. Do not create a replacement `0.0.1` source tag
     or overwrite the published version.

4. After each of the five public crates becomes visible, add and list the
   intended owner(s) for that specific crate. Repeat the following pair for
   `rjtd-core`, `rjtd-model`, `rjtd-export`, `rjtd-wasm`, and `rjtd-cli` only;
   never run it for `rjtd-testkit`. Replace the owner value with the approved
   GitHub identity. A team owner uses `github:<organization>:<team>`.

   ```sh
   crate=rjtd-core # replace with the newly visible public crate at each stage
   cargo owner --add "github:organization:team" "$crate" --registry crates-io
   cargo owner --list "$crate" --registry crates-io
   ```

   Named owners can also change owners; team owners can publish and yank but
   cannot manage owners. Do not grant either role to an untrusted identity.

5. After all five crate pages and docs.rs builds are healthy, the date-bearing
   tag is already pushed and remains the sole release-source tag. Do not make a
   second changelog-date commit. Any completion note is a normal follow-up
   commit and must not move the release tag.
