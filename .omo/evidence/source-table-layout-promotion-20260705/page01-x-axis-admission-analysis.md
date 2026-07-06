# PAGE01 X축 right-tick 승격 검토

## 결론

`PAGE 01_right_*Tick.jtd` 샘플은 `/DocumentText` line header의 X 단위 변화가 직접 보인다. 하지만 현재 증거만으로는 source-only table render 승격을 하면 안 된다.

현재 판정은 `source-only horizontal diagnostic only`다. 이유는 수평 단위 후보가 row 간 일관적이어도 page-space 좌표계로 옮기는 selector/admission gate가 아직 닫혀 있고, Y origin admission도 동시에 닫혀 있기 때문이다.

## 입력

- 기준: `rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid/PAGE 01.jtd`
- 비교:
  - `PAGE 01_right_1Tick.jtd`
  - `PAGE 01_right_2Tick.jtd`
  - `PAGE 01_right_3Tick.jtd`
  - `PAGE 01_right_4Tick.jtd`

## 비교 출력 요약

명령:

```bash
for n in 1 2 3 4; do
  ./rjtd/target/debug/rjtd source-y-probe-compare \
    "rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid/PAGE 01.jtd" \
    "rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_right_${n}Tick.jtd" |
    rg 'table-line-header-summary|table-flow-y-summary|admission'
done
```

관찰:

- right 1 tick: `baseFirstCellOffset=2`, `candidateFirstCellOffset=4`, `firstCellOffsetDelta=2`
- right 2 tick: `baseFirstCellOffset=2`, `candidateFirstCellOffset=6`, `firstCellOffsetDelta=4`
- right 3 tick: `baseFirstCellOffset=2`, `candidateFirstCellOffset=8`, `firstCellOffsetDelta=6`
- right 4 tick: `baseFirstCellOffset=2`, `candidateFirstCellOffset=10`, `firstCellOffsetDelta=8`
- 모든 right 샘플에서 `lineMarkRecordDelta=0`
- 모든 right 샘플에서 `admission ready=false reason=direct-source-diff-diagnostic-only`

즉 X축 변화는 선형이고 source 신호로는 충분히 관찰된다. 하지만 이 값은 아직 "source 단위의 수평 후보"이지 "페이지 좌표계 렌더 frame"이 아니다.

## 레이어 트리 게이트

명령:

```bash
./rjtd/target/debug/rjtd page-layer-tree \
  "rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_right_4Tick.jtd" 0 |
  jq -c '.root.ops[] | select(.type=="tableGridCandidate") | {
    sourceDerivedLayoutReadiness: {
      candidatePresent: .sourceDerivedLayoutReadiness.sourceDerivedLayoutCandidatePresent,
      renderable: .sourceDerivedLayoutReadiness.sourceDerivedLayoutRenderable,
      blocked: .sourceDerivedLayoutReadiness.sourceDerivedLayoutBlockedReason,
      rejectionReasons: .sourceDerivedLayoutReadiness.rejectionReasons
    },
    pageSpaceSolver: {
      stage: .pageSpaceSolver.solverStage,
      horizontalSolverReady: .pageSpaceSolver.horizontalSolverReady,
      yOriginSolverReady: .pageSpaceSolver.yOriginSolverReady,
      sourceDerivedLayoutRenderable: .pageSpaceSolver.sourceDerivedLayoutRenderable,
      pageOriginAuthority: .pageSpaceSolver.pageOriginAuthority
    },
    axisGate: {
      admissionReady: .pageSpaceSolver.sourceOnlyAxisAdmissionGate.admissionReady,
      horizontalAxisReady: .pageSpaceSolver.sourceOnlyAxisAdmissionGate.horizontalAxisReady,
      horizontalSelectorCandidatePresent: .pageSpaceSolver.sourceOnlyAxisAdmissionGate.horizontalSelectorCandidatePresent,
      yAxisReady: .pageSpaceSolver.sourceOnlyAxisAdmissionGate.yAxisReady,
      blockedReasons: .pageSpaceSolver.sourceOnlyAxisAdmissionGate.blockedReasons,
      blocked: .pageSpaceSolver.sourceOnlyAxisAdmissionGate.renderPromotionBlockedReason
    },
    pageYGate: {
      admissionReady: .pageSpaceSolver.sourcePageYTransformGate.sourceOnlyPageYRenderAdmissionGate.admissionReady,
      class: .pageSpaceSolver.sourcePageYTransformGate.sourceOnlyPageYRenderAdmissionGate.sourceOnlyPageYAdmissionClass,
      pageOriginAuthority: .pageSpaceSolver.sourcePageYTransformGate.sourceOnlyPageYRenderAdmissionGate.pageOriginAuthority,
      blocked: .pageSpaceSolver.sourcePageYTransformGate.sourceOnlyPageYRenderAdmissionGate.renderPromotionBlockedReason
    }
  }'
```

