# Task 3 Tooling Availability Evidence

Task: checkbox 3 from `.omo/plans/current-progress-next-plan.md`, "Confirm local sample and PDF tooling availability".

Run context:
- Workspace: `/Users/kimuj5090/Documents/rjtd`
- Timestamp command: `date '+%Y-%m-%d %H:%M:%S %Z'`
- Timestamp output: `2026-07-01 09:51:06 KST`
- Scope: read-only availability checks; no PDF regeneration, no product edits, no generated output edits.

## Referenced Surfaces

- `README.md:30` documents `cargo test --workspace`.
- `README.md:39` documents `scripts/regenerate-pdf-output.sh`; this task did not run it.
- `rjtd/crates/rjtd-cli/tests/streams.rs:4331` skips local PDF-backed PageMark tests when `rjtd-testdata/local-samples` is absent.
- `rjtd/crates/rjtd-export/src/lib.rs:8079` skips local `shanai_lan` export diagnostics unless the local sample JTD and reference PDF exist.
- `rjtd/crates/rjtd-model/src/lib.rs:82706` skips local `shanai_lan` model diagnostics unless the local sample JTD and reference PDF exist.
- `rjtd/crates/rjtd-model/src/lib.rs:84951` skips local `tsaiten` model diagnostics unless the local sample JTD and reference PDF exist.

## Fresh Command Evidence

Surface: terminal command in `/Users/kimuj5090/Documents/rjtd`.

| Check | Exact invocation | Exit | Stdout | Verdict |
| --- | --- | ---: | --- | --- |
| Dirty worktree probe | `git status --short --branch` | 0 | `## dev...origin/dev`; modified tracked files include `TODO.md`, `TODO.ja.md`, RFCs, CLI/model/export files; untracked `.omo/`, `AGENTS.md`, `rjtd/AGENTS.md` | available; dirty worktree noted, no generated outputs modified |
| `pdftoppm` | `command -v pdftoppm` | 0 | `/Users/kimuj5090/.cache/codex-runtimes/codex-primary-runtime/dependencies/bin/pdftoppm` | available |
| `pdfinfo` | `command -v pdfinfo` | 0 | `/Users/kimuj5090/.cache/codex-runtimes/codex-primary-runtime/dependencies/bin/pdfinfo` | available |
| `sips` | `command -v sips` | 0 | `/usr/bin/sips` | available |
| Swift | `command -v swift` | 0 | `/usr/bin/swift` | available |
| Swift/PDFKit | `swift -e 'import PDFKit; print("PDFKit available")'` | 0 | `PDFKit available` | available |
| Local sample directory | `test -d rjtd-testdata/local-samples` | 0 | empty stdout | available at `/Users/kimuj5090/Documents/rjtd/rjtd-testdata/local-samples` |
| PDF output directory | `test -d openjtd-samples/pdf-output` | 0 | empty stdout | available at `/Users/kimuj5090/Documents/rjtd/openjtd-samples/pdf-output` |
| Local sample metadata | `ls -ld rjtd-testdata/local-samples` | 0 | `drwxr-xr-x@ 74 kimuj5090  staff  2368 Jun 20 10:27 rjtd-testdata/local-samples` | available |
| PDF output metadata | `ls -ld openjtd-samples/pdf-output` | 0 | `drwxr-xr-x@ 64 kimuj5090  staff  2048 Jul  1 04:17 openjtd-samples/pdf-output` | available, cached/generated artifacts only |
| Local sample file count | `find rjtd-testdata/local-samples -maxdepth 1 -type f \| wc -l` | 0 | `      72` | available |
| PDF output file count | `find openjtd-samples/pdf-output -maxdepth 1 -type f \| wc -l` | 0 | `      62` | available, not treated as fresh visual QA |

## Availability Summary

- `rjtd-testdata/local-samples`: available.
- `openjtd-samples/pdf-output`: available, but treated as cached/generated output only.
- `pdftoppm`: available.
- `pdfinfo`: available.
- `sips`: available.
- Swift: available.
- Swift/PDFKit: available.

Verification gap status: no missing required tools or directories were observed. The generated `openjtd-samples/pdf-output` directory is available, but these artifacts are cached and were not regenerated or validated as fresh visual QA in this task.

## manualQa

### surfaceEvidence

| scenario id | criterion reference | surface | exact invocation | verdict | artifactRefs |
| --- | --- | --- | --- | --- | --- |
| T3-S1 | local-samples availability | terminal | `cd /Users/kimuj5090/Documents/rjtd && test -d rjtd-testdata/local-samples` | PASS | A1 |
| T3-S2 | pdf-output availability | terminal | `cd /Users/kimuj5090/Documents/rjtd && test -d openjtd-samples/pdf-output` | PASS | A1 |
| T3-S3 | `pdftoppm` availability | terminal | `cd /Users/kimuj5090/Documents/rjtd && command -v pdftoppm` | PASS | A1 |
| T3-S4 | `pdfinfo` availability | terminal | `cd /Users/kimuj5090/Documents/rjtd && command -v pdfinfo` | PASS | A1 |
| T3-S5 | `sips` availability | terminal | `cd /Users/kimuj5090/Documents/rjtd && command -v sips` | PASS | A1 |
| T3-S6 | Swift availability | terminal | `cd /Users/kimuj5090/Documents/rjtd && command -v swift` | PASS | A1 |
| T3-S7 | PDFKit availability through Swift | terminal | `cd /Users/kimuj5090/Documents/rjtd && swift -e 'import PDFKit; print("PDFKit available")'` | PASS | A1 |

### adversarialCases

| scenario id | criterion reference | adversarial class | expected behavior | verdict | artifactRefs |
| --- | --- | --- | --- | --- | --- |
| T3-A1 | dirty_worktree | concurrent/dirty state | Probe current dirty worktree and do not modify product files, generated PDFs, plan checkboxes, `.omo/boulder.json`, or `.omo/start-work/ledger.jsonl`. | PASS | A1 |
| T3-A2 | stale_state | stale cached claim | Use fresh commands with timestamp/context instead of prior logs or summaries. | PASS | A1 |
| T3-A3 | misleading_success_output | stdout-only success | Record exit statuses for every availability check, including tests with empty stdout. | PASS | A1 |
| T3-A4 | generated_or_cached_artifacts | cached generated outputs | Record `openjtd-samples/pdf-output` as available without treating it as fresh visual QA or regenerating PDFs. | PASS | A1 |

### artifactRefs

| id | kind | description | path |
| --- | --- | --- | --- |
| A1 | markdown evidence | Manual QA matrix and command transcript summary for task 3 tooling availability. | `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-3-tooling.md` |

## Cleanup

- Background processes: none started.
- Temporary files: none created.
- Generated PDFs: not regenerated.
- Product files: not edited.
