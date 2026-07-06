# PAGE01_DOWNTEST Diagnosis

Files inspected:
- ichitaro-source-y-probe/new/PAGE 01_DOWNTEST_BASE.jtd
- ichitaro-source-y-probe/new/PAGE 01_DOWNTEST_BASE.pdf
- ichitaro-source-y-probe/new/PAGE 01_DOWNTEST_1LINE.jtd
- ichitaro-source-y-probe/new/PAGE 01_DOWNTEST_1LINE.pdf

Conclusion:
- DOWNTEST_1LINE is a flow insertion sample, not an independent table-object
  Y-position sample.
- PAGE 01 START stays at the same PDF Y position.
- The table cells and PAGE 01 END all move down by 16.8 pt.

PDF coordinate facts:
- PAGE 01 START top: 86.29 pt in both files.
- R01C01 top: 119.89 pt -> 136.69 pt, delta +16.8 pt.
- R02C01 top: 153.49 pt -> 170.29 pt, delta +16.8 pt.
- PAGE 01 END top: 186.97 pt -> 203.77 pt, delta +16.8 pt.

JTD signal facts:
- /PageMark tuple signature is unchanged.
- /LineMark declared/parsed records changed from 8/7 to 9/8.
- One extra line-like record appears.
- Table source intervals shift from first/last 2/6 to 3/7.
- Table source range shifts from 105-314 to 116-325.
- Table line-header X offsets remain unchanged: 2,26,52.
- Layer-tree lineMarkRecordIndexes shift from [2,4] to [3,5].
- The stride remains 2, so lineMarkRowsExactAndContiguous is still false.
- sourceDerivedLayoutRenderable remains false.
- projectionKind remains diagnosticProjection.

Interpretation:
- The sample is useful for proving that inserting a line before a flow table
  moves downstream content by one PDF line step.
- It does not prove a source-only absolute table Y origin or table-only
  placement control, because the lower text moves with the table and PageMark
  is unchanged.

Revised sampling implication:
- If normal Ichitaro tables behave as inline flow content, then asking for
  "TOP fixed, table down, BOTTOM fixed" may be impossible or the wrong target.
- The useful next samples are flow variants with 0/1/2/3/4 inserted blank lines
  before the table, plus optional variants with text after the table, so the
  line-flow transform can be decoded honestly.
