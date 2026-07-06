# Ichitaro JTD Source-Y Probe Corpus Automation Brief

이 문서는 Codex/LLM/RPA 자동화 작업자에게 그대로 전달하기 위한 실행 지시서다.

목표는 Ichitaro 체험판에서 통제된 `.jtd` 원본과 Ichitaro가 직접 출력한 `.pdf` 쌍을 생성해, OpenJTD/rjtd 쪽에서 JTD 내부 `/PageMark`, `/LineMark`, `/PaperMark`, `/DocumentTextPositionTables` 계열 필드가 표의 y좌표, 행 높이, 페이지 위치, 줄 간격과 어떻게 대응되는지 차분 분석할 수 있게 만드는 것이다.

이 작업은 공개 포맷 스펙이 없다는 전제에서 진행한다. 따라서 샘플은 "보기 좋은 문서"가 아니라 "하나의 변수만 바꾼 실험 데이터"여야 한다.

## 0. 작업 원칙

1. 각 케이스는 반드시 `.jtd` 원본과 Ichitaro에서 직접 출력한 `.pdf`를 함께 만든다.
2. 한 케이스에서는 변수 하나만 바꾼다. 나머지 폰트, 글자 크기, 용지, 여백, 표 스타일, 문단 수, 셀 내용은 유지한다.
3. PDF를 출력한 뒤 같은 JTD를 다시 수정하지 않는다.
4. 다른 워드프로세서, 변환기, 프린터 드라이버, DOCX/RTF 경유 저장을 사용하지 않는다.
5. PDF는 반드시 Ichitaro의 PDF 출력/내보내기 기능으로 생성한다. 운영체제의 "Microsoft Print to PDF"나 브라우저 출력은 사용하지 않는다.
6. 파일명은 이 문서의 케이스 ID와 정확히 맞춘다.
7. 생성 실패, 체험판 제한, UI 자동화 실패가 있으면 빈 파일을 만들지 말고 `manifest.csv`에 `omitted`와 이유를 적는다.
8. 가능한 모든 텍스트는 ASCII 라벨을 사용한다. 예: `P01`, `R01C01`, `T1R02C03`.
9. 표는 Ichitaro의 기본 표 기능으로 만든다. 텍스트 박스, 도형, 이미지, 수식, 주석, 각주, 머리말/꼬리말, 플로팅 개체는 사용하지 않는다.
10. 표 테두리는 켠다. 선 종류/두께는 기본값으로 둔다.
11. 자동화가 정확한 수치를 입력할 수 없으면 "작게/중간/크게"처럼 시각적으로 뚜렷한 차이를 만들고, 실제 조작 내용을 `manifest.csv`의 `notes`에 적는다.
12. 샘플을 예쁘게 맞추려고 임의 보정을 하지 않는다. 실험 변수 외의 변경은 금지한다.

## 1. 산출물 구조

최종 산출물은 아래 구조의 폴더와 zip 파일이다.

```text
ichitaro-source-y-probe/
  README.md
  manifest.csv
  environment.txt
  automation-log.txt
  files/
    000_base_a.jtd
    000_base_a.pdf
    001_base_b_resave.jtd
    001_base_b_resave.pdf
    ...
```

최종 zip 이름:

```text
ichitaro-source-y-probe.zip
```

## 2. 환경 기록

`environment.txt`에 아래 정보를 기록한다.

```text
ichitaro_product_name:
ichitaro_version:
trial_or_paid: trial
os_name:
os_version:
locale:
default_printer:
pdf_export_method:
pdf_export_settings:
jtd_save_format_or_version:
font_default:
font_size_default:
paper_default:
margin_default:
created_at_local:
automation_tool:
operator_notes:
```

확실하지 않은 항목은 비워두지 말고 `unknown`이라고 적는다.

## 3. Manifest 스키마

`manifest.csv`는 UTF-8 CSV로 작성한다. 헤더는 정확히 아래와 같이 한다.

```csv
id,filename_stem,base_id,priority,status,changed_variable,expected_change,paper,orientation,margin_note,table_count,table_shape,row_count,column_count,paragraph_count_before,paragraph_count_after,creation_method,pdf_export_method,notes
```

`status` 값:

```text
created
omitted
failed
```

`priority` 값:

```text
P0
P1
P2
P3
```

P0는 최소 필수, P1은 강력 권장, P2는 있으면 분석력이 크게 올라가는 항목, P3는 보수적 확장 항목이다.

## 4. 공통 기준 문서 설정

