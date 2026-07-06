# WIP 분리와 복귀 계획

## 현재 상태

source-table-layout promotion 보조 작업은 다음 결론까지 끝났다.

- 샘플 코퍼스 정리: `ichitaro-source-y-probe/`를 `rjtd-testdata/local-samples/ichitaro-source-y-probe/` 아래로 이동하고 canonical 하위 구조를 정리했다.
- PAGE01 X축: right-tick 샘플은 `/DocumentText` line-header X 신호가 선형이지만 source-only axis admission은 false다.
- PAGE01 Y축: down 샘플은 `/LineMark` stride correlation이 있지만 table-only page-Y origin proof가 아니며 `flow-y-stride-only-diagnostic`으로 유지한다.
- Probe CLI: 출력 row/gate 이름을 `probe-cli-output-contract.md`로 고정했다.

## 커밋 단위 후보

아직 커밋하지 않는다. 사용자가 커밋을 요청하면 아래 단위로 나누는 것이 안전하다.

### 1. 샘플 코퍼스 경로 정리

포함 후보:

- `.gitignore`
- `rjtd-testdata/local-samples/README.md`
- `rjtd-testdata/local-samples/README.ja.md`

의미:

- private local sample corpus가 nested 구조로 들어가도 git에 노출되지 않게 한다.
- organized corpus path를 문서화한다.

주의:

- 실제 JTD/PDF/PNG 샘플은 ignore 대상이다.
- `.omo/ulw-loop/task-11-tsaiten-source-readiness-20260701/ledger.jsonl`은 별도 ULW 상태 파일이므로 이 단위에 섞지 않는다.

### 2. source-y probe CLI와 로컬 샘플 테스트

포함 후보:

- `rjtd/crates/rjtd-cli/src/main.rs`
- `rjtd/crates/rjtd-cli/src/probe_compare.rs`
- `rjtd/crates/rjtd-cli/src/probe_corpus.rs`
- `rjtd/crates/rjtd-cli/src/probe_line_diff.rs`
- `rjtd/crates/rjtd-cli/src/probe_manifest.rs`
- `rjtd/crates/rjtd-cli/src/probe_page_diff.rs`
- `rjtd/crates/rjtd-cli/src/probe_signals.rs`
- `rjtd/crates/rjtd-cli/src/probe_signals/`
- `rjtd/crates/rjtd-cli/src/probe_validation.rs`
- `rjtd/crates/rjtd-cli/tests/source_y_probe.rs`

의미:

- `source-y-probe-audit`
- `source-y-probe-compare`
- PAGE01 line-header X / flow-Y diagnostic rows
- 로컬 ignored 회귀 테스트

검증:

```bash
cd rjtd
cargo fmt --all --check
cargo check -p rjtd-cli
cargo test -p rjtd-cli --test source_y_probe -- --ignored --nocapture
```

주의:

- `source-y-probe-compare`는 diagnostic command다.
- 이 단위만으로 render admission을 열지 않는다.

### 3. model layer admission gate 보강

포함 후보:

- `rjtd/crates/rjtd-model/src/lib.rs`

의미:

- `sourceOnlyPageYAdmissionClass`
- `sourceOnlyPageYRenderAdmissionGate`
- `sourceOnlyAxisAdmissionGate` 차단 상태 보존
- PAGE01 right/down family의 non-admissible layer-tree contract

검증:

```bash
cd rjtd
cargo check -p rjtd-cli
cargo test -p rjtd-cli --test source_y_probe -- --ignored --nocapture
```

주의:

- `admissionReady:false`를 유지하는 보강이다.
- `renderPromoted` 또는 visible table geometry promotion과 묶으면 안 된다.

### 4. TODO/RFC/evidence 문서

포함 후보:

- `TODO.md`
- `TODO.ja.md`
- `openjtd-spec/rfc/0008-object-stream-candidates.md`
- `openjtd-spec/rfc/0008-object-stream-candidates.ja.md`
- `.omo/evidence/source-table-layout-promotion-20260705/`
- `.omo/ulw-loop/source-table-layout-promotion-20260705/`

의미:

- PAGE01 X/Y 비승격 결론
- Probe CLI output contract
- 추가 샘플이 왜 render promotion proof가 아닌지 handoff

주의:

- `.omo/evidence/`는 보통 추적 대상이 아니다. 사용자가 산출물 버전 관리를 원할 때만 포함한다.
- RFC/TODO는 코드 gate 이름과 blocker string을 맞춰야 한다.

### 5. 기존 current-progress-next-plan WIP

포함 후보:

- `.omo/evidence/current-progress-next-plan/*`
- `.omo/ulw-loop/task-11-tsaiten-source-readiness-20260701/ledger.jsonl`
- 기존 tracked `rjtd-model`/CLI 변경 중 source-table-layout promotion과 직접 무관한 항목

의미:

- 원래 큰 플랜의 `task-11 tsaiten source readiness`와 연결된 WIP다.

주의:

- source-y probe 작업과 한 커밋에 섞지 않는다.
- ULW goal 상태 mismatch가 있었으므로, 커밋 전에 최신 계획과 실제 diff를 다시 대조한다.

## 원래 플랜으로 돌아가는 순서

1. source-table-layout promotion 보조 작업은 `diagnostic-only / non-admissible` 결론으로 닫는다.
2. `current-progress-next-plan.md`의 unchecked lane으로 복귀한다.
3. 우선순위는 다음 순서가 안전하다.
   - task 11: `tsaiten` source-only readiness가 양쪽 visible table family에서 같은 admission contract를 만족하는지 확인한다.
   - task 18: 전체 Rust 검증을 실행하고 실패를 introduced/pre-existing/environmental로 분류한다.
   - task 19: visual QA는 task 18 이후 실행한다.
4. source-y probe에서 얻은 blocker는 `tsaiten` 승격 기준을 낮추는 근거가 아니다. 오히려 `sourceOnlyAxisAdmissionGate`와 `sourceOnlyPageYRenderAdmissionGate`가 false이면 visible fallback을 유지해야 한다는 반례로 사용한다.

## 현재까지 실행한 검증

```bash
cd rjtd
cargo fmt --all --check
cargo check -p rjtd-cli
cargo test -p rjtd-cli --test source_y_probe -- --ignored --nocapture
```

결과:

- format: pass
- check: pass
- local ignored source-y probe tests: `7 passed`
