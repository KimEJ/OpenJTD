# openjtd-samples

This space manages redistributable JTD sample documents and output artifacts for
OpenJTD.

Only files that can be published as part of OpenJTD should be included.

## Rights Boundary

Before committing or publishing a sample or an artifact derived from one,
confirm its provenance and redistribution permission and retain any applicable
notices. Apache-2.0 does not independently grant rights in sample content or
in generated output that represents that content.

## PDF Output Artifacts

`pdf-output/` contains generated PDFs for the local sample set. Regenerate them
from the repository root with:

```sh
scripts/regenerate-pdf-output.sh
```

The script reads `.jtd`, `.jtt`, and `.jttc` files from
`rjtd-testdata/local-samples/` and writes same-stem PDFs into `pdf-output/`.