핵심 출력:

```json
{
  "sourceDerivedLayoutReadiness": {
    "candidatePresent": true,
    "renderable": false,
    "blocked": "page-space-origin-and-row-baseline-unproven",
    "rejectionReasons": [
      "line-mark-rows-not-exact-source-boundaries",
      "page-space-origin-and-row-baseline-unproven",
      "source-derived-layout-not-renderable"
    ]
  },
  "pageSpaceSolver": {
    "stage": "blocked-y-origin-transform",
    "horizontalSolverReady": true,
    "yOriginSolverReady": false,
    "sourceDerivedLayoutRenderable": false,
    "pageOriginAuthority": "fallbackTextAnchors"
  },
  "axisGate": {
    "admissionReady": false,
    "horizontalAxisReady": false,
    "horizontalSelectorCandidatePresent": false,
    "yAxisReady": false,
    "blocked": "source-page-space-axis-selector-coupling-unproven"
  },
  "pageYGate": {
    "admissionReady": false,
    "class": "flow-y-stride-only-diagnostic",
    "pageOriginAuthority": "fallbackTextAnchors",
    "blocked": "source-page-y-render-admission-not-ready"
  }
}
```

## 구현 반영

로컬 샘플 ignored 테스트에 right-shift 레이어 트리 회귀 테스트를 추가했다.

- `local_new_probe_layer_tree_keeps_right_shift_non_admissible_when_available`
- 고정하는 계약:
  - `horizontalSolverReady=true`
  - `yOriginSolverReady=false`
  - `sourceOnlyAxisAdmissionGate.admissionReady=false`
  - blocked reasons include `source-only-horizontal-selector-absent`, `source-horizontal-axis-not-render-admissible`, `source-y-axis-not-render-admissible`
  - Y gate remains `flow-y-stride-only-diagnostic`

검증:

```bash
cargo fmt --all --check
cargo check -p rjtd-cli
cargo test -p rjtd-cli --test source_y_probe -- --ignored --nocapture
```

결과:

- `cargo fmt --all --check`: pass
- `cargo check -p rjtd-cli`: pass
- ignored local sample test: `7 passed`

## 다음 판단

right-tick 샘플은 X축 source signal 문서화와 회귀 테스트로는 충분하다. 하지만 render 승격에는 부족하다.

다음 승격 후보는 X 단독이 아니라 `page-space horizontal transform`과 `sourceOnlyAxisAdmissionGate`를 여는 증거다. 현재 필요한 증거는 다음 중 하나다.

- PageMark horizontal fields의 selector가 reference 없이 unique best로 선택되고 `horizontalAxisReady=true`가 되는 샘플군
- X selector와 Y selector가 같은 source-derived layout 후보에서 동시에 render-admissible이 되는 샘플군
- 또는 현재 PageMark word14/word21 후보의 page-space unit semantics를 독립적으로 증명하는 decoder
