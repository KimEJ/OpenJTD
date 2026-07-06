# Probe CLI 산출물 계약

## 목적

`ichitaro-source-y-probe` 로컬 샘플은 렌더 승격을 바로 여는 자료가 아니라, source signal과 admission blocker를 분리해서 기록하는 진단 코퍼스다. 아래 CLI 출력 이름은 handoff와 테스트에서 같은 의미로 사용한다.

## 명령 표면

현재 도움말에 노출되는 관련 명령:

```text
rjtd source-y-probe-audit <corpus-dir>
rjtd source-y-probe-compare <base.jtd> <candidate.jtd>
rjtd table-candidates <file.jtd>
rjtd page-layer-tree <file.jtd> <zero-based-page-index>
```

canonical local corpus path:

```text
rjtd-testdata/local-samples/ichitaro-source-y-probe/
```

canonical PAGE01 comparison path:

```text
rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid/
```

## `source-y-probe-audit`

역할:

- broad sweep corpus를 한 번에 훑는다.
- sample pair의 source signature, validation category, admission summary를 탭 구분 행으로 출력한다.
- 현재 organized corpus는 코퍼스 루트의 `manifest.csv`와 `corpus/baseline-sweep/` 파일 디렉터리를 사용한다.
- CLI 입력은 코퍼스 루트와 `corpus/baseline-sweep/` 직접 경로를 모두 허용한다. legacy `files/`는 오래된 local copy 호환 fallback이다.

현재 핵심 계약:

```text
summary cases=40 created=39 failed=1 omitted=0 missing-pairs=0
admission ready=false reason=diagnostic-only-corpus-insufficient-for-source-only-page-y-render-admission
```

이 명령은 corpus-wide render admission을 열지 않는다.

## `source-y-probe-compare`

역할:

- 두 JTD 파일의 source-level 차이를 비교한다.
- 직접 source diff가 보이면 최종 `admission` 행은 diagnostic-only로 유지한다.

주요 출력 행:

```text
summary
line-summary
line-delta-diff
page-summary
page-tuple-diff
table-summary
table-line-header-summary
table-flow-y-summary
table-flow-y-admission-summary
table-flow-y-hypothesis
admission
```

### X축 행

`table-line-header-summary`는 `/DocumentText` line-header의 table cell offset 변화를 기록한다.

PAGE01 right-tick 계약:

```text
right_1Tick firstCellOffsetDelta=2
right_2Tick firstCellOffsetDelta=4
right_3Tick firstCellOffsetDelta=6
right_4Tick firstCellOffsetDelta=8
```

해석:

- X축 source signal은 관찰된다.
- 하지만 이 행만으로 page-space x/width frame을 decode하지 않는다.
- layer tree의 `sourceOnlyAxisAdmissionGate.admissionReady=false`가 최종 admission을 막는다.

### Y축 행

`table-flow-y-summary`는 table rows와 `/LineMark` record index의 stride correlation을 기록한다.

PAGE01 down 계약:

```text
down_1Low lineMarkRecordDelta=1
down_2Low lineMarkRecordDelta=2
down_3Low lineMarkRecordDelta=3
down_4Low lineMarkRecordDelta=4
```

`table-flow-y-admission-summary`는 row source range와 LineMark row range가 render admission에 충분한지 판단한다.

현재 PAGE01 blocker:

```text
blocker=line-mark-rows-not-exact-source-boundaries
```

`table-flow-y-hypothesis`는 관찰된 stride signal을 non-admissible hypothesis로 명시한다.

현재 PAGE01 계약:

```text
strideCorrelationObserved=true
transformProven=false
renderAdmissible=false
hypothesis=line-mark-record-stride-correlates-with-flow-y
blockedReason=line-mark-rows-not-exact-source-boundaries
```

해석:

- Y축 flow signal은 관찰된다.
- 그러나 Ichitaro editor sample은 table-only Y movement가 아니라 following text를 함께 민다.
- 따라서 이 행은 decoder lead이지 render admission proof가 아니다.

## `table-candidates`

역할:

- `/DocumentText` control-run table candidate를 행 단위로 표시한다.
- PAGE01 baseline에서 expected contract:

```text
kind=documentTextControlRunTableCandidate range=- boundary=- basis=unit delimiter=0x000e intervals=2
cells=6/0/6 max-columns=3
text=R01C01\tR01C02\tR01C03
text=R02C01\tR02C02\tR02C03
```

이 명령은 table existence와 source segmentation을 확인한다. page-space geometry admission은 하지 않는다.

## `page-layer-tree`

역할:

- renderer/model layer JSON을 출력한다.
- admission gate의 최종 source-only render 상태는 이 명령에서 확인한다.

PAGE01 right/down 공통 blocker:

```json
{
  "sourceOnlyPageYAdmissionClass": "flow-y-stride-only-diagnostic",
  "pageOriginAuthority": "fallbackTextAnchors",
  "sourceOnlyPageYRenderAdmissionGate": {
    "admissionReady": false,
    "renderPromotionBlockedReason": "source-page-y-render-admission-not-ready"
  }
}
```

PAGE01 right-shift X축 blocker:

```json
{
  "pageSpaceSolver": {
    "horizontalSolverReady": true,
    "yOriginSolverReady": false
  },
  "sourceOnlyAxisAdmissionGate": {
    "admissionReady": false,
    "horizontalAxisReady": false,
    "horizontalSelectorCandidatePresent": false,
    "yAxisReady": false,
    "renderPromotionBlockedReason": "source-page-space-axis-selector-coupling-unproven"
  }
}
```

## 회귀 테스트

로컬 샘플이 있을 때 다음 ignored 테스트가 이 계약을 고정한다.

```bash
cd rjtd
cargo test -p rjtd-cli --test source_y_probe -- --ignored --nocapture
```

현재 결과:

```text
7 passed
```

중요 테스트:

- `local_new_probe_compare_reports_table_line_header_x_shift_when_available`
- `local_new_probe_compare_reports_table_flow_y_shift_when_available`
- `local_new_probe_layer_tree_keeps_flow_y_stride_diagnostic_non_admissible_when_available`
- `local_new_probe_layer_tree_keeps_right_shift_non_admissible_when_available`

## Handoff 규칙

- `table-line-header-summary`와 `table-flow-y-summary`는 signal rows다.
- `table-flow-y-admission-summary`, `table-flow-y-hypothesis`, `sourceOnlyPageYRenderAdmissionGate`, `sourceOnlyAxisAdmissionGate`는 admission rows/gates다.
- source signal이 있어도 admission gate가 false이면 렌더러나 exporter에서 visible render promotion을 하면 안 된다.
- historical evidence 파일에 legacy `ichitaro-source-y-probe/new/` 경로가 남아 있어도 canonical path는 `rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid/`다.
