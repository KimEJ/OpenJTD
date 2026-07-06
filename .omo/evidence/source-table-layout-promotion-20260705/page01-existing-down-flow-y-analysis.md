# PAGE 01 Existing Down Flow-Y Analysis

## Verdict

- 기존 `PAGE 01_down_1Low`..`4Low` 샘플은 계속 사용할 수 있다.
- 다만 의미는 “표만 독립 이동”이 아니라 “문서 흐름에서 표 앞에 줄이 추가되어 표 전체가 아래로 밀리는 flow-y 변화”다.
- PDF 좌표에서는 첫 셀 top이 `lineMarkRecordIndexes[0]` 증가량과 거의 선형으로 대응한다. 기준 라인 높이는 16.8pt다.
- down4만 residual이 -0.12pt인데 PDF extraction rounding/폰트 좌표 오차 범위로 보이며, X는 전 케이스 0.0pt로 고정이다.
- 렌더 승격은 아직 보류한다. 모든 케이스가 `pageOriginAuthority=fallbackTextAnchors`, `sourceDerivedLayoutRenderable=false`, `lineMarkRowsExactAndContiguous=false`다.

## Measurement Table

| case | shape | lineMarkRecordIndexes | source interval | PDF first cell top | dy vs base | expected dy | residual | dx vs base | authority | renderable |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| base | 2x3 | [2, 4] | 99-308 | 119.89 | 0.0 | 0.0 | 0.0 | 0.0 | fallbackTextAnchors | False |
| down1 | 2x3 | [3, 5] | 100-309 | 136.69 | 16.8 | 16.8 | 0.0 | 0.0 | fallbackTextAnchors | False |
| down2 | 2x3 | [4, 6] | 101-310 | 153.49 | 33.6 | 33.6 | 0.0 | 0.0 | fallbackTextAnchors | False |
| down3 | 2x3 | [5, 7] | 102-311 | 170.29 | 50.4 | 50.4 | 0.0 | 0.0 | fallbackTextAnchors | False |
| down4 | 2x3 | [6, 8] | 103-312 | 186.97 | 67.08 | 67.2 | -0.12 | 0.0 | fallbackTextAnchors | False |

## Compare Summaries

### down1 (PAGE 01_down_1Low)

- `line-summary	baseDeclared=8	candidateDeclared=9	baseParsed=7	candidateParsed=8	lineSignatureSame=false`
- `page-summary	baseFamily=fixed84	candidateFamily=fixed84	baseEntries=3	candidateEntries=3	pageTupleSignatureSame=true`
- `table-summary	baseCandidates=1	candidateCandidates=1	baseSparseCandidates=0	candidateSparseCandidates=0	baseNonEmptyCells=6	candidateNonEmptyCells=6	tableSignatureSame=false`
- `table-line-header-summary	baseRows=2	candidateRows=2	baseFirstCellOffset=2	candidateFirstCellOffset=2	firstCellOffsetDelta=0	baseCellOffsets=2,26,52|2,26,52	candidateCellOffsets=2,26,52|2,26,52	lineHeaderSignatureSame=false`
- `admission	ready=false	reason=direct-source-diff-diagnostic-only`

### down2 (PAGE 01_down_2Low)

- `line-summary	baseDeclared=8	candidateDeclared=10	baseParsed=7	candidateParsed=9	lineSignatureSame=false`
- `page-summary	baseFamily=fixed84	candidateFamily=fixed84	baseEntries=3	candidateEntries=3	pageTupleSignatureSame=true`
- `table-summary	baseCandidates=1	candidateCandidates=1	baseSparseCandidates=0	candidateSparseCandidates=0	baseNonEmptyCells=6	candidateNonEmptyCells=6	tableSignatureSame=false`
- `table-line-header-summary	baseRows=2	candidateRows=2	baseFirstCellOffset=2	candidateFirstCellOffset=2	firstCellOffsetDelta=0	baseCellOffsets=2,26,52|2,26,52	candidateCellOffsets=2,26,52|2,26,52	lineHeaderSignatureSame=false`
- `admission	ready=false	reason=direct-source-diff-diagnostic-only`

### down3 (PAGE 01_down_3Low)

- `line-summary	baseDeclared=8	candidateDeclared=11	baseParsed=7	candidateParsed=10	lineSignatureSame=false`
- `page-summary	baseFamily=fixed84	candidateFamily=fixed84	baseEntries=3	candidateEntries=3	pageTupleSignatureSame=true`
- `table-summary	baseCandidates=1	candidateCandidates=1	baseSparseCandidates=0	candidateSparseCandidates=0	baseNonEmptyCells=6	candidateNonEmptyCells=6	tableSignatureSame=false`
- `table-line-header-summary	baseRows=2	candidateRows=2	baseFirstCellOffset=2	candidateFirstCellOffset=2	firstCellOffsetDelta=0	baseCellOffsets=2,26,52|2,26,52	candidateCellOffsets=2,26,52|2,26,52	lineHeaderSignatureSame=false`
- `admission	ready=false	reason=direct-source-diff-diagnostic-only`

### down4 (PAGE 01_down_4Low)

- `line-summary	baseDeclared=8	candidateDeclared=12	baseParsed=7	candidateParsed=11	lineSignatureSame=false`
- `page-summary	baseFamily=fixed84	candidateFamily=fixed84	baseEntries=3	candidateEntries=3	pageTupleSignatureSame=true`
- `table-summary	baseCandidates=1	candidateCandidates=1	baseSparseCandidates=0	candidateSparseCandidates=0	baseNonEmptyCells=6	candidateNonEmptyCells=6	tableSignatureSame=false`
- `table-line-header-summary	baseRows=2	candidateRows=2	baseFirstCellOffset=2	candidateFirstCellOffset=2	firstCellOffsetDelta=0	baseCellOffsets=2,26,52|2,26,52	candidateCellOffsets=2,26,52|2,26,52	lineHeaderSignatureSame=false`
- `admission	ready=false	reason=direct-source-diff-diagnostic-only`

## Implementation Implication

- 이 묶음은 source-derived absolute table projection 승격에는 부족하지만, flow-line based y-shift 진단/모델 필드 검증에는 충분하다.
- `source-y-probe-compare`에는 기존 down 샘플을 flow-y shift로 읽는 `table-flow-y-summary` 진단을 추가했다.
- down1은 `lineMarkRecordDelta=1`, down4는 `lineMarkRecordDelta=4`로 출력되며 PDF y delta와 대응된다.
- 실제 PDF 렌더 반영은 page origin과 row boundary 의미를 더 해독한 뒤에 해야 한다.

Full JSON evidence: `.omo/evidence/source-table-layout-promotion-20260705/page01-existing-down-flow-y-analysis.json`
