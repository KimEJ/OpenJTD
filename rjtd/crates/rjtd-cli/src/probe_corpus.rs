use std::fs;
use std::path::{Path, PathBuf};

use crate::probe_manifest::{ManifestRow, read_manifest};
use crate::probe_signals::{JtdSignal, analyze_jtd};
use crate::probe_validation::{ValidationContext, admission_line, validation_line};

#[derive(Debug, Clone)]
struct CaseReport {
    row: ManifestRow,
    jtd_present: bool,
    pdf_present: bool,
    signal: Option<JtdSignal>,
}

#[derive(Clone, Copy)]
struct Baseline<'a> {
    id: &'a str,
    policy: &'static str,
    signal: &'a JtdSignal,
}

struct CorpusLayout {
    manifest_dir: PathBuf,
    files_dir: PathBuf,
}

pub fn source_y_probe_audit_lines(corpus_dir: &Path) -> Result<Vec<String>, String> {
    let layout = corpus_layout(corpus_dir);
    let rows = read_manifest(&layout.manifest_dir)?;
    let reports = rows
        .into_iter()
        .map(|row| case_report(&layout.files_dir, row))
        .collect::<Result<Vec<_>, _>>()?;
    let global_baseline = global_baseline(&reports);

    let created = reports
        .iter()
        .filter(|report| report.row.status == "created")
        .count();
    let failed = reports
        .iter()
        .filter(|report| report.row.status == "failed")
        .count();
    let omitted = reports
        .iter()
        .filter(|report| report.row.status == "omitted")
        .count();
    let missing_pairs = reports
        .iter()
        .filter(|report| {
            report.row.status == "created" && (!report.jtd_present || !report.pdf_present)
        })
        .count();

    let mut lines = Vec::new();
    lines.push(format!(
        "summary\tcases={}\tcreated={created}\tfailed={failed}\tomitted={omitted}\tmissing-pairs={missing_pairs}\tbaseline={}",
        reports.len(),
        global_baseline.map_or("-", |baseline| baseline.id)
    ));
    for report in &reports {
        lines.push(case_line(report));
        let baseline = compare_baseline(report, &reports, global_baseline);
        if let (Some(base), Some(signal)) = (baseline, report.signal.as_ref()) {
            lines.push(compare_line(report, base, signal));
        }
        lines.push(validation_line(ValidationContext {
            id: &report.row.id,
            status: &report.row.status,
            changed_variable: &report.row.changed_variable,
            jtd_present: report.jtd_present,
            pdf_present: report.pdf_present,
            signal: report.signal.as_ref(),
            baseline: baseline.map(|base| base.signal),
        }));
    }
    lines.push(admission_line(failed, missing_pairs));
    Ok(lines)
}

fn case_report(files_dir: &Path, row: ManifestRow) -> Result<CaseReport, String> {
    let jtd_path = files_dir.join(format!("{}.jtd", row.stem));
    let pdf_path = files_dir.join(format!("{}.pdf", row.stem));
    let jtd_present = jtd_path.is_file();
    let pdf_present = pdf_path.is_file();
    let signal = if jtd_present {
        let bytes = fs::read(&jtd_path)
            .map_err(|error| format!("reading {}: {error}", jtd_path.display()))?;
        Some(analyze_jtd(&bytes))
    } else {
        None
    };
    Ok(CaseReport {
        row,
        jtd_present,
        pdf_present,
        signal,
    })
}

fn corpus_layout(corpus_dir: &Path) -> CorpusLayout {
    if corpus_dir.join("manifest.csv").is_file() {
        return CorpusLayout {
            manifest_dir: corpus_dir.to_path_buf(),
            files_dir: corpus_files_dir(corpus_dir),
        };
    }

    for manifest_dir in [
        corpus_dir.parent(),
        corpus_dir.parent().and_then(Path::parent),
    ]
    .into_iter()
    .flatten()
    {
        if manifest_dir.join("manifest.csv").is_file() {
            return CorpusLayout {
                manifest_dir: manifest_dir.to_path_buf(),
                files_dir: corpus_dir.to_path_buf(),
            };
        }
    }

    CorpusLayout {
        manifest_dir: corpus_dir.to_path_buf(),
        files_dir: corpus_files_dir(corpus_dir),
    }
}

