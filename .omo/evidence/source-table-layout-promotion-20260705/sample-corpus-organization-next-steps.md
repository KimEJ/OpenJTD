# 샘플 코퍼스 정리와 다음 작업

## 정리 결과

샘플 코퍼스는 루트 `ichitaro-source-y-probe/`에서 아래 위치로 이동했다.

`rjtd-testdata/local-samples/ichitaro-source-y-probe/`

새 구조:

- `corpus/baseline-sweep/`: 기존 `files/`에 있던 broad source-y probe JTD/PDF 쌍
- `corpus/page01-grid/`: 기존 `new/`에 있던 PAGE 01 baseline/right/down/DOWNTEST JTD/PDF 쌍
- `scratch/misc-editor-saves/`: `asdf`, `zxcv`, `○○○`, `来訪メモ` 같은 임시 저장본
- `scratch/ichitaro-lock-files/`: Ichitaro `*.$td` 잠금 파일
- `evidence/screenshots/`: `screen-current*.png`
- `notes/duplicates/`: `README 2.md`, `manifest 2.csv` 등 복사본

정책:

- canonical 샘플은 `corpus/` 아래만 본다.
- `scratch/`는 보존하지만 회귀/승격 근거로 쓰지 않는다.
- `evidence/`는 분석 산출물이고, JTD/PDF 샘플 근거 자체는 아니다.
- `rjtd-testdata/local-samples/ichitaro-source-y-probe/` 전체는 private local sample corpus로 git ignore 대상이다.

## 다음 작업 후보

1. PAGE01 X축 승격 가능성 검토 - 완료
   - right-tick 샘플은 `firstCellOffsetDelta=2/4/6/8`로 source 신호가 비교적 직접적이다.
   - 결론: X 신호는 진단으로 고정 가능하지만 렌더 승격은 아직 불가하다.
   - 근거: `page01-x-axis-admission-analysis.md`
   - 회귀 테스트: `local_new_probe_layer_tree_keeps_right_shift_non_admissible_when_available`

2. PAGE01 Y축 결론을 TODO/RFC에 반영 - 완료
   - `flow-y-stride-only-diagnostic`
   - `admissionReady=false`
   - `pageOriginAuthority=fallbackTextAnchors`
   - "표만 Y 독립 이동 불가" 전제를 문서화한다.

3. probe CLI 산출물 정리 - 완료
   - `source-y-probe-audit`
   - `source-y-probe-compare`
   - `table-flow-y-summary`
   - `table-flow-y-hypothesis`
   - 위 명령/출력 이름을 진행 문서와 handoff에 맞췄다.
   - `source-y-probe-audit`는 코퍼스 루트와 `corpus/baseline-sweep/` 직접 입력을 모두 같은 manifest/files layout으로 해석한다.
   - 근거: `probe-cli-output-contract.md`

4. WIP commit 단위 설계 - 완료
   - 샘플 경로 정리
   - probe CLI 모듈 추가
   - PAGE01 Y 비승격 gate
   - 문서/evidence 정리
   - 서로 섞지 않는 atomic 단위로 나눠야 한다.
   - 근거: `wip-split-and-return-plan.md`

5. 원래 `current-progress-next-plan.md`로 복귀 - 완료
   - source-y probe는 별도 결론이 났다.
   - `task-11 tsaiten source readiness`를 최신 layer-tree 출력으로 재확인했다.
   - 결론: 승격하지 않는다. scoring visible table은 source-derived candidate가 생겼지만 `source-derived-layout-not-renderable` 상태이고, lower visible table은 `source-page-y-render-admission-not-ready` 상태다.
   - 두 visible table 모두 `referenceFallbackUsed:true`, `sourceOnlyPageYAdmissionReady:false`, `sourceOnlyAxisAdmissionGate.admissionReady:false`를 유지한다.
   - 따라서 task 18 full Rust verification과 task 19 visual QA는 실행 가능 검증 명령으로는 돌릴 수 있지만, 원 플랜의 "task 11 이후 승격 검증" 의미로 완료 처리하면 안 된다.
   - 근거: `../current-progress-next-plan/task-11-resume-after-page01-20260706.md`
