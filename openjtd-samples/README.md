# openjtd-samples

This space manages redistributable JTD sample documents and output artifacts for
OpenJTD.

Only files that can be published as part of OpenJTD should be included.

## PDF Output Artifacts

`pdf-output/` contains generated PDFs for the local sample set. Regenerate them
from the repository root with:

```sh
scripts/regenerate-pdf-output.sh
```

The script reads `.jtd`, `.jtt`, and `.jttc` files from
`rjtd-testdata/local-samples/` and writes same-stem PDFs into `pdf-output/`.
