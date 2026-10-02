# rjtd-testdata

This space manages fixtures, expected outputs, and regression test data for verifying the rjtd implementation.

Track original sample documents separately from derived expected results.

`local-samples/` is available for personal files used during local manual checks.
Only redistributable fixtures and derived expected results should be committed.

## Verification Tiers

Run these commands from `rjtd/`:

```sh
# Workspace checks, including local tests when their inputs are available.
cargo test --workspace --locked --no-fail-fast

# Subset excluding test names that contain local_.
cargo test --workspace --locked --no-fail-fast -- --skip local_

# Explicit controlled-probe tests; requires the local source-y corpus.
cargo test -p rjtd-cli --test source_y_probe --locked -- --ignored
```

Synthetic fixtures are constructed in Rust tests. Many real-document tests use
untracked `local-samples/` files and return early if the source or reference PDF
is absent; some are explicitly ignored. Consequently, an `ok` result does not
always mean the sample assertions ran. Record the command, available input set,
and skipped cases when reporting results. Ensure cloud-placeholder files are
downloaded before a local run.

The source-y probe tests expect `local-samples/ichitaro-source-y-probe/`, including
its manifest and `corpus/baseline-sweep/` and `corpus/page01-grid/` directories.
Its local provenance notes distinguish native-authored documents from
RTF-import surrogates. Keep that distinction when promoting a decoding rule.

PDF generation checks establish structural plausibility. Fidelity checks also
need trusted reference PDFs and, for artifact tests, generated output under
`openjtd-samples/pdf-output/`. The exporter tests retain known pagination and
orientation divergences as evidence; accepting those known differences does
not mean layout equivalence. A redistributable minimum corpus and explicit
executed/skipped reporting remain [planned work](../docs/ROADMAP.md#next-priorities).

## Rights Boundary

Before committing a fixture or expected result, verify its provenance and
redistribution permission and retain any applicable notices. The root
Apache-2.0 license does not independently grant rights in test-input content or
in generated results that represent it. Local samples must remain local unless
their rights permit publication.
