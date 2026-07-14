# 수동 검토 가이드

생성 시각: 2026-07-05T02:15:05+09:00

이 폴더의 `files/` 아래에는 현재 Ichitaro `.jtd` 파일과 Ichitaro에서 내보낸 `.pdf` 파일이 서로 매칭된 39개 쌍이 들어 있습니다. 최신 패스에서는 Computer Use로 생성된 RTF 원본을 Ichitaro에서 연 뒤, 동시에 PDF 내보내기를 켠 상태로 `.jtd`로 저장하여 샘플 28개를 추가했습니다.

## 출처

- 네이티브/직접 Ichitaro UI 샘플: `000_base_a`, `001_base_b_resave`, `010_table_moved_down_small`, `011_table_moved_down_large`, `020_row1_height_plus`, `021_row2_height_plus`, `022_row3_height_plus`, `040_top_margin_plus`, `050_wrapped_one_cell`, `070_two_tables_vertical`, `080_plain_paragraph_lines_only`.
- RTF 가져오기 기반 대체 샘플: `010a_table_after_1_paragraph`, `011a_table_after_4_paragraphs`, `012a_table_after_8_paragraphs`, `013_table_moved_right`, `030_col1_width_plus`, `031_col2_width_plus`, `032_table_width_plus_both_cols`, `040a_top_margin_20mm`, `040b_top_margin_30mm_baseline`, `040c_top_margin_40mm`, `040d_top_margin_50mm`, `040e_top_margin_60mm`, `053_font_size_table_plus`, `054_font_size_paragraph_plus`, `055_table_cell_line_spacing_plus`, `056_paragraph_line_spacing_plus`, `060_table_3x3`, `061_table_2x5`, `062_table_1x3`, `063_table_4x2`, `064_merged_header`, `066_empty_cells`, `074a_many_paragraphs_then_small_table_page2`, `074b_table_near_page_bottom_no_split`, `074c_table_crosses_page_boundary`, `074d_many_row_table_2col_simple`, `081_plain_paragraph_line_spacing_plus`, `082_plain_paragraph_font_size_plus`.
- 실패한 네이티브/직접 샘플: `074_multi_page_table`. 이 ID에 대해서는 오염된 자리표시자 아티팩트를 저장하지 않았습니다.

RTF 가져오기 샘플은 폭넓은 파서 검증과 source-y 탐색에는 유용하지만, 네이티브 Ichitaro 작성 의미론을 엄밀하게 증명하지는 않습니다. 시각 검토가 끝날 때까지는 생성된 대체 샘플로 취급하세요.

## 반드시 검토하거나 네이티브로 다시 만들 항목

- `010a_table_after_1_paragraph`, `011a_table_after_4_paragraphs`, `012a_table_after_8_paragraphs`: table-y 대체 샘플로만 사용하세요. 정확한 Ichitaro 객체 이동이 필요하면 네이티브 문단 삽입과 표 배치로 다시 만드세요.
- `040a_top_margin_20mm`부터 `040e_top_margin_60mm`까지: 정확한 델타로 사용하기 전에 가져온 여백을 네이티브 `040_top_margin_plus` 샘플과 대조해 검증하세요.
- `053_font_size_table_plus`, `054_font_size_paragraph_plus`, `055_table_cell_line_spacing_plus`, `056_paragraph_line_spacing_plus`, `081_plain_paragraph_line_spacing_plus`, `082_plain_paragraph_font_size_plus`: 레이아웃 기준값으로 사용하기 전에 가져온 글꼴 및 줄 간격 메트릭을 검증하세요.
- `064_merged_header`: 현재 샘플은 근사치입니다. RTF 원본은 실제 병합된 헤더 셀 하나가 아니라 두 개의 표 블록을 사용합니다.
- `074a_many_paragraphs_then_small_table_page2`, `074b_table_near_page_bottom_no_split`, `074c_table_crosses_page_boundary`, `074d_many_row_table_2col_simple`: 페이지 경계 동작은 레이아웃에 민감합니다. 정확한 행 이어짐 또는 분할/비분할 의미론이 중요하면 네이티브로 다시 만드세요.

## 수동 생성 절차

1. Ichitaro를 열고 새 문서를 시작합니다.
2. OpenJTD/PDF 추출 결과가 텍스트를 안정적으로 비교할 수 있도록 `P01`, `R01C01`, `R02C02` 같은 단순 ASCII 라벨을 사용합니다.
3. 엄밀한 의미론이 필요할 때는 RTF를 가져오지 말고 Ichitaro의 표/괘선 도구로 의도한 표를 네이티브로 만듭니다.
4. 열 너비 샘플은 대상 열 경계를 직접 드래그하고 다른 열은 고정 상태로 유지합니다.
5. 행 높이 샘플은 대상 행 경계를 직접 드래그하고 앞쪽 행의 위쪽 위치는 고정 상태로 유지합니다.
6. 여백 샘플은 문서 스타일/페이지 설정 대화상자를 사용하고 정확한 밀리미터 값을 `manifest.csv`에 기록합니다.
7. 페이지 경계 샘플은 표 앞에 충분한 문단을 삽입한 뒤 저장하고 PDF로 내보낸 다음, 표가 의도한 페이지에서 시작하는지, 이어지는지, 또는 분할되는지 시각적으로 확인합니다.
8. `ichitaro-source-y-probe/files/`에 `.jtd`로 저장하고, 매칭되는 `.pdf`가 옆에 생기도록 Ichitaro 동시 PDF 내보내기를 켜 둡니다.
9. 빈 페이지 또는 내용이 드문 페이지가 의도된 경우에만 Ichitaro의 빈 페이지 경고를 수락합니다.
10. `*.jtd.$$$` 파일을 삭제하기 전에 Ichitaro를 닫습니다. 이 파일들은 코퍼스 아티팩트가 아니라 잠금/백업 파일입니다.

## UI 참고 사항

- 접근성 계층에 따라 메뉴 라벨이 일본어로 보이거나 문자가 깨져 보일 수 있습니다. 검토 작업에서는 허용 가능한 상태이므로, 정확한 메뉴 텍스트보다 안정적인 위치와 대화상자 구조에 의존하세요.
- 이 자동화 워크플로를 재현할 때 RTF 가져오기 표 변환 대화상자에서는 Word 호환 프레임 옵션이 아니라 괘선/표 변환 옵션을 사용해야 합니다.
- 체험판 또는 백업 알림이 나타나면 문서가 올바르게 저장되었는지 판단하기 전에 먼저 닫으세요.
- 샘플을 생성 완료로 표시하기 전에 디스크에서 `sample_id.jtd`와 `sample_id.pdf`가 모두 존재하는지 항상 확인하세요.
