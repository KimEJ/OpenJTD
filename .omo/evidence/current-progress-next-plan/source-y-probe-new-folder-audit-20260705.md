# Source-Y Probe New Folder Audit (2026-07-05)

## Scope

Checked samples under `ichitaro-source-y-probe/new`.

## Files Found

- Baseline: `PAGE 01.jtd`, `PAGE 01.pdf`
- Down sweep: `PAGE 01_down_1Low` through `PAGE 01_down_4Low`
- Right sweep: `PAGE 01_right_1Tick` through `PAGE 01_right_4Tick`
- Extra temporary files: `PAGE 01_down_4Low.jtd.$$$`, `PAGE 01_right_4Tick.jtd.$$$`

The extra `.$$$` files were not used and were not deleted.

## PDF Check

All PDFs are one-page A4 files.

The visual table is present in PDF output. `pdfplumber` reports `rects=14` and
`lines=0` for each file, so the ruled grid is represented as rectangles rather
than line objects.

### Measured Cell Coordinates

| sample | `CELL1` x0 | `CELL1` top | delta x | delta top |
|---|---:|---:|---:|---:|
| `PAGE 01` | 90.36 | 119.89 | 0.00 | 0.00 |
| `PAGE 01_down_1Low` | 90.36 | 136.69 | 0.00 | 16.80 |
| `PAGE 01_down_2Low` | 90.36 | 153.49 | 0.00 | 33.60 |
| `PAGE 01_down_3Low` | 90.36 | 170.29 | 0.00 | 50.40 |
| `PAGE 01_down_4Low` | 90.36 | 186.97 | 0.00 | 67.08 |
| `PAGE 01_right_1Tick` | 95.64 | 119.89 | 5.28 | 0.00 |
| `PAGE 01_right_2Tick` | 100.92 | 119.89 | 10.56 | 0.00 |
| `PAGE 01_right_3Tick` | 106.32 | 119.89 | 15.96 | 0.00 |
| `PAGE 01_right_4Tick` | 111.60 | 119.89 | 21.24 | 0.00 |

PDF-side isolation is good:

- Down samples move vertically with stable x.
- Right samples move horizontally with stable y.
- Page size stays A4.

## JTD Decode Check

Commands used:

```bash
cd /Users/kimuj5090/Documents/rjtd/rjtd
./target/debug/rjtd source-y-probe-compare "../ichitaro-source-y-probe/new/PAGE 01.jtd" "<candidate>.jtd"
./target/debug/rjtd page-layer-tree "<sample>.jtd" 0
./target/debug/rjtd table-candidates "../ichitaro-source-y-probe/new/PAGE 01.jtd"
./target/debug/rjtd text-tokens "../ichitaro-source-y-probe/new/PAGE 01.jtd"
```

Key results:

- `table-candidates` is empty for `PAGE 01`.
- `source-y-probe-compare` reports `tableCandidates=0`, `sparseTableCandidates=0`, and `baseNonEmptyCells=0` for all new samples.
- `page-layer-tree` returns no `sourceOnlyPageYRenderAdmissionGate` for the new samples because no table candidate is decoded.
- Text tokens include `CELL1`, `CELL2`, and `CELL3`, separated by control tokens, but the current table candidate extractor does not recognize this one-row table shape.

### Down Sweep

The down sweep changes decoded `/LineMark` monotonically:

- `PAGE 01_down_1Low`: declared line count 7
- `PAGE 01_down_2Low`: declared line count 8
- `PAGE 01_down_3Low`: declared line count 9
- `PAGE 01_down_4Low`: declared line count 10

Page tuple signatures stay the same against baseline. This is useful line-mark
diagnostic evidence, but not table render admission evidence because there is no
decoded table candidate.

### Right Sweep

Right samples differ from baseline at decoded `/LineMark` level, but the tick
series itself is decoded-signature-silent:

- `right_1` vs `right_2`: `sourceSignatureSame=true`
- `right_1` vs `right_3`: `sourceSignatureSame=true`
- `right_1` vs `right_4`: `sourceSignatureSame=true`

The full `.jtd` file hashes differ, so the movement is likely in an undecoded
stream/metadata path. This confirms the need for a raw-stream or whole-file diff
diagnostic stage.

Additional stream hashing narrowed the tick-series movement to `/DocumentText`
and `/RelatedDocuments`. The following streams are identical across
`right_1Tick` through `right_4Tick`:

- `/LineMark`
- `/PageMark`
- `/DocumentViewStyles`
- `/TextLayoutStyle`
- `/Header`
- `/PaperMark`
- `/DocumentPeripheralThree`
- `/Footnote`

`/DocumentText` has a clear tick-correlated word pattern:

| word index | byte offset | right 1 | right 2 | right 3 | right 4 |
|---:|---:|---:|---:|---:|---:|
| 32 | 64 | `0x0002` | `0x0004` | `0x0006` | `0x0008` |
| 60 | 120 | `0x0002` | `0x0004` | `0x0006` | `0x0008` |
| 88 | 176 | `0x0002` | `0x0004` | `0x0006` | `0x0008` |
| 116 | 232 | `0x0002` | `0x0004` | `0x0006` | `0x0008` |

Related balancing fields also move in opposite or adjacent patterns:

| word index | byte offset | right 1 | right 2 | right 3 | right 4 |
|---:|---:|---:|---:|---:|---:|
| 48 | 96 | `0x0051` | `0x004f` | `0x004d` | `0x004b` |
| 71 | 142 | `0x004e` | `0x0050` | `0x0052` | `0x0054` |
| 104 | 208 | `0x0051` | `0x004f` | `0x004d` | `0x004b` |

These are strong raw diagnostic candidates for horizontal placement. They are
not render authority yet because this sample shape does not decode into table
candidates.

## Decision

Do not delete the existing samples.

The new samples are useful but do not replace the old corpus:

- Existing samples still provide decoded table candidates and regression
  coverage for multi-row table shapes.
- The new samples are visually cleaner, but currently decode as text/control
  content with zero table candidates.
- The new right sweep is especially useful for raw diff work because PDF motion
  is monotonic while decoded signatures are silent across ticks.

## Requested Next Sample Adjustment

Recreate this same clean sweep as a multi-row native Ichitaro table, preferably
the same 3-row x 2-column shape used by `000_base_a`:

- `R01C01`, `R01C02`
- `R02C01`, `R02C02`
- `R03C01`, `R03C02`

Required shape:

- At least 2 rows; 3 rows is safer because current decoded table candidate
  evidence is strongest on the previous 3x2 baseline.
- Same page setup, font, row heights, column widths, and content across all
  variants.
- One intended edit per file.

Required variants:

- Baseline.
- Down sweep with 1, 2, 3, 4 line or tick increments.
- Right sweep with 1, 2, 3, 4 tick increments.

Keep the current `new` folder. It is useful negative/diagnostic evidence, but it
is not enough for render promotion.
