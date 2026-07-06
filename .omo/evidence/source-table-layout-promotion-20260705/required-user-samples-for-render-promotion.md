# Source-Only 렌더 승격 결론과 샘플 상태

상태: 현재 `PAGE01` 샘플은 진단용으로는 충분히 쓸 수 있지만, source-only 렌더링 승격 근거로는 부족하다. 사용자는 Ichitaro 에디터에서 표만 y 방향으로 독립 이동하는 기능이 불가능하다고 확인했다.

## 추가 증거가 필요한 이유

현재 `PAGE01` baseline/right/down/down-test 파일들은 모두 아래 상태다.

- `sourceOnlyPageYRenderAdmissionGate.admissionReady=false`
- `pageOriginAuthority=fallbackTextAnchors`
- `lineMarkPageOriginPresent=false`
- `lineMarkPageOriginStridePresent=true`
- `lineMarkRowsExactAndContiguous=false`
- `decoded-line-mark-page-y-transform-missing`

down 샘플에서는 유용한 flow-y 상관관계도 보인다.

- `lineMarkRecordDelta=1,2,3,4`
- `firstRowSourceStartDelta=1,2,3,4`
- `table-flow-y-hypothesis.strideCorrelationObserved=true`
- `transformProven=false`
- `renderAdmissible=false`

하지만 이것은 "신호가 있다"는 증거일 뿐이고, "이 신호가 렌더링 가능한 page-y 좌표다"라는 증거는 아니다.

## 이미 받은 샘플

아래 성격의 샘플은 이미 받았다.

- 표를 한 줄 또는 여러 줄 아래로 내리면 아래 텍스트도 함께 밀리는 flow-y 샘플
- `PAGE01_DOWNTEST_*`
- `PAGE 01_down_1Low`부터 `PAGE 01_down_4Low`
- right tick 샘플

따라서 "표만 y 방향으로 독립 이동한 샘플"은 더 요청하지 않는다. Ichitaro 에디터 자체에서 표만 y 방향으로 이동할 수 없다는 전제를 따른다.

현재 flow-y 샘플은 아래 사실을 이미 증명한다.

- 표와 아래 텍스트가 문서 flow 안에서 함께 이동한다.
- down 단계에 따라 `lineMarkRecordDelta`와 `firstRowSourceStartDelta`가 1, 2, 3, 4로 같이 움직인다.
- 즉, flow-y 상관관계는 있다.

하지만 이 샘플들은 아래를 아직 증명하지 못한다.

- 이 상관관계가 렌더링 가능한 절대 page-y 좌표라는 점
- `fallbackTextAnchors`가 아닌 source-only page-y authority
- `lineMarkPageOriginPresent=true`
- `lineMarkRowsExactAndContiguous=true`
- decoded in-file y transform

## 추가 신규 샘플 상태

현재 추가로 요구할 신규 샘플은 없다.

이전에 후보로 적었던 `PAGE02_BASE_DIRECT_ORIGIN`, `PAGE02_DOWN_*`, `PAGE02_ABSOLUTE_Y_ORIGIN*` 계열은 "Ichitaro에서 표만 독립 y 배치가 가능하다면"이라는 조건부 요청이었다. 사용자가 독립 y 배치가 불가능하다고 확인했으므로 이 요청은 더 이상 유효하지 않다.

기존 `PAGE01` flow-y 샘플이 현재 가능한 샘플 형태의 끝으로 본다.

## 확정 결론

`PAGE01` 계열에 대해서는 source-only y 렌더 승격을 하지 않는다.

이유:

- flow-y 상관관계는 확인됐다.
- 하지만 그 상관관계는 문서 flow 안에서 표와 아래 텍스트가 함께 밀리는 현상이다.
- `pageOriginAuthority=fallbackTextAnchors` 상태이며, 렌더 가능한 source-only page-y origin이 아니다.
- `lineMarkPageOriginPresent=false`이고 `decoded-line-mark-page-y-transform-missing`이다.
- 사용자가 표만 독립 y 배치하는 Ichitaro UI/설정이 불가능하다고 확인했다.

따라서 올바른 구현 상태는 다음과 같다.

- `source-y-probe-compare`는 flow-y 신호와 비승격 hypothesis를 출력한다.
- `page-layer-tree`는 `sourceOnlyPageYRenderAdmissionGate.admissionReady=false`를 유지하고 `sourceOnlyPageYAdmissionClass=flow-y-stride-only-diagnostic`으로 분류한다.
- 렌더러는 이 신호를 표의 절대 y 좌표로 승격하지 않는다.
- 같은 down 샘플을 추가로 요구하지 않는다.

## 남는 작업

코드 쪽 남는 작업은 신규 샘플 요청이 아니라 회귀 안전장치다.

- `PAGE01` flow-y 샘플에서 `table-flow-y-hypothesis.transformProven=false`가 계속 출력되는지 확인한다.
- `PAGE01` flow-y 샘플에서 `sourceOnlyPageYRenderAdmissionGate.admissionReady=false`가 계속 유지되는지 테스트한다.
- 이 상태를 ULW evidence와 최종 계획에 기록한다.

## 수락 조건

현재 결론을 검증할 때는 아래 명령을 사용한다.

```bash
./rjtd/target/debug/rjtd source-y-probe-compare \
  "rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid/PAGE 01.jtd" \
  "rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_down_4Low.jtd"

./rjtd/target/debug/rjtd page-layer-tree \
  "rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_down_4Low.jtd" 0
```

수락 조건:

- `source-y-probe-compare`가 `table-flow-y-hypothesis`를 출력한다.
- 해당 hypothesis는 `strideCorrelationObserved=true`, `transformProven=false`, `renderAdmissible=false`여야 한다.
- `page-layer-tree`가 `sourceOnlyPageYRenderAdmissionGate.admissionReady=false`를 유지해야 한다.
- `page-layer-tree`가 `sourceOnlyPageYAdmissionClass=flow-y-stride-only-diagnostic`을 출력해야 한다.
- `pageOriginAuthority=fallbackTextAnchors`가 유지되어야 한다.

그 전까지 올바른 동작은 진단 출력과 blocked-safe non-promotion이다. 즉, 신호는 보여주되 렌더링으로 승격하지 않는다.
