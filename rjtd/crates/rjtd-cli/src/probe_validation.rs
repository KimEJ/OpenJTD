use crate::probe_signals::JtdSignal;

pub struct ValidationContext<'a> {
    pub id: &'a str,
    pub status: &'a str,
    pub changed_variable: &'a str,
    pub jtd_present: bool,
    pub pdf_present: bool,
    pub signal: Option<&'a JtdSignal>,
    pub baseline: Option<&'a JtdSignal>,
}

pub fn validation_line(ctx: ValidationContext<'_>) -> String {
    let intended = intended_kind(ctx.id, ctx.changed_variable);
    let (result, reason) = validation_result(&ctx, intended);
    format!(
        "validation\t{}\tintended={intended}\tresult={result}\tsourceOnlyAdmissionReady=false\treason={reason}",
        ctx.id
    )
}

pub fn admission_line(failed: usize, missing_pairs: usize) -> String {
    format!(
        "admission\tready=false\treason=diagnostic-only-corpus-insufficient-for-source-only-page-y-render-admission\tfailedCases={failed}\tmissingPairs={missing_pairs}"
    )
}

fn validation_result(
    ctx: &ValidationContext<'_>,
    intended: &'static str,
) -> (&'static str, &'static str) {
    if ctx.status == "failed" {
        return ("failed-no-artifact", "manifest-status-failed");
    }
    if ctx.status == "created" && (!ctx.jtd_present || !ctx.pdf_present) {
        return ("missing-pair", "created-row-missing-jtd-or-pdf");
    }
    let Some(signal) = ctx.signal else {
        return ("not-created", "no-jtd-signal");
    };
    let Some(base) = ctx.baseline else {
        return ("pass-control", "baseline-unavailable");
    };

    match intended {
        "control" => ("pass-control", "control-case"),
        "table-y-position" => table_y_result(signal, base),
        "table-x-position" => source_silent_result(signal, base),
        "table-column-width" | "table-width" | "table-shape" | "merged-header" | "empty-cell"
        | "table-font-size" | "table-line-spacing" => source_signature_result(signal, base),
        "line-mark-row-height" => diff_result(signal.line_signature != base.line_signature),
        "page-mark-margin-candidate" => page_mark_result(signal, base),
        "plain-paragraph-control"
        | "plain-paragraph-font-size"
        | "plain-paragraph-line-spacing" => plain_paragraph_result(signal, base),
        "page-boundary-table" => source_signature_result(signal, base),
        "multi-table-diagnostic" => multi_table_result(signal, base),
        "multi-page-table" => multi_page_result(signal, base),
        _ => source_signature_result(signal, base),
    }
}

fn table_y_result(signal: &JtdSignal, base: &JtdSignal) -> (&'static str, &'static str) {
    if signal.source_signature_hash == base.source_signature_hash {
        (
            "fail-intended-effect-not-observed",
            "source-signature-identical-to-baseline",
        )
    } else {
        (
            "pass-diagnostic-signal",
            "source-signature-differs-from-baseline",
        )
    }
}

fn source_silent_result(signal: &JtdSignal, base: &JtdSignal) -> (&'static str, &'static str) {
    if signal.source_signature_hash == base.source_signature_hash {
        (
            "source-silent-reference-visible-or-unproven",
            "source-signature-identical-to-baseline",
        )
    } else {
        (
            "pass-diagnostic-signal",
            "source-signature-differs-from-baseline",
        )
    }
}

fn page_mark_result(signal: &JtdSignal, base: &JtdSignal) -> (&'static str, &'static str) {
    if signal.page_tuple_signature != base.page_tuple_signature {
        (
            "pass-diagnostic-signal",
            "page-tuple-signature-differs-from-baseline",
        )
    } else {
        source_silent_result(signal, base)
    }
}

fn plain_paragraph_result(signal: &JtdSignal, base: &JtdSignal) -> (&'static str, &'static str) {
    if signal.table_candidate_count == 0 && signal.line_signature != base.line_signature {
        (
            "pass-diagnostic-signal",
            "line-mark-control-without-table-candidates",
        )
    } else {
        (
            "fail-intended-effect-not-observed",
            "plain-control-signal-not-isolated",
        )
    }
}

fn multi_table_result(signal: &JtdSignal, base: &JtdSignal) -> (&'static str, &'static str) {
    if signal.line_signature != base.line_signature && signal.table_candidate_count > 0 {
        (
            "pass-diagnostic-signal",
            "multi-table-source-signal-present",
        )
    } else {
        (
            "fail-intended-effect-not-observed",
            "multi-table-source-signal-weak",
        )
    }
}

fn multi_page_result(signal: &JtdSignal, base: &JtdSignal) -> (&'static str, &'static str) {
    if signal.page_entries != base.page_entries
        || signal.line_signature != base.line_signature
        || signal.table_signature != base.table_signature
    {
        ("pass-diagnostic-signal", "multi-page-source-signal-present")
    } else {
        (
            "fail-intended-effect-not-observed",
            "multi-page-source-signal-weak",
        )
    }
}

fn source_signature_result(signal: &JtdSignal, base: &JtdSignal) -> (&'static str, &'static str) {
    if signal.source_signature_hash == base.source_signature_hash {
        ("pass-control", "source-signature-identical-to-baseline")
    } else {
        (
            "pass-diagnostic-signal",
            "source-signature-differs-from-baseline",
        )
    }
}

fn diff_result(differs: bool) -> (&'static str, &'static str) {
    if differs {
        (
            "pass-diagnostic-signal",
            "expected-source-signal-differs-from-baseline",
        )
    } else {
        (
            "fail-intended-effect-not-observed",
            "expected-source-signal-identical-to-baseline",
        )
    }
}

fn intended_kind(id: &str, changed: &str) -> &'static str {
    if changed == "baseline" || changed == "resave_only" || id.ends_with("_baseline") {
        "control"
    } else if id.contains("multi_page") || changed.contains("multi_page") {
        "multi-page-table"
    } else if changed.contains("page_break_table") || changed.contains("page_boundary_table") {
        "page-boundary-table"
    } else if changed.contains("table_y_position") || id.contains("table_moved_down") {
        "table-y-position"
    } else if changed.contains("table_x_position") {
        "table-x-position"
    } else if changed.contains("column_width") {
        "table-column-width"
    } else if changed.contains("table_width") {
        "table-width"
    } else if changed.contains("merged_header") {
        "merged-header"
    } else if changed.contains("table_shape") {
        "table-shape"
    } else if changed.contains("empty_cell") {
        "empty-cell"
    } else if changed.contains("table_font_size") {
        "table-font-size"
    } else if changed.contains("paragraph_font_size") {
        "plain-paragraph-font-size"
    } else if changed.contains("table_line_spacing") {
        "table-line-spacing"
    } else if changed.contains("paragraph_line_spacing") {
        "plain-paragraph-line-spacing"
    } else if changed.contains("row") || changed.contains("wrap") || id.contains("wrapped") {
        "line-mark-row-height"
    } else if changed.contains("top_margin") || id.contains("top_margin") {
        "page-mark-margin-candidate"
    } else if id.contains("two_tables") {
        "multi-table-diagnostic"
    } else if id.contains("plain_paragraph") {
        "plain-paragraph-control"
    } else {
        "diagnostic"
    }
}