기준 문서 계열은 아래 설정을 사용한다.

- 용지: A4
- 방향: 세로
- 여백: Ichitaro 기본값
- 기본 글꼴: Ichitaro 기본값
- 기본 글자 크기: Ichitaro 기본값 또는 10.5pt
- 표: 2열 x 3행
- 표 테두리: 켬
- 표 위 일반 문단: 1줄, 텍스트 `P01`
- 표 아래 일반 문단: 없음, 별도 지시가 있는 케이스만 추가
- 셀 텍스트:
  - 1행 1열: `R01C01`
  - 1행 2열: `R01C02`
  - 2행 1열: `R02C01`
  - 2행 2열: `R02C02`
  - 3행 1열: `R03C01`
  - 3행 2열: `R03C02`

## 5. 작업 순서

1. 새 폴더 `ichitaro-source-y-probe/files/`를 만든다.
2. `000_base_a`를 새 문서로 직접 만든다.
3. `001_base_b_resave`는 `000_base_a`를 열어 아무 변경 없이 다른 이름으로 저장하고 PDF 출력한다. 이 항목은 저장 비결정성/ID 변동 확인용이다.
4. 이후 케이스는 가능한 한 `000_base_a`를 복제해서 만든다.
5. 각 케이스마다 JTD 저장 후 바로 PDF를 출력한다.
6. `manifest.csv`를 계속 갱신한다.
7. 마지막에 모든 `.jtd`와 `.pdf` 쌍이 있는지 검사한다.
8. `README.md`, `environment.txt`, `automation-log.txt`, `manifest.csv`와 `files/`를 zip으로 묶는다.

## 6. 최소 성공 기준

시간이나 체험판 제한이 있으면 최소한 아래 P0 케이스를 완료한다.

- `000_base_a`
- `001_base_b_resave`
- `010_table_moved_down_small`
- `011_table_moved_down_large`
- `020_row1_height_plus`
- `021_row2_height_plus`
- `022_row3_height_plus`
- `030_top_margin_plus`
- `040_wrapped_one_cell`
- `050_two_tables_vertical`
- `060_multi_page_table`
- `070_plain_paragraph_lines_only`

P0가 하나라도 실패하면 `manifest.csv`에 실패 이유를 자세히 적는다.

## 7. 케이스 목록

### A. 기준/비결정성 확인

#### 000_base_a

- priority: P0
- base_id: none
- 새 문서로 직접 생성
- 공통 기준 문서 설정 그대로
- expected_change: baseline

#### 001_base_b_resave

- priority: P0
- base_id: 000_base_a
- `000_base_a`를 열어 내용 변경 없이 다른 이름으로 저장
- expected_change: no visible change; binary/source ID changes only if Ichitaro rewrites metadata

#### 002_base_c_from_scratch

- priority: P1
- base_id: none
- `000_base_a`와 같은 내용을 새 문서에서 다시 직접 생성
- expected_change: same visual layout, independent object IDs/stream ordering possible

### B. 표 전체 y 위치 이동

#### 010_table_moved_down_small

- priority: P0
- base_id: 000_base_a
- 표 전체만 아래로 조금 이동
- 방법 우선순위:
  1. 표 앞에 빈 문단 1줄 추가
  2. UI가 지원하면 표 개체 위치만 아래로 이동
- expected_change: table top y increases slightly; row heights unchanged

#### 011_table_moved_down_large

- priority: P0
- base_id: 000_base_a
- 표 전체만 아래로 크게 이동
- 표 앞에 일반 문단 `P02`, `P03`, `P04`, `P05`를 추가
- expected_change: table top y increases substantially; row heights unchanged

#### 012_table_moved_up_if_possible

- priority: P1
- base_id: 000_base_a
- 표가 기준보다 위에 오도록 조정
- 가능하면 표 위 문단 `P01`을 제거하거나 표 위치를 위로 이동
- expected_change: table top y decreases

#### 013_table_moved_right

- priority: P1
- base_id: 000_base_a
- 표 전체만 오른쪽으로 이동
- y 관련 값이 바뀌면 안 되는 x축 음성 대조군
- expected_change: x changes; y should remain stable

#### 014_table_moved_left_if_possible

- priority: P2
- base_id: 000_base_a
- 표 전체만 왼쪽으로 이동 가능하면 생성
- expected_change: x changes; y should remain stable

### C. 행 높이 변화

#### 020_row1_height_plus

- priority: P0
- base_id: 000_base_a
- 1행 높이만 크게
- expected_change: row1 height increases; row2/row3 top y shifts downward