fn corpus_files_dir(corpus_dir: &Path) -> PathBuf {
    let organized_dir = corpus_dir.join("corpus").join("baseline-sweep");
    if organized_dir.is_dir() {
        organized_dir
    } else if corpus_dir.join("files").is_dir() {
        corpus_dir.join("files")
    } else {
        corpus_dir.to_path_buf()
    }
}

fn case_line(report: &CaseReport) -> String {
    let Some(signal) = report.signal.as_ref() else {
        return format!(
            "case\t{}\tpriority={}\tstatus={}\tjtd={}\tpdf={}\tlineMarkLen=-\tlineDeclared=-\tlineParsed=-\tpageFamily=-\tpageEntries=-\ttableCandidates=-\tsparseTableCandidates=-\tnonEmptyCells=-\tsourceSignatureHash=-",
            report.row.id,
            report.row.priority,
            report.row.status,
            present(report.jtd_present),
            present(report.pdf_present)
        );
    };
    format!(
        "case\t{}\tpriority={}\tstatus={}\tjtd={}\tpdf={}\tlineMarkLen={}\tlineDeclared={}\tlineParsed={}\tpageFamily={}\tpageEntries={}\ttableCandidates={}\tsparseTableCandidates={}\tnonEmptyCells={}\tsourceSignatureHash=0x{:016x}",
        report.row.id,
        report.row.priority,
        report.row.status,
        present(report.jtd_present),
        present(report.pdf_present),
        signal.line_len,
        signal.line_declared_count,
        signal.line_parsed_records,
        signal.page_family,
        signal.page_entries,
        signal.table_candidate_count,
        signal.sparse_table_candidate_count,
        signal.table_non_empty_cell_count,
        signal.source_signature_hash
    )
}

fn global_baseline(reports: &[CaseReport]) -> Option<Baseline<'_>> {
    reports
        .iter()
        .find(|report| report.row.id == "000_base_a")
        .and_then(|report| report.signal.as_ref().map(|signal| ("000_base_a", signal)))
        .or_else(|| {
            reports.iter().find_map(|report| {
                report
                    .signal
                    .as_ref()
                    .map(|signal| (report.row.id.as_str(), signal))
            })
        })
        .map(|(id, signal)| Baseline {
            id,
            policy: "global-baseline",
            signal,
        })
}

fn compare_baseline<'a>(
    report: &'a CaseReport,
    reports: &'a [CaseReport],
    global: Option<Baseline<'a>>,
) -> Option<Baseline<'a>> {
    if let Some(id) = rtf_margin_sweep_base_id(report)
        && let Some(signal) = signal_for_id(reports, id)
    {
        return Some(Baseline {
            id,
            policy: "rtf-margin-sweep-baseline",
            signal,
        });
    }
    if report.row.base_id != "none"
        && let Some(signal) = signal_for_id(reports, &report.row.base_id)
    {
        return Some(Baseline {
            id: &report.row.base_id,
            policy: "manifest-base",
            signal,
        });
    }
    global.map(|baseline| Baseline {
        policy: if report.row.base_id == "none" {
            "global-baseline"
        } else {
            "fallback-global-baseline"
        },
        ..baseline
    })
}

fn rtf_margin_sweep_base_id(report: &CaseReport) -> Option<&'static str> {
    if report.row.id.starts_with("040")
        && report.row.id != "040_top_margin_plus"
        && report.row.changed_variable == "top_margin"
    {
        Some("040b_top_margin_30mm_baseline")
    } else {
        None
    }
}

fn signal_for_id<'a>(reports: &'a [CaseReport], id: &str) -> Option<&'a JtdSignal> {
    reports
        .iter()
        .find(|report| report.row.id == id)
        .and_then(|report| report.signal.as_ref())
}

fn compare_line(report: &CaseReport, base: Baseline<'_>, signal: &JtdSignal) -> String {
    format!(
        "compare\t{}\tbase={}\tbasePolicy={}\tsourceSignatureSame={}\tlineSignatureSame={}\tpageTupleSignatureSame={}\ttableSignatureSame={}",
        report.row.id,
        base.id,
        base.policy,
        signal.source_signature_hash == base.signal.source_signature_hash,
        signal.line_signature == base.signal.line_signature,
        signal.page_tuple_signature == base.signal.page_tuple_signature,
        signal.table_signature == base.signal.table_signature
    )
}

fn present(value: bool) -> &'static str {
    if value { "present" } else { "missing" }
}
