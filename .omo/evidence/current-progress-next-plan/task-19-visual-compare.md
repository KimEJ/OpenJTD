# Task 19 PDF visual comparison

Comparison scope: page 1 PNGs rendered from generated and reference PDFs with pdftoppm. Pixel metrics are computed on the shared top-left overlap when one raster axis differs by 1 px. This is evidence for visual review, not a claim of pixel equivalence.

contact_sheet: .omo/evidence/current-progress-next-plan/task-19-compare/task-19-page1-comparison-contact-sheet.png

| sample | generated raster | reference raster | mean abs channel diff | RMSE | pixels >5 | pixels >25 | pixels >80 | triptych |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| ichitaro-20030228030923-success-002-success_data-test | 1075x1518 | 1075x1518 | 11.467 | 11.451 | 9.491% | 7.383% | 5.521% | .omo/evidence/current-progress-next-plan/task-19-compare/ichitaro-20030228030923-success-002-success_data-test-page1-triptych.png |
| ichitaro-20030120132956-0007-sp-dat-tsaiten | 1241x1754 | 1241x1755 | 11.486 | 12.206 | 12.335% | 7.632% | 5.106% | .omo/evidence/current-progress-next-plan/task-19-compare/ichitaro-20030120132956-0007-sp-dat-tsaiten-page1-triptych.png |
| ichitaro-20030315134715-success-001-success_data-shanai_lan | 1754x1241 | 1755x1241 | 24.544 | 17.582 | 16.464% | 14.658% | 11.112% | .omo/evidence/current-progress-next-plan/task-19-compare/ichitaro-20030315134715-success-001-success_data-shanai_lan-page1-triptych.png |

## Visual notes

- `success_data-test`: page count and raster dimensions match. The layout is broadly aligned, with visible differences concentrated around stroke/text anti-aliasing and title/table outlines.
- `tsaiten`: page count matches, but the reference raster is 1 px taller. The generated page is visibly close but the title/table/text blocks do not overlay exactly; the diff highlights the main table and body text regions.
- `shanai_lan`: page count matches, but the generated raster is 1 px narrower. This is the largest visual mismatch: many reference network connector lines are absent or not aligned in the generated output, consistent with line-rule rendering still being gated rather than promoted.

Disposition: generated PDFs are usable for visual evidence, but this comparison does not establish pixel equivalence. `shanai_lan` remains the clearest visible gap.