#### 021_row2_height_plus

- priority: P0
- base_id: 000_base_a
- 2행 높이만 크게
- expected_change: row2 height increases; row3 top y shifts downward

#### 022_row3_height_plus

- priority: P0
- base_id: 000_base_a
- 3행 높이만 크게
- expected_change: only final table bottom changes; earlier row tops stable

#### 023_all_rows_height_plus

- priority: P1
- base_id: 000_base_a
- 모든 행 높이를 같은 양만큼 크게
- expected_change: all row deltas increase uniformly

#### 024_row1_height_plus_large

- priority: P1
- base_id: 000_base_a
- 1행 높이를 `020`보다 더 크게
- expected_change: same field family as 020 with larger magnitude

#### 025_row_heights_stair_step

- priority: P2
- base_id: 000_base_a
- 1행 작게, 2행 중간, 3행 크게
- expected_change: distinct row delta sequence

### D. 열 너비/x축 음성 대조군

#### 030_col1_width_plus

- priority: P1
- base_id: 000_base_a
- 1열 너비만 크게
- expected_change: column width/x changes; table y/row heights should remain stable unless wrapping occurs

#### 031_col2_width_plus

- priority: P1
- base_id: 000_base_a
- 2열 너비만 크게
- expected_change: column width/x changes; table y/row heights should remain stable unless wrapping occurs

#### 032_table_width_plus_both_cols

- priority: P2
- base_id: 000_base_a
- 두 열 너비를 모두 키워 표 전체 폭 증가
- expected_change: x/width changes; y should remain stable

### E. 여백/용지/페이지 좌표계

#### 040_top_margin_plus

- priority: P0
- base_id: 000_base_a
- 상단 여백만 크게
- expected_change: page content y origin/table y shifts or page-space transform changes

#### 041_top_margin_minus_if_possible

- priority: P1
- base_id: 000_base_a
- 상단 여백만 작게
- expected_change: inverse of 040

#### 042_bottom_margin_plus

- priority: P1
- base_id: 000_base_a
- 하단 여백만 크게
- expected_change: page capacity changes; table top may remain stable

#### 043_left_margin_plus

- priority: P2
- base_id: 000_base_a
- 왼쪽 여백만 크게
- expected_change: x origin changes; y should remain stable

#### 044_paper_landscape

- priority: P1
- base_id: 000_base_a
- 용지 방향만 가로
- expected_change: page geometry changes; table local row deltas should remain interpretable

#### 045_paper_b5_if_available

- priority: P2
- base_id: 000_base_a
- 용지만 B5로 변경 가능하면 생성
- expected_change: page geometry changes

#### 046_paper_letter_if_available

- priority: P3
- base_id: 000_base_a
- 용지만 Letter로 변경 가능하면 생성
- expected_change: page geometry changes

### F. 셀 내부 줄바꿈/폰트/줄 간격

#### 050_wrapped_one_cell

- priority: P0
- base_id: 000_base_a
- `R02C01` 셀에 긴 문장을 넣어 셀 내부 줄바꿈 발생
- 추천 텍스트: `R02C01 LONG TEXT LONG TEXT LONG TEXT LONG TEXT LONG TEXT`
- expected_change: row2 height may increase automatically

#### 051_wrapped_row1_cell

- priority: P1
- base_id: 000_base_a
- `R01C01`만 긴 텍스트로 변경
- expected_change: row1 height may increase; lower rows shift

#### 052_wrapped_each_row

- priority: P1
- base_id: 000_base_a
- 각 행 1열에 긴 텍스트를 넣어 각 행에 줄바꿈이 생기게 함
- expected_change: multiple row heights change

#### 053_font_size_table_plus

- priority: P1
- base_id: 000_base_a
- 표 안 텍스트 글자 크기만 크게
- expected_change: cell text metrics and possibly row heights change

#### 054_font_size_paragraph_plus

- priority: P2
- base_id: 000_base_a
- 표 위 일반 문단 글자 크기만 크게
- expected_change: table top y may shift; table row deltas unchanged

#### 055_table_cell_line_spacing_plus

- priority: P1
- base_id: 000_base_a
- 표 안 텍스트 줄 간격만 크게
- expected_change: cell line metric/row height changes

#### 056_paragraph_line_spacing_plus

- priority: P1
- base_id: 000_base_a
- 표 위 일반 문단 줄 간격만 크게
- expected_change: table top y may shift; table row deltas unchanged

### G. 표 구조 변형

#### 060_table_3x3

