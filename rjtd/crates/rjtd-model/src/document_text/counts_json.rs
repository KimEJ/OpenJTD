use super::*;
use crate::*;

pub(crate) fn text_count_ranges_json(ranges: &[TextCountRange]) -> String {
    let mut output = String::from("[");
    for (index, range) in ranges.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_count_range_json(&mut output, range);
    }
    output.push(']');
    output
}

pub(crate) fn text_control_boundaries_json(boundaries: &[TextControlBoundary]) -> String {
    let mut output = String::from("[");
    for (index, boundary) in boundaries.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_control_boundary_json(&mut output, boundary);
    }
    output.push(']');
    output
}

pub(crate) fn text_boundary_candidates_json(candidates: &[TextBoundaryCandidate]) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_boundary_candidate_json(&mut output, candidate);
    }
    output.push(']');
    output
}

pub(crate) fn text_paragraph_boundary_candidates_json(
    candidates: &[TextParagraphBoundaryCandidate],
) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_paragraph_boundary_candidate_json(&mut output, candidate);
    }
    output.push(']');
    output
}

pub(crate) fn push_text_boundary_candidate_json(
    output: &mut String,
    candidate: &TextBoundaryCandidate,
) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&json_string(candidate.kind()));
    output.push_str(",\"textCountRangeIndex\":");
    output.push_str(&candidate.text_count_range_index().to_string());
    output.push_str(",\"basis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"delimiterCode\":");
    output.push_str(&candidate.delimiter_code().to_string());
    output.push_str(",\"delimiterCodeHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        candidate.delimiter_code()
    )));
    output.push_str(",\"intervalCount\":");
    output.push_str(&candidate.interval_count().to_string());
    output.push_str(",\"firstIntervalIndex\":");
    output.push_str(&candidate.first_interval_index().to_string());
    output.push_str(",\"lastIntervalIndex\":");
    output.push_str(&candidate.last_interval_index().to_string());
    output.push_str(",\"sourceStart\":");
    output.push_str(&candidate.source_start().to_string());
    output.push_str(",\"sourceEnd\":");
    output.push_str(&candidate.source_end().to_string());
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_text_paragraph_boundary_candidate_json(
    output: &mut String,
    candidate: &TextParagraphBoundaryCandidate,
) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&json_string(candidate.kind()));
    output.push_str(",\"textBoundaryCandidateIndex\":");
    output.push_str(&candidate.text_boundary_candidate_index().to_string());
    output.push_str(",\"textCountRangeIndex\":");
    output.push_str(&candidate.text_count_range_index().to_string());
    output.push_str(",\"sourceStart\":");
    output.push_str(&candidate.source_start().to_string());
    output.push_str(",\"sourceEnd\":");
    output.push_str(&candidate.source_end().to_string());
    output.push_str(",\"textCountRangeSpan\":");
    output.push_str(&candidate.text_count_range_span().to_string());
    output.push_str(",\"rule\":");
    output.push_str(&json_string(candidate.rule()));
    output.push_str(",\"lineWordEvidence\":");
    push_text_layout_exact_evidence_json(output, candidate.line_word_evidence());
    output.push_str(",\"pageFieldEvidence\":");
    push_text_layout_exact_evidence_json(output, candidate.page_field_evidence());
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_text_layout_exact_evidence_json(
    output: &mut String,
    evidence: &TextLayoutExactEvidence,
) {
    output.push_str("{\"target\":");
    output.push_str(&json_string(evidence.target()));
    output.push_str(",\"base\":");
    output.push_str(&json_string(evidence.base()));
    output.push_str(",\"delta\":");
    output.push_str(&evidence.delta().to_string());
    output.push('}');
}

pub(crate) fn push_text_control_boundary_json(output: &mut String, boundary: &TextControlBoundary) {
    output.push_str("{\"index\":");
    output.push_str(&boundary.index().to_string());
    output.push_str(",\"code\":");
    output.push_str(&boundary.code().to_string());
    output.push_str(",\"codeHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", boundary.code())));
    output.push_str(",\"sourceSpan\":");
    match boundary.source_span() {
        Some(span) => push_text_source_span_json(output, span),
        None => output.push_str("null"),
    }
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_text_source_span_json(output: &mut String, span: &TextSourceSpan) {
    output.push_str("{\"byteStart\":");
    output.push_str(&span.byte_start().to_string());
    output.push_str(",\"byteEnd\":");
    output.push_str(&span.byte_end().to_string());
    output.push_str(",\"unitStart\":");
    output.push_str(&span.unit_start().to_string());
    output.push_str(",\"unitEnd\":");
    output.push_str(&span.unit_end().to_string());
    output.push('}');
}

pub(crate) fn push_text_count_range_json(output: &mut String, range: &TextCountRange) {
    output.push_str("{\"index\":");
    output.push_str(&range.index().to_string());
    output.push_str(",\"family\":");
    output.push_str(&json_string(range.family()));
    output.push_str(",\"start\":");
    output.push_str(&range.start().to_string());
    output.push_str(",\"end\":");
    output.push_str(&range.end().to_string());
    output.push_str(",\"span\":");
    output.push_str(&range.span().to_string());
    output.push_str(",\"declaredStart\":");
    output.push_str(&range.declared_start().to_string());
    output.push_str(",\"declaredEnd\":");
    output.push_str(&range.declared_end().to_string());
    output.push_str(",\"tailFields\":");
    push_u16_array_json(output, range.tail_fields());
    output.push_str(",\"documentTextOverlaps\":");
    text_count_range_overlaps_json(output, range.document_text_overlaps());
    output.push_str(",\"controlRangeOverlaps\":");
    text_count_control_range_overlaps_json(output, range.control_range_overlaps());
    output.push_str(",\"decoded\":false,\"rawHex\":");
    output.push_str(&json_string(&hex_bytes(range.raw())));
    output.push('}');
}

pub(crate) fn text_count_range_overlaps_json(
    output: &mut String,
    overlaps: &[TextCountRangeOverlap],
) {
    output.push('[');
    for (index, overlap) in overlaps.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"basis\":");
        output.push_str(&json_string(overlap.basis().as_str()));
        output.push_str(",\"blockIndex\":");
        output.push_str(&overlap.block_index().to_string());
        output.push_str(",\"inlineIndex\":");
        output.push_str(&overlap.inline_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&overlap.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&overlap.source_end().to_string());
        output.push_str(",\"text\":");
        output.push_str(&json_string(overlap.text()));
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn text_count_control_range_overlaps_json(
    output: &mut String,
    overlaps: &[TextCountControlRangeOverlap],
) {
    output.push('[');
    for (index, overlap) in overlaps.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"basis\":");
        output.push_str(&json_string(overlap.basis().as_str()));
        output.push_str(",\"delimiterCode\":");
        output.push_str(&overlap.delimiter_code().to_string());
        output.push_str(",\"delimiterCodeHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", overlap.delimiter_code())));
        output.push_str(",\"rangeCount\":");
        output.push_str(&overlap.range_count().to_string());
        output.push_str(",\"firstRangeIndex\":");
        output.push_str(&overlap.first_range_index().to_string());
        output.push_str(",\"lastRangeIndex\":");
        output.push_str(&overlap.last_range_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&overlap.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&overlap.source_end().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}
