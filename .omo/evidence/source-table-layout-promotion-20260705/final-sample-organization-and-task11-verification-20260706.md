# Final Sample Organization And Task 11 Verification

Date: 2026-07-06 KST

## Result

Sample corpus organization is complete. PAGE01 right/down samples are retained as diagnostic-only evidence, not render-promotion evidence.

The original `current-progress-next-plan.md` task 11 was rechecked after PAGE01 work. `tsaiten` remains non-promotable:

- scoring visible table: `sourceReplacementBlockedReason=source-derived-layout-not-renderable`
- lower/diagnostic table family: `sourceReplacementBlockedReason=source-page-y-render-admission-not-ready`
- all extracted replacement blockers keep `sourceOnlyPageYAdmissionReady=false`

No reference fallback was suppressed.

## CLI Surface Checks

`source-y-probe-audit` accepts both the corpus root and the direct organized files directory:

```text
source-y-probe-audit rjtd-testdata/local-samples/ichitaro-source-y-probe
summary cases=40 created=39 failed=1 omitted=0 missing-pairs=0 baseline=000_base_a

source-y-probe-audit rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/baseline-sweep
summary cases=40 created=39 failed=1 omitted=0 missing-pairs=0 baseline=000_base_a
```

PAGE01 right shift still exposes only diagnostic signals:

```text
firstCellOffsetDelta=8
blocker=line-mark-rows-not-exact-source-boundaries
admission ready=false reason=direct-source-diff-diagnostic-only
```

## Verification

Passed:

```bash
cd rjtd
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p rjtd-cli --test source_y_probe -- --ignored --nocapture
```

Additional checks:

- `git diff --check` passed.
- Probe modules are all below 250 pure LOC after splitting `probe_format.rs`.
- LSP hook could not run because the local stable toolchain does not provide `rust-analyzer`; cargo checks above were used as the executable verification.
