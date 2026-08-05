use rjtd_model::{
    DocumentAutoText, DocumentFont, DocumentPageMark, DocumentPaperMark, DocumentTocEntry,
    page_mark_u16_geometry_profile,
};

use super::primitives::{
    hex, push_json_string, push_option_u16_hex_json, push_option_u16_json, push_option_u32_json,
    push_u16_array_json, push_u16_hex_array_json, push_u32_array_json, push_u32_hex_array_json,
};
use super::text_layout::push_text_source_span_json;

pub(crate) fn push_document_font_json(output: &mut String, font: &DocumentFont) {
    output.push_str("{\"sourceStream\":");
    push_json_string(output, font.source_stream());
    output.push_str(",\"id\":");
    output.push_str(&font.id().to_string());
    output.push_str(",\"offset\":");
    output.push_str(&font.offset().to_string());
    output.push_str(",\"name\":");
    push_json_string(output, font.name());
    output.push_str(",\"rawHex\":");
    push_json_string(output, &hex(font.raw()));
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_document_auto_text_json(output: &mut String, auto_text: &DocumentAutoText) {
    output.push_str("{\"sourceStream\":");
    push_json_string(output, auto_text.source_stream());
    output.push_str(",\"offset\":");
    output.push_str(&auto_text.offset().to_string());
    output.push_str(",\"text\":");
    push_json_string(output, auto_text.text());
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_document_toc_entry_json(output: &mut String, entry: &DocumentTocEntry) {
    output.push_str("{\"title\":");
    push_json_string(output, entry.title());
    output.push_str(",\"pageLabel\":");
    push_json_string(output, entry.page_label());
    output.push_str(",\"sourceSpan\":");
    push_text_source_span_json(output, entry.source_span());
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_document_page_mark_json(output: &mut String, page_mark: &DocumentPageMark) {
    output.push_str("{\"sourceStream\":");
    push_json_string(output, page_mark.source_stream());
    output.push_str(",\"family\":");
    push_json_string(output, page_mark.family());
    output.push_str(",\"headerCount\":");
    output.push_str(&page_mark.header_count().to_string());
    output.push_str(",\"headerStride\":");
    output.push_str(&page_mark.header_stride().to_string());
    output.push_str(",\"headerLastIndex\":");
    output.push_str(&page_mark.header_last_index().to_string());
    output.push_str(",\"entryCount\":");
    output.push_str(&page_mark.entries().len().to_string());
    output.push_str(",\"trailingByteLength\":");
    output.push_str(&page_mark.trailing_byte_len().to_string());
    output.push_str(",\"entries\":[");
    for (index, entry) in page_mark.entries().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&entry.row_index().to_string());
        output.push_str(",\"index\":");
        push_option_u32_json(output, entry.index());
        output.push_str(",\"flags\":");
        push_option_u32_json(output, entry.flags());
        output.push_str(",\"flagsHex\":");
        if let Some(flags) = entry.flags() {
            push_json_string(output, &format!("0x{flags:08x}"));
        } else {
            output.push_str("null");
        }
        output.push_str(",\"lineStart\":");
        push_option_u32_json(output, entry.line_start());
        output.push_str(",\"lineEnd\":");
        push_option_u32_json(output, entry.line_end());
        output.push_str(",\"rawLength\":");
        output.push_str(&entry.raw_len().to_string());
        output.push_str(",\"rawHex\":");
        push_json_string(output, &hex(entry.raw()));
        output.push_str(",\"u16Fields\":");
        push_u16_array_json(output, entry.u16_fields());
        output.push_str(",\"u16FieldsHex\":");
        push_u16_hex_array_json(output, entry.u16_fields());
        output.push_str(",\"u16GeometryClass\":");
        push_json_string(output, entry.u16_geometry_profile().class_name());
        output.push_str(",\"u16SubrecordScan\":");
        push_page_mark_u16_subrecord_scan_json(
            output,
            entry.u16_fields(),
            page_mark_entry_stream_byte_offset(page_mark, index),
        );
        output.push_str(",\"u32Fields\":");
        push_u32_array_json(output, entry.u32_fields());
        output.push_str(",\"u32FieldsHex\":");
        push_u32_hex_array_json(output, entry.u32_fields());
        output.push_str(",\"u16GeometryHypotheses\":");
        push_page_mark_u16_geometry_hypotheses_json(output, entry.u16_fields());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"decoded\":false}");
}

fn page_mark_entry_stream_byte_offset(page_mark: &DocumentPageMark, entry_index: usize) -> usize {
    12 + page_mark
        .entries()
        .iter()
        .take(entry_index)
        .map(|entry| entry.raw_len())
        .sum::<usize>()
}

fn push_page_mark_u16_subrecord_scan_json(
    output: &mut String,
    fields: &[u16],
    entry_stream_byte_offset: usize,
) {
    let candidates = page_mark_u16_subrecord_candidates(fields);
    output.push_str("{\"source\":\"/PageMark raw u16 subrecord scan\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidates.len().to_string());
    output.push_str(",\"candidates\":[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let u32_fields = page_mark_u16_subrecord_u32_fields(&candidate.words);
        output.push_str("{\"entryRelativeByteOffset\":");
        output.push_str(&candidate.byte_offset.to_string());
        output.push_str(",\"streamByteOffset\":");
        output.push_str(&(entry_stream_byte_offset + candidate.byte_offset).to_string());
        output.push_str(",\"wordIndex\":");
        output.push_str(&candidate.word_index.to_string());
        output.push_str(",\"words\":");
        push_u16_array_json(output, &candidate.words);
        output.push_str(",\"wordsHex\":");
        push_u16_hex_array_json(output, &candidate.words);
        output.push_str(",\"u32Fields\":");
        push_u32_array_json(output, &u32_fields);
        output.push_str(",\"u32FieldsHex\":");
        push_u32_hex_array_json(output, &u32_fields);
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("]}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageMarkU16SubrecordCandidate {
    word_index: usize,
    byte_offset: usize,
    words: [u16; 8],
}

fn page_mark_u16_subrecord_candidates(fields: &[u16]) -> Vec<PageMarkU16SubrecordCandidate> {
    fields
        .windows(8)
        .enumerate()
        .filter_map(|(word_index, window)| {
            let words = [
                window[0], window[1], window[2], window[3], window[4], window[5], window[6],
                window[7],
            ];
            page_mark_u16_subrecord_words_look_plausible(&words).then_some(
                PageMarkU16SubrecordCandidate {
                    word_index,
                    byte_offset: word_index * 2,
                    words,
                },
            )
        })
        .collect()
}

fn page_mark_u16_subrecord_words_look_plausible(words: &[u16; 8]) -> bool {
    words[3] == 0 && words[5] == 0 && words[7] == 0 && words[4] <= words[6]
}

fn page_mark_u16_subrecord_u32_fields(words: &[u16; 8]) -> [u32; 4] {
    [
        (u32::from(words[0]) << 16) | u32::from(words[1]),
        (u32::from(words[2]) << 16) | u32::from(words[3]),
        (u32::from(words[4]) << 16) | u32::from(words[5]),
        (u32::from(words[6]) << 16) | u32::from(words[7]),
    ]
}

fn push_page_mark_u16_geometry_hypotheses_json(output: &mut String, fields: &[u16]) {
    let field = |index: usize| fields.get(index).copied();
    let word_10 = field(10);
    let word_13 = field(13);
    let word_14 = field(14);
    let word_17 = field(17);
    let word_18 = field(18);
    let word_19 = field(19);
    let word_21 = field(21);
    let profile = page_mark_u16_geometry_profile(fields);
    let word_13_plus_14 = word_13
        .zip(word_14)
        .and_then(|(left, right)| left.checked_add(right));
    let word_21_minus_13 = word_21
        .zip(word_13)
        .and_then(|(full, primary)| full.checked_sub(primary));
    let selected_field_indexes = [10usize, 13, 14, 17, 18, 19, 20, 21];

    output.push_str("{\"source\":\"/PageMark\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"profile\":");
    push_json_string(output, profile.class_name());
    output.push_str(",\"selectedFields\":[");
    for (index, word_index) in selected_field_indexes.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"wordIndex\":");
        output.push_str(&word_index.to_string());
        output.push_str(",\"value\":");
        push_option_u16_json(output, field(*word_index));
        output.push_str(",\"hex\":");
        push_option_u16_hex_json(output, field(*word_index));
        output.push('}');
    }
    output.push(']');
    output.push_str(",\"word10EqualsWord13\":");
    output.push_str(if word_10.zip(word_13).is_some_and(|(a, b)| a == b) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"word17EqualsWord18\":");
    output.push_str(if word_17.zip(word_18).is_some_and(|(a, b)| a == b) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"word18EqualsWord19\":");
    output.push_str(if word_18.zip(word_19).is_some_and(|(a, b)| a == b) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"word20Is0x00ff\":");
    output.push_str(if profile.word20_is_00ff() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"word13PlusWord14\":");
    push_option_u16_json(output, word_13_plus_14);
    output.push_str(",\"word13PlusWord14EqualsWord21\":");
    output.push_str(
        if word_13_plus_14
            .zip(word_21)
            .is_some_and(|(sum, word_21)| sum == word_21)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"word21MinusWord13\":");
    push_option_u16_json(output, word_21_minus_13);
    output.push_str(",\"word21MinusWord13EqualsWord14\":");
    output.push_str(
        if word_21_minus_13
            .zip(word_14)
            .is_some_and(|(difference, word_14)| difference == word_14)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"word19EqualsWord13\":");
    output.push_str(if word_19.zip(word_13).is_some_and(|(a, b)| a == b) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedFieldsAllZero\":");
    output.push_str(if profile.selected_fields_all_zero() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nonZeroAdditiveUnitCandidate\":");
    output.push_str(if profile.non_zero_additive_unit_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"layoutComparisons\":null");
    output.push_str(
        ",\"renderPromotionContribution\":\"page-mark-u16-horizontal-geometry-candidate-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, "page-mark-u16-geometry-semantics-unproven");
    output.push('}');
}

pub(crate) fn push_document_paper_mark_json(output: &mut String, paper_mark: &DocumentPaperMark) {
    output.push_str("{\"sourceStream\":");
    push_json_string(output, paper_mark.source_stream());
    output.push_str(",\"headerCount\":");
    output.push_str(&paper_mark.header_count().to_string());
    output.push_str(",\"headerStride\":");
    output.push_str(&paper_mark.header_stride().to_string());
    output.push_str(",\"headerLastIndex\":");
    output.push_str(&paper_mark.header_last_index().to_string());
    output.push_str(",\"entryCount\":");
    output.push_str(&paper_mark.entries().len().to_string());
    output.push_str(",\"entries\":[");
    for (index, entry) in paper_mark.entries().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&entry.row_index().to_string());
        output.push_str(",\"index\":");
        output.push_str(&entry.index().to_string());
        output.push_str(",\"flags\":");
        output.push_str(&entry.flags().to_string());
        output.push_str(",\"flagsHex\":");
        push_json_string(output, &format!("0x{:08x}", entry.flags()));
        output.push_str(",\"rawLength\":");
        output.push_str(&entry.raw_len().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"decoded\":false}");
}