- priority: P1
- base_id: none
- 3열 x 3행 표
- 모든 셀 `R01C01` 식 라벨
- expected_change: different column count, same row count

#### 061_table_2x5

- priority: P1
- base_id: none
- 2열 x 5행 표
- expected_change: more row boundary records

#### 062_table_1x3

- priority: P2
- base_id: none
- 1열 x 3행 표
- expected_change: simpler column structure

#### 063_table_4x2

- priority: P2
- base_id: none
- 4열 x 2행 표
- expected_change: more columns, fewer rows

#### 064_merged_header

- priority: P1
- base_id: 000_base_a
- 1행 두 셀 병합
- 병합 셀 텍스트: `HEADER`
- expected_change: merge metadata appears; row y should still be recoverable

#### 065_merged_left_column

- priority: P2
- base_id: 000_base_a
- 1열의 1~2행 병합
- expected_change: vertical merge metadata may affect row/cell records

#### 066_empty_cells

- priority: P1
- base_id: 000_base_a
- `R02C02`, `R03C01`을 빈 셀로 둔다
- expected_change: layout should remain; text-position records may be absent for empty cells

#### 067_no_borders

- priority: P2
- base_id: 000_base_a
- 표 테두리만 끈다
- expected_change: border style changes; row y should remain stable

#### 068_thick_borders

- priority: P3
- base_id: 000_base_a
- 표 선 두께만 굵게
- expected_change: border metrics may affect table bbox or not

### H. 다중 표/페이지

#### 070_two_tables_vertical

- priority: P0
- base_id: none
- 같은 페이지에 2열 x 2행 표 2개
- 위 표 셀: `T1R01C01` 등
- 아래 표 셀: `T2R01C01` 등
- 두 표 사이 일반 문단 2줄: `BETWEEN01`, `BETWEEN02`
- expected_change: two table families on one page

#### 071_two_tables_close

- priority: P1
- base_id: none
- 같은 페이지에 표 2개를 최대한 가깝게 배치
- expected_change: adjacent table-family separation stress test

#### 072_table_after_10_paragraphs

- priority: P1
- base_id: none
- 표 위에 일반 문단 `P01`부터 `P10`까지 10줄
- 그 아래 2열 x 3행 표
- expected_change: table appears lower due to ordinary paragraphs

#### 073_table_near_page_bottom

- priority: P1
- base_id: none
- 표가 페이지 하단 근처에 오게 배치하되 다음 페이지로 넘어가지 않게 함
- expected_change: near-bottom y coordinates without page break

#### 074_multi_page_table

- priority: P0
- base_id: none
- 행을 충분히 많이 만들어 표가 2페이지로 넘어가게 함
- 2열, 최소 40행 권장
- 각 행 라벨: `R001C01`, `R001C02`, ...
- expected_change: row continuation/page break semantics

#### 075_two_page_two_tables

- priority: P1
- base_id: none
- 1페이지에 표 1개, 2페이지에 표 1개
- 각 표 앞에 페이지 식별 문단 `PAGE1`, `PAGE2`
- expected_change: same table pattern repeated on different page origins

#### 076_table_split_by_manual_page_break

- priority: P2
- base_id: none
- 표 앞에 수동 페이지 나누기를 넣거나, 첫 페이지 끝/둘째 페이지 시작을 명확히 만드는 문서
- expected_change: manual page break records versus automatic flow

### I. 일반 문단/LineMark 음성 및 양성 대조군

#### 080_plain_paragraph_lines_only

- priority: P0
- base_id: none
- 표 없이 일반 문단 12줄만 작성
- 텍스트: `P01`부터 `P12`
- expected_change: ordinary line marks without table records

#### 081_plain_paragraph_line_spacing_plus

- priority: P1
- base_id: 080_plain_paragraph_lines_only
- 일반 문단 줄 간격만 크게
- expected_change: LineMark-related deltas change without table PageMark family

#### 082_plain_paragraph_font_size_plus

- priority: P1
- base_id: 080_plain_paragraph_lines_only
- 일반 문단 글자 크기만 크게
- expected_change: text metrics/line height change without table records

#### 083_plain_paragraph_top_margin_plus

- priority: P2
- base_id: 080_plain_paragraph_lines_only
- 상단 여백만 크게
- expected_change: page origin/content top relation changes without table

### J. 같은 시각 결과를 다른 원인으로 만드는 대조군

#### 090_table_lower_by_paragraphs

- priority: P2
- base_id: 000_base_a
- 표 위에 일반 문단을 추가해 표를 아래로 보냄
- expected_change: flow layout changes before table

#### 091_table_lower_by_direct_position_if_possible

- priority: P2
- base_id: 000_base_a
- 문단 추가 없이 표 개체 위치 조정으로 표를 아래로 보냄
- 지원되지 않으면 omitted
- expected_change: visual y similar to 090, source cause different

#### 092_same_row_height_by_manual_and_wrap

- priority: P3
- base_id: none
- `020_row1_height_plus`와 비슷한 1행 높이를 셀 줄바꿈으로 만들기
- expected_change: visual row height similar, source cause different

### K. 보수적 확장/버전 안정성

#### 100_save_as_template_if_available

- priority: P3
- base_id: 000_base_a
- JTT/JTTC 템플릿 저장이 가능하면 같은 기준 문서를 템플릿 형식으로 저장하고 PDF 출력
- 파일 확장자는 실제 저장 형식에 맞춘다
- expected_change: template container comparison

#### 101_jtd_compatibility_save_if_available

- priority: P3
- base_id: 000_base_a
- 이전 버전 JTD 호환 저장 옵션이 있으면 사용
- expected_change: stream layout/version differences

#### 102_open_resave_after_pdf

- priority: P3
- base_id: 000_base_a
- `000_base_a`를 다시 열고 아무 변경 없이 재저장한 뒤 PDF 출력
- expected_change: persistence stability check

## 8. README.md 내용

`README.md`에는 아래 내용을 포함한다.

```md
# Ichitaro Source-Y Probe Corpus

This corpus was generated with Ichitaro for reverse-engineering source-only table/page-y layout semantics in JTD files.

## Important

- PDFs were exported directly from Ichitaro.
- PDFs are observational references only.
- JTD files are the primary source artifacts.
- Each case changes one intended variable from its base document.

## Folder

- `files/`: JTD/PDF pairs.
- `manifest.csv`: case metadata.
- `environment.txt`: software and export environment.
- `automation-log.txt`: creation log and omissions.

## Known limitations

<write trial limitations, failed cases, uncertain UI settings, and any manual interventions here>
```

## 9. automation-log.txt 내용

`automation-log.txt`에는 아래를 시간순으로 적는다.

```text
[timestamp] started Ichitaro
[timestamp] created 000_base_a.jtd
[timestamp] exported 000_base_a.pdf
[timestamp] created 001_base_b_resave.jtd
...
[timestamp] omitted 045_paper_b5_if_available: B5 not available in UI
```

## 10. 검수 체크리스트

최종 zip 생성 전 아래를 검사한다.

1. `manifest.csv`의 `status=created` 행마다 `files/<filename_stem>.jtd`가 존재한다.
2. `manifest.csv`의 `status=created` 행마다 `files/<filename_stem>.pdf`가 존재한다.
3. PDF를 열었을 때 각 문서의 표/문단 라벨이 보인다.
4. `000_base_a`와 `001_base_b_resave`는 시각적으로 같아야 한다.
5. P0 케이스는 생성 또는 실패 사유가 반드시 기록되어야 한다.
6. omitted/failed 케이스는 빈 `.jtd` 또는 `.pdf` 파일을 만들지 않는다.
7. zip 내부 최상위 폴더 이름은 `ichitaro-source-y-probe/`다.

## 11. 자동화 작업자에게 주는 판단 기준

- 자동화가 UI에서 특정 값을 정확히 입력하지 못하면, 해당 케이스를 포기하지 말고 시각적으로 명확한 차이를 만든 뒤 `notes`에 실제 조작을 기록한다.
- 단, 실험 변수 외의 항목이 함께 바뀌는 경우에는 케이스를 `failed`로 기록하고 파일을 산출하지 않는다.
- PDF 출력이 실패하면 JTD만 제출하지 말고 해당 케이스를 `failed`로 기록한다.
- Ichitaro 체험판 제한으로 특정 저장 형식이나 PDF 출력이 막히면 `environment.txt`와 `manifest.csv`에 정확히 기록한다.
- 가장 중요한 것은 많은 케이스를 대충 만드는 것이 아니라, P0/P1 케이스를 변수 통제된 상태로 만드는 것이다.

## 12. 완료 보고 형식

작업이 끝나면 아래 형식으로 보고한다.

```text
Created: <number> cases
Omitted: <number> cases
Failed: <number> cases
P0 complete: yes/no
Output zip: <absolute path to ichitaro-source-y-probe.zip>
Main limitations:
- <limitation 1>
- <limitation 2>
```

