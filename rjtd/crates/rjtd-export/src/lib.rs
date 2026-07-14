#![doc = include_str!("../README.md")]

use std::collections::{BTreeMap, BTreeSet};

use rjtd_core::record::UnknownRecordKind;
use rjtd_core::style_stream::{
    StyleStreamRecordSummary, StyleStreamSubrecordSummary, summarize_style_stream,
};
use rjtd_model::{
    Block, Document, DocumentAutoText, DocumentCore, DocumentFont, DocumentPageMark,
    DocumentPaperMark, DocumentTocEntry, Inline, ObjectEmbeddedPressSnapshotCandidate,
    ObjectEmbeddedPressVectorPathCandidate, ObjectEmbeddingFrameCandidate,
    ObjectFdmConnectorCandidate, ObjectFdmIndexBbox, ObjectFdmIndexEntryCandidate,
    ObjectFdmTextCandidate, ObjectFdmTextIndexEntryCandidate, ObjectFdmVectorCommandCandidate,
    ObjectFdmVectorCommandSourceSegment, ObjectFdmVectorCurveSegment, ObjectFdmVectorEllipse,
    ObjectFdmVectorPoint, ObjectFdmVectorSegmentCandidate, ObjectFigureLinkCandidate,
    ObjectFigureLinkRowCandidate, ObjectFrameRecordCandidate, ObjectFrameReferenceRowCandidate,
    ObjectImageDimensions, ObjectImageHeaderFieldCandidates, ObjectImageNumericHeaderField,
    ObjectImagePayloadEnvelope, ObjectImagePayloadSpan, ObjectImageSourcePathCandidate,
    ObjectJseq3FormulaCandidate, ObjectJsfartArtCandidate, ObjectJsfartArtPaintCandidate,
    ObjectJsfartStreamProfileCandidate, ObjectStreamCandidate, ObjectStreamOwnershipCandidate,
    ObjectStreamOwnershipReferenceCandidate, ObjectVisualListCandidate, StyleRef, TableCandidate,
    TableCandidateColumnSegment, TableCandidateInterval, TextBoundaryCandidate,
    TextControlBoundary, TextCountControlRangeOverlap, TextCountRange, TextCountRangeOverlap,
    TextLayoutExactEvidence, TextParagraphBoundaryCandidate, TextSourceSpan, UnknownObject,
    page_mark_u16_geometry_profile,
};

const EMBEDDED_PRESS_RECORD_PAINT_STATE_82: u32 = 0x82;
const SUCCESS_DATA_TEST_FDM_VECTOR_PATH: &str = "/FigureData/main_data/FDMVector";
const SUCCESS_DATA_TEST_Q4_SOURCE_LEFT: i32 = -15784;
const SUCCESS_DATA_TEST_Q4_SOURCE_TOP: i32 = -10213;
const SUCCESS_DATA_TEST_Q4_SOURCE_RIGHT: i32 = -10584;
const SUCCESS_DATA_TEST_Q4_SOURCE_BOTTOM: i32 = -9013;
const SUCCESS_DATA_TEST_Q4_TARGET_X_PX: f32 = 93.3;
const SUCCESS_DATA_TEST_Q4_TARGET_Y_PX: f32 = 663.3;
const SUCCESS_DATA_TEST_Q4_TARGET_WIDTH_PX: f32 = 491.4;
const SUCCESS_DATA_TEST_Q5_TARGET_X_PX: f32 = 490.7;
const SUCCESS_DATA_TEST_Q5_TARGET_Y_PX: f32 = 795.0;
const SUCCESS_DATA_TEST_Q5_TARGET_WIDTH_PX: f32 = 74.6;
const SUCCESS_DATA_TEST_Q5_TARGET_HEIGHT_PX: f32 = 110.0;

pub fn to_plain_text(document: &Document) -> String {
    let mut output = String::new();

    for block in document.blocks() {
        if let Block::Paragraph(paragraph) = block {
            for inline in paragraph.inlines() {
                push_inline_visible_text(&mut output, inline);
            }
            output.push('\n');
        }
    }

    output
}

#[cfg(not(target_arch = "wasm32"))]
pub fn to_pdf(document: &Document) -> Result<Vec<u8>, String> {
    to_pdf_with_file_name(document, "")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn to_pdf_with_file_name(document: &Document, file_name: &str) -> Result<Vec<u8>, String> {
    let mut core = DocumentCore::from_document(document.clone());
    if !file_name.is_empty() {
        core.set_file_name(file_name);
    }
    let mut svg_pages = Vec::new();

    for page in 0..core.page_count() {
        svg_pages.push(
            core.render_page_svg(page)
                .map_err(|error| error.to_string())?,
        );
    }

    svgs_to_pdf(&svg_pages)
}

pub fn to_html(document: &Document) -> String {
    let mut output = String::new();
    output.push_str(
        "<!DOCTYPE html>\n<html lang=\"ja\">\n<head><meta charset=\"UTF-8\"></head>\n<body>\n",
    );

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                output.push_str("<p>");
                for inline in paragraph.inlines() {
                    push_inline_html(&mut output, inline);
                }
                output.push_str("</p>\n");
            }
            Block::Unknown(_) => {}
        }
    }

    output.push_str("</body>\n</html>\n");
    output
}

fn push_inline_html(output: &mut String, inline: &Inline) {
    match inline {
        Inline::Text(text) => push_html_escaped(output, text.text()),
        Inline::Ruby(ruby) => {
            output.push_str("<ruby>");
            push_html_escaped(output, ruby.base_text());
            output.push_str("<rt>");
            push_html_escaped(output, ruby.annotation_text());
            output.push_str("</rt></ruby>");
        }
        Inline::Unknown(_) => {}
    }
}

fn push_html_escaped(output: &mut String, text: &str) {
    for ch in text.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            _ => output.push(ch),
        }
    }
}

pub fn to_markdown(document: &Document) -> String {
    let mut output = String::new();

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                for inline in paragraph.inlines() {
                    push_inline_visible_text(&mut output, inline);
                }
                output.push_str("\n\n");
            }
            Block::Unknown(_) => {
                output.push_str("<!-- UnknownBlock preserved by rjtd -->\n\n");
            }
        }
    }

    output
}

pub fn to_json(document: &Document) -> String {
    let mut output = String::new();

    output.push_str("{\"metadata\":{\"title\":");
    match document.metadata().title() {
        Some(title) => push_json_string(&mut output, title),
        None => output.push_str("null"),
    }
    output.push_str("},\"blocks\":[");
    for (index, block) in document.blocks().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_block_json(&mut output, block);
    }
    output.push_str("],\"unknownStyles\":[");
    for (index, style) in document.unknown_styles().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"name\":");
        match style.name() {
            Some(name) => push_json_string(&mut output, name),
            None => output.push_str("null"),
        }
        let summary = summarize_style_stream(style.payload());
        output.push_str(",\"family\":");
        push_json_string(&mut output, summary.family().as_str());
        output.push_str(",\"headerU32Be\":");
        push_u32_array_json(&mut output, summary.header_u32_be());
        output.push_str(",\"headerU16Be\":");
        push_u16_array_json(&mut output, summary.header_u16_be());
        output.push_str(",\"recordLayout\":");
        push_json_string(&mut output, summary.record_layout().as_str());
        output.push_str(",\"recordCount\":");
        output.push_str(&summary.records().len().to_string());
        output.push_str(",\"records\":");
        push_style_records_json(&mut output, summary.records());
        output.push_str(",\"decoded\":false");
        output.push_str(",\"source\":");
        push_unknown_source_json(&mut output, style.source());
        output.push_str(",\"payloadHex\":");
        push_json_string(&mut output, &hex(style.payload()));
        output.push('}');
    }
    output.push_str("],\"unknownObjects\":[");
    for (index, object) in document.unknown_objects().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_unknown_object_json(&mut output, object);
    }
    output.push_str("],\"objectStreamCandidates\":[");
    for (index, candidate) in document.object_stream_candidates().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_stream_candidate_json(&mut output, candidate);
    }
    output.push_str("],\"objectFrameRecords\":[");
    for (index, record) in document.object_frame_records().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_frame_record_candidate_json(&mut output, record);
    }
    output.push_str("],\"objectEmbeddingFrames\":[");
    for (index, frame) in document.object_embedding_frames().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_embedding_frame_candidate_json(&mut output, frame);
    }
    output.push_str("],\"textCountRanges\":[");
    for (index, range) in document.text_count_ranges().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_count_range_json(&mut output, range);
    }
    output.push_str("],\"textControlBoundaries\":[");
    for (index, boundary) in document.text_control_boundaries().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_control_boundary_json(&mut output, boundary);
    }
    output.push_str("],\"textBoundaryCandidates\":[");
    for (index, candidate) in document.text_boundary_candidates().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_boundary_candidate_json(&mut output, candidate);
    }
    output.push_str("],\"textParagraphBoundaryCandidates\":[");
    for (index, candidate) in document
        .text_paragraph_boundary_candidates()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_text_paragraph_boundary_candidate_json(&mut output, candidate);
    }
    output.push_str("],\"tableCandidates\":[");
    for (index, candidate) in document.table_candidates().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_candidate_json(&mut output, candidate);
    }
    output.push_str("],\"autoTextCandidates\":[");
    for (index, auto_text) in document.auto_texts().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_document_auto_text_json(&mut output, auto_text);
    }
    output.push_str("],\"tocEntries\":[");
    for (index, entry) in document.toc_entries().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_document_toc_entry_json(&mut output, entry);
    }
    output.push_str("],\"pageMarks\":[");
    for (index, page_mark) in document.page_marks().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_document_page_mark_json(&mut output, page_mark);
    }
    output.push_str("],\"paperMarks\":[");
    for (index, paper_mark) in document.paper_marks().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_document_paper_mark_json(&mut output, paper_mark);
    }
    output.push_str("],\"rawStreams\":[");
    for (index, stream) in document.raw_streams().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"name\":");
        push_json_string(&mut output, stream.name());
        output.push_str(",\"size\":");
        output.push_str(&stream.bytes().len().to_string());
        output.push('}');
    }
    output.push_str("],\"fonts\":[");
    for (index, font) in document.fonts().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_document_font_json(&mut output, font);
    }
    output.push_str("]}");

    output
}

fn push_document_font_json(output: &mut String, font: &DocumentFont) {
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

fn push_document_auto_text_json(output: &mut String, auto_text: &DocumentAutoText) {
    output.push_str("{\"sourceStream\":");
    push_json_string(output, auto_text.source_stream());
    output.push_str(",\"offset\":");
    output.push_str(&auto_text.offset().to_string());
    output.push_str(",\"text\":");
    push_json_string(output, auto_text.text());
    output.push_str(",\"decoded\":false}");
}

fn push_document_toc_entry_json(output: &mut String, entry: &DocumentTocEntry) {
    output.push_str("{\"title\":");
    push_json_string(output, entry.title());
    output.push_str(",\"pageLabel\":");
    push_json_string(output, entry.page_label());
    output.push_str(",\"sourceSpan\":");
    push_text_source_span_json(output, entry.source_span());
    output.push_str(",\"decoded\":false}");
}

fn push_document_page_mark_json(output: &mut String, page_mark: &DocumentPageMark) {
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

fn push_document_paper_mark_json(output: &mut String, paper_mark: &DocumentPaperMark) {
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

fn push_block_json(output: &mut String, block: &Block) {
    match block {
        Block::Paragraph(paragraph) => {
            output.push_str("{\"type\":\"paragraph\",\"style\":");
            push_style_json(output, paragraph.style());
            output.push_str(",\"inlines\":[");
            for (index, inline) in paragraph.inlines().iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                match inline {
                    Inline::Text(text) => {
                        output.push_str("{\"type\":\"text\",\"text\":");
                        push_json_string(output, text.text());
                        output.push_str(",\"style\":");
                        push_style_json(output, text.style());
                        if let Some(span) = text.source_span() {
                            output.push_str(",\"sourceSpan\":");
                            push_text_source_span_json(output, span);
                        }
                        output.push('}');
                    }
                    Inline::Ruby(ruby) => {
                        output.push_str("{\"type\":\"ruby\",\"baseText\":");
                        push_json_string(output, ruby.base_text());
                        output.push_str(",\"annotationText\":");
                        push_json_string(output, ruby.annotation_text());
                        output.push_str(",\"annotationSelector\":");
                        output.push_str(&ruby.annotation_selector().to_string());
                        output.push_str(",\"annotationObject\":");
                        push_unknown_object_json(output, ruby.annotation_source());
                        output.push('}');
                    }
                    Inline::Unknown(object) => {
                        output.push_str("{\"type\":\"unknown\",\"object\":");
                        push_unknown_object_json(output, object);
                        output.push('}');
                    }
                }
            }
            output.push_str("]}");
        }
        Block::Unknown(block) => {
            output.push_str("{\"type\":\"unknown\",\"source\":");
            push_unknown_source_json(output, block.source());
            output.push_str(",\"payloadHex\":");
            push_json_string(output, &hex(block.payload()));
            output.push('}');
        }
    }
}

fn push_inline_visible_text(output: &mut String, inline: &Inline) {
    match inline {
        Inline::Text(text) => output.push_str(text.text()),
        Inline::Ruby(ruby) => output.push_str(ruby.base_text()),
        Inline::Unknown(_) => {}
    }
}

fn push_style_json(output: &mut String, style: Option<&StyleRef>) {
    match style {
        Some(style) => {
            output.push_str("{\"id\":");
            push_json_string(output, style.id());
            output.push('}');
        }
        None => output.push_str("null"),
    }
}

fn push_unknown_object_json(output: &mut String, object: &UnknownObject) {
    output.push_str("{\"source\":");
    push_unknown_source_json(output, object.source());
    output.push_str(",\"payloadHex\":");
    push_json_string(output, &hex(object.payload()));
    output.push('}');
}

fn push_object_frame_record_candidate_json(
    output: &mut String,
    record: &ObjectFrameRecordCandidate,
) {
    output.push_str("{\"sourcePath\":");
    push_json_string(output, record.source_path());
    output.push_str(",\"rowIndex\":");
    output.push_str(&record.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&record.row_start().to_string());
    output.push_str(",\"recordLen\":");
    output.push_str(&record.record_len().to_string());
    output.push_str(",\"recordKind\":");
    output.push_str(&record.record_kind().to_string());
    output.push_str(",\"recordKindHex\":");
    push_json_string(output, &format!("0x{:04x}", record.record_kind()));
    output.push_str(",\"declaredRecordBytes\":");
    output.push_str(&record.declared_record_bytes().to_string());
    output.push_str(",\"objectId\":");
    output.push_str(&record.object_id().to_string());
    output.push_str(",\"objectType\":");
    output.push_str(&record.object_type().to_string());
    output.push_str(",\"objectTypeHex\":");
    push_json_string(output, &format!("0x{:04x}", record.object_type()));
    output.push_str(",\"geometry\":{\"x\":");
    output.push_str(&record.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&record.y().to_string());
    output.push_str(",\"width\":");
    output.push_str(&record.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&record.height().to_string());
    output.push_str("},\"rowPrefixHex\":");
    push_json_string(output, &hex(record.row_prefix()));
    output.push_str(",\"decoded\":false}");
}

fn push_object_embedding_frame_candidate_json(
    output: &mut String,
    frame: &ObjectEmbeddingFrameCandidate,
) {
    output.push_str("{\"sourcePath\":");
    push_json_string(output, frame.source_path());
    output.push_str(",\"rowIndex\":");
    output.push_str(&frame.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&frame.row_start().to_string());
    output.push_str(",\"embeddingIndex\":");
    output.push_str(&frame.embedding_index().to_string());
    output.push_str(",\"className\":");
    push_json_string(output, frame.class_name());
    output.push_str(",\"primarySize\":{\"width\":");
    output.push_str(&frame.primary_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&frame.primary_height().to_string());
    output.push_str("},\"frameRef\":");
    output.push_str(&frame.frame_ref().to_string());
    output.push_str(",\"frameSize\":{\"width\":");
    output.push_str(&frame.frame_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&frame.frame_height().to_string());
    output.push_str("},\"rowPrefixHex\":");
    push_json_string(output, &hex(frame.row_prefix()));
    output.push_str(",\"decoded\":false}");
}

fn push_object_stream_candidate_json(output: &mut String, candidate: &ObjectStreamCandidate) {
    output.push_str("{\"path\":");
    push_json_string(output, candidate.path());
    output.push_str(",\"size\":");
    output.push_str(&candidate.size().to_string());
    output.push_str(",\"reasons\":[");
    for (index, reason) in candidate.reasons().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(output, reason.as_str());
    }
    output.push_str("],\"ownershipCandidate\":");
    if let Some(ownership) = candidate.ownership_candidate() {
        push_object_stream_ownership_candidate_json(output, ownership);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"ownershipReferences\":[");
    for (index, reference) in candidate
        .ownership_reference_candidates()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_object_stream_ownership_reference_candidate_json(output, reference);
    }
    output.push_str("],\"frameReferenceRows\":[");
    for (index, row) in candidate
        .frame_reference_row_candidates()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_object_frame_reference_row_candidate_json(output, row);
    }
    output.push_str("],\"figureLink\":");
    if let Some(link) = candidate.figure_link_candidate() {
        push_object_figure_link_candidate_json(output, link);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"fdmIndexEntries\":[");
    for (index, entry) in candidate.fdm_index_entry_candidates().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_index_entry_candidate_json(
            output,
            entry,
            candidate.fdm_raw_vector_commands(),
        );
    }
    output.push_str("],\"fdmTextIndexEntries\":[");
    for (index, entry) in candidate
        .fdm_text_index_entry_candidates()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_text_index_entry_candidate_json(output, entry);
    }
    output.push_str("],\"fdmRawVectorSegmentCount\":");
    output.push_str(&candidate.fdm_raw_vector_segments().len().to_string());
    output.push_str(",\"fdmRawVectorSegments\":[");
    for (index, segment) in candidate.fdm_raw_vector_segments().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_vector_segment_candidate_json(output, segment);
    }
    output.push_str("],\"fdmRawVectorCommandCount\":");
    output.push_str(&candidate.fdm_raw_vector_commands().len().to_string());
    output.push_str(",\"fdmRawVectorCommands\":[");
    for (index, command) in candidate.fdm_raw_vector_commands().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_vector_command_candidate_json(output, command);
    }
    output.push_str("],\"successDataTestFdmReferenceProjections\":");
    push_success_data_test_fdm_reference_projections_json(output, candidate);
    output.push_str(",\"fdmTextCount\":");
    output.push_str(&candidate.fdm_text_candidates().len().to_string());
    output.push_str(",\"fdmTextCandidates\":[");
    for (index, text) in candidate.fdm_text_candidates().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_text_candidate_json(output, text);
    }
    output.push_str("],\"imageSignatures\":[");
    for (index, hit) in candidate.image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        push_json_string(output, hit.kind());
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"imagePayloads\":[");
    for (index, span) in candidate.image_payload_spans().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_payload_span_json(output, span);
    }
    output.push_str("],\"svgOffsets\":");
    push_usize_array_json(output, candidate.svg_offsets());
    output.push_str(",\"soOffsets\":");
    push_usize_array_json(output, candidate.so_offsets());
    output.push_str(",\"visualList\":");
    if let Some(visual_list) = candidate.visual_list_candidate() {
        push_object_visual_list_candidate_json(output, visual_list);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"embeddedPressSnapshot\":");
    if let Some(snapshot) = candidate.embedded_press_snapshot_candidate() {
        push_object_embedded_press_snapshot_candidate_json(output, snapshot);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"jseq3Formula\":");
    if let Some(formula) = candidate.jseq3_formula_candidate() {
        push_object_jseq3_formula_candidate_json(output, formula);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"jsfartStreamProfile\":");
    if let Some(profile) = candidate.jsfart_stream_profile_candidate() {
        push_object_jsfart_stream_profile_candidate_json(output, profile);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"jsfartArt\":");
    if let Some(art) = candidate.jsfart_art_candidate() {
        push_object_jsfart_art_candidate_json(output, art);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"payloadPrefixHex\":");
    push_json_string(output, &hex(candidate.payload_prefix()));
    output.push_str(",\"decoded\":false}");
}

fn push_object_jsfart_stream_profile_candidate_json(
    output: &mut String,
    profile: &ObjectJsfartStreamProfileCandidate,
) {
    output.push_str("{\"format\":\"JSFart2Contents\",\"source\":\"stream-prefix\",\"sourceCandidateType\":\"objectStream\",\"magicFamily\":");
    push_json_string(output, profile.magic_family());
    output.push_str(",\"magicFamilyHex\":");
    push_json_string(output, profile.magic_family_hex());
    output.push_str(",\"magicOffset\":");
    output.push_str(&profile.magic_offset().to_string());
    output.push_str(",\"magicAsciiOrUtf16Preview\":");
    push_json_string(output, profile.magic_ascii_or_utf16_preview());
    output.push_str(",\"headerPrefixHex\":");
    push_json_string(output, &hex(profile.header_prefix()));
    output.push_str(",\"structuredArtCandidatePresent\":");
    output.push_str(if profile.structured_art_candidate_present() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"renderable\":false,\"decoded\":false,\"renderPromotionBlockedReason\":");
    push_json_string(output, profile.render_promotion_blocked_reason());
    output.push('}');
}

fn push_object_figure_link_candidate_json(output: &mut String, link: &ObjectFigureLinkCandidate) {
    output.push_str("{\"headerWordsBe\":");
    push_u16_array_json(output, link.header_words_be());
    output.push_str(",\"declaredRowCountCandidate\":");
    push_option_u16_json(output, link.declared_row_count_candidate());
    output.push_str(",\"rowStride\":");
    output.push_str(&link.row_stride().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&link.rows().len().to_string());
    output.push_str(",\"rows\":[");
    for (index, row) in link.rows().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_figure_link_row_candidate_json(output, row);
    }
    output.push_str("],\"geometryDecoded\":false,\"decoded\":false}");
}

fn push_object_figure_link_row_candidate_json(
    output: &mut String,
    row: &ObjectFigureLinkRowCandidate,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&row.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&row.row_start().to_string());
    output.push_str(",\"wordsBe\":");
    push_u16_array_json(output, row.words_be());
    output.push_str(",\"groupIndexCandidate\":");
    push_option_u16_json(output, row.group_index_candidate());
    output.push_str(",\"sourceIdCandidate\":");
    push_option_u16_json(output, row.source_id_candidate());
    output.push_str(",\"relationKindCandidate\":");
    push_option_u16_json(output, row.relation_kind_candidate());
    output.push_str(",\"relationKindCandidateHex\":");
    push_option_u16_hex_json(output, row.relation_kind_candidate());
    output.push_str(",\"targetRowIndexCandidate\":");
    push_option_u16_json(output, row.target_row_index_candidate());
    output.push_str(",\"rowHex\":");
    push_json_string(output, &hex(row.row()));
    output.push_str(",\"decoded\":false}");
}

fn push_object_jsfart_art_candidate_json(output: &mut String, art: &ObjectJsfartArtCandidate) {
    output.push_str("{\"format\":\"JSFart2Contents\",\"magic\":");
    push_json_string(output, art.magic());
    output.push_str(",\"magicOffset\":");
    output.push_str(&art.magic_offset().to_string());
    output.push_str(",\"width\":");
    output.push_str(&art.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&art.height().to_string());
    output.push_str(",\"frameCandidate\":");
    if let Some(frame) = art.frame_candidate() {
        output.push_str("{\"left\":");
        output.push_str(&frame.left().to_string());
        output.push_str(",\"top\":");
        output.push_str(&frame.top().to_string());
        output.push_str(",\"right\":");
        output.push_str(&frame.right().to_string());
        output.push_str(",\"bottom\":");
        output.push_str(&frame.bottom().to_string());
        output.push_str(",\"contentLeft\":");
        output.push_str(&frame.content_left().to_string());
        output.push_str(",\"contentTop\":");
        output.push_str(&frame.content_top().to_string());
        output.push_str(",\"contentRight\":");
        output.push_str(&frame.content_right().to_string());
        output.push_str(",\"contentBottom\":");
        output.push_str(&frame.content_bottom().to_string());
        output.push_str(",\"cornerRadiusX\":");
        output.push_str(&frame.corner_radius_x().to_string());
        output.push_str(",\"cornerRadiusY\":");
        output.push_str(&frame.corner_radius_y().to_string());
        output.push_str(",\"strokeWidthCandidate\":");
        push_option_u32_json(output, frame.stroke_width_candidate());
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"paintCandidate\":");
    if let Some(paint) = art.paint_candidate() {
        push_object_jsfart_art_paint_candidate_json(output, paint);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"headerPrefixHex\":");
    push_json_string(output, &hex(art.header_prefix()));
    output.push_str(",\"renderable\":false,\"decoded\":false}");
}

fn push_object_jsfart_art_paint_candidate_json(
    output: &mut String,
    paint: &ObjectJsfartArtPaintCandidate,
) {
    output.push_str("{\"styleWord1\":");
    output.push_str(&paint.style_word_1().to_string());
    output.push_str(",\"styleWord1Hex\":");
    push_json_string(output, &format!("0x{:08x}", paint.style_word_1()));
    output.push_str(",\"styleWord2\":");
    output.push_str(&paint.style_word_2().to_string());
    output.push_str(",\"styleWord2Hex\":");
    push_json_string(output, &format!("0x{:08x}", paint.style_word_2()));
    output.push_str(",\"paintColorCandidate\":");
    output.push_str(&paint.paint_color_candidate().to_string());
    output.push_str(",\"paintColorCandidateHex\":");
    push_json_string(output, &format!("0x{:08x}", paint.paint_color_candidate()));
    output.push_str(",\"paintFlagCandidate\":");
    output.push_str(&paint.paint_flag_candidate().to_string());
    output.push_str(",\"paintFlagCandidateHex\":");
    push_json_string(output, &format!("0x{:08x}", paint.paint_flag_candidate()));
    output.push_str(",\"effectWordCandidate\":");
    output.push_str(&paint.effect_word_candidate().to_string());
    output.push_str(",\"effectWordCandidateHex\":");
    push_json_string(output, &format!("0x{:08x}", paint.effect_word_candidate()));
    output.push_str(",\"decoded\":false}");
}

fn push_object_jseq3_formula_candidate_json(
    output: &mut String,
    formula: &ObjectJseq3FormulaCandidate,
) {
    output.push_str("{\"format\":\"JSEQ3Contents\",\"magic\":");
    push_json_string(output, formula.magic());
    output.push_str(",\"magicOffset\":");
    output.push_str(&formula.magic_offset().to_string());
    output.push_str(",\"soTrailerOffset\":");
    push_option_usize_json(output, formula.so_trailer_offset());
    output.push_str(",\"soTrailerLength\":");
    push_option_usize_json(output, formula.so_trailer_length());
    output.push_str(",\"soTrailerFields\":");
    push_u32_array_json(output, formula.so_trailer_fields());
    output.push_str(",\"textMarkers\":[");
    for (index, marker) in formula.text_markers().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"text\":");
        push_json_string(output, marker.text());
        output.push_str(",\"offset\":");
        output.push_str(&marker.offset().to_string());
        output.push_str(",\"encoding\":");
        push_json_string(output, marker.encoding());
        output.push('}');
    }
    output.push_str("],\"headerPrefixHex\":");
    push_json_string(output, &hex(formula.header_prefix()));
    output.push_str(",\"renderable\":false,\"decoded\":false}");
}

fn push_object_embedded_press_snapshot_candidate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    output.push_str("{\"format\":\"JSSnapShot32\",\"magic\":");
    push_json_string(output, snapshot.magic());
    output.push_str(",\"bodyLengthCandidate\":");
    output.push_str(&snapshot.body_length_candidate().to_string());
    output.push_str(",\"formatMarker\":");
    push_json_string(output, snapshot.format_marker());
    output.push_str(",\"objectCountCandidate\":");
    output.push_str(&snapshot.object_count_candidate().to_string());
    output.push_str(",\"objectTableOffsetCandidate\":");
    output.push_str(&snapshot.object_table_offset_candidate().to_string());
    output.push_str(",\"payloadLengthCandidate\":");
    output.push_str(&snapshot.payload_length_candidate().to_string());
    output.push_str(",\"width\":");
    output.push_str(&snapshot.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&snapshot.height().to_string());
    output.push_str(",\"vectorSegmentCount\":");
    output.push_str(&snapshot.vector_segments().len().to_string());
    output.push_str(",\"vectorPathCount\":");
    output.push_str(&snapshot.vector_paths().len().to_string());
    output.push_str(",\"textureBezierHeaderSummary\":");
    push_embedded_press_texture_bezier_header_summary_json(output, snapshot);
    output.push_str(",\"paintStateTransitions\":");
    push_embedded_press_paint_state_transitions_json(output, snapshot);
    output.push_str(",\"stateRecordSummary\":");
    push_embedded_press_state_record_summary_json(output, snapshot);
    output.push_str(",\"vectorSegmentPreview\":");
    push_object_embedded_press_snapshot_vector_segment_preview_json(output, snapshot);
    output.push_str(",\"headerPrefixHex\":");
    push_json_string(output, &hex(snapshot.header_prefix()));
    output.push_str(",\"renderable\":");
    output.push_str(if snapshot.vector_segments().is_empty() {
        "false"
    } else {
        "true"
    });
    output.push_str(",\"decoded\":false}");
}

fn push_object_embedded_press_snapshot_vector_segment_preview_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    output.push('[');
    for (index, segment) in snapshot.vector_segments().iter().take(8).enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"x1\":");
        output.push_str(&segment.x1().to_string());
        output.push_str(",\"y1\":");
        output.push_str(&segment.y1().to_string());
        output.push_str(",\"x2\":");
        output.push_str(&segment.x2().to_string());
        output.push_str(",\"y2\":");
        output.push_str(&segment.y2().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

fn push_embedded_press_texture_bezier_header_summary_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let mut path_count = 0usize;
    let mut first_header = None;
    let mut homogeneous = true;
    for path in snapshot.vector_paths() {
        let Some(header) = path.texture_bezier_header() else {
            continue;
        };
        path_count += 1;
        match first_header {
            Some(first) if first != header => homogeneous = false,
            None => first_header = Some(header),
            _ => {}
        }
    }

    let Some(header) = first_header else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"pathCount\":");
    output.push_str(&path_count.to_string());
    output.push_str(",\"pointCount\":");
    output.push_str(&header.point_count().to_string());
    output.push_str(",\"byteCount\":");
    output.push_str(&header.byte_count().to_string());
    output.push_str(",\"flags\":");
    output.push_str(&header.flags().to_string());
    output.push_str(",\"flagsHex\":");
    push_json_string(output, &format!("0x{:08x}", header.flags()));
    output.push_str(",\"homogeneous\":");
    output.push_str(if homogeneous { "true" } else { "false" });
    output.push('}');
}

fn push_embedded_press_paint_state_transitions_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let mut ranges = Vec::new();
    let mut current_48_word0 = None;
    let mut current_70_word0 = None;
    let mut current_70_word3 = None;
    let mut current_82_word5 = None;

    for (path_index, path) in snapshot.vector_paths().iter().enumerate() {
        if let Some(value) = embedded_press_path_state_word(path, 0x48, 0) {
            current_48_word0 = Some(value);
        }
        if let Some(value) = embedded_press_path_state_word(path, 0x70, 0) {
            current_70_word0 = Some(value);
        }
        if let Some(value) = embedded_press_path_state_word(path, 0x70, 3) {
            current_70_word3 = Some(value);
        }
        if let Some(value) =
            embedded_press_path_state_word(path, EMBEDDED_PRESS_RECORD_PAINT_STATE_82, 5)
        {
            current_82_word5 = Some(value);
        }

        let key = (
            path.kind(),
            current_48_word0,
            current_70_word0,
            current_70_word3,
            current_82_word5,
        );
        match ranges.last_mut() {
            Some((_, end, known_key)) if *known_key == key => *end = path_index,
            _ => ranges.push((path_index, path_index, key)),
        }
    }

    output.push('[');
    for (range_index, (start, end, key)) in ranges.iter().enumerate() {
        if range_index > 0 {
            output.push(',');
        }
        let paths = &snapshot.vector_paths()[*start..=*end];
        let explicit_state_path_count = paths
            .iter()
            .filter(|path| !path.state_records().is_empty())
            .count();
        let texture_header_count = paths
            .iter()
            .filter(|path| path.texture_bezier_header().is_some())
            .count();

        output.push_str("{\"pathKind\":");
        push_json_string(output, key.0.as_str());
        output.push_str(",\"startPathIndex\":");
        output.push_str(&start.to_string());
        output.push_str(",\"endPathIndex\":");
        output.push_str(&end.to_string());
        output.push_str(",\"pathCount\":");
        output.push_str(&(end - start + 1).to_string());
        output.push_str(",\"explicitStatePathCount\":");
        output.push_str(&explicit_state_path_count.to_string());
        output.push_str(",\"inheritedStatePathCount\":");
        output.push_str(&(end - start + 1 - explicit_state_path_count).to_string());
        output.push_str(",\"textureBezierHeaderCount\":");
        output.push_str(&texture_header_count.to_string());
        output.push_str(",\"currentState\":{\"record48Word0\":");
        push_option_u32_hex_json(output, key.1);
        output.push_str(",\"record70Word0\":");
        push_option_u32_hex_json(output, key.2);
        output.push_str(",\"record70Word3\":");
        push_option_u32_hex_json(output, key.3);
        output.push_str(",\"record82Word5\":");
        push_option_u32_hex_json(output, key.4);
        output.push_str("},\"explicitStateValues\":{\"record48Word0\":");
        push_u32_hex_array_json(
            output,
            &embedded_press_path_state_word_values(paths, 0x48, 0),
        );
        output.push_str(",\"record70Word0\":");
        push_u32_hex_array_json(
            output,
            &embedded_press_path_state_word_values(paths, 0x70, 0),
        );
        output.push_str(",\"record70Word3\":");
        push_u32_hex_array_json(
            output,
            &embedded_press_path_state_word_values(paths, 0x70, 3),
        );
        output.push_str(",\"record82Word5\":");
        push_u32_hex_array_json(
            output,
            &embedded_press_path_state_word_values(paths, EMBEDDED_PRESS_RECORD_PAINT_STATE_82, 5),
        );
        output.push_str("},\"decoded\":false}");
    }
    output.push(']');
}

fn embedded_press_path_state_word(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    record_type: u32,
    word_index: usize,
) -> Option<u32> {
    path.state_records()
        .iter()
        .rev()
        .find(|record| record.record_type() == record_type)
        .and_then(|record| record.payload_le32_words().get(word_index).copied())
}

fn embedded_press_path_state_word_values(
    paths: &[ObjectEmbeddedPressVectorPathCandidate],
    record_type: u32,
    word_index: usize,
) -> Vec<u32> {
    paths
        .iter()
        .filter_map(|path| embedded_press_path_state_word(path, record_type, word_index))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn push_u32_hex_array_json(output: &mut String, values: &[u32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(output, &format!("0x{value:08x}"));
    }
    output.push(']');
}

fn push_embedded_press_state_record_summary_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let mut type_counts = std::collections::BTreeMap::<u32, usize>::new();
    let mut state_record_count = 0usize;
    for path in snapshot.vector_paths() {
        for record in path.state_records() {
            state_record_count += 1;
            *type_counts.entry(record.record_type()).or_default() += 1;
        }
    }

    output.push_str("{\"pathCount\":");
    output.push_str(&snapshot.vector_paths().len().to_string());
    output.push_str(",\"stateRecordCount\":");
    output.push_str(&state_record_count.to_string());
    output.push_str(",\"recordTypes\":[");
    for (index, (record_type, count)) in type_counts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"recordType\":");
        output.push_str(&record_type.to_string());
        output.push_str(",\"recordTypeHex\":");
        push_json_string(output, &format!("0x{record_type:08x}"));
        output.push_str(",\"count\":");
        output.push_str(&count.to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"paintState82Preview\":[");

    let mut preview_count = 0usize;
    for (path_index, path) in snapshot.vector_paths().iter().enumerate() {
        for (record_index, record) in path.state_records().iter().enumerate() {
            if record.record_type() != 0x82 || preview_count >= 8 {
                continue;
            }
            let words = record.payload_le32_words();
            if preview_count > 0 {
                output.push(',');
            }
            output.push_str("{\"pathIndex\":");
            output.push_str(&path_index.to_string());
            output.push_str(",\"pathKind\":");
            push_json_string(output, path.kind().as_str());
            output.push_str(",\"recordIndex\":");
            output.push_str(&record_index.to_string());
            output.push_str(",\"offset\":");
            output.push_str(&record.offset().to_string());
            output.push_str(",\"payloadWordCount\":");
            output.push_str(&words.len().to_string());
            output.push_str(",\"payloadLe32WordsPreview\":");
            let preview_len = words.len().min(8);
            push_u32_array_json(output, &words[..preview_len]);
            output.push_str(",\"word3Candidate\":");
            push_option_u32_json(output, words.get(3).copied());
            output.push_str(",\"word3CandidateHex\":");
            push_option_u32_hex_json(output, words.get(3).copied());
            output.push_str(",\"word5Candidate\":");
            push_option_u32_json(output, words.get(5).copied());
            output.push_str(",\"word5CandidateHex\":");
            push_option_u32_hex_json(output, words.get(5).copied());
            output.push_str(",\"decoded\":false}");
            preview_count += 1;
        }
    }
    output.push_str("],\"decoded\":false}");
}

fn push_object_visual_list_candidate_json(
    output: &mut String,
    visual_list: &ObjectVisualListCandidate,
) {
    output.push_str("{\"format\":\"BMDV\",\"declaredSize\":");
    output.push_str(&visual_list.declared_size().to_string());
    output.push_str(",\"magicOffset\":");
    output.push_str(&visual_list.magic_offset().to_string());
    output.push_str(",\"magic\":");
    push_json_string(output, visual_list.magic());
    output.push_str(",\"version\":");
    output.push_str(&visual_list.version().to_string());
    output.push_str(",\"flags\":");
    output.push_str(&visual_list.flags().to_string());
    output.push_str(",\"width\":");
    output.push_str(&visual_list.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&visual_list.height().to_string());
    output.push_str(",\"rowStride\":");
    output.push_str(&visual_list.row_stride().to_string());
    output.push_str(",\"bitDepth\":");
    output.push_str(&visual_list.bit_depth().to_string());
    output.push_str(",\"xPixelsPerMeter\":");
    output.push_str(&visual_list.x_pixels_per_meter().to_string());
    output.push_str(",\"yPixelsPerMeter\":");
    output.push_str(&visual_list.y_pixels_per_meter().to_string());
    output.push_str(",\"rleDataOffset\":");
    output.push_str(&visual_list.rle_data_offset().to_string());
    output.push_str(",\"rleDataLength\":");
    output.push_str(&visual_list.rle_data_len().to_string());
    output.push_str(",\"pixelCount\":");
    output.push_str(&visual_list.pixels().len().to_string());
    output.push_str(",\"rleEncoding\":\"bmp-rle8-like\",\"renderable\":true,\"decoded\":false}");
}

fn push_object_stream_ownership_candidate_json(
    output: &mut String,
    ownership: &ObjectStreamOwnershipCandidate,
) {
    output.push_str("{\"basis\":");
    push_json_string(output, ownership.basis());
    output.push_str(",\"family\":");
    push_json_string(output, ownership.family());
    output.push_str(",\"storagePath\":");
    if let Some(storage_path) = ownership.storage_path() {
        push_json_string(output, storage_path);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"embeddingIndex\":");
    if let Some(index) = ownership.embedding_index() {
        output.push_str(&index.to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"streamRole\":");
    push_json_string(output, ownership.stream_role());
    output.push_str(",\"decoded\":false}");
}

fn push_object_stream_ownership_reference_candidate_json(
    output: &mut String,
    reference: &ObjectStreamOwnershipReferenceCandidate,
) {
    output.push_str("{\"targetPath\":");
    push_json_string(output, reference.target_path());
    output.push_str(",\"encoding\":");
    push_json_string(output, reference.encoding());
    output.push_str(",\"totalMatches\":");
    output.push_str(&reference.total_matches().to_string());
    output.push_str(",\"offsets\":");
    push_usize_array_json(output, reference.offsets());
    output.push_str(",\"decoded\":false}");
}

fn push_object_frame_reference_row_candidate_json(
    output: &mut String,
    row: &ObjectFrameReferenceRowCandidate,
) {
    output.push_str("{\"targetPath\":");
    push_json_string(output, row.target_path());
    output.push_str(",\"encoding\":");
    push_json_string(output, row.encoding());
    output.push_str(",\"stride\":");
    output.push_str(&row.stride().to_string());
    output.push_str(",\"fieldOffset\":");
    output.push_str(&row.field_offset().to_string());
    output.push_str(",\"offset\":");
    output.push_str(&row.offset().to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&row.row_start().to_string());
    output.push_str(",\"family\":");
    push_json_string(output, row.family());
    output.push_str(",\"rowHex\":");
    push_json_string(output, &hex(row.row()));
    output.push_str(",\"suffixLink\":");
    if let Some(link) = row.suffix_link() {
        output.push_str("{\"relation\":");
        push_json_string(output, link.relation());
        output.push_str(",\"suffixFamily\":");
        push_json_string(output, link.suffix_family());
        output.push_str(",\"matchedRowStart\":");
        output.push_str(&link.matched_row_start().to_string());
        output.push_str(",\"matchedRowIndex\":");
        output.push_str(&link.matched_row_index().to_string());
        output.push_str(",\"decoded\":false}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

fn push_object_fdm_index_entry_candidate_json(
    output: &mut String,
    entry: &ObjectFdmIndexEntryCandidate,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) {
    output.push_str("{\"indexPath\":");
    push_json_string(output, entry.index_path());
    output.push_str(",\"vectorPath\":");
    push_json_string(output, entry.vector_path());
    output.push_str(",\"rowIndex\":");
    output.push_str(&entry.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&entry.index_offset().to_string());
    output.push_str(",\"vectorOffset\":");
    output.push_str(&entry.vector_offset().to_string());
    output.push_str(",\"nextVectorOffset\":");
    output.push_str(&entry.next_vector_offset().to_string());
    output.push_str(",\"vectorLength\":");
    output.push_str(&entry.vector_len().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&entry.kind().to_string());
    output.push_str(",\"kindHex\":");
    push_json_string(output, &format!("0x{:04x}", entry.kind()));
    output.push_str(",\"bbox\":");
    push_object_fdm_index_bbox_json(output, entry.bbox());
    output.push_str(",\"validVectorOffset\":");
    output.push_str(if entry.valid_vector_offset() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"offsetFieldReferenceCandidates\":");
    push_object_fdm_index_offset_field_reference_candidates_json(output, entry, raw_commands);
    output.push_str(",\"vectorPrefixHex\":");
    push_json_string(output, &hex(entry.vector_prefix()));
    output.push_str(",\"vectorCommandCount\":");
    output.push_str(&entry.vector_commands().len().to_string());
    output.push_str(",\"vectorCommandBboxCount\":");
    output.push_str(
        &entry
            .vector_commands()
            .iter()
            .filter(|command| command.bbox().is_some())
            .count()
            .to_string(),
    );
    output.push_str(",\"vectorCommands\":[");
    for (index, command) in entry.vector_commands().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_vector_command_candidate_json(output, command);
    }
    output.push_str("],\"connectorCandidateCount\":");
    output.push_str(&entry.connector_candidates().len().to_string());
    output.push_str(",\"connectorCandidates\":[");
    for (index, candidate) in entry.connector_candidates().iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_connector_candidate_json(output, candidate);
    }
    output.push_str("],\"imageSignatures\":[");
    for (index, hit) in entry.image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        push_json_string(output, hit.kind());
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"segmentImageSignatures\":[");
    for (index, hit) in entry.segment_image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        push_json_string(output, hit.kind());
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"decoded\":false}");
}

fn push_object_fdm_index_offset_field_reference_candidates_json(
    output: &mut String,
    entry: &ObjectFdmIndexEntryCandidate,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) {
    let bbox = entry.bbox();
    let fields = [
        Some(("vectorOffset", entry.vector_offset())),
        non_negative_i32_offset("bbox.left", bbox.left()),
        non_negative_i32_offset("bbox.top", bbox.top()),
        non_negative_i32_offset("bbox.right", bbox.right()),
        non_negative_i32_offset("bbox.bottom", bbox.bottom()),
    ];
    output.push('[');
    let mut emitted = 0usize;
    for field in fields.into_iter().flatten() {
        emitted += push_object_fdm_index_offset_field_reference_candidate_json(
            output,
            emitted,
            field.0,
            field.1,
            raw_commands,
        );
    }
    output.push(']');
}

fn non_negative_i32_offset(field_name: &'static str, value: i32) -> Option<(&'static str, usize)> {
    (value >= 0).then_some((field_name, value as usize))
}

fn push_object_fdm_index_offset_field_reference_candidate_json(
    output: &mut String,
    emitted: usize,
    field_name: &str,
    field_value: usize,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) -> usize {
    let command_matches = raw_commands
        .iter()
        .filter(|command| command.relative_offset() == field_value)
        .map(ObjectFdmVectorCommandCandidate::relative_offset)
        .collect::<Vec<_>>();
    let segment_matches = raw_commands
        .iter()
        .filter(|command| {
            command
                .source_segment()
                .is_some_and(|segment| segment.relative_offset() == field_value)
        })
        .map(ObjectFdmVectorCommandCandidate::relative_offset)
        .collect::<Vec<_>>();

    let mut local_emitted = 0usize;
    if !command_matches.is_empty() {
        if emitted + local_emitted > 0 {
            output.push(',');
        }
        output.push_str("{\"offsetField\":");
        push_json_string(output, field_name);
        output.push_str(",\"offsetValue\":");
        output.push_str(&field_value.to_string());
        output.push_str(",\"matchKind\":\"command-relative-offset-field\"");
        output.push_str(",\"referenceSource\":\"fdmRawVectorCommands.relativeOffset\"");
        output.push_str(",\"matchedCommandRelativeOffsets\":");
        push_usize_array_json(output, &command_matches);
        output.push_str(",\"decoded\":false}");
        local_emitted += 1;
    }
    if !segment_matches.is_empty() {
        if emitted + local_emitted > 0 {
            output.push(',');
        }
        output.push_str("{\"offsetField\":");
        push_json_string(output, field_name);
        output.push_str(",\"offsetValue\":");
        output.push_str(&field_value.to_string());
        output.push_str(",\"matchKind\":\"source-segment-relative-offset-field\"");
        output
            .push_str(",\"referenceSource\":\"fdmRawVectorCommands.sourceSegment.relativeOffset\"");
        output.push_str(",\"sourceSegmentRelativeOffset\":");
        output.push_str(&field_value.to_string());
        output.push_str(",\"sourceSegmentBackedCommandCount\":");
        output.push_str(&segment_matches.len().to_string());
        output.push_str(",\"matchedCommandRelativeOffsets\":");
        push_usize_array_json(output, &segment_matches);
        output.push_str(",\"decoded\":false}");
        local_emitted += 1;
    }
    local_emitted
}

fn push_object_fdm_connector_candidate_json(
    output: &mut String,
    candidate: ObjectFdmConnectorCandidate,
) {
    output.push_str("{\"commandIndex\":");
    output.push_str(&candidate.command_index().to_string());
    output.push_str(",\"relativeOffset\":");
    output.push_str(&candidate.relative_offset().to_string());
    output.push_str(",\"markerHex\":");
    push_json_string(output, &hex(&candidate.marker()));
    output.push_str(",\"primitiveKind\":");
    push_json_string(output, candidate.primitive_kind());
    output.push_str(",\"styleWord\":");
    output.push_str(&candidate.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    push_json_string(output, &format!("0x{:04x}", candidate.style_word()));
    output.push_str(",\"fillColor\":");
    push_fdm_vector_optional_color_json(output, candidate.fill_color());
    output.push_str(",\"strokeColor\":");
    push_fdm_vector_optional_color_json(output, candidate.stroke_color());
    output.push_str(",\"candidateBasis\":");
    push_json_string(output, candidate.basis());
    output.push_str(",\"sourceEndpoints\":");
    push_fdm_connector_candidate_source_endpoints_json(output, candidate);
    output.push_str(",\"sourceBbox\":");
    push_object_fdm_index_bbox_json(output, candidate.source_bbox());
    output.push_str(",\"sourceSpan\":");
    output.push_str(&candidate.source_span().to_string());
    output.push_str(",\"endpointDelta\":{\"x\":");
    output.push_str(&candidate.endpoint_dx().to_string());
    output.push_str(",\"y\":");
    output.push_str(&candidate.endpoint_dy().to_string());
    output.push('}');
    output.push_str(",\"endpointDistanceSquared\":");
    output.push_str(&candidate.endpoint_distance_squared().to_string());
    output.push_str(",\"pathPointCount\":");
    output.push_str(&candidate.path_point_count().to_string());
    output.push_str(",\"pathSegmentCount\":");
    output.push_str(&candidate.path_segment_count().to_string());
    output.push_str(",\"orthogonalSegmentCount\":");
    output.push_str(&candidate.orthogonal_segment_count().to_string());
    output.push_str(",\"diagonalSegmentCount\":");
    output.push_str(&candidate.diagonal_segment_count().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&candidate.curve_segment_count().to_string());
    output.push_str(",\"compoundChildOffsetCount\":");
    output.push_str(&candidate.compound_child_offset_count().to_string());
    output.push_str(",\"axisAligned\":");
    output.push_str(if candidate.axis_aligned() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"orientation\":");
    push_json_string(output, candidate.orientation());
    output.push_str(",\"decoded\":false}");
}

fn push_fdm_connector_candidate_source_endpoints_json(
    output: &mut String,
    candidate: ObjectFdmConnectorCandidate,
) {
    output.push_str("{\"start\":");
    push_fdm_vector_point_json(output, candidate.source_start());
    output.push_str(",\"end\":");
    push_fdm_vector_point_json(output, candidate.source_end());
    output.push('}');
}

fn push_object_fdm_vector_command_candidate_json(
    output: &mut String,
    command: &ObjectFdmVectorCommandCandidate,
) {
    output.push_str("{\"commandIndex\":");
    output.push_str(&command.command_index().to_string());
    output.push_str(",\"relativeOffset\":");
    output.push_str(&command.relative_offset().to_string());
    output.push_str(",\"sourceVectorRelativeOffset\":");
    push_option_usize_json(output, command.source_vector_relative_offset());
    output.push_str(",\"sourceSegment\":");
    if let Some(source_segment) = command.source_segment() {
        push_object_fdm_vector_command_source_segment_json(output, source_segment);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"recordLength\":");
    output.push_str(&command.record_len().to_string());
    output.push_str(",\"declaredRecordLength\":");
    output.push_str(&command.declared_record_len().to_string());
    output.push_str(",\"styleWord\":");
    output.push_str(&command.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    push_json_string(output, &format!("0x{:04x}", command.style_word()));
    output.push_str(",\"markerHex\":");
    push_json_string(output, &hex(command.marker()));
    output.push_str(",\"primitiveKind\":");
    push_json_string(output, fdm_vector_primitive_kind(command));
    output.push_str(",\"fillColor\":");
    push_fdm_vector_optional_color_json(output, command.fill_color());
    output.push_str(",\"strokeColor\":");
    push_fdm_vector_optional_color_json(output, command.stroke_color());
    output.push_str(",\"bbox\":");
    if let Some(bbox) = command.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"pathPointCount\":");
    output.push_str(&command.path_points().len().to_string());
    output.push_str(",\"pathClosed\":");
    output.push_str(if fdm_vector_path_is_closed(command.path_points()) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pathPoints\":");
    push_fdm_vector_points_json(output, command.path_points());
    output.push_str(",\"pathBbox\":");
    if let Some(bbox) = fdm_vector_path_points_bbox(command.path_points()) {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&command.curve_segments().len().to_string());
    output.push_str(",\"curveSegments\":");
    push_fdm_vector_curve_segments_json(output, command.curve_segments());
    output.push_str(",\"ellipse\":");
    if let Some(ellipse) = command.ellipse() {
        push_fdm_vector_ellipse_json(output, ellipse);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"compoundChildOffsets\":");
    push_u16_array_json(output, command.compound_child_offsets());
    output.push_str(",\"decoded\":false}");
}

fn push_object_fdm_vector_command_source_segment_json(
    output: &mut String,
    source_segment: ObjectFdmVectorCommandSourceSegment,
) {
    output.push_str("{\"relativeOffset\":");
    output.push_str(&source_segment.relative_offset().to_string());
    output.push_str(",\"localOffset\":");
    output.push_str(&source_segment.local_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&source_segment.declared_len().to_string());
    output.push_str(",\"commandCount\":");
    output.push_str(&source_segment.command_count().to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&source_segment.command_index().to_string());
    output.push_str(",\"commandOffset\":");
    output.push_str(&source_segment.command_offset().to_string());
    output.push('}');
}

fn push_success_data_test_fdm_reference_projections_json(
    output: &mut String,
    candidate: &ObjectStreamCandidate,
) {
    if candidate.path() != SUCCESS_DATA_TEST_FDM_VECTOR_PATH {
        output.push_str("[]");
        return;
    }
    let raw_commands = candidate.fdm_raw_vector_commands();
    output.push('[');
    let mut emitted = 0usize;
    for projection in success_data_test_fdm_reference_projections(candidate) {
        let commands = raw_commands
            .iter()
            .filter(|command| success_data_test_fdm_projection_command(projection, command))
            .collect::<Vec<_>>();
        if commands.is_empty() {
            continue;
        }
        if emitted > 0 {
            output.push(',');
        }
        emitted += 1;
        output.push_str("{\"role\":");
        push_json_string(output, projection.role);
        output.push_str(",\"sourcePath\":");
        push_json_string(output, candidate.path());
        output.push_str(",\"projectionKind\":\"successDataTestFdmReferenceProjection\",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"referenceBacked\":true");
        output.push_str(",\"scaleMode\":");
        push_json_string(output, projection.scale_mode);
        output.push_str(",\"sourceBbox\":{\"left\":");
        output.push_str(&projection.source_left.to_string());
        output.push_str(",\"top\":");
        output.push_str(&projection.source_top.to_string());
        output.push_str(",\"right\":");
        output.push_str(&projection.source_right.to_string());
        output.push_str(",\"bottom\":");
        output.push_str(&projection.source_bottom.to_string());
        output.push_str("},\"referenceTargetBboxPx\":{\"x\":");
        output.push_str(&format!("{:.3}", projection.target_x_px));
        output.push_str(",\"y\":");
        output.push_str(&format!("{:.3}", projection.target_y_px));
        output.push_str(",\"width\":");
        output.push_str(&format!("{:.3}", projection.target_width_px));
        output.push_str(",\"height\":");
        output.push_str(&format!("{:.3}", projection.target_height_px));
        output.push_str("},\"commandCount\":");
        output.push_str(&commands.len().to_string());
        output.push_str(",\"sourceCohort\":");
        push_success_data_test_fdm_source_cohort_json(output, &commands);
        output.push_str(",\"renderPromotionBlockedReason\":");
        push_json_string(
            output,
            success_data_test_fdm_source_cohort(&commands).blocked_reason(),
        );
        output.push_str(",\"primitiveOwnershipComparison\":");
        push_success_data_test_fdm_primitive_ownership_comparison_json(
            output,
            projection,
            &commands,
            candidate.fdm_index_entry_candidates(),
            None,
        );
        output.push_str(",\"subdiagrams\":[");
        if let Some(subdiagrams) = success_data_test_q4_fdm_subdiagrams(projection, &commands) {
            for (index, subdiagram) in subdiagrams.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                output.push_str("{\"index\":");
                output.push_str(&subdiagram.index.to_string());
                output.push_str(",\"groupingSource\":\"nearest-main-circle-source-center\",\"groupingDecoded\":false,\"paintOrderDecoded\":false");
                output.push_str(",\"anchorRelativeOffset\":");
                output.push_str(&subdiagram.anchor_relative_offset.to_string());
                output.push_str(",\"anchorSourcePoint\":");
                push_fdm_vector_point_json(output, subdiagram.center);
                output.push_str(",\"commandCount\":");
                output.push_str(&subdiagram.commands.len().to_string());
                output.push_str(",\"sourceCohort\":");
                push_success_data_test_fdm_source_cohort_json(output, &subdiagram.commands);
                output.push_str(",\"renderPromotionBlockedReason\":");
                push_json_string(
                    output,
                    success_data_test_fdm_source_cohort(&subdiagram.commands).blocked_reason(),
                );
                output.push_str(",\"primitiveOwnershipComparison\":");
                push_success_data_test_fdm_primitive_ownership_comparison_json(
                    output,
                    projection,
                    &subdiagram.commands,
                    candidate.fdm_index_entry_candidates(),
                    Some((subdiagram.center, subdiagram.anchor_radius)),
                );
                output.push('}');
            }
        }
        output.push_str("]}");
    }
    output.push(']');
}

#[derive(Copy, Clone)]
struct SuccessDataTestFdmProjection {
    role: &'static str,
    source_left: i32,
    source_top: i32,
    source_right: i32,
    source_bottom: i32,
    target_x_px: f32,
    target_y_px: f32,
    target_width_px: f32,
    target_height_px: f32,
    scale_mode: &'static str,
}

fn success_data_test_fdm_reference_projections(
    candidate: &ObjectStreamCandidate,
) -> Vec<SuccessDataTestFdmProjection> {
    let q4_target_height_px = success_data_test_uniform_target_height_px(
        SUCCESS_DATA_TEST_Q4_SOURCE_LEFT,
        SUCCESS_DATA_TEST_Q4_SOURCE_TOP,
        SUCCESS_DATA_TEST_Q4_SOURCE_RIGHT,
        SUCCESS_DATA_TEST_Q4_SOURCE_BOTTOM,
        SUCCESS_DATA_TEST_Q4_TARGET_WIDTH_PX,
    );
    let mut projections = vec![SuccessDataTestFdmProjection {
        role: "q4-angle-diagrams",
        source_left: SUCCESS_DATA_TEST_Q4_SOURCE_LEFT,
        source_top: SUCCESS_DATA_TEST_Q4_SOURCE_TOP,
        source_right: SUCCESS_DATA_TEST_Q4_SOURCE_RIGHT,
        source_bottom: SUCCESS_DATA_TEST_Q4_SOURCE_BOTTOM,
        target_x_px: SUCCESS_DATA_TEST_Q4_TARGET_X_PX,
        target_y_px: SUCCESS_DATA_TEST_Q4_TARGET_Y_PX,
        target_width_px: SUCCESS_DATA_TEST_Q4_TARGET_WIDTH_PX,
        target_height_px: q4_target_height_px,
        scale_mode: "uniform-units-from-horizontal-span",
    }];
    if let Some(q5_projection) =
        success_data_test_q5_fdm_projection_from_segments(candidate.fdm_raw_vector_segments())
    {
        projections.push(q5_projection);
    }
    projections
}

fn success_data_test_uniform_target_height_px(
    source_left: i32,
    source_top: i32,
    source_right: i32,
    source_bottom: i32,
    target_width_px: f32,
) -> f32 {
    let source_width = source_right.saturating_sub(source_left).abs().max(1) as f32;
    let source_height = source_bottom.saturating_sub(source_top).abs().max(1) as f32;
    source_height / source_width * target_width_px
}

fn success_data_test_q5_fdm_projection_from_segments(
    segments: &[ObjectFdmVectorSegmentCandidate],
) -> Option<SuccessDataTestFdmProjection> {
    let nonzero_span_segments = segments
        .iter()
        .filter(|segment| {
            segment.source_width() > 0 && segment.source_height() > 0 && segment.bbox().is_some()
        })
        .collect::<Vec<_>>();
    if nonzero_span_segments.len() < 2 {
        return None;
    }

    let first_offset = nonzero_span_segments.first()?.relative_offset();
    let mut selected = nonzero_span_segments
        .iter()
        .copied()
        .filter(|segment| segment.relative_offset() != first_offset);
    let first = selected.next()?;
    let first_bbox = first.bbox().map(normalize_fdm_bbox)?;
    let (mut left, mut top, mut right, mut bottom) = first_bbox;
    for segment in selected {
        let bbox = segment.bbox().map(normalize_fdm_bbox)?;
        left = left.min(bbox.0);
        top = top.min(bbox.1);
        right = right.max(bbox.2);
        bottom = bottom.max(bbox.3);
    }

    Some(SuccessDataTestFdmProjection {
        role: "q5-solid-diagram",
        source_left: left,
        source_top: top,
        source_right: right,
        source_bottom: bottom,
        target_x_px: SUCCESS_DATA_TEST_Q5_TARGET_X_PX,
        target_y_px: SUCCESS_DATA_TEST_Q5_TARGET_Y_PX,
        target_width_px: SUCCESS_DATA_TEST_Q5_TARGET_WIDTH_PX,
        target_height_px: SUCCESS_DATA_TEST_Q5_TARGET_HEIGHT_PX,
        scale_mode: "independent-reference-box",
    })
}

fn success_data_test_fdm_projection_command(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    let Some(bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let (center_x, center_y) = fdm_bbox_center(bbox);
    center_x >= projection.source_left
        && center_x <= projection.source_right
        && center_y >= projection.source_top
        && center_y <= projection.source_bottom
}

#[derive(Debug)]
struct SuccessDataTestFdmSubdiagram<'a> {
    index: usize,
    anchor_relative_offset: usize,
    center: ObjectFdmVectorPoint,
    anchor_radius: i32,
    commands: Vec<&'a ObjectFdmVectorCommandCandidate>,
}

fn success_data_test_q4_fdm_subdiagrams<'a>(
    projection: SuccessDataTestFdmProjection,
    commands: &[&'a ObjectFdmVectorCommandCandidate],
) -> Option<Vec<SuccessDataTestFdmSubdiagram<'a>>> {
    if projection.role != "q4-angle-diagrams" {
        return None;
    }
    let mut subdiagrams = commands
        .iter()
        .filter_map(|&command| {
            let ellipse = command.ellipse()?;
            success_data_test_fdm_reference_ellipse_has_center_marker(projection, command, ellipse)
                .then(|| SuccessDataTestFdmSubdiagram {
                    index: 0,
                    anchor_relative_offset: command.relative_offset(),
                    center: ellipse.center(),
                    anchor_radius: ellipse.radius_x().max(ellipse.radius_y()),
                    commands: Vec::new(),
                })
        })
        .collect::<Vec<_>>();
    if subdiagrams.len() < 2 {
        return None;
    }
    subdiagrams.sort_by_key(|subdiagram| {
        (
            subdiagram.center.x(),
            subdiagram.center.y(),
            subdiagram.anchor_relative_offset,
        )
    });
    for (index, subdiagram) in subdiagrams.iter_mut().enumerate() {
        subdiagram.index = index;
    }

    for &command in commands {
        let Some(center) = success_data_test_fdm_command_source_center(command) else {
            continue;
        };
        let Some((group_index, _)) = subdiagrams
            .iter()
            .enumerate()
            .map(|(index, subdiagram)| {
                (index, fdm_point_distance_squared(center, subdiagram.center))
            })
            .min_by_key(|(_, distance)| *distance)
        else {
            continue;
        };
        subdiagrams[group_index].commands.push(command);
    }

    subdiagrams
        .iter()
        .all(|subdiagram| !subdiagram.commands.is_empty())
        .then_some(subdiagrams)
}

fn success_data_test_fdm_command_source_center(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<(i32, i32)> {
    if let Some(ellipse) = command.ellipse() {
        let center = ellipse.center();
        return Some((center.x(), center.y()));
    }
    let bbox = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox)?;
    Some(fdm_bbox_center(bbox))
}

fn success_data_test_fdm_reference_ellipse_has_center_marker(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
    ellipse: ObjectFdmVectorEllipse,
) -> bool {
    if projection.role != "q4-angle-diagrams" || command.marker() != b"\x01\x00\x04\x60" {
        return false;
    }
    let source_height = projection
        .source_bottom
        .saturating_sub(projection.source_top)
        .abs()
        .max(1);
    ellipse.radius_x() == ellipse.radius_y()
        && ellipse.radius_x().saturating_mul(2) >= source_height.saturating_mul(4) / 5
}

fn success_data_test_fdm_reference_ellipse_is_control_marker(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
    ellipse: ObjectFdmVectorEllipse,
) -> bool {
    if projection.role != "q4-angle-diagrams" || command.marker() != b"\xff\x00\x04\x60" {
        return false;
    }
    let source_height = projection
        .source_bottom
        .saturating_sub(projection.source_top)
        .abs()
        .max(1);
    ellipse.radius_x() == ellipse.radius_y()
        && ellipse.radius_x().saturating_mul(6) <= source_height
}

fn fdm_point_distance_squared(left: (i32, i32), right: ObjectFdmVectorPoint) -> i64 {
    let dx = i64::from(left.0) - i64::from(right.x());
    let dy = i64::from(left.1) - i64::from(right.y());
    dx.saturating_mul(dx).saturating_add(dy.saturating_mul(dy))
}

#[derive(Debug)]
struct SuccessDataTestFdmSourceCohort {
    command_relative_offsets: Vec<usize>,
    source_vector_offset_start: Option<usize>,
    source_vector_offset_end: Option<usize>,
    source_vector_offset_count: usize,
    segment_backed_count: usize,
    raw_span_count: usize,
    segment_offsets: Vec<usize>,
}

impl SuccessDataTestFdmSourceCohort {
    fn blocked_reason(&self) -> &'static str {
        if self.raw_span_count > 0 && self.segment_backed_count > 0 {
            "mixed-raw-and-segment-cohorts"
        } else if self.segment_offsets.len() > 1 {
            "multiple-source-segment-cohorts"
        } else {
            "source-owner-candidate-unproven"
        }
    }
}

fn success_data_test_fdm_source_cohort(
    commands: &[&ObjectFdmVectorCommandCandidate],
) -> SuccessDataTestFdmSourceCohort {
    let mut segment_offsets = std::collections::BTreeSet::new();
    let mut command_relative_offsets = Vec::new();
    let mut source_vector_offset_start: Option<usize> = None;
    let mut source_vector_offset_end: Option<usize> = None;
    let mut source_vector_offset_count = 0usize;
    let mut segment_backed_count = 0usize;
    for command in commands {
        command_relative_offsets.push(command.relative_offset());
        if let Some(source_vector_relative_offset) = command.source_vector_relative_offset() {
            source_vector_offset_count += 1;
            source_vector_offset_start = Some(
                source_vector_offset_start
                    .map(|start| start.min(source_vector_relative_offset))
                    .unwrap_or(source_vector_relative_offset),
            );
            source_vector_offset_end = Some(
                source_vector_offset_end
                    .map(|end| end.max(source_vector_relative_offset))
                    .unwrap_or(source_vector_relative_offset),
            );
        }
        if let Some(source_segment) = command.source_segment() {
            segment_backed_count += 1;
            segment_offsets.insert(source_segment.relative_offset());
        }
    }
    let raw_span_count = commands.len().saturating_sub(segment_backed_count);
    SuccessDataTestFdmSourceCohort {
        command_relative_offsets,
        source_vector_offset_start,
        source_vector_offset_end,
        source_vector_offset_count,
        segment_backed_count,
        raw_span_count,
        segment_offsets: segment_offsets.into_iter().collect(),
    }
}

fn push_success_data_test_fdm_source_cohort_json(
    output: &mut String,
    commands: &[&ObjectFdmVectorCommandCandidate],
) {
    let cohort = success_data_test_fdm_source_cohort(commands);
    output.push_str("{\"provenance\":\"fdm-vector-command\",\"ownershipBasis\":\"fdmVectorCommandProvenance\",\"ownershipProven\":false");
    output.push_str(",\"ownershipPromotionBlockedReason\":");
    push_json_string(output, cohort.blocked_reason());
    output.push_str(",\"sourceVectorOffsetStart\":");
    push_option_usize_json(output, cohort.source_vector_offset_start);
    output.push_str(",\"sourceVectorOffsetEnd\":");
    push_option_usize_json(output, cohort.source_vector_offset_end);
    output.push_str(",\"commandRelativeOffsets\":");
    push_usize_array_json(output, &cohort.command_relative_offsets);
    output.push_str(",\"sourceVectorOffsetCommandCount\":");
    output.push_str(&cohort.source_vector_offset_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&cohort.segment_backed_count.to_string());
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&cohort.raw_span_count.to_string());
    output.push_str(",\"sourceSegmentCohortCount\":");
    output.push_str(&cohort.segment_offsets.len().to_string());
    output.push_str(",\"sourceSegmentRelativeOffsets\":");
    push_usize_array_json(output, &cohort.segment_offsets);
    output.push('}');
}

#[derive(Debug)]
struct SuccessDataTestFdmPrimitiveOwnershipClassification<'a> {
    command: &'a ObjectFdmVectorCommandCandidate,
    role_candidates: Vec<&'static str>,
    classification_basis: Vec<&'static str>,
    index_row_references: Vec<SuccessDataTestFdmIndexRowReference>,
}

#[derive(Debug)]
struct SuccessDataTestFdmIndexRowReference {
    row_index: usize,
    index_offset: usize,
    vector_offset: usize,
    valid_vector_offset: bool,
    offset_field: &'static str,
    offset_value: usize,
    match_kind: &'static str,
}

fn push_success_data_test_fdm_primitive_ownership_comparison_json(
    output: &mut String,
    projection: SuccessDataTestFdmProjection,
    commands: &[&ObjectFdmVectorCommandCandidate],
    index_entries: &[ObjectFdmIndexEntryCandidate],
    anchor: Option<(ObjectFdmVectorPoint, i32)>,
) {
    let classifications = commands
        .iter()
        .map(|&command| {
            success_data_test_fdm_primitive_ownership_classification(
                projection,
                command,
                index_entries,
                anchor,
            )
        })
        .collect::<Vec<_>>();
    output.push_str("{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false");
    output.push_str(
        ",\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\"",
    );
    output.push_str(",\"commandCount\":");
    output.push_str(&classifications.len().to_string());
    push_success_data_test_fdm_role_count_json(
        output,
        "mainCircleAnchorCount",
        &classifications,
        "main-circle-anchor",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "lineCandidateCount",
        &classifications,
        "line-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "radialLineCandidateCount",
        &classifications,
        "radial-line-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "chordCandidateCount",
        &classifications,
        "chord-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "arcCandidateCount",
        &classifications,
        "arc-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "connectorCandidateCount",
        &classifications,
        "connector-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "surfaceBoundaryCandidateCount",
        &classifications,
        "surface-boundary-candidate",
    );
    output.push_str(",\"indexRowReferenceCandidateCount\":");
    output.push_str(
        &classifications
            .iter()
            .map(|classification| classification.index_row_references.len())
            .sum::<usize>()
            .to_string(),
    );
    output.push_str(",\"validVectorOffsetIndexRowReferenceCount\":");
    output.push_str(
        &classifications
            .iter()
            .flat_map(|classification| classification.index_row_references.iter())
            .filter(|reference| reference.valid_vector_offset)
            .count()
            .to_string(),
    );
    output.push_str(",\"ownershipGate\":");
    push_success_data_test_fdm_primitive_ownership_gate_json(output, &classifications);
    output.push_str(",\"offsetFieldAuthorityGate\":");
    push_success_data_test_fdm_offset_field_authority_gate_json(output, &classifications);
    output.push_str(",\"rowFanoutSegmentOwnerGate\":");
    push_success_data_test_fdm_row_fanout_segment_owner_gate_json(output, &classifications);
    output.push_str(",\"primitiveOwnershipAdmissionGate\":");
    push_success_data_test_fdm_primitive_ownership_admission_gate_json(output, &classifications);
    output.push_str(",\"indexRowOrderPromotionGate\":");
    push_success_data_test_fdm_index_row_order_promotion_gate_json(output, &classifications);
    output.push_str(",\"indexRowReferenceRoleCandidateGroups\":");
    push_success_data_test_fdm_index_row_reference_role_candidate_groups_json(
        output,
        &classifications,
    );
    output.push_str(",\"classifications\":[");
    for (index, classification) in classifications.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"relativeOffset\":");
        output.push_str(&classification.command.relative_offset().to_string());
        output.push_str(",\"primitiveKind\":");
        push_json_string(output, fdm_vector_primitive_kind(classification.command));
        output.push_str(",\"markerHex\":");
        push_json_string(output, &hex(classification.command.marker()));
        output.push_str(",\"sourceSegmentBacked\":");
        output.push_str(if classification.command.source_segment().is_some() {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"sourceSegmentRelativeOffset\":");
        push_option_usize_json(
            output,
            classification
                .command
                .source_segment()
                .map(|segment| segment.relative_offset()),
        );
        output.push_str(",\"roleCandidates\":");
        push_json_string_slice_array(output, &classification.role_candidates);
        output.push_str(",\"classificationBasis\":");
        push_json_string_slice_array(output, &classification.classification_basis);
        output.push_str(",\"indexRowReferenceCandidates\":");
        push_success_data_test_fdm_index_row_references_json(
            output,
            &classification.index_row_references,
        );
        output.push('}');
    }
    output.push_str("]}");
}

fn push_success_data_test_fdm_role_count_json(
    output: &mut String,
    field_name: &str,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
    role: &str,
) {
    let count = classifications
        .iter()
        .filter(|classification| classification.role_candidates.contains(&role))
        .count();
    output.push(',');
    push_json_string(output, field_name);
    output.push(':');
    output.push_str(&count.to_string());
}

#[derive(Debug, Default)]
struct SuccessDataTestFdmIndexRowOrderPromotionGate {
    command_count: usize,
    referenced_command_relative_offsets: BTreeSet<usize>,
    referenced_row_indexes: BTreeSet<usize>,
    row_command_pairs: BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
    row_to_command_relative_offsets: BTreeMap<usize, BTreeSet<usize>>,
    reference_count: usize,
    valid_vector_offset_reference_count: usize,
    command_relative_offset_field_reference_count: usize,
    source_segment_relative_offset_field_reference_count: usize,
}

impl SuccessDataTestFdmIndexRowOrderPromotionGate {
    fn referenced_command_count(&self) -> usize {
        self.referenced_command_relative_offsets.len()
    }

    fn unreferenced_command_count(&self) -> usize {
        self.command_count
            .saturating_sub(self.referenced_command_count())
    }

    fn unique_row_index_count(&self) -> usize {
        self.referenced_row_indexes.len()
    }

    fn all_commands_referenced_by_index_rows_candidate(&self) -> bool {
        self.command_count > 0 && self.unreferenced_command_count() == 0
    }

    fn one_to_one_row_command_reference_candidate(&self) -> bool {
        self.reference_count == self.referenced_command_count()
            && self.reference_count == self.unique_row_index_count()
    }

    fn single_row_backs_multiple_commands_candidate(&self) -> bool {
        self.row_to_command_relative_offsets
            .values()
            .any(|offsets| offsets.len() > 1)
    }

    fn row_order_matches_command_order_candidate(&self) -> bool {
        success_data_test_fdm_row_command_pairs_are_monotonic(&self.row_command_pairs)
    }
}

#[derive(Debug)]
struct SuccessDataTestFdmOffsetFieldAuthorityGate {
    command_count: usize,
    reference_count: usize,
    valid_vector_offset_reference_count: usize,
    command_relative_offset_field_reference_count: usize,
    source_segment_relative_offset_field_reference_count: usize,
    unclassified_offset_field_reference_count: usize,
    raw_span_command_count: usize,
    segment_backed_command_count: usize,
    mixed_offset_field_namespaces: bool,
    mixed_command_provenance_cohorts: bool,
    all_references_use_command_relative_offset_field: bool,
    all_references_use_source_segment_relative_offset_field: bool,
    render_promotion_blocked_reason: &'static str,
}

fn success_data_test_fdm_offset_field_authority_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmOffsetFieldAuthorityGate {
    let order_gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let raw_span_command_count = classifications
        .iter()
        .filter(|classification| classification.command.source_segment().is_none())
        .count();
    let segment_backed_command_count = classifications.len().saturating_sub(raw_span_command_count);
    let unclassified_offset_field_reference_count = order_gate
        .reference_count
        .saturating_sub(order_gate.command_relative_offset_field_reference_count)
        .saturating_sub(order_gate.source_segment_relative_offset_field_reference_count);
    let mixed_offset_field_namespaces = order_gate.command_relative_offset_field_reference_count
        > 0
        && order_gate.source_segment_relative_offset_field_reference_count > 0;
    let mixed_command_provenance_cohorts =
        raw_span_command_count > 0 && segment_backed_command_count > 0;
    let all_references_use_command_relative_offset_field = order_gate.reference_count > 0
        && order_gate.command_relative_offset_field_reference_count == order_gate.reference_count;
    let all_references_use_source_segment_relative_offset_field = order_gate.reference_count > 0
        && order_gate.source_segment_relative_offset_field_reference_count
            == order_gate.reference_count;
    let render_promotion_blocked_reason = if mixed_offset_field_namespaces {
        "fdm-index-offset-field-authority-mixed-command-and-segment-fields"
    } else if mixed_command_provenance_cohorts {
        "fdm-index-offset-field-authority-mixed-raw-and-segment-cohorts"
    } else if unclassified_offset_field_reference_count > 0 {
        "fdm-index-offset-field-authority-unclassified-fields"
    } else if order_gate.valid_vector_offset_reference_count == 0 {
        "fdm-index-offset-field-authority-valid-vector-offset-missing"
    } else {
        "fdm-index-offset-field-authority-semantics-unproven"
    };

    SuccessDataTestFdmOffsetFieldAuthorityGate {
        command_count: order_gate.command_count,
        reference_count: order_gate.reference_count,
        valid_vector_offset_reference_count: order_gate.valid_vector_offset_reference_count,
        command_relative_offset_field_reference_count: order_gate
            .command_relative_offset_field_reference_count,
        source_segment_relative_offset_field_reference_count: order_gate
            .source_segment_relative_offset_field_reference_count,
        unclassified_offset_field_reference_count,
        raw_span_command_count,
        segment_backed_command_count,
        mixed_offset_field_namespaces,
        mixed_command_provenance_cohorts,
        all_references_use_command_relative_offset_field,
        all_references_use_source_segment_relative_offset_field,
        render_promotion_blocked_reason,
    }
}

fn push_success_data_test_fdm_offset_field_authority_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_offset_field_authority_gate(classifications);
    output.push_str("{\"basis\":\"fdm-index-offset-field-authority-gate\",\"source\":\"FDMIndex row offset fields+FDMVector command provenance\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"offsetFieldAuthorityDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-offset-field-authority-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, gate.render_promotion_blocked_reason);
    output.push_str(",\"commandCount\":");
    output.push_str(&gate.command_count.to_string());
    output.push_str(",\"referenceCount\":");
    output.push_str(&gate.reference_count.to_string());
    output.push_str(",\"validVectorOffsetReferenceCount\":");
    output.push_str(&gate.valid_vector_offset_reference_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"unclassifiedOffsetFieldReferenceCount\":");
    output.push_str(&gate.unclassified_offset_field_reference_count.to_string());
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&gate.raw_span_command_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&gate.segment_backed_command_count.to_string());
    output.push_str(",\"mixedOffsetFieldNamespaces\":");
    output.push_str(if gate.mixed_offset_field_namespaces {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedCommandProvenanceCohorts\":");
    output.push_str(if gate.mixed_command_provenance_cohorts {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allReferencesUseCommandRelativeOffsetField\":");
    output.push_str(if gate.all_references_use_command_relative_offset_field {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allReferencesUseSourceSegmentRelativeOffsetField\":");
    output.push_str(
        if gate.all_references_use_source_segment_relative_offset_field {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

#[derive(Debug)]
struct SuccessDataTestFdmRowFanoutSegmentOwnerGate {
    command_count: usize,
    reference_count: usize,
    unique_row_index_count: usize,
    command_relative_offset_field_reference_count: usize,
    source_segment_relative_offset_field_reference_count: usize,
    fanout_row_count: usize,
    fanout_reference_count: usize,
    fanout_command_relative_offset_field_reference_count: usize,
    fanout_source_segment_relative_offset_field_reference_count: usize,
    max_row_fanout: usize,
    multi_command_row_indexes: Vec<usize>,
    rows_with_multiple_command_refs: Vec<SuccessDataTestFdmRowFanoutSegmentOwnerRow>,
    one_to_one_row_command_reference_candidate: bool,
    single_row_backs_multiple_commands_candidate: bool,
    mixed_offset_field_namespaces: bool,
    mixed_command_provenance_cohorts: bool,
    fanout_rows_use_command_relative_offset_fields: bool,
    fanout_rows_use_source_segment_offset_fields: bool,
    raw_span_command_count: usize,
    segment_backed_command_count: usize,
    render_promotion_blocked_reason: &'static str,
}

#[derive(Debug)]
struct SuccessDataTestFdmRowFanoutSegmentOwnerRow {
    row_index: usize,
    command_reference_count: usize,
    command_relative_offsets: Vec<usize>,
    match_kinds: Vec<&'static str>,
}

fn success_data_test_fdm_row_fanout_segment_owner_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmRowFanoutSegmentOwnerGate {
    let order_gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let raw_span_command_count = classifications
        .iter()
        .filter(|classification| classification.command.source_segment().is_none())
        .count();
    let segment_backed_command_count = classifications.len().saturating_sub(raw_span_command_count);
    let mut multi_command_row_indexes = Vec::new();
    let mut fanout_reference_count = 0usize;
    let mut fanout_command_relative_offset_field_reference_count = 0usize;
    let mut fanout_source_segment_relative_offset_field_reference_count = 0usize;
    let mut max_row_fanout = 0usize;
    let mut rows_with_multiple_command_refs = Vec::new();
    for (row_index, command_offsets) in &order_gate.row_to_command_relative_offsets {
        max_row_fanout = max_row_fanout.max(command_offsets.len());
        if command_offsets.len() <= 1 {
            continue;
        }
        multi_command_row_indexes.push(*row_index);
        let row_pairs = order_gate
            .row_command_pairs
            .iter()
            .filter(|pair| pair.row_index == *row_index)
            .collect::<Vec<_>>();
        for pair in &row_pairs {
            fanout_reference_count += 1;
            match pair.match_kind {
                "command-relative-offset-field" => {
                    fanout_command_relative_offset_field_reference_count += 1;
                }
                "source-segment-relative-offset-field" => {
                    fanout_source_segment_relative_offset_field_reference_count += 1;
                }
                _ => {}
            }
        }
        rows_with_multiple_command_refs.push(SuccessDataTestFdmRowFanoutSegmentOwnerRow {
            row_index: *row_index,
            command_reference_count: row_pairs.len(),
            command_relative_offsets: row_pairs
                .iter()
                .map(|pair| pair.command_relative_offset)
                .collect(),
            match_kinds: row_pairs
                .iter()
                .map(|pair| pair.match_kind)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
        });
    }
    let mixed_offset_field_namespaces = order_gate.command_relative_offset_field_reference_count
        > 0
        && order_gate.source_segment_relative_offset_field_reference_count > 0;
    let mixed_command_provenance_cohorts =
        raw_span_command_count > 0 && segment_backed_command_count > 0;
    let single_row_backs_multiple_commands_candidate =
        order_gate.single_row_backs_multiple_commands_candidate();
    let one_to_one_row_command_reference_candidate =
        order_gate.one_to_one_row_command_reference_candidate();
    let fanout_rows_use_command_relative_offset_fields = fanout_reference_count > 0
        && fanout_command_relative_offset_field_reference_count == fanout_reference_count;
    let fanout_rows_use_source_segment_offset_fields = fanout_reference_count > 0
        && fanout_source_segment_relative_offset_field_reference_count == fanout_reference_count;
    let render_promotion_blocked_reason = if single_row_backs_multiple_commands_candidate {
        "fdm-index-row-fanout-segment-owner-multi-command-single-row"
    } else if !one_to_one_row_command_reference_candidate {
        "fdm-index-row-fanout-segment-owner-not-one-to-one"
    } else if mixed_offset_field_namespaces {
        "fdm-index-row-fanout-segment-owner-offset-namespace-mixed"
    } else if mixed_command_provenance_cohorts {
        "fdm-index-row-fanout-segment-owner-mixed-raw-and-segment-cohorts"
    } else {
        "fdm-index-row-fanout-segment-owner-semantics-unproven"
    };

    SuccessDataTestFdmRowFanoutSegmentOwnerGate {
        command_count: order_gate.command_count,
        reference_count: order_gate.reference_count,
        unique_row_index_count: order_gate.unique_row_index_count(),
        command_relative_offset_field_reference_count: order_gate
            .command_relative_offset_field_reference_count,
        source_segment_relative_offset_field_reference_count: order_gate
            .source_segment_relative_offset_field_reference_count,
        fanout_row_count: multi_command_row_indexes.len(),
        fanout_reference_count,
        fanout_command_relative_offset_field_reference_count,
        fanout_source_segment_relative_offset_field_reference_count,
        max_row_fanout,
        multi_command_row_indexes,
        rows_with_multiple_command_refs,
        one_to_one_row_command_reference_candidate,
        single_row_backs_multiple_commands_candidate,
        mixed_offset_field_namespaces,
        mixed_command_provenance_cohorts,
        fanout_rows_use_command_relative_offset_fields,
        fanout_rows_use_source_segment_offset_fields,
        raw_span_command_count,
        segment_backed_command_count,
        render_promotion_blocked_reason,
    }
}

fn push_success_data_test_fdm_row_fanout_segment_owner_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_row_fanout_segment_owner_gate(classifications);
    output.push_str("{\"basis\":\"fdm-index-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"rowFanoutDecoded\":false,\"segmentOwnerDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-row-fanout-segment-owner-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, gate.render_promotion_blocked_reason);
    output.push_str(",\"commandCount\":");
    output.push_str(&gate.command_count.to_string());
    output.push_str(",\"referenceCount\":");
    output.push_str(&gate.reference_count.to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&gate.unique_row_index_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"fanoutRowCount\":");
    output.push_str(&gate.fanout_row_count.to_string());
    output.push_str(",\"fanoutReferenceCount\":");
    output.push_str(&gate.fanout_reference_count.to_string());
    output.push_str(",\"fanoutCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .fanout_command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"fanoutSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .fanout_source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"maxRowFanout\":");
    output.push_str(&gate.max_row_fanout.to_string());
    output.push_str(",\"multiCommandRowIndexes\":");
    push_usize_array_json(output, &gate.multi_command_row_indexes);
    output.push_str(",\"rowsWithMultipleCommandRefs\":");
    push_success_data_test_fdm_row_fanout_segment_owner_rows_json(
        output,
        &gate.rows_with_multiple_command_refs,
    );
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if gate.one_to_one_row_command_reference_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if gate.single_row_backs_multiple_commands_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedOffsetFieldNamespaces\":");
    output.push_str(if gate.mixed_offset_field_namespaces {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedCommandProvenanceCohorts\":");
    output.push_str(if gate.mixed_command_provenance_cohorts {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseCommandRelativeOffsetFields\":");
    output.push_str(if gate.fanout_rows_use_command_relative_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseSourceSegmentOffsetFields\":");
    output.push_str(if gate.fanout_rows_use_source_segment_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&gate.raw_span_command_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&gate.segment_backed_command_count.to_string());
    output.push('}');
}

fn push_success_data_test_fdm_row_fanout_segment_owner_rows_json(
    output: &mut String,
    rows: &[SuccessDataTestFdmRowFanoutSegmentOwnerRow],
) {
    output.push('[');
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&row.row_index.to_string());
        output.push_str(",\"commandReferenceCount\":");
        output.push_str(&row.command_reference_count.to_string());
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(output, &row.command_relative_offsets);
        output.push_str(",\"matchKinds\":");
        push_json_string_slice_array(output, &row.match_kinds);
        output.push('}');
    }
    output.push(']');
}

#[derive(Debug)]
struct SuccessDataTestFdmPrimitiveOwnershipGate {
    row_command_gap_p95: Option<f32>,
    row_direction_mismatch: bool,
    multi_command_single_row: bool,
    all_commands_referenced_by_index_rows_candidate: bool,
    one_to_one_row_command_reference_candidate: bool,
    mixed_raw_and_segment_cohorts: bool,
    raw_span_command_count: usize,
    segment_backed_command_count: usize,
    ownership_proven: bool,
    render_ownership_blocked_reason: &'static str,
    render_ownership_blocked_reasons: Vec<&'static str>,
}

fn success_data_test_fdm_primitive_ownership_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmPrimitiveOwnershipGate {
    let order_gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let raw_span_command_count = classifications
        .iter()
        .filter(|classification| classification.command.source_segment().is_none())
        .count();
    let segment_backed_command_count = classifications.len().saturating_sub(raw_span_command_count);
    let row_direction_mismatch = !order_gate.row_order_matches_command_order_candidate();
    let multi_command_single_row = order_gate.single_row_backs_multiple_commands_candidate();
    let all_commands_referenced_by_index_rows_candidate =
        order_gate.all_commands_referenced_by_index_rows_candidate();
    let one_to_one_row_command_reference_candidate =
        order_gate.one_to_one_row_command_reference_candidate();
    let mixed_raw_and_segment_cohorts =
        raw_span_command_count > 0 && segment_backed_command_count > 0;
    let mut render_ownership_blocked_reasons = Vec::new();
    if row_direction_mismatch {
        render_ownership_blocked_reasons.push("row-command-direction-mismatch");
    }
    if !all_commands_referenced_by_index_rows_candidate {
        render_ownership_blocked_reasons.push("index-row-reference-coverage-incomplete");
    }
    if multi_command_single_row {
        render_ownership_blocked_reasons.push("multi-command-single-index-row");
    }
    if mixed_raw_and_segment_cohorts {
        render_ownership_blocked_reasons.push("mixed-raw-and-segment-cohorts");
    }
    if !one_to_one_row_command_reference_candidate {
        render_ownership_blocked_reasons.push("row-command-reference-not-one-to-one");
    }
    let render_ownership_blocked_reason = render_ownership_blocked_reasons
        .first()
        .copied()
        .unwrap_or("fdm-index-row-ownership-unproven");

    SuccessDataTestFdmPrimitiveOwnershipGate {
        row_command_gap_p95: success_data_test_fdm_command_gap_p95(
            &order_gate.referenced_command_relative_offsets,
        ),
        row_direction_mismatch,
        multi_command_single_row,
        all_commands_referenced_by_index_rows_candidate,
        one_to_one_row_command_reference_candidate,
        mixed_raw_and_segment_cohorts,
        raw_span_command_count,
        segment_backed_command_count,
        ownership_proven: false,
        render_ownership_blocked_reason,
        render_ownership_blocked_reasons,
    }
}

fn success_data_test_fdm_command_gap_p95(offsets: &BTreeSet<usize>) -> Option<f32> {
    let mut gaps = Vec::new();
    let mut previous_offset = None;
    for offset in offsets.iter().copied() {
        if let Some(previous) = previous_offset {
            gaps.push(offset.saturating_sub(previous));
        }
        previous_offset = Some(offset);
    }
    if gaps.is_empty() {
        return None;
    }
    gaps.sort_unstable();
    let rank = ((gaps.len() as f32) * 0.95).ceil() as usize;
    let index = rank.saturating_sub(1).min(gaps.len() - 1);
    Some(gaps[index] as f32)
}

fn push_success_data_test_fdm_primitive_ownership_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_primitive_ownership_gate(classifications);
    output.push_str("{\"basis\":\"fdm-index-row-reference-primitive-ownership-gate\",\"source\":\"FDMIndex row references+FDMVector command provenance\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"ownershipProven\":");
    output.push_str(if gate.ownership_proven {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paintOrderDecoded\":false,\"renderOwnershipPromoted\":false");
    output.push_str(",\"renderOwnershipBlockedReason\":");
    push_json_string(output, gate.render_ownership_blocked_reason);
    output.push_str(",\"renderOwnershipBlockedReasons\":");
    push_json_string_slice_array(output, &gate.render_ownership_blocked_reasons);
    output.push_str(",\"rowCommandGapP95\":");
    push_option_f32_json(output, gate.row_command_gap_p95);
    output.push_str(",\"rowDirectionMismatch\":");
    output.push_str(if gate.row_direction_mismatch {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"multiCommandSingleRow\":");
    output.push_str(if gate.multi_command_single_row {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allCommandsReferencedByIndexRowsCandidate\":");
    output.push_str(if gate.all_commands_referenced_by_index_rows_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if gate.one_to_one_row_command_reference_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedRawAndSegmentCohorts\":");
    output.push_str(if gate.mixed_raw_and_segment_cohorts {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&gate.raw_span_command_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&gate.segment_backed_command_count.to_string());
    output.push('}');
}

fn push_success_data_test_fdm_primitive_ownership_admission_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let ownership_gate = success_data_test_fdm_primitive_ownership_gate(classifications);
    let offset_field_gate = success_data_test_fdm_offset_field_authority_gate(classifications);
    let row_fanout_gate = success_data_test_fdm_row_fanout_segment_owner_gate(classifications);
    let role_groups =
        success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);

    let mut role_fanout_blocked_role_candidates = Vec::new();
    let mut role_vector_offset_authority_blocked_role_candidates = Vec::new();
    let mut role_vector_offset_authority_blocked_reasons = Vec::new();
    let mut role_valid_vector_offset_missing_role_candidates = Vec::new();
    let mut role_paint_order_blocked_role_candidates = Vec::new();
    let mut role_paint_order_authority_pending_role_candidates = Vec::new();
    for group in role_groups.values() {
        if success_data_test_fdm_role_group_single_row_backs_multiple_commands(group) {
            role_fanout_blocked_role_candidates.push(group.role_candidate);
        }
        let role_vector_offset_authority_blocked_reason =
            success_data_test_fdm_role_vector_offset_authority_blocked_reason(group);
        push_unique_static_str(
            &mut role_vector_offset_authority_blocked_reasons,
            role_vector_offset_authority_blocked_reason,
        );
        role_vector_offset_authority_blocked_role_candidates.push(group.role_candidate);
        if group.valid_vector_offset_reference_count == 0 && group.reference_count > 0 {
            role_valid_vector_offset_missing_role_candidates.push(group.role_candidate);
        }
        let paint_order_profile =
            success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
        if paint_order_profile.continuity_blocked() {
            role_paint_order_blocked_role_candidates.push(group.role_candidate);
        } else if paint_order_profile.paint_order_authority_pending() {
            role_paint_order_authority_pending_role_candidates.push(group.role_candidate);
        }
    }

    let role_fanout_blocked_group_count = role_fanout_blocked_role_candidates.len();
    let role_vector_offset_authority_blocked_group_count =
        role_vector_offset_authority_blocked_role_candidates.len();
    let role_valid_vector_offset_missing_group_count =
        role_valid_vector_offset_missing_role_candidates.len();
    let role_paint_order_blocked_group_count = role_paint_order_blocked_role_candidates.len();
    let role_paint_order_authority_pending_group_count =
        role_paint_order_authority_pending_role_candidates.len();
    let mut render_promotion_blocked_reasons = Vec::new();
    for reason in &ownership_gate.render_ownership_blocked_reasons {
        push_unique_static_str(&mut render_promotion_blocked_reasons, reason);
    }
    push_unique_static_str(
        &mut render_promotion_blocked_reasons,
        offset_field_gate.render_promotion_blocked_reason,
    );
    push_unique_static_str(
        &mut render_promotion_blocked_reasons,
        row_fanout_gate.render_promotion_blocked_reason,
    );
    if role_fanout_blocked_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "fdm-index-role-row-fanout-multi-command-single-row",
        );
    }
    for reason in &role_vector_offset_authority_blocked_reasons {
        push_unique_static_str(&mut render_promotion_blocked_reasons, reason);
    }
    if role_valid_vector_offset_missing_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "fdm-index-role-valid-vector-offset-missing",
        );
    }
    if role_paint_order_blocked_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "role-paint-order-continuity-unproven",
        );
    }
    if role_paint_order_authority_pending_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "role-paint-order-authority-unproven",
        );
    }
    let render_admission_ready = render_promotion_blocked_reasons.is_empty();
    let render_promotion_blocked_reason = render_promotion_blocked_reasons
        .first()
        .copied()
        .unwrap_or("none");

    output.push_str("{\"basis\":\"fdm-primitive-ownership-admission-gate\",\"source\":\"ownershipGate+offsetFieldAuthorityGate+rowFanoutSegmentOwnerGate+roleFanoutSegmentOwnerGate+paintOrderContinuityProfile\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"ownershipProven\":false,\"paintOrderDecoded\":false");
    output.push_str(",\"renderAdmissionReady\":");
    output.push_str(if render_admission_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"fdm-primitive-ownership-admission-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, render_promotion_blocked_reason);
    output.push_str(",\"renderPromotionBlockedReasons\":");
    push_json_string_slice_array(output, &render_promotion_blocked_reasons);
    output.push_str(",\"commandCount\":");
    output.push_str(
        &ownership_gate
            .raw_span_command_count
            .saturating_add(ownership_gate.segment_backed_command_count)
            .to_string(),
    );
    output.push_str(",\"referenceCount\":");
    output.push_str(&offset_field_gate.reference_count.to_string());
    output.push_str(",\"roleGroupCount\":");
    output.push_str(&role_groups.len().to_string());
    output.push_str(",\"ownershipGateBlockedReason\":");
    push_json_string(output, ownership_gate.render_ownership_blocked_reason);
    output.push_str(",\"offsetFieldAuthorityBlockedReason\":");
    push_json_string(output, offset_field_gate.render_promotion_blocked_reason);
    output.push_str(",\"rowFanoutSegmentOwnerBlockedReason\":");
    push_json_string(output, row_fanout_gate.render_promotion_blocked_reason);
    output.push_str(",\"projectionRowFanoutBlocked\":");
    output.push_str(
        if row_fanout_gate.single_row_backs_multiple_commands_candidate {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"roleFanoutBlockedGroupCount\":");
    output.push_str(&role_fanout_blocked_group_count.to_string());
    output.push_str(",\"roleFanoutBlockedRoleCandidates\":");
    push_json_string_slice_array(output, &role_fanout_blocked_role_candidates);
    output.push_str(",\"roleVectorOffsetAuthorityBlockedGroupCount\":");
    output.push_str(&role_vector_offset_authority_blocked_group_count.to_string());
    output.push_str(",\"roleVectorOffsetAuthorityBlockedRoleCandidates\":");
    push_json_string_slice_array(
        output,
        &role_vector_offset_authority_blocked_role_candidates,
    );
    output.push_str(",\"roleVectorOffsetAuthorityBlockedReasons\":");
    push_json_string_slice_array(output, &role_vector_offset_authority_blocked_reasons);
    output.push_str(",\"roleValidVectorOffsetMissingGroupCount\":");
    output.push_str(&role_valid_vector_offset_missing_group_count.to_string());
    output.push_str(",\"roleValidVectorOffsetMissingRoleCandidates\":");
    push_json_string_slice_array(output, &role_valid_vector_offset_missing_role_candidates);
    output.push_str(",\"rolePaintOrderBlockedGroupCount\":");
    output.push_str(&role_paint_order_blocked_group_count.to_string());
    output.push_str(",\"rolePaintOrderBlockedRoleCandidates\":");
    push_json_string_slice_array(output, &role_paint_order_blocked_role_candidates);
    output.push_str(",\"rolePaintOrderAuthorityPendingGroupCount\":");
    output.push_str(&role_paint_order_authority_pending_group_count.to_string());
    output.push_str(",\"rolePaintOrderAuthorityPendingRoleCandidates\":");
    push_json_string_slice_array(output, &role_paint_order_authority_pending_role_candidates);
    output.push('}');
}

fn push_unique_static_str(values: &mut Vec<&'static str>, value: &'static str) {
    if value != "none" && !values.contains(&value) {
        values.push(value);
    }
}

fn success_data_test_fdm_index_row_order_promotion_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmIndexRowOrderPromotionGate {
    let mut gate = SuccessDataTestFdmIndexRowOrderPromotionGate {
        command_count: classifications.len(),
        ..SuccessDataTestFdmIndexRowOrderPromotionGate::default()
    };

    for classification in classifications {
        for reference in &classification.index_row_references {
            gate.reference_count += 1;
            gate.referenced_command_relative_offsets
                .insert(classification.command.relative_offset());
            gate.referenced_row_indexes.insert(reference.row_index);
            gate.row_command_pairs
                .insert(SuccessDataTestFdmIndexRowCommandPair {
                    row_index: reference.row_index,
                    command_relative_offset: classification.command.relative_offset(),
                    match_kind: reference.match_kind,
                });
            gate.row_to_command_relative_offsets
                .entry(reference.row_index)
                .or_default()
                .insert(classification.command.relative_offset());
            if reference.valid_vector_offset {
                gate.valid_vector_offset_reference_count += 1;
            }
            match reference.match_kind {
                "command-relative-offset-field" => {
                    gate.command_relative_offset_field_reference_count += 1;
                }
                "source-segment-relative-offset-field" => {
                    gate.source_segment_relative_offset_field_reference_count += 1;
                }
                _ => {}
            }
        }
    }
    gate
}

fn push_success_data_test_fdm_index_row_order_promotion_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let render_promotion_blocked_reasons =
        success_data_test_fdm_index_row_order_promotion_blocked_reasons(classifications, &gate);
    let render_promotion_blocked_reason = render_promotion_blocked_reasons
        .first()
        .copied()
        .unwrap_or("none");
    output.push_str("{\"basis\":\"fdm-index-row-reference-command-order\",\"decoded\":false,\"ownershipProven\":false,\"paintOrderDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-row-order-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, render_promotion_blocked_reason);
    output.push_str(",\"renderPromotionBlockedReasons\":");
    push_json_string_slice_array(output, &render_promotion_blocked_reasons);
    output.push_str(",\"commandCount\":");
    output.push_str(&gate.command_count.to_string());
    output.push_str(",\"referencedCommandCount\":");
    output.push_str(&gate.referenced_command_count().to_string());
    output.push_str(",\"unreferencedCommandCount\":");
    output.push_str(&gate.unreferenced_command_count().to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&gate.unique_row_index_count().to_string());
    output.push_str(",\"referenceCount\":");
    output.push_str(&gate.reference_count.to_string());
    output.push_str(",\"validVectorOffsetReferenceCount\":");
    output.push_str(&gate.valid_vector_offset_reference_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"allCommandsReferencedByIndexRowsCandidate\":");
    output.push_str(if gate.all_commands_referenced_by_index_rows_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if gate.one_to_one_row_command_reference_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if gate.single_row_backs_multiple_commands_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowOrderMatchesCommandOrderCandidate\":");
    output.push_str(if gate.row_order_matches_command_order_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"referencedCommandRelativeOffsets\":");
    push_usize_array_json(
        output,
        &gate
            .referenced_command_relative_offsets
            .iter()
            .copied()
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"referencedRowIndexes\":");
    push_usize_array_json(
        output,
        &gate
            .referenced_row_indexes
            .iter()
            .copied()
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"rowCommandPairs\":");
    push_success_data_test_fdm_index_row_command_pairs_json(output, &gate.row_command_pairs);
    output.push_str(
        ",\"renderPaintOrderBasisCandidate\":\"fdm-index-row-command-pairs\",\"renderPaintOrderBasisDecoded\":false",
    );
    output.push('}');
}

fn success_data_test_fdm_index_row_order_promotion_blocked_reasons(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
    gate: &SuccessDataTestFdmIndexRowOrderPromotionGate,
) -> Vec<&'static str> {
    let mut reasons = Vec::new();
    if !gate.all_commands_referenced_by_index_rows_candidate() {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-reference-coverage-incomplete",
        );
    }
    if !gate.one_to_one_row_command_reference_candidate() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-reference-not-one-to-one");
    }
    if gate.single_row_backs_multiple_commands_candidate() {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-single-row-backs-multiple-commands",
        );
    }
    if !gate.row_order_matches_command_order_candidate() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-non-monotonic");
    }
    if gate.reference_count > 0 && gate.valid_vector_offset_reference_count == 0 {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-valid-vector-offset-missing",
        );
    }
    if gate.command_relative_offset_field_reference_count > 0
        && gate.source_segment_relative_offset_field_reference_count > 0
    {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-offset-namespace-mixed");
    }

    let role_groups =
        success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);
    let mut role_paint_order_continuity_blocked = false;
    let mut role_paint_order_authority_pending = false;
    for group in role_groups.values() {
        let profile =
            success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
        role_paint_order_continuity_blocked |= profile.continuity_blocked();
        role_paint_order_authority_pending |= profile.paint_order_authority_pending();
    }
    if role_paint_order_continuity_blocked {
        push_unique_static_str(&mut reasons, "role-paint-order-continuity-unproven");
    }
    if role_paint_order_authority_pending {
        push_unique_static_str(&mut reasons, "role-paint-order-authority-unproven");
    }
    if reasons.is_empty() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-paint-authority-unproven");
    }
    reasons
}

#[derive(Debug, Default)]
struct SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup {
    role_candidate: &'static str,
    reference_count: usize,
    valid_vector_offset_reference_count: usize,
    valid_command_relative_offset_field_reference_count: usize,
    valid_source_segment_relative_offset_field_reference_count: usize,
    command_relative_offset_field_reference_count: usize,
    source_segment_relative_offset_field_reference_count: usize,
    command_relative_offsets: BTreeSet<usize>,
    row_indexes: BTreeSet<usize>,
    row_command_pairs: BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
}

#[derive(Debug)]
struct SuccessDataTestFdmRolePaintOrderContinuityProfile {
    span_min: Option<usize>,
    span_max: Option<usize>,
    role_command_count: usize,
    command_count_in_span: usize,
    interleaved_non_role_command_count: usize,
    max_command_offset_gap: usize,
    continuity_score: f32,
}

impl SuccessDataTestFdmRolePaintOrderContinuityProfile {
    fn span_contiguous_candidate(&self) -> bool {
        self.role_command_count > 0
            && self.command_count_in_span == self.role_command_count
            && self.interleaved_non_role_command_count == 0
    }

    fn continuity_blocked(&self) -> bool {
        !self.span_contiguous_candidate()
    }

    fn paint_order_authority_pending(&self) -> bool {
        self.span_contiguous_candidate()
    }

    fn render_promotion_blocked_reason(&self) -> &'static str {
        if self.continuity_blocked() {
            "role-span-interleaved-non-role-commands"
        } else {
            "role-paint-order-authority-unproven"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SuccessDataTestFdmIndexRowCommandPair {
    row_index: usize,
    command_relative_offset: usize,
    match_kind: &'static str,
}

fn success_data_test_fdm_index_row_reference_role_candidate_groups(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> BTreeMap<&'static str, SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup> {
    let mut groups =
        BTreeMap::<&'static str, SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup>::new();
    for classification in classifications {
        if classification.index_row_references.is_empty() {
            continue;
        }
        for role_candidate in &classification.role_candidates {
            let group = groups.entry(*role_candidate).or_insert_with(|| {
                SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup {
                    role_candidate,
                    ..SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup::default()
                }
            });
            group
                .command_relative_offsets
                .insert(classification.command.relative_offset());
            for reference in &classification.index_row_references {
                group.reference_count += 1;
                group.row_indexes.insert(reference.row_index);
                group
                    .row_command_pairs
                    .insert(SuccessDataTestFdmIndexRowCommandPair {
                        row_index: reference.row_index,
                        command_relative_offset: classification.command.relative_offset(),
                        match_kind: reference.match_kind,
                    });
                if reference.valid_vector_offset {
                    group.valid_vector_offset_reference_count += 1;
                    match reference.match_kind {
                        "command-relative-offset-field" => {
                            group.valid_command_relative_offset_field_reference_count += 1;
                        }
                        "source-segment-relative-offset-field" => {
                            group.valid_source_segment_relative_offset_field_reference_count += 1;
                        }
                        _ => {}
                    }
                }
                match reference.match_kind {
                    "command-relative-offset-field" => {
                        group.command_relative_offset_field_reference_count += 1;
                    }
                    "source-segment-relative-offset-field" => {
                        group.source_segment_relative_offset_field_reference_count += 1;
                    }
                    _ => {}
                }
            }
        }
    }
    groups
}

fn success_data_test_fdm_role_group_single_row_backs_multiple_commands(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) -> bool {
    let mut row_to_command_count = BTreeMap::<usize, usize>::new();
    for pair in &group.row_command_pairs {
        *row_to_command_count.entry(pair.row_index).or_default() += 1;
    }
    row_to_command_count.values().any(|count| *count > 1)
}

fn push_success_data_test_fdm_index_row_reference_role_candidate_groups_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let groups = success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);

    output.push('[');
    for (index, group) in groups.values().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"roleCandidate\":");
        push_json_string(output, group.role_candidate);
        output.push_str(",\"ownershipProven\":false");
        output.push_str(
            ",\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\"",
        );
        output.push_str(",\"referenceCount\":");
        output.push_str(&group.reference_count.to_string());
        output.push_str(",\"validVectorOffsetReferenceCount\":");
        output.push_str(&group.valid_vector_offset_reference_count.to_string());
        output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
        output.push_str(
            &group
                .command_relative_offset_field_reference_count
                .to_string(),
        );
        output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
        output.push_str(
            &group
                .source_segment_relative_offset_field_reference_count
                .to_string(),
        );
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(
            output,
            &group
                .command_relative_offsets
                .iter()
                .copied()
                .collect::<Vec<_>>(),
        );
        output.push_str(",\"rowIndexes\":");
        push_usize_array_json(
            output,
            &group.row_indexes.iter().copied().collect::<Vec<_>>(),
        );
        output.push_str(",\"uniqueCommandRelativeOffsetCount\":");
        output.push_str(&group.command_relative_offsets.len().to_string());
        output.push_str(",\"uniqueRowIndexCount\":");
        output.push_str(&group.row_indexes.len().to_string());
        output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
        output.push_str(
            if group.reference_count == group.command_relative_offsets.len()
                && group.reference_count == group.row_indexes.len()
            {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
        output.push_str(
            if group.row_indexes.len() == 1 && group.command_relative_offsets.len() > 1 {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"rowOrderMatchesCommandOrderCandidate\":");
        output.push_str(
            if success_data_test_fdm_row_command_pairs_are_monotonic(&group.row_command_pairs) {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"rowCommandPairs\":");
        push_success_data_test_fdm_index_row_command_pairs_json(output, &group.row_command_pairs);
        output.push_str(",\"roleVectorOffsetAuthorityGate\":");
        push_success_data_test_fdm_role_vector_offset_authority_gate_json(output, group);
        output.push_str(",\"roleFanoutSegmentOwnerGate\":");
        push_success_data_test_fdm_role_fanout_segment_owner_gate_json(output, group);
        output.push_str(",\"decoded\":false,\"paintOrderContinuityProfile\":");
        push_success_data_test_fdm_role_paint_order_continuity_profile_json(
            output,
            group,
            classifications,
        );
        output.push('}');
    }
    output.push(']');
}

fn success_data_test_fdm_role_vector_offset_authority_blocked_reason(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) -> &'static str {
    let mixed_valid_offset_namespaces = group.valid_command_relative_offset_field_reference_count
        > 0
        && group.valid_source_segment_relative_offset_field_reference_count > 0;
    if group.valid_vector_offset_reference_count == 0 {
        "fdm-index-role-vector-offset-authority-valid-vector-offset-missing"
    } else if mixed_valid_offset_namespaces {
        "fdm-index-role-vector-offset-authority-mixed-valid-offset-namespaces"
    } else {
        "fdm-index-role-vector-offset-authority-semantics-unproven"
    }
}

fn push_success_data_test_fdm_role_vector_offset_authority_gate_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) {
    let invalid_vector_offset_reference_count = group
        .reference_count
        .saturating_sub(group.valid_vector_offset_reference_count);
    let invalid_command_relative_offset_field_reference_count = group
        .command_relative_offset_field_reference_count
        .saturating_sub(group.valid_command_relative_offset_field_reference_count);
    let invalid_source_segment_relative_offset_field_reference_count = group
        .source_segment_relative_offset_field_reference_count
        .saturating_sub(group.valid_source_segment_relative_offset_field_reference_count);
    let mixed_offset_namespaces_among_valid_refs =
        group.valid_command_relative_offset_field_reference_count > 0
            && group.valid_source_segment_relative_offset_field_reference_count > 0;
    let all_valid_references_use_command_relative_offset_field =
        group.valid_vector_offset_reference_count > 0
            && group.valid_command_relative_offset_field_reference_count
                == group.valid_vector_offset_reference_count;
    let all_valid_references_use_source_segment_relative_offset_field =
        group.valid_vector_offset_reference_count > 0
            && group.valid_source_segment_relative_offset_field_reference_count
                == group.valid_vector_offset_reference_count;
    let all_references_have_invalid_vector_offset =
        group.reference_count > 0 && group.valid_vector_offset_reference_count == 0;
    let render_promotion_blocked_reason =
        success_data_test_fdm_role_vector_offset_authority_blocked_reason(group);

    output.push_str("{\"basis\":\"fdm-index-role-vector-offset-authority-gate\",\"source\":\"FDMIndex.vectorOffset+FDMIndex role offset fields\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"roleCandidate\":");
    push_json_string(output, group.role_candidate);
    output.push_str(",\"roleVectorOffsetAuthorityDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"fdm-index-role-vector-offset-authority-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, render_promotion_blocked_reason);
    output.push_str(",\"referenceCount\":");
    output.push_str(&group.reference_count.to_string());
    output.push_str(",\"validVectorOffsetReferenceCount\":");
    output.push_str(&group.valid_vector_offset_reference_count.to_string());
    output.push_str(",\"invalidVectorOffsetReferenceCount\":");
    output.push_str(&invalid_vector_offset_reference_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"validCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .valid_command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"validSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .valid_source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"invalidCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(&invalid_command_relative_offset_field_reference_count.to_string());
    output.push_str(",\"invalidSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(&invalid_source_segment_relative_offset_field_reference_count.to_string());
    output.push_str(",\"allValidReferencesUseCommandRelativeOffsetField\":");
    output.push_str(if all_valid_references_use_command_relative_offset_field {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allValidReferencesUseSourceSegmentRelativeOffsetField\":");
    output.push_str(
        if all_valid_references_use_source_segment_relative_offset_field {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"mixedOffsetNamespacesAmongValidReferences\":");
    output.push_str(if mixed_offset_namespaces_among_valid_refs {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allReferencesHaveInvalidVectorOffset\":");
    output.push_str(if all_references_have_invalid_vector_offset {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

fn push_success_data_test_fdm_role_fanout_segment_owner_gate_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) {
    let mut row_to_pairs = BTreeMap::<usize, Vec<SuccessDataTestFdmIndexRowCommandPair>>::new();
    for pair in &group.row_command_pairs {
        row_to_pairs.entry(pair.row_index).or_default().push(*pair);
    }

    let mut fanout_row_count = 0usize;
    let mut fanout_reference_count = 0usize;
    let mut fanout_command_relative_offset_field_reference_count = 0usize;
    let mut fanout_source_segment_relative_offset_field_reference_count = 0usize;
    let mut max_row_fanout = 0usize;
    for pairs in row_to_pairs.values() {
        max_row_fanout = max_row_fanout.max(pairs.len());
        if pairs.len() <= 1 {
            continue;
        }
        fanout_row_count += 1;
        fanout_reference_count += pairs.len();
        for pair in pairs {
            match pair.match_kind {
                "command-relative-offset-field" => {
                    fanout_command_relative_offset_field_reference_count += 1;
                }
                "source-segment-relative-offset-field" => {
                    fanout_source_segment_relative_offset_field_reference_count += 1;
                }
                _ => {}
            }
        }
    }

    let one_to_one_row_command_reference_candidate = group.reference_count
        == group.command_relative_offsets.len()
        && group.reference_count == group.row_indexes.len();
    let single_row_backs_multiple_commands_candidate =
        row_to_pairs.values().any(|pairs| pairs.len() > 1);
    let mixed_offset_field_namespaces = group.command_relative_offset_field_reference_count > 0
        && group.source_segment_relative_offset_field_reference_count > 0;
    let fanout_rows_use_command_relative_offset_fields = fanout_reference_count > 0
        && fanout_command_relative_offset_field_reference_count == fanout_reference_count;
    let fanout_rows_use_source_segment_offset_fields = fanout_reference_count > 0
        && fanout_source_segment_relative_offset_field_reference_count == fanout_reference_count;
    let render_promotion_blocked_reason = if single_row_backs_multiple_commands_candidate {
        "fdm-index-role-row-fanout-multi-command-single-row"
    } else if !one_to_one_row_command_reference_candidate {
        "fdm-index-role-row-reference-not-one-to-one"
    } else if mixed_offset_field_namespaces {
        "fdm-index-role-offset-namespace-mixed"
    } else if group.valid_vector_offset_reference_count == 0 {
        "fdm-index-role-valid-vector-offset-missing"
    } else {
        "fdm-index-role-segment-owner-semantics-unproven"
    };

    output.push_str("{\"basis\":\"fdm-index-role-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex role row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"roleCandidate\":");
    push_json_string(output, group.role_candidate);
    output.push_str(",\"roleOwnershipDecoded\":false,\"segmentOwnerDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"fdm-index-role-row-fanout-segment-owner-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, render_promotion_blocked_reason);
    output.push_str(",\"referenceCount\":");
    output.push_str(&group.reference_count.to_string());
    output.push_str(",\"uniqueCommandRelativeOffsetCount\":");
    output.push_str(&group.command_relative_offsets.len().to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&group.row_indexes.len().to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"fanoutRowCount\":");
    output.push_str(&fanout_row_count.to_string());
    output.push_str(",\"fanoutReferenceCount\":");
    output.push_str(&fanout_reference_count.to_string());
    output.push_str(",\"fanoutCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(&fanout_command_relative_offset_field_reference_count.to_string());
    output.push_str(",\"fanoutSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(&fanout_source_segment_relative_offset_field_reference_count.to_string());
    output.push_str(",\"maxRowFanout\":");
    output.push_str(&max_row_fanout.to_string());
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if one_to_one_row_command_reference_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if single_row_backs_multiple_commands_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedOffsetFieldNamespaces\":");
    output.push_str(if mixed_offset_field_namespaces {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseCommandRelativeOffsetFields\":");
    output.push_str(if fanout_rows_use_command_relative_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseSourceSegmentOffsetFields\":");
    output.push_str(if fanout_rows_use_source_segment_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowsWithMultipleCommandRefs\":");
    push_success_data_test_fdm_role_fanout_rows_json(output, &row_to_pairs);
    output.push('}');
}

fn push_success_data_test_fdm_role_fanout_rows_json(
    output: &mut String,
    row_to_pairs: &BTreeMap<usize, Vec<SuccessDataTestFdmIndexRowCommandPair>>,
) {
    output.push('[');
    let mut emitted = 0usize;
    for (row_index, pairs) in row_to_pairs {
        if pairs.len() <= 1 {
            continue;
        }
        if emitted > 0 {
            output.push(',');
        }
        emitted += 1;
        let command_relative_offsets = pairs
            .iter()
            .map(|pair| pair.command_relative_offset)
            .collect::<Vec<_>>();
        let match_kinds = pairs
            .iter()
            .map(|pair| pair.match_kind)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        output.push_str("{\"rowIndex\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"commandReferenceCount\":");
        output.push_str(&pairs.len().to_string());
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(output, &command_relative_offsets);
        output.push_str(",\"matchKinds\":");
        push_json_string_slice_array(output, &match_kinds);
        output.push('}');
    }
    output.push(']');
}

fn push_success_data_test_fdm_role_paint_order_continuity_profile_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    output.push_str("{\"basis\":\"fdm-index-row-reference-role-command-span\",\"decoded\":false,\"sourceBacked\":true,\"paintOrderDecoded\":false");
    let profile = success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
    output.push_str(",\"commandRelativeOffsetSpanMin\":");
    push_option_usize_json(output, profile.span_min);
    output.push_str(",\"commandRelativeOffsetSpanMax\":");
    push_option_usize_json(output, profile.span_max);
    output.push_str(",\"roleCommandCount\":");
    output.push_str(&profile.role_command_count.to_string());
    output.push_str(",\"commandCountInSpan\":");
    output.push_str(&profile.command_count_in_span.to_string());
    output.push_str(",\"interleavedNonRoleCommandCount\":");
    output.push_str(&profile.interleaved_non_role_command_count.to_string());
    output.push_str(",\"hasInterleavedNonRoleCommands\":");
    output.push_str(if profile.interleaved_non_role_command_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"maxCommandOffsetGap\":");
    output.push_str(&profile.max_command_offset_gap.to_string());
    output.push_str(",\"commandOffsetContinuityScore\":");
    output.push_str(&format!("{:.3}", profile.continuity_score));
    output.push_str(",\"spanContiguousCandidate\":");
    output.push_str(if profile.span_contiguous_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paintOrderAuthorityPending\":");
    output.push_str(if profile.paint_order_authority_pending() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"continuityBlocked\":");
    output.push_str(if profile.continuity_blocked() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, profile.render_promotion_blocked_reason());
    output.push('}');
}

fn success_data_test_fdm_role_paint_order_continuity_profile(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmRolePaintOrderContinuityProfile {
    let span_min = group.command_relative_offsets.iter().next().copied();
    let span_max = group.command_relative_offsets.iter().next_back().copied();
    let role_command_count = group.command_relative_offsets.len();
    let command_count_in_span = match (span_min, span_max) {
        (Some(min), Some(max)) => classifications
            .iter()
            .filter(|classification| {
                let offset = classification.command.relative_offset();
                offset >= min && offset <= max
            })
            .count(),
        _ => 0,
    };
    let interleaved_non_role_command_count =
        command_count_in_span.saturating_sub(role_command_count);
    let mut max_command_offset_gap = 0usize;
    let mut previous_offset = None;
    for offset in group.command_relative_offsets.iter().copied() {
        if let Some(previous) = previous_offset {
            max_command_offset_gap = max_command_offset_gap.max(offset.saturating_sub(previous));
        }
        previous_offset = Some(offset);
    }
    let continuity_score = if command_count_in_span == 0 {
        0.0
    } else {
        role_command_count as f32 / command_count_in_span as f32
    };

    SuccessDataTestFdmRolePaintOrderContinuityProfile {
        span_min,
        span_max,
        role_command_count,
        command_count_in_span,
        interleaved_non_role_command_count,
        max_command_offset_gap,
        continuity_score,
    }
}

fn success_data_test_fdm_row_command_pairs_are_monotonic(
    pairs: &BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
) -> bool {
    let mut previous_command_relative_offset = None;
    for pair in pairs {
        if previous_command_relative_offset
            .is_some_and(|previous| pair.command_relative_offset < previous)
        {
            return false;
        }
        previous_command_relative_offset = Some(pair.command_relative_offset);
    }
    true
}

fn push_success_data_test_fdm_index_row_command_pairs_json(
    output: &mut String,
    pairs: &BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
) {
    output.push('[');
    for (index, pair) in pairs.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&pair.row_index.to_string());
        output.push_str(",\"commandRelativeOffset\":");
        output.push_str(&pair.command_relative_offset.to_string());
        output.push_str(",\"matchKind\":");
        push_json_string(output, pair.match_kind);
        output.push('}');
    }
    output.push(']');
}

fn success_data_test_fdm_primitive_ownership_classification<'a>(
    projection: SuccessDataTestFdmProjection,
    command: &'a ObjectFdmVectorCommandCandidate,
    index_entries: &[ObjectFdmIndexEntryCandidate],
    anchor: Option<(ObjectFdmVectorPoint, i32)>,
) -> SuccessDataTestFdmPrimitiveOwnershipClassification<'a> {
    let mut role_candidates = Vec::new();
    let mut classification_basis = Vec::new();
    if let Some(ellipse) = command.ellipse() {
        if success_data_test_fdm_reference_ellipse_has_center_marker(projection, command, ellipse) {
            role_candidates.push("main-circle-anchor");
            classification_basis.push("large-01000460-ellipse-anchor");
        } else if success_data_test_fdm_reference_ellipse_is_control_marker(
            projection, command, ellipse,
        ) {
            role_candidates.push("arc-candidate");
            role_candidates.push("control-ellipse-marker");
            classification_basis.push("tiny-ff000460-ellipse-control-marker");
        } else {
            role_candidates.push("arc-candidate");
            classification_basis.push("ellipse-boundary-primitive");
        }
    } else {
        let is_two_point_line = fdm_vector_marker_is_line(command.marker())
            && command.curve_segments().is_empty()
            && command.path_points().len() == 2;
        if is_two_point_line {
            role_candidates.push("line-candidate");
            classification_basis.push("fdm-line-marker-two-point-path");
            if let Some((center, radius)) = anchor {
                let boundary_count =
                    success_data_test_fdm_anchor_boundary_point_count(command, center, radius);
                let center_count =
                    success_data_test_fdm_anchor_center_point_count(command, center, radius);
                if boundary_count >= 2 {
                    role_candidates.push("chord-candidate");
                    classification_basis.push("both-endpoints-near-anchor-boundary");
                } else if boundary_count >= 1 && center_count >= 1 {
                    role_candidates.push("radial-line-candidate");
                    classification_basis.push("one-endpoint-near-anchor-center-one-near-boundary");
                }
            }
        }
        if !command.curve_segments().is_empty()
            || fdm_vector_marker_is_bezier_curve(command.marker())
        {
            role_candidates.push("arc-candidate");
            classification_basis.push("fdm-bezier-marker-or-control-points");
        }
        if command.path_points().len() >= 3 && !fdm_vector_path_is_closed(command.path_points()) {
            role_candidates.push("surface-boundary-candidate");
            classification_basis.push("open-polyline-with-three-or-more-points");
        }
        if success_data_test_fdm_connector_candidate(command) {
            role_candidates.push("connector-candidate");
            classification_basis.push("long-open-source-path");
        }
    }
    if role_candidates.is_empty() {
        role_candidates.push("unclassified-primitive");
        classification_basis.push("no-current-role-rule");
    }
    SuccessDataTestFdmPrimitiveOwnershipClassification {
        command,
        role_candidates,
        classification_basis,
        index_row_references: success_data_test_fdm_index_row_references(command, index_entries),
    }
}

fn success_data_test_fdm_index_row_references(
    command: &ObjectFdmVectorCommandCandidate,
    index_entries: &[ObjectFdmIndexEntryCandidate],
) -> Vec<SuccessDataTestFdmIndexRowReference> {
    let mut references = Vec::new();
    for entry in index_entries {
        let bbox = entry.bbox();
        let offset_value = bbox.left();
        if offset_value < 0 {
            continue;
        }
        let offset_value = offset_value as usize;
        let match_kind = if offset_value == command.relative_offset() {
            Some("command-relative-offset-field")
        } else if command
            .source_segment()
            .is_some_and(|segment| segment.relative_offset() == offset_value)
        {
            Some("source-segment-relative-offset-field")
        } else {
            None
        };
        let Some(match_kind) = match_kind else {
            continue;
        };
        references.push(SuccessDataTestFdmIndexRowReference {
            row_index: entry.row_index(),
            index_offset: entry.index_offset(),
            vector_offset: entry.vector_offset(),
            valid_vector_offset: entry.valid_vector_offset(),
            offset_field: "bbox.left",
            offset_value,
            match_kind,
        });
    }
    references
}

fn push_success_data_test_fdm_index_row_references_json(
    output: &mut String,
    references: &[SuccessDataTestFdmIndexRowReference],
) {
    output.push('[');
    for (index, reference) in references.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&reference.row_index.to_string());
        output.push_str(",\"indexOffset\":");
        output.push_str(&reference.index_offset.to_string());
        output.push_str(",\"vectorOffset\":");
        output.push_str(&reference.vector_offset.to_string());
        output.push_str(",\"validVectorOffset\":");
        output.push_str(if reference.valid_vector_offset {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"offsetField\":");
        push_json_string(output, reference.offset_field);
        output.push_str(",\"offsetValue\":");
        output.push_str(&reference.offset_value.to_string());
        output.push_str(",\"matchKind\":");
        push_json_string(output, reference.match_kind);
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

fn success_data_test_fdm_anchor_boundary_point_count(
    command: &ObjectFdmVectorCommandCandidate,
    center: ObjectFdmVectorPoint,
    radius: i32,
) -> usize {
    let tolerance = (radius / 12).max(24) as f32;
    command
        .path_points()
        .iter()
        .filter(|point| (fdm_point_distance(center, **point) - radius as f32).abs() <= tolerance)
        .count()
}

fn success_data_test_fdm_anchor_center_point_count(
    command: &ObjectFdmVectorCommandCandidate,
    center: ObjectFdmVectorPoint,
    radius: i32,
) -> usize {
    let tolerance = (radius / 8).max(24) as f32;
    command
        .path_points()
        .iter()
        .filter(|point| fdm_point_distance(center, **point) <= tolerance)
        .count()
}

fn success_data_test_fdm_connector_candidate(command: &ObjectFdmVectorCommandCandidate) -> bool {
    if command.ellipse().is_some() || fdm_vector_path_is_closed(command.path_points()) {
        return false;
    }
    let Some(bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let source_width = bbox.2.saturating_sub(bbox.0);
    let source_height = bbox.3.saturating_sub(bbox.1);
    source_width.max(source_height) >= 500
}

fn fdm_point_distance(left: ObjectFdmVectorPoint, right: ObjectFdmVectorPoint) -> f32 {
    let dx = (left.x() - right.x()) as f32;
    let dy = (left.y() - right.y()) as f32;
    (dx * dx + dy * dy).sqrt()
}

fn push_json_string_slice_array(output: &mut String, values: &[&str]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(output, value);
    }
    output.push(']');
}

fn fdm_vector_command_source_bbox(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<ObjectFdmIndexBbox> {
    if !command.path_points().is_empty() {
        let mut points =
            Vec::with_capacity(command.path_points().len() + command.curve_segments().len() * 2);
        points.extend_from_slice(command.path_points());
        for segment in command.curve_segments() {
            points.push(segment.control_1());
            points.push(segment.control_2());
        }
        return fdm_vector_path_points_bbox(&points);
    }
    command.ellipse().map(fdm_vector_ellipse_bbox)
}

fn fdm_vector_ellipse_bbox(ellipse: ObjectFdmVectorEllipse) -> ObjectFdmIndexBbox {
    let center = ellipse.center();
    ObjectFdmIndexBbox::new(
        center.x().saturating_sub(ellipse.radius_x()),
        center.y().saturating_sub(ellipse.radius_y()),
        center.x().saturating_add(ellipse.radius_x()),
        center.y().saturating_add(ellipse.radius_y()),
    )
}

fn normalize_fdm_bbox(bbox: ObjectFdmIndexBbox) -> (i32, i32, i32, i32) {
    (
        bbox.left().min(bbox.right()),
        bbox.top().min(bbox.bottom()),
        bbox.left().max(bbox.right()),
        bbox.top().max(bbox.bottom()),
    )
}

fn fdm_bbox_center(bbox: (i32, i32, i32, i32)) -> (i32, i32) {
    let center_x = i64::from(bbox.0) + (i64::from(bbox.2) - i64::from(bbox.0)) / 2;
    let center_y = i64::from(bbox.1) + (i64::from(bbox.3) - i64::from(bbox.1)) / 2;
    (center_x as i32, center_y as i32)
}

fn push_fdm_vector_points_json(output: &mut String, points: &[ObjectFdmVectorPoint]) {
    output.push('[');
    for (index, point) in points.iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_vector_point_json(output, point);
    }
    output.push(']');
}

fn push_fdm_vector_point_json(output: &mut String, point: ObjectFdmVectorPoint) {
    output.push_str("{\"x\":");
    output.push_str(&point.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&point.y().to_string());
    output.push('}');
}

fn push_fdm_vector_curve_segments_json(
    output: &mut String,
    segments: &[ObjectFdmVectorCurveSegment],
) {
    output.push('[');
    for (index, segment) in segments.iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"control1\":");
        push_fdm_vector_point_json(output, segment.control_1());
        output.push_str(",\"control2\":");
        push_fdm_vector_point_json(output, segment.control_2());
        output.push('}');
    }
    output.push(']');
}

fn push_fdm_vector_ellipse_json(output: &mut String, ellipse: ObjectFdmVectorEllipse) {
    output.push_str("{\"center\":");
    push_fdm_vector_point_json(output, ellipse.center());
    output.push_str(",\"radiusX\":");
    output.push_str(&ellipse.radius_x().to_string());
    output.push_str(",\"radiusY\":");
    output.push_str(&ellipse.radius_y().to_string());
    output.push_str(",\"color\":");
    push_fdm_vector_optional_color_json(output, ellipse.color());
    output.push('}');
}

fn push_fdm_vector_optional_color_json(output: &mut String, color: Option<u32>) {
    match color.and_then(fdm_vector_css_color) {
        Some(color) => push_json_string(output, &color),
        None => output.push_str("null"),
    }
}

fn push_object_fdm_vector_segment_candidate_json(
    output: &mut String,
    segment: &ObjectFdmVectorSegmentCandidate,
) {
    output.push_str("{\"relativeOffset\":");
    output.push_str(&segment.relative_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&segment.declared_len().to_string());
    output.push_str(",\"commandCount\":");
    output.push_str(&segment.command_count().to_string());
    output.push_str(",\"commandOffsets\":");
    push_u16_array_json(output, segment.command_offsets());
    output.push_str(",\"bbox\":");
    if let Some(bbox) = segment.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceSpanCandidate\":{\"width\":");
    output.push_str(&segment.source_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&segment.source_height().to_string());
    output.push_str("},\"decoded\":false}");
}

fn push_object_fdm_text_candidate_json(output: &mut String, candidate: &ObjectFdmTextCandidate) {
    output.push_str("{\"text\":");
    push_json_string(output, candidate.text());
    output.push_str(",\"textOffset\":");
    output.push_str(&candidate.text_offset().to_string());
    output.push_str(",\"markerOffset\":");
    output.push_str(&candidate.marker_offset().to_string());
    output.push_str(",\"rawTextHex\":");
    push_json_string(output, &hex(candidate.raw_text()));
    output.push_str(",\"bbox\":");
    if let Some(bbox) = candidate.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

fn push_object_fdm_text_index_entry_candidate_json(
    output: &mut String,
    candidate: &ObjectFdmTextIndexEntryCandidate,
) {
    output.push_str("{\"indexPath\":");
    push_json_string(output, candidate.index_path());
    output.push_str(",\"textPath\":");
    push_json_string(output, candidate.text_path());
    output.push_str(",\"rowIndex\":");
    output.push_str(&candidate.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&candidate.index_offset().to_string());
    output.push_str(",\"textRecordOffset\":");
    output.push_str(&candidate.text_record_offset().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&candidate.kind().to_string());
    output.push_str(",\"kindHex\":");
    push_json_string(output, &format!("0x{:04x}", candidate.kind()));
    output.push_str(",\"validTextRecordOffset\":");
    output.push_str(if candidate.valid_text_record_offset() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bbox\":");
    push_object_fdm_index_bbox_json(output, candidate.bbox());
    output.push_str(",\"textRecordBbox\":");
    if let Some(bbox) = candidate.text_record_bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"textRecordPrefixHex\":");
    push_json_string(output, &hex(candidate.text_record_prefix()));
    output.push_str(",\"decoded\":false}");
}

fn fdm_vector_path_points_bbox(points: &[ObjectFdmVectorPoint]) -> Option<ObjectFdmIndexBbox> {
    let first = *points.first()?;
    let mut left = first.x();
    let mut top = first.y();
    let mut right = first.x();
    let mut bottom = first.y();

    for point in points.iter().copied().skip(1) {
        left = left.min(point.x());
        top = top.min(point.y());
        right = right.max(point.x());
        bottom = bottom.max(point.y());
    }

    Some(ObjectFdmIndexBbox::new(left, top, right, bottom))
}

fn fdm_vector_path_is_closed(points: &[ObjectFdmVectorPoint]) -> bool {
    points.len() >= 2 && points.first() == points.last()
}

fn fdm_vector_primitive_kind(command: &ObjectFdmVectorCommandCandidate) -> &'static str {
    if command.ellipse().is_some() {
        "ellipse"
    } else if !command.curve_segments().is_empty() {
        "cubicBezier"
    } else if fdm_vector_marker_is_bezier_curve(command.marker()) {
        "quadraticBezier"
    } else {
        "polyline"
    }
}

fn fdm_vector_marker_is_bezier_curve(marker: &[u8; 4]) -> bool {
    marker == b"\xff\x00\x09\x60" || marker == b"\x00\x00\x09\x60" || marker == b"\x01\x00\x09\x60"
}

fn fdm_vector_marker_is_line(marker: &[u8; 4]) -> bool {
    marker == b"\xff\x00\x01\x60" || marker == b"\x00\x00\x01\x60" || marker == b"\x01\x00\x01\x60"
}

fn fdm_vector_css_color(color: u32) -> Option<String> {
    if color > 0x00ff_ffff {
        return None;
    }
    let blue = (color >> 16) & 0xff;
    let green = (color >> 8) & 0xff;
    let red = color & 0xff;
    Some(format!("#{red:02x}{green:02x}{blue:02x}"))
}

fn push_object_fdm_index_bbox_json(output: &mut String, bbox: ObjectFdmIndexBbox) {
    output.push_str("{\"left\":");
    output.push_str(&bbox.left().to_string());
    output.push_str(",\"top\":");
    output.push_str(&bbox.top().to_string());
    output.push_str(",\"right\":");
    output.push_str(&bbox.right().to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&bbox.bottom().to_string());
    output.push('}');
}

fn push_object_image_payload_span_json(output: &mut String, span: &ObjectImagePayloadSpan) {
    output.push_str("{\"kind\":");
    push_json_string(output, span.kind());
    output.push_str(",\"mime\":");
    push_json_string(output, span.mime());
    output.push_str(",\"signatureOffset\":");
    output.push_str(&span.signature_offset().to_string());
    output.push_str(",\"start\":");
    output.push_str(&span.start().to_string());
    output.push_str(",\"end\":");
    output.push_str(&span.end().to_string());
    output.push_str(",\"length\":");
    output.push_str(&span.len().to_string());
    output.push_str(",\"complete\":");
    output.push_str(if span.complete() { "true" } else { "false" });
    output.push_str(",\"dimensions\":");
    push_object_image_dimensions_json(output, span.dimensions());
    output.push_str(",\"objectEnvelope\":");
    push_object_image_payload_envelope_json(output, span.envelope());
    output.push_str(",\"payloadPrefixHex\":");
    push_json_string(
        output,
        &hex(&span.payload()[..span.payload().len().min(16)]),
    );
    output.push_str(",\"decoded\":false}");
}

fn push_object_image_dimensions_json(
    output: &mut String,
    dimensions: Option<ObjectImageDimensions>,
) {
    if let Some(dimensions) = dimensions {
        output.push_str("{\"width\":");
        output.push_str(&dimensions.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&dimensions.height().to_string());
        output.push('}');
    } else {
        output.push_str("null");
    }
}

fn push_object_image_payload_envelope_json(
    output: &mut String,
    envelope: &ObjectImagePayloadEnvelope,
) {
    output.push_str("{\"headerStart\":");
    output.push_str(&envelope.header_start().to_string());
    output.push_str(",\"headerEnd\":");
    output.push_str(&envelope.header_end().to_string());
    output.push_str(",\"headerLength\":");
    output.push_str(&envelope.header_len().to_string());
    output.push_str(",\"headerPrefixHex\":");
    push_json_string(
        output,
        &hex(&envelope.header()[..envelope.header().len().min(16)]),
    );
    output.push_str(",\"headerFields\":");
    push_object_image_header_fields_json(output, envelope.header_fields());
    output.push_str(",\"trailerStart\":");
    output.push_str(&envelope.trailer_start().to_string());
    output.push_str(",\"trailerEnd\":");
    output.push_str(&envelope.trailer_end().to_string());
    output.push_str(",\"trailerLength\":");
    output.push_str(&envelope.trailer_len().to_string());
    output.push_str(",\"trailerPrefixHex\":");
    push_json_string(
        output,
        &hex(&envelope.trailer()[..envelope.trailer().len().min(16)]),
    );
    output.push_str(",\"declaredPayloadLength\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&length.value().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"declaredPayloadLengthOffset\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&length.offset().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"declaredPayloadLengthEndian\":");
    if let Some(length) = envelope.declared_payload_length() {
        push_json_string(output, length.endian());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

fn push_object_image_header_fields_json(
    output: &mut String,
    fields: &ObjectImageHeaderFieldCandidates,
) {
    output.push_str("{\"u16LePrefix\":[");
    for (index, field) in fields.u16_le_prefix().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_numeric_header_field_json(output, field);
    }
    output.push_str("],\"u32LePrefix\":[");
    for (index, field) in fields.u32_le_prefix().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_numeric_header_field_json(output, field);
    }
    output.push_str("],\"sourcePathCandidate\":");
    if let Some(path) = fields.source_path_candidate() {
        push_object_image_source_path_candidate_json(output, path);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

fn push_object_image_numeric_header_field_json(
    output: &mut String,
    field: &ObjectImageNumericHeaderField,
) {
    output.push_str("{\"offset\":");
    output.push_str(&field.offset().to_string());
    output.push_str(",\"value\":");
    output.push_str(&field.value().to_string());
    output.push('}');
}

fn push_object_image_source_path_candidate_json(
    output: &mut String,
    path: &ObjectImageSourcePathCandidate,
) {
    output.push_str("{\"lengthOffset\":");
    output.push_str(&path.length_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&path.declared_length().to_string());
    output.push_str(",\"bytesStart\":");
    output.push_str(&path.bytes_start().to_string());
    output.push_str(",\"bytesEnd\":");
    output.push_str(&path.bytes_end().to_string());
    output.push_str(",\"nulTerminated\":");
    output.push_str(if path.nul_terminated() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bytesHex\":");
    push_json_string(output, &hex(path.bytes()));
    output.push_str(",\"textLossy\":");
    push_json_string(output, path.text_lossy());
    output.push_str(",\"decoded\":false}");
}

fn push_usize_array_json(output: &mut String, values: &[usize]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

fn push_text_count_range_json(output: &mut String, range: &TextCountRange) {
    output.push_str("{\"index\":");
    output.push_str(&range.index().to_string());
    output.push_str(",\"family\":");
    push_json_string(output, range.family());
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
    push_text_count_range_overlaps_json(output, range.document_text_overlaps());
    output.push_str(",\"controlRangeOverlaps\":");
    push_text_count_control_range_overlaps_json(output, range.control_range_overlaps());
    output.push_str(",\"decoded\":false,\"rawHex\":");
    push_json_string(output, &hex(range.raw()));
    output.push('}');
}

fn push_text_control_boundary_json(output: &mut String, boundary: &TextControlBoundary) {
    output.push_str("{\"index\":");
    output.push_str(&boundary.index().to_string());
    output.push_str(",\"code\":");
    output.push_str(&boundary.code().to_string());
    output.push_str(",\"codeHex\":");
    push_json_string(output, &format!("0x{:04x}", boundary.code()));
    output.push_str(",\"sourceSpan\":");
    match boundary.source_span() {
        Some(span) => push_text_source_span_json(output, span),
        None => output.push_str("null"),
    }
    output.push_str(",\"decoded\":false}");
}

fn push_text_boundary_candidate_json(output: &mut String, candidate: &TextBoundaryCandidate) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    push_json_string(output, candidate.kind());
    output.push_str(",\"textCountRangeIndex\":");
    output.push_str(&candidate.text_count_range_index().to_string());
    output.push_str(",\"basis\":");
    push_json_string(output, candidate.basis().as_str());
    output.push_str(",\"delimiterCode\":");
    output.push_str(&candidate.delimiter_code().to_string());
    output.push_str(",\"delimiterCodeHex\":");
    push_json_string(output, &format!("0x{:04x}", candidate.delimiter_code()));
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

fn push_text_paragraph_boundary_candidate_json(
    output: &mut String,
    candidate: &TextParagraphBoundaryCandidate,
) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    push_json_string(output, candidate.kind());
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
    push_json_string(output, candidate.rule());
    output.push_str(",\"lineWordEvidence\":");
    push_text_layout_exact_evidence_json(output, candidate.line_word_evidence());
    output.push_str(",\"pageFieldEvidence\":");
    push_text_layout_exact_evidence_json(output, candidate.page_field_evidence());
    output.push_str(",\"decoded\":false}");
}

fn push_table_candidate_json(output: &mut String, candidate: &TableCandidate) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    push_json_string(output, candidate.kind());
    output.push_str(",\"textBoundaryCandidateIndex\":");
    output.push_str(&candidate.text_boundary_candidate_index().to_string());
    output.push_str(",\"textCountRangeIndex\":");
    output.push_str(&candidate.text_count_range_index().to_string());
    output.push_str(",\"basis\":");
    push_json_string(output, candidate.basis().as_str());
    output.push_str(",\"delimiterCode\":");
    output.push_str(&candidate.delimiter_code().to_string());
    output.push_str(",\"delimiterCodeHex\":");
    push_json_string(output, &format!("0x{:04x}", candidate.delimiter_code()));
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
    output.push_str(",\"intervals\":");
    push_table_candidate_intervals_json(
        output,
        candidate.intervals(),
        candidate.is_row_like() || candidate.is_sparse_document_text_control_run_candidate(),
    );
    output.push_str(",\"cellLike\":");
    output.push_str(if candidate.is_cell_like() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowLike\":");
    output.push_str(if candidate.is_row_like() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"observedTable\":");
    if candidate.is_row_like() {
        push_observed_table_json(output, candidate);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sparse\":");
    output.push_str(
        if candidate.is_sparse_document_text_control_run_candidate() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&candidate.cell_count_candidate().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&candidate.empty_cell_count_candidate().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&candidate.non_empty_cell_count_candidate().to_string());
    output.push_str(",\"sparseObservedTable\":");
    if candidate.is_sparse_document_text_control_run_candidate() {
        push_sparse_observed_table_json(output, candidate);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sparseTopologyCandidate\":");
    if let Some(topology) = candidate.sparse_topology_candidate() {
        push_sparse_topology_candidate_json(output, candidate, &topology);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rule\":");
    push_json_string(output, candidate.rule());
    output.push_str(",\"decoded\":false}");
}

fn push_sparse_observed_table_json(output: &mut String, candidate: &TableCandidate) {
    output.push_str("{\"source\":\"sparseDocumentTextControlRows\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"maxColumnCountCandidate\":");
    output.push_str(&candidate.max_column_segment_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&candidate.cell_count_candidate().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&candidate.empty_cell_count_candidate().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&candidate.non_empty_cell_count_candidate().to_string());
    output.push_str(",\"rows\":");
    push_sparse_table_rows_json(output, candidate.intervals());
    output.push_str(",\"topologyCandidate\":");
    if let Some(topology) = candidate.sparse_topology_candidate() {
        push_sparse_topology_candidate_json(output, candidate, &topology);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"geometryDecoded\":false,\"decoded\":false}");
}

fn push_sparse_topology_candidate_json(
    output: &mut String,
    candidate: &TableCandidate,
    topology: &rjtd_model::TableCandidateSparseTopologyCandidate,
) {
    output.push_str("{\"source\":\"sparseDocumentTextControlRows\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&topology.row_count().to_string());
    output.push_str(",\"maxColumnCountCandidate\":");
    output.push_str(&topology.max_column_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&topology.cell_count().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&topology.empty_cell_count().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&topology.non_empty_cell_count().to_string());
    output.push_str(",\"rows\":[");
    for (index, row) in topology.rows().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&row.index().to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&row.source_interval_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&row.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&row.source_end().to_string());
        output.push_str(",\"cellCount\":");
        output.push_str(&row.cell_count().to_string());
        output.push_str(",\"emptyCellCount\":");
        output.push_str(&row.empty_cell_count().to_string());
        output.push_str(",\"nonEmptyCellCount\":");
        output.push_str(&row.non_empty_cell_count().to_string());
        output.push_str(",\"firstNonEmptyColumnIndex\":");
        push_option_usize_json(output, row.first_non_empty_column_index());
        output.push_str(",\"lastNonEmptyColumnIndex\":");
        push_option_usize_json(output, row.last_non_empty_column_index());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"columns\":[");
    for (index, column) in topology.columns().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&column.index().to_string());
        output.push_str(",\"observedCellCount\":");
        output.push_str(&column.observed_cell_count().to_string());
        output.push_str(",\"emptyCellCount\":");
        output.push_str(&column.empty_cell_count().to_string());
        output.push_str(",\"nonEmptyCellCount\":");
        output.push_str(&column.non_empty_cell_count().to_string());
        output.push_str(",\"firstNonEmptyRowIndex\":");
        push_option_usize_json(output, column.first_non_empty_row_index());
        output.push_str(",\"lastNonEmptyRowIndex\":");
        push_option_usize_json(output, column.last_non_empty_row_index());
        output.push_str(",\"sourceStart\":");
        push_option_usize_json(output, column.source_start());
        output.push_str(",\"sourceEnd\":");
        push_option_usize_json(output, column.source_end());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"geometryDecoded\":false,\"decoded\":false}");
}

fn push_sparse_table_rows_json(output: &mut String, rows: &[TableCandidateInterval]) {
    output.push('[');
    for (row_array_index, row) in rows.iter().enumerate() {
        if row_array_index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&row.index().to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&row.source_interval_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&row.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&row.source_end().to_string());
        output.push_str(",\"textPreview\":");
        push_json_string(output, row.text_preview());
        output.push_str(",\"cellCount\":");
        output.push_str(&row.column_segments().len().to_string());
        output.push_str(",\"cells\":[");
        for (cell_array_index, cell) in row.column_segments().iter().enumerate() {
            if cell_array_index > 0 {
                output.push(',');
            }
            output.push_str("{\"index\":");
            output.push_str(&cell.index().to_string());
            output.push_str(",\"kind\":");
            push_json_string(output, cell.kind().as_str());
            output.push_str(",\"charStart\":");
            output.push_str(&cell.char_start().to_string());
            output.push_str(",\"charEnd\":");
            output.push_str(&cell.char_end().to_string());
            output.push_str(",\"sourceStart\":");
            push_option_usize_json(output, cell.source_start());
            output.push_str(",\"sourceEnd\":");
            push_option_usize_json(output, cell.source_end());
            output.push_str(",\"text\":");
            push_json_string(output, cell.text());
            output.push_str(",\"empty\":");
            output.push_str(if cell.text().is_empty() {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"decoded\":false}");
        }
        output.push_str("],\"decoded\":false}");
    }
    output.push(']');
}

fn push_observed_table_json(output: &mut String, candidate: &TableCandidate) {
    let row_count = candidate.intervals().len();
    output.push_str("{\"rowCount\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"colCount\":1,\"cellCount\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"source\":\"tableCandidate\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"basis\":");
    push_json_string(output, candidate.basis().as_str());
    output.push_str(",\"delimiterCode\":");
    output.push_str(&candidate.delimiter_code().to_string());
    output.push_str(",\"delimiterCodeHex\":");
    push_json_string(output, &format!("0x{:04x}", candidate.delimiter_code()));
    output.push_str(",\"columnSplitCandidateRows\":");
    output.push_str(&candidate.column_split_candidate_row_count().to_string());
    output.push_str(",\"maxColumnSegmentCount\":");
    output.push_str(&candidate.max_column_segment_count().to_string());
    output.push_str(",\"columnSegmentPatternConsistent\":");
    output.push_str(if candidate.column_segment_pattern_consistent() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"columnSegmentPatternMismatchRows\":");
    output.push_str(&candidate.column_segment_pattern_mismatch_rows().to_string());
    output.push_str(",\"columnGridCandidate\":");
    if let Some(grid) = candidate.column_segment_grid_candidate() {
        push_column_grid_candidate_json(output, candidate, &grid);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"columnSplittingDecoded\":false");
    output.push_str(",\"decoded\":false}");
}

fn push_column_grid_candidate_json(
    output: &mut String,
    candidate: &TableCandidate,
    grid: &rjtd_model::TableCandidateColumnGridCandidate,
) {
    output.push_str("{\"source\":\"columnSegments\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&grid.row_count().to_string());
    output.push_str(",\"colCountCandidate\":");
    output.push_str(&grid.column_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&grid.cell_count().to_string());
    output.push_str(",\"columnSplitCandidateRows\":");
    output.push_str(&grid.split_row_count().to_string());
    output.push_str(",\"maxColumnSegmentCount\":");
    output.push_str(&candidate.max_column_segment_count().to_string());
    output.push_str(",\"columnSegmentPatternConsistent\":true");
    output.push_str(",\"columnSegmentPatternMismatchRows\":0");
    output.push_str(",\"pattern\":[");
    for (index, kind) in grid.pattern().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(output, kind.as_str());
    }
    output.push_str("],\"geometryDecoded\":false,\"decoded\":false}");
}

fn push_table_candidate_intervals_json(
    output: &mut String,
    intervals: &[TableCandidateInterval],
    emit_column_segments: bool,
) {
    output.push('[');
    for (index, interval) in intervals.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&interval.index().to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&interval.source_interval_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&interval.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&interval.source_end().to_string());
        output.push_str(",\"textPreview\":");
        push_json_string(output, interval.text_preview());
        output.push_str(",\"textCharCount\":");
        output.push_str(&interval.text_char_count().to_string());
        output.push_str(",\"lineBreakCount\":");
        output.push_str(&interval.line_break_count().to_string());
        output.push_str(",\"columnSegments\":");
        if emit_column_segments {
            push_table_candidate_column_segments_json(output, interval.column_segments());
        } else {
            output.push_str("[]");
        }
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

fn push_table_candidate_column_segments_json(
    output: &mut String,
    segments: &[TableCandidateColumnSegment],
) {
    output.push('[');
    for (index, segment) in segments.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&segment.index().to_string());
        output.push_str(",\"kind\":");
        push_json_string(output, segment.kind().as_str());
        output.push_str(",\"charStart\":");
        output.push_str(&segment.char_start().to_string());
        output.push_str(",\"charEnd\":");
        output.push_str(&segment.char_end().to_string());
        output.push_str(",\"sourceStart\":");
        push_option_usize_json(output, segment.source_start());
        output.push_str(",\"sourceEnd\":");
        push_option_usize_json(output, segment.source_end());
        output.push_str(",\"text\":");
        push_json_string(output, segment.text());
        output.push_str(",\"charCount\":");
        output.push_str(&segment.text().chars().count().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

fn push_text_layout_exact_evidence_json(output: &mut String, evidence: &TextLayoutExactEvidence) {
    output.push_str("{\"target\":");
    push_json_string(output, evidence.target());
    output.push_str(",\"base\":");
    push_json_string(output, evidence.base());
    output.push_str(",\"delta\":");
    output.push_str(&evidence.delta().to_string());
    output.push('}');
}

fn push_text_source_span_json(output: &mut String, span: &TextSourceSpan) {
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

fn push_text_count_range_overlaps_json(output: &mut String, overlaps: &[TextCountRangeOverlap]) {
    output.push('[');
    for (index, overlap) in overlaps.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"basis\":");
        push_json_string(output, overlap.basis().as_str());
        output.push_str(",\"blockIndex\":");
        output.push_str(&overlap.block_index().to_string());
        output.push_str(",\"inlineIndex\":");
        output.push_str(&overlap.inline_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&overlap.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&overlap.source_end().to_string());
        output.push_str(",\"text\":");
        push_json_string(output, overlap.text());
        output.push('}');
    }
    output.push(']');
}

fn push_text_count_control_range_overlaps_json(
    output: &mut String,
    overlaps: &[TextCountControlRangeOverlap],
) {
    output.push('[');
    for (index, overlap) in overlaps.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"basis\":");
        push_json_string(output, overlap.basis().as_str());
        output.push_str(",\"delimiterCode\":");
        output.push_str(&overlap.delimiter_code().to_string());
        output.push_str(",\"delimiterCodeHex\":");
        push_json_string(output, &format!("0x{:04x}", overlap.delimiter_code()));
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

fn push_unknown_source_json(output: &mut String, source: &UnknownRecordKind) {
    output.push_str("{\"tag\":");
    match source.tag() {
        Some(tag) => output.push_str(&tag.to_string()),
        None => output.push_str("null"),
    }
    output.push('}');
}

fn push_u32_array_json(output: &mut String, values: &[u32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

fn push_option_usize_json(output: &mut String, value: Option<usize>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

fn push_option_f32_json(output: &mut String, value: Option<f32>) {
    match value {
        Some(value) if value.is_finite() => output.push_str(&format!("{value:.3}")),
        _ => output.push_str("null"),
    }
}

fn push_option_u16_json(output: &mut String, value: Option<u16>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

fn push_option_u16_hex_json(output: &mut String, value: Option<u16>) {
    match value {
        Some(value) => push_json_string(output, &format!("0x{value:04x}")),
        None => output.push_str("null"),
    }
}

fn push_option_u32_json(output: &mut String, value: Option<u32>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

fn push_option_u32_hex_json(output: &mut String, value: Option<u32>) {
    match value {
        Some(value) => push_json_string(output, &format!("0x{value:08x}")),
        None => output.push_str("null"),
    }
}

fn push_style_records_json(output: &mut String, records: &[StyleStreamRecordSummary]) {
    output.push('[');
    for (index, record) in records.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"offset\":");
        output.push_str(&record.offset().to_string());
        output.push_str(",\"code\":");
        output.push_str(&record.code().to_string());
        output.push_str(",\"codeHex\":");
        push_json_string(output, &format!("0x{:04x}", record.code()));
        output.push_str(",\"payloadLength\":");
        output.push_str(&record.payload_len().to_string());
        output.push_str(",\"label\":");
        match record.label() {
            Some(label) => push_json_string(output, label),
            None => output.push_str("null"),
        }
        output.push_str(",\"subrecordCount\":");
        output.push_str(&record.subrecords().len().to_string());
        output.push_str(",\"subrecords\":");
        push_style_subrecords_json(output, record.subrecords());
        output.push('}');
    }
    output.push(']');
}

fn push_style_subrecords_json(output: &mut String, records: &[StyleStreamSubrecordSummary]) {
    output.push('[');
    for (index, record) in records.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"offset\":");
        output.push_str(&record.offset().to_string());
        output.push_str(",\"code\":");
        output.push_str(&record.code().to_string());
        output.push_str(",\"codeHex\":");
        push_json_string(output, &format!("0x{:04x}", record.code()));
        output.push_str(",\"payloadLength\":");
        output.push_str(&record.payload_len().to_string());
        output.push_str(",\"payloadHex\":");
        push_json_string(output, &hex(record.payload()));
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

fn push_u16_array_json(output: &mut String, values: &[u16]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

fn push_u16_hex_array_json(output: &mut String, values: &[u16]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(output, &format!("0x{value:04x}"));
    }
    output.push(']');
}

fn push_json_string(output: &mut String, value: &str) {
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0c}' => output.push_str("\\f"),
            character if character < ' ' => {
                output.push_str("\\u");
                output.push_str(&format!("{:04x}", character as u32));
            }
            character => output.push(character),
        }
    }
    output.push('"');
}

fn hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

#[cfg(not(target_arch = "wasm32"))]
fn create_fontdb() -> usvg::fontdb::Database {
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    for dir in &[
        "ttfs",
        "ttfs/windows",
        "ttfs/hwp",
        "/System/Library/Fonts",
        "/System/Library/Fonts/Supplemental",
        "/Library/Fonts",
    ] {
        if std::path::Path::new(dir).exists() {
            fontdb.load_fonts_dir(dir);
        }
    }
    load_macos_mobile_asset_fonts(&mut fontdb);

    fontdb.set_serif_family("Hiragino Mincho ProN");
    fontdb.set_sans_serif_family("Hiragino Sans");
    fontdb.set_monospace_family("Menlo");
    fontdb
}

#[cfg(not(target_arch = "wasm32"))]
fn load_macos_mobile_asset_fonts(fontdb: &mut usvg::fontdb::Database) {
    let base = std::path::Path::new("/System/Library/AssetsV2");
    let Ok(entries) = std::fs::read_dir(base) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if name.starts_with("com_apple_MobileAsset_Font") {
            load_font_dirs_recursive(fontdb, &path, 0);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_font_dirs_recursive(
    fontdb: &mut usvg::fontdb::Database,
    path: &std::path::Path,
    depth: usize,
) {
    if depth > 4 {
        return;
    }
    fontdb.load_fonts_dir(path);

    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            load_font_dirs_recursive(fontdb, &path, depth + 1);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn add_font_fallbacks(svg: &str) -> String {
    svg.replace(
        "font-family=\"Hiragino Sans, Hiragino Kaku Gothic ProN, Yu Gothic, Meiryo, Noto Sans CJK JP, sans-serif\"",
        "font-family=\"Hiragino Sans, Hiragino Kaku Gothic ProN, Hiragino Sans GB, Yu Gothic, Meiryo, Apple SD Gothic Neo, Noto Sans CJK JP, sans-serif\"",
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn svgs_to_pdf(svg_pages: &[String]) -> Result<Vec<u8>, String> {
    if svg_pages.is_empty() {
        return Err("no pages to export".to_string());
    }

    let options = usvg::Options {
        fontdb: std::sync::Arc::new(create_fontdb()),
        ..Default::default()
    };

    use pdf_writer::{Finish, Pdf, Ref};
    use std::collections::HashMap;

    let mut alloc = Ref::new(1);
    let catalog_ref = alloc.bump();
    let page_tree_ref = alloc.bump();

    struct PageData {
        chunk: pdf_writer::Chunk,
        svg_ref: Ref,
        width: f32,
        height: f32,
    }

    let mut page_datas = Vec::new();

    for svg in svg_pages {
        let svg_with_fallback = add_font_fallbacks(svg);
        let tree = usvg::Tree::from_str(&svg_with_fallback, &options)
            .map_err(|error| format!("SVG parse failed: {error}"))?;
        let (chunk, svg_ref) = svg2pdf::to_chunk(&tree, svg2pdf::ConversionOptions::default())
            .map_err(|error| format!("SVG chunk conversion failed: {error:?}"))?;
        let dpi_ratio = 72.0 / 96.0;
        page_datas.push(PageData {
            chunk,
            svg_ref,
            width: tree.size().width() * dpi_ratio,
            height: tree.size().height() * dpi_ratio,
        });
    }

    let mut page_refs = Vec::new();
    let mut renumbered_chunks = Vec::new();
    let mut svg_refs_remapped = Vec::new();

    for page_data in &page_datas {
        let page_ref = alloc.bump();
        page_refs.push(page_ref);
        let mut map = HashMap::new();
        let renumbered = page_data
            .chunk
            .renumber(|old| *map.entry(old).or_insert_with(|| alloc.bump()));
        let remapped_svg_ref = map
            .get(&page_data.svg_ref)
            .copied()
            .unwrap_or(page_data.svg_ref);
        renumbered_chunks.push(renumbered);
        svg_refs_remapped.push(remapped_svg_ref);
    }

    let mut pdf = Pdf::new();
    pdf.set_version(1, 4);
    pdf.catalog(catalog_ref).pages(page_tree_ref);
    pdf.pages(page_tree_ref)
        .count(page_refs.len() as i32)
        .kids(page_refs.iter().copied());

    let svg_name = pdf_writer::Name(b"S1");
    for (index, page_data) in page_datas.iter().enumerate() {
        let page_ref = page_refs[index];
        let content_ref = alloc.bump();
        let svg_ref = svg_refs_remapped[index];

        let mut page = pdf.page(page_ref);
        page.media_box(pdf_writer::Rect::new(
            0.0,
            0.0,
            page_data.width,
            page_data.height,
        ));
        page.parent(page_tree_ref);
        page.contents(content_ref);

        let mut resources = page.resources();
        resources.proc_sets_all();
        resources.x_objects().pair(svg_name, svg_ref);
        resources.finish();
        page.finish();

        let mut content = pdf_writer::Content::new();
        content.save_state();
        content.set_fill_rgb(1.0, 1.0, 1.0);
        content.rect(0.0, 0.0, page_data.width, page_data.height);
        content.fill_nonzero();
        content.restore_state();
        content.save_state();
        content.transform([page_data.width, 0.0, 0.0, page_data.height, 0.0, 0.0]);
        content.x_object(svg_name);
        content.restore_state();
        pdf.stream(content_ref, &content.finish());
    }

    for chunk in &renumbered_chunks {
        pdf.extend(chunk);
    }

    let info_ref = alloc.bump();
    pdf.document_info(info_ref)
        .producer(pdf_writer::TextStr("rjtd"));

    let mut bytes = pdf.finish();
    scrub_embedded_pdf_eof_markers(&mut bytes);
    ensure_pdf_form_xobject_form_types(&mut bytes)?;
    validate_pdf_preview_safety(&bytes)?;
    Ok(bytes)
}

#[cfg(not(target_arch = "wasm32"))]
fn ensure_pdf_form_xobject_form_types(bytes: &mut Vec<u8>) -> Result<(), String> {
    let xref_offset = pdf_startxref_offset(bytes)?;
    let mut body = bytes[..xref_offset].to_vec();
    if insert_pdf_form_xobject_form_types(&mut body) == 0 {
        return Ok(());
    }

    let root_ref = parse_pdf_trailer_ref(bytes, b"/Root")
        .ok_or_else(|| "generated PDF trailer is missing /Root".to_string())?;
    let info_ref = parse_pdf_trailer_ref(bytes, b"/Info");
    let offsets = collect_pdf_object_offsets(&body)?;

    let xref_offset = body.len();
    body.extend(b"xref\n0 ");
    let xref_len = offsets
        .last()
        .map(|(object_id, _)| object_id + 1)
        .unwrap_or(1);
    body.extend(xref_len.to_string().as_bytes());
    body.push(b'\n');
    body.extend(b"0000000000 65535 f\r\n");

    let mut next_offset = offsets.iter().peekable();
    for object_id in 1..xref_len {
        if next_offset
            .peek()
            .is_some_and(|(used_id, _)| *used_id == object_id)
        {
            let (_, offset) = next_offset.next().unwrap();
            body.extend(format!("{offset:010} 00000 n\r\n").as_bytes());
        } else {
            body.extend(b"0000000000 65535 f\r\n");
        }
    }

    body.extend(b"trailer\n<<\n  /Size ");
    body.extend(xref_len.to_string().as_bytes());
    body.extend(b"\n  /Root ");
    body.extend(root_ref.to_string().as_bytes());
    body.extend(b" 0 R");
    if let Some(info_ref) = info_ref {
        body.extend(b"\n  /Info ");
        body.extend(info_ref.to_string().as_bytes());
        body.extend(b" 0 R");
    }
    body.extend(b"\n>>\nstartxref\n");
    body.extend(xref_offset.to_string().as_bytes());
    body.extend(b"\n%%EOF");

    *bytes = body;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn insert_pdf_form_xobject_form_types(bytes: &mut Vec<u8>) -> usize {
    let mut inserted = 0usize;
    let mut position = 0usize;
    while let Some(relative_offset) = find_subslice(&bytes[position..], b"/Subtype /Form") {
        let subtype_offset = position + relative_offset;
        let Some(object_start) = find_pdf_object_start_before(bytes, subtype_offset) else {
            position = subtype_offset + b"/Subtype /Form".len();
            continue;
        };
        let Some(stream_offset) = find_pdf_stream_marker_after(bytes, subtype_offset) else {
            position = subtype_offset + b"/Subtype /Form".len();
            continue;
        };
        let dictionary = &bytes[object_start..stream_offset];
        if dictionary
            .windows(b"/FormType".len())
            .any(|w| w == b"/FormType")
        {
            position = subtype_offset + b"/Subtype /Form".len();
            continue;
        }

        let insert_offset = bytes[subtype_offset..stream_offset]
            .iter()
            .position(|byte| *byte == b'\n')
            .map(|newline| subtype_offset + newline + 1)
            .unwrap_or(subtype_offset + b"/Subtype /Form".len());
        bytes.splice(
            insert_offset..insert_offset,
            b"  /FormType 1\n".iter().copied(),
        );
        inserted += 1;
        position = insert_offset + b"  /FormType 1\n".len();
    }
    inserted
}

#[cfg(not(target_arch = "wasm32"))]
fn find_pdf_object_start_before(bytes: &[u8], offset: usize) -> Option<usize> {
    let object_marker = find_last_subslice(bytes.get(..offset)?, b" obj")?;
    let line_start = bytes[..object_marker]
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(0, |newline| newline + 1);
    Some(line_start)
}

#[cfg(not(target_arch = "wasm32"))]
fn find_pdf_stream_marker_after(bytes: &[u8], offset: usize) -> Option<usize> {
    let line_feed = find_subslice(bytes.get(offset..)?, b"\nstream")?;
    Some(offset + line_feed)
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_startxref_offset(bytes: &[u8]) -> Result<usize, String> {
    let marker_offset = find_last_subslice(bytes, b"startxref")
        .ok_or_else(|| "generated PDF is missing startxref".to_string())?;
    let mut position = marker_offset + b"startxref".len();
    position = pdf_skip_whitespace(bytes, position);
    let start = position;
    while position < bytes.len() && bytes[position].is_ascii_digit() {
        position += 1;
    }
    let value = std::str::from_utf8(&bytes[start..position])
        .ok()
        .and_then(|text| text.parse::<usize>().ok())
        .ok_or_else(|| "generated PDF has invalid startxref offset".to_string())?;
    if !bytes
        .get(value..)
        .is_some_and(|tail| tail.starts_with(b"xref"))
    {
        return Err("generated PDF startxref does not point to an xref table".to_string());
    }
    Ok(value)
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_pdf_trailer_ref(bytes: &[u8], key: &[u8]) -> Option<usize> {
    let key_offset = find_subslice(bytes, key)?;
    let mut position = pdf_skip_whitespace(bytes, key_offset + key.len());
    let start = position;
    while position < bytes.len() && bytes[position].is_ascii_digit() {
        position += 1;
    }
    let object_id = std::str::from_utf8(&bytes[start..position])
        .ok()?
        .parse::<usize>()
        .ok()?;
    position = pdf_skip_whitespace(bytes, position);
    if !bytes.get(position..)?.starts_with(b"0") {
        return None;
    }
    position = pdf_skip_whitespace(bytes, position + 1);
    if !bytes.get(position..)?.starts_with(b"R") {
        return None;
    }
    Some(object_id)
}

#[cfg(not(target_arch = "wasm32"))]
fn collect_pdf_object_offsets(bytes: &[u8]) -> Result<Vec<(usize, usize)>, String> {
    let mut offsets = Vec::new();
    let mut line_start = 0usize;
    while line_start < bytes.len() {
        if let Some(object_id) = parse_pdf_object_header(bytes, line_start) {
            offsets.push((object_id, line_start));
        }
        let Some(relative_newline) = bytes[line_start..].iter().position(|byte| *byte == b'\n')
        else {
            break;
        };
        line_start += relative_newline + 1;
    }
    offsets.sort_by_key(|(object_id, _)| *object_id);
    if offsets.windows(2).any(|window| window[0].0 == window[1].0) {
        return Err("generated PDF contains duplicate object ids".to_string());
    }
    Ok(offsets)
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_pdf_object_header(bytes: &[u8], offset: usize) -> Option<usize> {
    let mut position = offset;
    while position < bytes.len() && bytes[position].is_ascii_digit() {
        position += 1;
    }
    if position == offset {
        return None;
    }
    let object_id = std::str::from_utf8(&bytes[offset..position])
        .ok()?
        .parse::<usize>()
        .ok()?;
    position = pdf_skip_plain_spaces(bytes, position);
    if !bytes.get(position..)?.starts_with(b"0") {
        return None;
    }
    position = pdf_skip_plain_spaces(bytes, position + 1);
    if !bytes.get(position..)?.starts_with(b"obj") {
        return None;
    }
    Some(object_id)
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_skip_plain_spaces(bytes: &[u8], mut position: usize) -> usize {
    while position < bytes.len() && matches!(bytes[position], b'\t' | b' ') {
        position += 1;
    }
    position
}

#[cfg(not(target_arch = "wasm32"))]
fn scrub_embedded_pdf_eof_markers(bytes: &mut [u8]) {
    let Some(final_eof_offset) = find_last_subslice(bytes, b"%%EOF") else {
        return;
    };

    let mut position = 0usize;
    while position < final_eof_offset {
        let Some(relative_offset) = find_subslice(&bytes[position..final_eof_offset], b"%%EOF")
        else {
            break;
        };
        let marker_offset = position + relative_offset;
        if pdf_eof_marker_is_embedded_cmap_comment(bytes, marker_offset) {
            bytes[marker_offset + 4] = b'D';
        }
        position = marker_offset + b"%%EOF".len();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_eof_marker_is_embedded_cmap_comment(bytes: &[u8], marker_offset: usize) -> bool {
    let prefix_start = marker_offset.saturating_sub(96);
    let suffix_end = bytes.len().min(marker_offset + 64);
    let prefix = &bytes[prefix_start..marker_offset];
    let suffix = &bytes[marker_offset + b"%%EOF".len()..suffix_end];

    find_subslice(prefix, b"%%EndResource").is_some()
        && (suffix.starts_with(b"\nendstream") || suffix.starts_with(b"\r\nendstream"))
}

#[cfg(not(target_arch = "wasm32"))]
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(not(target_arch = "wasm32"))]
fn find_last_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .rposition(|window| window == needle)
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_pdf_preview_safety(bytes: &[u8]) -> Result<(), String> {
    let issues = pdf_preview_blocking_issues(bytes);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "generated PDF contains Preview/PDFKit risky transparency constructs: {}",
            issues.join(", ")
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_preview_blocking_issues(bytes: &[u8]) -> Vec<&'static str> {
    pdf_preview_safety_issues(bytes)
        .into_iter()
        .filter(|issue| *issue != "soft-mask")
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_preview_safety_issues(bytes: &[u8]) -> Vec<&'static str> {
    let mut issues = Vec::new();
    if pdf_contains_token_sequence(bytes, &[b"/Group", b"<<"]) {
        issues.push("transparency-group-dictionary");
    }
    if pdf_contains_token_sequence(bytes, &[b"/S", b"/Transparency"]) {
        issues.push("transparency-group-subtype");
    }
    if pdf_contains_token_sequence(bytes, &[b"/SMask"]) {
        issues.push("soft-mask");
    }
    issues
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_contains_token_sequence(bytes: &[u8], tokens: &[&[u8]]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    for start in 0..bytes.len() {
        let Some(mut position) = pdf_match_token_at(bytes, start, tokens[0]) else {
            continue;
        };
        let mut matched = true;
        for token in &tokens[1..] {
            position = pdf_skip_whitespace(bytes, position);
            let Some(next_position) = pdf_match_token_at(bytes, position, token) else {
                matched = false;
                break;
            };
            position = next_position;
        }
        if matched {
            return true;
        }
    }
    false
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_match_token_at(bytes: &[u8], position: usize, token: &[u8]) -> Option<usize> {
    if token.is_empty() || !bytes.get(position..)?.starts_with(token) {
        return None;
    }
    let end = position + token.len();
    if token == b"<<" || token == b">>" {
        return Some(end);
    }
    if end < bytes.len() && !pdf_is_delimiter(bytes[end]) {
        return None;
    }
    Some(end)
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_skip_whitespace(bytes: &[u8], mut position: usize) -> usize {
    while position < bytes.len() && matches!(bytes[position], 0 | b'\t' | b'\n' | 12 | b'\r' | b' ')
    {
        position += 1;
    }
    position
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_is_delimiter(byte: u8) -> bool {
    matches!(
        byte,
        0 | b'\t'
            | b'\n'
            | 12
            | b'\r'
            | b' '
            | b'('
            | b')'
            | b'<'
            | b'>'
            | b'['
            | b']'
            | b'{'
            | b'}'
            | b'/'
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rjtd_core::record::UnknownRecordKind;
    use rjtd_model::{
        Block, Document, Inline, Metadata, ObjectImageDeclaredLengthCandidate,
        ObjectImagePayloadEnvelope, ObjectImagePayloadLocation, ObjectImagePayloadSpan,
        ObjectImageSignatureHit, ObjectStreamCandidate, ObjectStreamCandidateEvidence,
        ObjectStreamCandidateReason, Paragraph, RawStream, RubyAnnotation, StyleRef,
        TextControlBoundary, TextRun, UnknownBlock, UnknownObject, UnknownStyle, parse_document,
    };
    use std::{
        collections::BTreeSet,
        fs,
        path::{Path, PathBuf},
        process::Command,
    };

    #[cfg(not(target_arch = "wasm32"))]
    fn count_pdf_eof_markers(pdf: &[u8]) -> usize {
        pdf.windows(b"%%EOF".len())
            .filter(|window| *window == b"%%EOF")
            .count()
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[derive(Debug, Clone, Copy)]
    struct PdfMediaBox {
        width: f32,
        height: f32,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl PdfMediaBox {
        fn close_to(self, other: Self) -> bool {
            const MEDIA_BOX_TOLERANCE_PT: f32 = 1.0;
            (self.width - other.width).abs() <= MEDIA_BOX_TOLERANCE_PT
                && (self.height - other.height).abs() <= MEDIA_BOX_TOLERANCE_PT
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[derive(Debug, Clone, Copy)]
    struct LocalReferencePdfKnownDivergence {
        expected_reference_page_count: usize,
        expected_output_page_count: usize,
        page_count_reason: &'static str,
        media_box_divergence: Option<LocalReferencePdfKnownMediaBoxDivergence>,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl LocalReferencePdfKnownDivergence {
        const fn pagination(
            expected_reference_page_count: usize,
            expected_output_page_count: usize,
        ) -> Self {
            Self {
                expected_reference_page_count,
                expected_output_page_count,
                page_count_reason: LOCAL_REFERENCE_FALLBACK_PAGINATION_DIVERGES_FROM_REFERENCE,
                media_box_divergence: None,
            }
        }

        const fn pagination_with_media_box(
            expected_reference_page_count: usize,
            expected_output_page_count: usize,
            media_box_divergence: LocalReferencePdfKnownMediaBoxDivergence,
        ) -> Self {
            Self {
                expected_reference_page_count,
                expected_output_page_count,
                page_count_reason: LOCAL_REFERENCE_FALLBACK_PAGINATION_DIVERGES_FROM_REFERENCE,
                media_box_divergence: Some(media_box_divergence),
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[derive(Debug, Clone, Copy)]
    struct LocalReferencePdfKnownMediaBoxDivergence {
        expected_reference_media_box: PdfMediaBox,
        expected_output_media_box: PdfMediaBox,
        reason: &'static str,
    }

    #[cfg(not(target_arch = "wasm32"))]
    const LOCAL_REFERENCE_FALLBACK_PAGINATION_DIVERGES_FROM_REFERENCE: &str =
        "fallback-pagination-diverges-from-reference";
    #[cfg(not(target_arch = "wasm32"))]
    const LOCAL_REFERENCE_PAPER_ORIENTATION_SOURCE_DECODE_UNPROVEN: &str =
        "paper-orientation-source-decode-unproven";

    #[cfg(not(target_arch = "wasm32"))]
    #[derive(Debug, Clone, Copy)]
    struct PngRatioRegionCheck {
        label: &'static str,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        min_non_white: usize,
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[derive(Debug, Clone, Copy)]
    struct LocalPdfSmokeFixture {
        source_name: &'static str,
        output_pdf_name: &'static str,
        page_checks: &'static [&'static str],
        sips_region_check: Option<PngRatioRegionCheck>,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl LocalPdfSmokeFixture {
        fn source_path(self, sample_dir: &Path) -> PathBuf {
            sample_dir.join(self.source_name)
        }

        fn output_pdf_path(self, output_dir: &Path) -> PathBuf {
            output_dir.join(self.output_pdf_name)
        }

        fn source_with_reference_pdf_exists(self, sample_dir: &Path) -> bool {
            let sample_path = self.source_path(sample_dir);
            sample_path.exists() && sample_path.with_extension("pdf").exists()
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    const SUCCESS_DATA_TEST_PAGE_CHECKS: &[&str] = &["1:10000", "2:500"];
    #[cfg(not(target_arch = "wasm32"))]
    const SHANAI_LAN_PAGE_CHECKS: &[&str] = &["1:5000"];
    #[cfg(not(target_arch = "wasm32"))]
    const A5_PAGE_CHECKS: &[&str] = &["1:300", "6:3000"];
    #[cfg(not(target_arch = "wasm32"))]
    const FAX02_PAGE_CHECKS: &[&str] = &["1:10000"];

    #[cfg(not(target_arch = "wasm32"))]
    const LOCAL_PDF_SMOKE_FIXTURES: &[LocalPdfSmokeFixture] = &[
        LocalPdfSmokeFixture {
            source_name: "ichitaro-20030228030923-success-002-success_data-test.jtd",
            output_pdf_name: "ichitaro-20030228030923-success-002-success_data-test.pdf",
            page_checks: SUCCESS_DATA_TEST_PAGE_CHECKS,
            sips_region_check: Some(PngRatioRegionCheck {
                label: "title region",
                left: 0.05,
                top: 0.07,
                right: 0.92,
                bottom: 0.20,
                min_non_white: 3_000,
            }),
        },
        LocalPdfSmokeFixture {
            source_name: "ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd",
            output_pdf_name: "ichitaro-20030315134715-success-001-success_data-shanai_lan.pdf",
            page_checks: SHANAI_LAN_PAGE_CHECKS,
            sips_region_check: None,
        },
        LocalPdfSmokeFixture {
            source_name: "a5.jtd",
            output_pdf_name: "a5.pdf",
            page_checks: A5_PAGE_CHECKS,
            sips_region_check: None,
        },
        LocalPdfSmokeFixture {
            source_name: "fax02.jtt",
            output_pdf_name: "fax02.pdf",
            page_checks: FAX02_PAGE_CHECKS,
            sips_region_check: None,
        },
    ];

    #[cfg(not(target_arch = "wasm32"))]
    fn local_sample_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples")
    }

    fn test_json_string(value: &str) -> String {
        let mut output = String::new();
        push_json_string(&mut output, value);
        output
    }

    fn test_json_string_array(values: &[&str]) -> String {
        let mut output = String::new();
        push_json_string_slice_array(&mut output, values);
        output
    }

    fn tail_after_occurrence<'a>(haystack: &'a str, marker: &str, occurrence: usize) -> &'a str {
        let mut tail = haystack;
        for index in 0..=occurrence {
            let Some((_, next_tail)) = tail.split_once(marker) else {
                panic!("missing JSON marker occurrence {index} for {marker}");
            };
            tail = next_tail;
        }
        tail
    }

    fn assert_json_string_field_after(
        haystack: &str,
        marker: &str,
        occurrence: usize,
        field: &str,
        expected: &str,
    ) {
        let fragment = format!("\"{field}\":{}", test_json_string(expected));
        let tail = tail_after_occurrence(haystack, marker, occurrence);
        assert!(
            tail.contains(&fragment),
            "missing JSON field {field}={expected:?} after marker {marker}"
        );
    }

    fn assert_json_number_field_after(
        haystack: &str,
        marker: &str,
        occurrence: usize,
        field: &str,
        expected: &str,
    ) {
        let fragment = format!("\"{field}\":{expected}");
        let tail = tail_after_occurrence(haystack, marker, occurrence);
        assert!(
            tail.contains(&fragment),
            "missing JSON field {field}={expected} after marker {marker}"
        );
    }

    fn assert_json_bool_field_after(
        haystack: &str,
        marker: &str,
        occurrence: usize,
        field: &str,
        expected: bool,
    ) {
        let fragment = format!("\"{field}\":{}", if expected { "true" } else { "false" });
        let tail = tail_after_occurrence(haystack, marker, occurrence);
        assert!(
            tail.contains(&fragment),
            "missing JSON field {field}={expected} after marker {marker}"
        );
    }

    fn assert_json_string_array_field_after(
        haystack: &str,
        marker: &str,
        occurrence: usize,
        field: &str,
        expected: &[&str],
    ) {
        let fragment = format!("\"{field}\":{}", test_json_string_array(expected));
        let tail = tail_after_occurrence(haystack, marker, occurrence);
        assert!(
            tail.contains(&fragment),
            "missing JSON string array field {field}={expected:?} after marker {marker}"
        );
    }

    #[test]
    fn fdm_bbox_center_handles_extreme_bounds_without_overflow() {
        assert_eq!(
            fdm_bbox_center((i32::MIN, i32::MIN, i32::MAX, i32::MAX)),
            (-1, -1)
        );
        assert_eq!(fdm_bbox_center((-3, -3, -2, -2)), (-3, -3));
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pdf_media_box_sizes(pdf: &[u8]) -> Vec<PdfMediaBox> {
        let mut sizes = Vec::new();
        let mut position = 0usize;
        while let Some(relative_offset) = find_subslice(&pdf[position..], b"/MediaBox") {
            let media_box_offset = position + relative_offset;
            let mut cursor = pdf_skip_whitespace(pdf, media_box_offset + b"/MediaBox".len());
            if !pdf.get(cursor..).is_some_and(|tail| tail.starts_with(b"[")) {
                position = media_box_offset + b"/MediaBox".len();
                continue;
            }
            cursor += 1;

            let Some(x0) = pdf_parse_number(pdf, &mut cursor) else {
                position = media_box_offset + b"/MediaBox".len();
                continue;
            };
            let Some(y0) = pdf_parse_number(pdf, &mut cursor) else {
                position = media_box_offset + b"/MediaBox".len();
                continue;
            };
            let Some(x1) = pdf_parse_number(pdf, &mut cursor) else {
                position = media_box_offset + b"/MediaBox".len();
                continue;
            };
            let Some(y1) = pdf_parse_number(pdf, &mut cursor) else {
                position = media_box_offset + b"/MediaBox".len();
                continue;
            };
            sizes.push(PdfMediaBox {
                width: (x1 - x0).abs(),
                height: (y1 - y0).abs(),
            });
            position = cursor;
        }
        sizes
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pdf_parse_number(bytes: &[u8], position: &mut usize) -> Option<f32> {
        *position = pdf_skip_whitespace(bytes, *position);
        let start = *position;
        if bytes
            .get(*position)
            .is_some_and(|byte| matches!(*byte, b'+' | b'-'))
        {
            *position += 1;
        }
        let mut saw_digit = false;
        while bytes.get(*position).is_some_and(|byte| {
            let numeric = byte.is_ascii_digit() || *byte == b'.';
            saw_digit |= byte.is_ascii_digit();
            numeric
        }) {
            *position += 1;
        }
        if !saw_digit {
            return None;
        }
        std::str::from_utf8(&bytes[start..*position])
            .ok()?
            .parse::<f32>()
            .ok()
    }

    #[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
    fn png_non_white_count_in_ratio_region(
        path: &Path,
        left_ratio: f32,
        top_ratio: f32,
        right_ratio: f32,
        bottom_ratio: f32,
    ) -> Result<usize, String> {
        let image = image::ImageReader::open(path)
            .map_err(|error| error.to_string())?
            .decode()
            .map_err(|error| error.to_string())?
            .to_rgb8();
        let width = image.width();
        let height = image.height();
        if width == 0 || height == 0 {
            return Err("PNG image has zero size".to_string());
        }

        let left = ((width as f32) * left_ratio).floor().max(0.0) as u32;
        let top = ((height as f32) * top_ratio).floor().max(0.0) as u32;
        let right = ((width as f32) * right_ratio).ceil().min(width as f32) as u32;
        let bottom = ((height as f32) * bottom_ratio).ceil().min(height as f32) as u32;
        if left >= right || top >= bottom {
            return Err(format!(
                "invalid PNG region {left},{top}..{right},{bottom} for {width}x{height}"
            ));
        }

        let mut non_white = 0usize;
        for y in top..bottom {
            for x in left..right {
                let pixel = image.get_pixel(x, y);
                if pixel[0] < 245 || pixel[1] < 245 || pixel[2] < 245 {
                    non_white += 1;
                }
            }
        }
        Ok(non_white)
    }

    #[test]
    fn exports_markdown_from_document_model() {
        let paragraph = Paragraph::new(vec![Inline::Text(TextRun::new("hello", None))], None);
        let document = Document::new(
            Metadata::new(Some("sample".to_string())),
            vec![Block::Paragraph(paragraph)],
        );

        assert_eq!(to_markdown(&document), "hello\n\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn exports_pdf_from_document_model() {
        let document = Document::from_plain_text("銀河鉄道\n午后の授業");
        let pdf = to_pdf(&document).unwrap();
        let pdf_text = String::from_utf8_lossy(&pdf);

        assert!(pdf.starts_with(b"%PDF-1.4"));
        assert!(pdf.windows(5).any(|window| window == b"/Page"));
        assert!(pdf_text.contains("/MediaBox [0 0 "));
        assert!(pdf_text.contains("1 1 1 rg\n0 0 "));
        assert!(pdf_text.contains(" re\nf\nQ\nq\n"));
        assert!(pdf_text.contains("/S1 Do"));
        assert!(pdf_text.contains("/Subtype /Form"));
        assert!(pdf_text.contains("/FormType 1"));
        assert!(pdf_text.contains("/Producer (rjtd)"));
        assert!(!pdf_text.contains("/SMask"));
        assert!(pdf_preview_safety_issues(&pdf).is_empty());
        assert_eq!(count_pdf_eof_markers(&pdf), 1);
        assert!(pdf.ends_with(b"%%EOF"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn pdf_media_box_parser_extracts_page_sizes() {
        let pdf = b"1 0 obj\n<< /Type /Page\n/MediaBox [0 0 419.55 595.275]\n>>\nendobj\n2 0 obj\n<< /Type /Page /MediaBox [-10 -20 90 180] >>\nendobj\n";

        let sizes = pdf_media_box_sizes(pdf);

        assert_eq!(sizes.len(), 2);
        assert!(sizes[0].close_to(PdfMediaBox {
            width: 419.55,
            height: 595.275,
        }));
        assert!(sizes[1].close_to(PdfMediaBox {
            width: 100.0,
            height: 200.0,
        }));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn exports_pdf_does_not_apply_layout_hints_from_filename_only() {
        let document = Document::from_plain_text(&vec!["銀河鉄道の夜"; 80].join("\n"));
        let pdf = to_pdf_with_file_name(&document, "a5.jtd").unwrap();
        let pdf_text = String::from_utf8_lossy(&pdf);

        assert!(pdf.starts_with(b"%PDF-1.4"));
        assert!(pdf_text.contains("/MediaBox [0 0 595.5 842.25]"));
        assert!(pdf_text.contains("1 1 1 rg\n0 0 595.5 842.25"));
        assert!(pdf_text.contains(" re\nf\nQ\nq\n"));
        assert!(pdf_text.contains("q\n595.5 0 0 842.25 0 0 cm"));
        assert!(pdf_text.contains("/S1 Do\nQ"));
        assert!(pdf_text.contains("/FormType 1"));
        assert!(!pdf_text.contains("/Group <<"));
        assert!(!pdf_text.contains("/S /Transparency"));
        assert!(!pdf_text.contains("/SMask"));
        assert!(pdf_preview_safety_issues(&pdf).is_empty());
        assert_eq!(count_pdf_eof_markers(&pdf), 1);
        assert!(pdf.ends_with(b"%%EOF"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn embeds_svg_chunk_with_preview_safe_page_wrapper_contract() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="80" viewBox="0 0 120 80"><rect width="120" height="80" fill="#fff"/><circle cx="60" cy="40" r="24" fill="#123456"/></svg>"##;
        let pdf = svgs_to_pdf(&[svg.to_string()]).unwrap();
        let pdf_text = String::from_utf8_lossy(&pdf);

        assert!(pdf.starts_with(b"%PDF-1.4"));
        assert!(pdf_text.contains("/MediaBox [0 0 90 60]"));
        assert!(pdf_text.contains("1 1 1 rg\n0 0 90 60 re\nf\nQ\nq\n"));
        assert!(pdf_text.contains("90 0 0 60 0 0 cm\n/S1 Do\nQ"));
        assert!(pdf_text.contains("/Subtype /Form"));
        assert!(pdf_text.contains("/FormType 1"));
        assert!(pdf_text.contains("/BBox [0 0 120 80]"));
        assert!(pdf_text.contains("/Matrix [0.008333334 0 0 0.0125 0 0]"));
        assert!(!pdf_text.contains("/Group <<"));
        assert!(!pdf_text.contains("/S /Transparency"));
        assert!(!pdf_text.contains("/SMask"));
        assert!(pdf_preview_safety_issues(&pdf).is_empty());
        assert_eq!(count_pdf_eof_markers(&pdf), 1);
        assert!(pdf.ends_with(b"%%EOF"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn scrubs_embedded_cmap_eof_markers_but_keeps_file_eof() {
        let mut pdf = b"%PDF-1.4\n1 0 obj\n<< /Length 45 >>\nstream\n%%EndResource\n%%EOF\nendstream\nendobj\nstartxref\n0\n%%EOF"
            .to_vec();

        scrub_embedded_pdf_eof_markers(&mut pdf);

        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("%%EndResource\n%%EOD\nendstream"));
        assert!(pdf.ends_with(b"%%EOF"));
        assert_eq!(count_pdf_eof_markers(&pdf), 1);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn preview_safety_scanner_catches_flexible_pdf_token_spacing() {
        let pdf =
            b"%PDF-1.4\n1 0 obj\n<< /Group\n  << /S\t/Transparency >> /SMask 2 0 R >>\nendobj";
        assert_eq!(
            pdf_preview_safety_issues(pdf),
            vec![
                "transparency-group-dictionary",
                "transparency-group-subtype",
                "soft-mask"
            ]
        );
        assert_eq!(
            pdf_preview_blocking_issues(pdf),
            vec![
                "transparency-group-dictionary",
                "transparency-group-subtype"
            ]
        );

        assert!(!pdf_contains_token_sequence(
            b"<< /Subtype /Form >>",
            &[b"/S"]
        ));
    }

    #[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
    #[test]
    fn local_complex_pdfs_rasterize_with_macos_sips_when_available() {
        let sample_dir = local_sample_dir();
        if !sample_dir.exists() {
            return;
        }

        let mut failures = Vec::new();
        let mut rendered_count = 0usize;

        let any_sample_present = LOCAL_PDF_SMOKE_FIXTURES
            .iter()
            .any(|fixture| fixture.source_with_reference_pdf_exists(&sample_dir));
        if !any_sample_present {
            return;
        }

        for fixture in LOCAL_PDF_SMOKE_FIXTURES {
            let sample = fixture.source_name;
            let sample_path = fixture.source_path(&sample_dir);
            if !sample_path.exists() || !sample_path.with_extension("pdf").exists() {
                continue;
            }

            let result = fs::read(&sample_path)
                .map_err(|error| error.to_string())
                .and_then(|bytes| parse_document(&bytes).map_err(|error| error.to_string()))
                .and_then(|document| {
                    to_pdf_with_file_name(&document, &sample_path.to_string_lossy())
                });
            let pdf = match result {
                Ok(pdf) => pdf,
                Err(error) => {
                    failures.push(format!("{}: {error}", sample_path.display()));
                    continue;
                }
            };

            let temp_dir = std::env::temp_dir()
                .join(format!("rjtd-sips-smoke-{}-{sample}", std::process::id()));
            if let Err(error) = fs::create_dir_all(&temp_dir) {
                failures.push(format!("{}: create temp dir failed: {error}", sample));
                continue;
            }
            let pdf_path = temp_dir.join("sample.pdf");
            let png_path = temp_dir.join("sample.png");
            let module_cache_path = temp_dir.join("swift-module-cache");
            if let Err(error) = fs::create_dir_all(&module_cache_path) {
                failures.push(format!(
                    "{}: create Swift module cache failed: {error}",
                    sample
                ));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }
            if let Err(error) = fs::write(&pdf_path, &pdf) {
                failures.push(format!("{}: write temp pdf failed: {error}", sample));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }

            let output = match Command::new("sips")
                .arg("-s")
                .arg("format")
                .arg("png")
                .arg(&pdf_path)
                .arg("--out")
                .arg(&png_path)
                .output()
            {
                Ok(output) => output,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
                Err(error) => {
                    failures.push(format!("{}: run sips failed: {error}", sample));
                    let _ = fs::remove_dir_all(&temp_dir);
                    continue;
                }
            };

            if !output.status.success() {
                failures.push(format!(
                    "{}: sips failed with status {:?}: {}",
                    sample,
                    output.status.code(),
                    String::from_utf8_lossy(&output.stderr)
                ));
            } else if fs::metadata(&png_path)
                .map(|metadata| metadata.len() == 0)
                .unwrap_or(true)
            {
                failures.push(format!("{}: sips did not create a non-empty PNG", sample));
            } else {
                let png_output = match Command::new("swift")
                    .env("CLANG_MODULE_CACHE_PATH", &module_cache_path)
                    .arg("-e")
                    .arg(PNG_VISIBLE_CONTENT_SWIFT)
                    .arg(&png_path)
                    .output()
                {
                    Ok(output) => output,
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
                    Err(error) => {
                        failures.push(format!("{}: run Swift PNG check failed: {error}", sample));
                        let _ = fs::remove_dir_all(&temp_dir);
                        continue;
                    }
                };
                if !png_output.status.success() {
                    failures.push(format!(
                        "{}: sips PNG visible-content check failed with status {:?}: stdout={} stderr={}",
                        sample,
                        png_output.status.code(),
                        String::from_utf8_lossy(&png_output.stdout),
                        String::from_utf8_lossy(&png_output.stderr)
                    ));
                    let _ = fs::remove_dir_all(&temp_dir);
                    continue;
                }
                if let Some(check) = fixture.sips_region_check {
                    match png_non_white_count_in_ratio_region(
                        &png_path,
                        check.left,
                        check.top,
                        check.right,
                        check.bottom,
                    ) {
                        Ok(non_white) if non_white >= check.min_non_white => {}
                        Ok(non_white) => failures.push(format!(
                            "{}: sips {} rendered too few non-white pixels ({non_white})",
                            sample, check.label
                        )),
                        Err(error) => failures.push(format!(
                            "{}: sips {} check failed: {error}",
                            sample, check.label
                        )),
                    }
                }
                rendered_count += 1;
            }

            let _ = fs::remove_dir_all(&temp_dir);
        }

        assert_eq!(failures, Vec::<String>::new());
        assert!(rendered_count >= 1);
    }

    #[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
    #[test]
    fn local_complex_pdfs_render_visible_content_with_macos_pdfkit_when_available() {
        let sample_dir = local_sample_dir();
        if !sample_dir.exists() {
            return;
        }

        let mut failures = Vec::new();
        let mut rendered_count = 0usize;

        let any_sample_present = LOCAL_PDF_SMOKE_FIXTURES
            .iter()
            .any(|fixture| fixture.source_with_reference_pdf_exists(&sample_dir));
        if !any_sample_present {
            return;
        }

        for fixture in LOCAL_PDF_SMOKE_FIXTURES {
            let sample = fixture.source_name;
            let sample_path = fixture.source_path(&sample_dir);
            if !sample_path.exists() || !sample_path.with_extension("pdf").exists() {
                continue;
            }

            let result = fs::read(&sample_path)
                .map_err(|error| error.to_string())
                .and_then(|bytes| parse_document(&bytes).map_err(|error| error.to_string()))
                .and_then(|document| {
                    to_pdf_with_file_name(&document, &sample_path.to_string_lossy())
                });
            let pdf = match result {
                Ok(pdf) => pdf,
                Err(error) => {
                    failures.push(format!("{}: {error}", sample_path.display()));
                    continue;
                }
            };

            let temp_dir = std::env::temp_dir()
                .join(format!("rjtd-pdfkit-smoke-{}-{sample}", std::process::id()));
            if let Err(error) = fs::create_dir_all(&temp_dir) {
                failures.push(format!("{}: create temp dir failed: {error}", sample));
                continue;
            }
            let pdf_path = temp_dir.join("sample.pdf");
            let module_cache_path = temp_dir.join("swift-module-cache");
            if let Err(error) = fs::create_dir_all(&module_cache_path) {
                failures.push(format!(
                    "{}: create Swift module cache failed: {error}",
                    sample
                ));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }
            if let Err(error) = fs::write(&pdf_path, &pdf) {
                failures.push(format!("{}: write temp pdf failed: {error}", sample));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }

            let mut command = Command::new("swift");
            command
                .env("CLANG_MODULE_CACHE_PATH", &module_cache_path)
                .arg("-e")
                .arg(PDFKIT_VISIBLE_CONTENT_SWIFT)
                .arg(&pdf_path);
            for page_check in fixture.page_checks {
                command.arg(page_check);
            }
            let output = match command.output() {
                Ok(output) => output,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
                Err(error) => {
                    failures.push(format!(
                        "{}: run Swift PDFKit check failed: {error}",
                        sample
                    ));
                    let _ = fs::remove_dir_all(&temp_dir);
                    continue;
                }
            };

            if !output.status.success() {
                failures.push(format!(
                    "{}: PDFKit visible-content check failed with status {:?}: stdout={} stderr={}",
                    sample,
                    output.status.code(),
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            } else {
                rendered_count += 1;
            }

            let _ = fs::remove_dir_all(&temp_dir);
        }

        assert_eq!(failures, Vec::<String>::new());
        assert!(rendered_count >= 1);
    }

    #[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
    #[test]
    fn local_complex_pdfs_render_visible_content_with_macos_coregraphics_when_available() {
        let sample_dir = local_sample_dir();
        if !sample_dir.exists() {
            return;
        }

        let mut failures = Vec::new();
        let mut rendered_count = 0usize;

        let any_sample_present = LOCAL_PDF_SMOKE_FIXTURES
            .iter()
            .any(|fixture| fixture.source_with_reference_pdf_exists(&sample_dir));
        if !any_sample_present {
            return;
        }

        for fixture in LOCAL_PDF_SMOKE_FIXTURES {
            let sample = fixture.source_name;
            let sample_path = fixture.source_path(&sample_dir);
            if !sample_path.exists() || !sample_path.with_extension("pdf").exists() {
                continue;
            }

            let result = fs::read(&sample_path)
                .map_err(|error| error.to_string())
                .and_then(|bytes| parse_document(&bytes).map_err(|error| error.to_string()))
                .and_then(|document| {
                    to_pdf_with_file_name(&document, &sample_path.to_string_lossy())
                });
            let pdf = match result {
                Ok(pdf) => pdf,
                Err(error) => {
                    failures.push(format!("{}: {error}", sample_path.display()));
                    continue;
                }
            };

            let temp_dir = std::env::temp_dir().join(format!(
                "rjtd-coregraphics-smoke-{}-{sample}",
                std::process::id()
            ));
            if let Err(error) = fs::create_dir_all(&temp_dir) {
                failures.push(format!("{}: create temp dir failed: {error}", sample));
                continue;
            }
            let pdf_path = temp_dir.join("sample.pdf");
            let module_cache_path = temp_dir.join("swift-module-cache");
            if let Err(error) = fs::create_dir_all(&module_cache_path) {
                failures.push(format!(
                    "{}: create Swift module cache failed: {error}",
                    sample
                ));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }
            if let Err(error) = fs::write(&pdf_path, &pdf) {
                failures.push(format!("{}: write temp pdf failed: {error}", sample));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }

            let mut command = Command::new("swift");
            command
                .env("CLANG_MODULE_CACHE_PATH", &module_cache_path)
                .arg("-e")
                .arg(COREGRAPHICS_VISIBLE_CONTENT_SWIFT)
                .arg(&pdf_path);
            for page_check in fixture.page_checks {
                command.arg(page_check);
            }
            let output = match command.output() {
                Ok(output) => output,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
                Err(error) => {
                    failures.push(format!(
                        "{}: run Swift CoreGraphics check failed: {error}",
                        sample
                    ));
                    let _ = fs::remove_dir_all(&temp_dir);
                    continue;
                }
            };

            if !output.status.success() {
                failures.push(format!(
                    "{}: CoreGraphics visible-content check failed with status {:?}: stdout={} stderr={}",
                    sample,
                    output.status.code(),
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            } else {
                rendered_count += 1;
            }

            let _ = fs::remove_dir_all(&temp_dir);
        }

        assert_eq!(failures, Vec::<String>::new());
        assert!(rendered_count >= 1);
    }

    #[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
    const PDFKIT_VISIBLE_CONTENT_SWIFT: &str = r#"
import CoreGraphics
import Foundation
import PDFKit

let path = CommandLine.arguments[1]
guard let document = PDFDocument(url: URL(fileURLWithPath: path)) else {
    fputs("PDFKit could not load document\n", stderr)
    exit(2)
}
if document.pageCount == 0 {
    fputs("PDFKit loaded zero pages\n", stderr)
    exit(3)
}

let requestedSpecs = Array(CommandLine.arguments.dropFirst(2))
var pageChecks: [(page: Int, minNonWhite: Int)] = []
if requestedSpecs.isEmpty {
    pageChecks = Array(1...min(document.pageCount, 2)).map { (page: $0, minNonWhite: 1) }
} else {
    for spec in requestedSpecs {
        let parts = spec.split(separator: ":", maxSplits: 1).map(String.init)
        guard let page = Int(parts[0]), page > 0 else {
            fputs("PDFKit invalid page check spec \(spec)\n", stderr)
            exit(4)
        }
        var minNonWhite = 1
        if parts.count == 2 {
            guard let parsedMinNonWhite = Int(parts[1]), parsedMinNonWhite > 0 else {
                fputs("PDFKit invalid minimum non-white spec \(spec)\n", stderr)
                exit(4)
            }
            minNonWhite = parsedMinNonWhite
        }
        pageChecks.append((page: page, minNonWhite: minNonWhite))
    }
}
var totalNonWhite = 0
var pageSummaries: [String] = []
for check in pageChecks {
    let oneBasedPageIndex = check.page
    if oneBasedPageIndex < 1 || oneBasedPageIndex > document.pageCount {
        fputs("PDFKit requested page \(oneBasedPageIndex) outside 1...\(document.pageCount)\n", stderr)
        exit(5)
    }
    let pageIndex = oneBasedPageIndex - 1
    guard let page = document.page(at: pageIndex) else {
        continue
    }
    let box = page.bounds(for: .mediaBox)
    let width = max(1, Int(box.width.rounded(.up)))
    let height = max(1, Int(box.height.rounded(.up)))
    var bytes = [UInt8](repeating: 255, count: width * height * 4)
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    guard let context = CGContext(
        data: &bytes,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: width * 4,
        space: colorSpace,
        bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
    ) else {
        fputs("Could not create CGContext\n", stderr)
        exit(6)
    }
    context.setFillColor(CGColor(red: 1, green: 1, blue: 1, alpha: 1))
    context.fill(CGRect(x: 0, y: 0, width: width, height: height))
    page.draw(with: .mediaBox, to: context)

    var pageNonWhite = 0
    var byteIndex = 0
    while byteIndex < bytes.count {
        if bytes[byteIndex] < 245 || bytes[byteIndex + 1] < 245 || bytes[byteIndex + 2] < 245 {
            pageNonWhite += 1
        }
        byteIndex += 4
    }
    if pageNonWhite < check.minNonWhite {
        fputs("PDFKit rendered \(pageNonWhite) non-white pixels on page \(pageIndex + 1), below minimum \(check.minNonWhite)\n", stderr)
        exit(7)
    }
    totalNonWhite += pageNonWhite
    pageSummaries.append("\(oneBasedPageIndex):\(pageNonWhite)")
}

let checkedSummary = pageSummaries.joined(separator: ",")
print("pages \(document.pageCount) checked \(checkedSummary) nonWhite \(totalNonWhite)")
"#;

    #[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
    const COREGRAPHICS_VISIBLE_CONTENT_SWIFT: &str = r#"
import CoreGraphics
import Foundation

let path = CommandLine.arguments[1]
let url = URL(fileURLWithPath: path) as CFURL
guard let document = CGPDFDocument(url) else {
    fputs("CGPDFDocument could not load document\n", stderr)
    exit(2)
}
let pageCount = document.numberOfPages
if pageCount == 0 {
    fputs("CGPDFDocument loaded zero pages\n", stderr)
    exit(3)
}

let requestedSpecs = Array(CommandLine.arguments.dropFirst(2))
var pageChecks: [(page: Int, minNonWhite: Int)] = []
if requestedSpecs.isEmpty {
    pageChecks = Array(1...min(pageCount, 2)).map { (page: $0, minNonWhite: 1) }
} else {
    for spec in requestedSpecs {
        let parts = spec.split(separator: ":", maxSplits: 1).map(String.init)
        guard let page = Int(parts[0]), page > 0 else {
            fputs("CoreGraphics invalid page check spec \(spec)\n", stderr)
            exit(4)
        }
        var minNonWhite = 1
        if parts.count == 2 {
            guard let parsedMinNonWhite = Int(parts[1]), parsedMinNonWhite > 0 else {
                fputs("CoreGraphics invalid minimum non-white spec \(spec)\n", stderr)
                exit(4)
            }
            minNonWhite = parsedMinNonWhite
        }
        pageChecks.append((page: page, minNonWhite: minNonWhite))
    }
}
var totalNonWhite = 0
var pageSummaries: [String] = []
for check in pageChecks {
    let pageIndex = check.page
    if pageIndex < 1 || pageIndex > pageCount {
        fputs("CoreGraphics requested page \(pageIndex) outside 1...\(pageCount)\n", stderr)
        exit(5)
    }
    guard let page = document.page(at: pageIndex) else {
        continue
    }
    let box = page.getBoxRect(.mediaBox)
    let width = max(1, Int(box.width.rounded(.up)))
    let height = max(1, Int(box.height.rounded(.up)))
    var bytes = [UInt8](repeating: 255, count: width * height * 4)
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    guard let context = CGContext(
        data: &bytes,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: width * 4,
        space: colorSpace,
        bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
    ) else {
        fputs("Could not create CGContext\n", stderr)
        exit(6)
    }
    context.setFillColor(CGColor(red: 1, green: 1, blue: 1, alpha: 1))
    context.fill(CGRect(x: 0, y: 0, width: width, height: height))
    context.drawPDFPage(page)

    var pageNonWhite = 0
    var byteIndex = 0
    while byteIndex < bytes.count {
        if bytes[byteIndex] < 245 || bytes[byteIndex + 1] < 245 || bytes[byteIndex + 2] < 245 {
            pageNonWhite += 1
        }
        byteIndex += 4
    }
    if pageNonWhite < check.minNonWhite {
        fputs("CoreGraphics rendered \(pageNonWhite) non-white pixels on page \(pageIndex), below minimum \(check.minNonWhite)\n", stderr)
        exit(7)
    }
    totalNonWhite += pageNonWhite
    pageSummaries.append("\(pageIndex):\(pageNonWhite)")
}

let checkedSummary = pageSummaries.joined(separator: ",")
print("pages \(pageCount) checked \(checkedSummary) nonWhite \(totalNonWhite)")
"#;

    #[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
    const PNG_VISIBLE_CONTENT_SWIFT: &str = r#"
import CoreGraphics
import Foundation
import ImageIO

let path = CommandLine.arguments[1]
let url = URL(fileURLWithPath: path) as CFURL
guard let source = CGImageSourceCreateWithURL(url, nil),
      let image = CGImageSourceCreateImageAtIndex(source, 0, nil) else {
    fputs("Could not load PNG image\n", stderr)
    exit(2)
}
let width = image.width
let height = image.height
if width == 0 || height == 0 {
    fputs("PNG image has zero size\n", stderr)
    exit(3)
}
var bytes = [UInt8](repeating: 255, count: width * height * 4)
let colorSpace = CGColorSpaceCreateDeviceRGB()
guard let context = CGContext(
    data: &bytes,
    width: width,
    height: height,
    bitsPerComponent: 8,
    bytesPerRow: width * 4,
    space: colorSpace,
    bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
) else {
    fputs("Could not create CGContext\n", stderr)
    exit(4)
}
context.setFillColor(CGColor(red: 1, green: 1, blue: 1, alpha: 1))
context.fill(CGRect(x: 0, y: 0, width: width, height: height))
context.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))

var nonWhite = 0
var byteIndex = 0
while byteIndex < bytes.count {
    if bytes[byteIndex] < 245 || bytes[byteIndex + 1] < 245 || bytes[byteIndex + 2] < 245 {
        nonWhite += 1
    }
    byteIndex += 4
}
print("png \(width)x\(height) nonWhite \(nonWhite)")
if nonWhite == 0 {
    fputs("PNG rendered no visible non-white pixels\n", stderr)
    exit(5)
}
"#;

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn local_samples_export_to_valid_pdf_when_available() {
        let sample_dir = local_sample_dir();
        if !sample_dir.exists() {
            return;
        }

        let mut paths = fs::read_dir(&sample_dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(|extension| matches!(extension, "jtd" | "jtt" | "jttc"))
                    && path.with_extension("pdf").exists()
            })
            .collect::<Vec<_>>();
        paths.sort();
        if paths.is_empty() {
            return;
        }

        let mut failures = Vec::new();
        let mut pdf_count = 0usize;
        let mut total_pdf_bytes = 0usize;

        for path in &paths {
            let result = fs::read(path)
                .map_err(|error| error.to_string())
                .and_then(|bytes| parse_document(&bytes).map_err(|error| error.to_string()))
                .and_then(|document| to_pdf_with_file_name(&document, &path.to_string_lossy()));

            match result {
                Ok(pdf) => {
                    if !pdf.starts_with(b"%PDF-") {
                        failures.push(format!("{}: missing PDF header", path.display()));
                    }
                    if !pdf.windows(5).any(|window| window == b"/Page") {
                        failures.push(format!("{}: missing /Page marker", path.display()));
                    }
                    if !pdf.windows(5).any(|window| window == b"%%EOF") {
                        failures.push(format!("{}: missing EOF marker", path.display()));
                    }
                    if pdf.len() < 512 {
                        failures.push(format!("{}: suspiciously small PDF", path.display()));
                    }
                    if !pdf.windows(10).any(|window| window == b"/ToUnicode") {
                        failures.push(format!("{}: missing ToUnicode text map", path.display()));
                    }
                    if !pdf.windows(12).any(|window| window == b"/CIDFontType") {
                        failures.push(format!("{}: missing CID font resource", path.display()));
                    }
                    let form_xobject_count = pdf_byte_pattern_count(&pdf, b"/Subtype /Form");
                    let form_type_count = pdf_byte_pattern_count(&pdf, b"/FormType 1");
                    if form_xobject_count == 0 {
                        failures.push(format!("{}: missing Form XObject wrapper", path.display()));
                    }
                    if form_type_count != form_xobject_count {
                        failures.push(format!(
                            "{}: Form XObject /FormType coverage mismatch ({form_type_count}/{form_xobject_count})",
                            path.display()
                        ));
                    }
                    let preview_safety_issues = pdf_preview_blocking_issues(&pdf);
                    if !preview_safety_issues.is_empty() {
                        failures.push(format!(
                            "{}: Preview/PDFKit risky PDF constructs: {}",
                            path.display(),
                            preview_safety_issues.join(", ")
                        ));
                    }
                    if path
                        .file_name()
                        .and_then(|value| value.to_str())
                        .is_some_and(|file_name| file_name == "a6.jtd")
                    {
                        let page_object_count = pdf_page_object_count(&pdf);
                        if page_object_count != 114 {
                            failures.push(format!(
                                "{}: expected 114 PDF page objects, got {page_object_count}",
                                path.display()
                            ));
                        }
                        if !pdf.windows(10).any(|window| window == b"/Count 114") {
                            failures.push(format!("{}: missing /Count 114", path.display()));
                        }
                        if pdf_byte_pattern_count(&pdf, b"/MediaBox [0 0 297.675") != 114 {
                            failures.push(format!(
                                "{}: A6 portrait MediaBox does not cover all pages",
                                path.display()
                            ));
                        }
                    }
                    pdf_count += 1;
                    total_pdf_bytes += pdf.len();
                }
                Err(error) => failures.push(format!("{}: {error}", path.display())),
            }
        }

        assert_eq!(failures, Vec::<String>::new());
        assert!(pdf_count >= 1);
        assert!(total_pdf_bytes > pdf_count * 512);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn local_pdf_output_artifacts_have_preview_compatible_form_xobjects_when_available() {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let sample_dir = project_root.join("rjtd-testdata/local-samples");
        let output_dir = project_root.join("openjtd-samples/pdf-output");
        if !sample_dir.exists() || !output_dir.exists() {
            return;
        }

        let mut paths = fs::read_dir(&sample_dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(|extension| matches!(extension, "jtd" | "jtt" | "jttc"))
            })
            .collect::<Vec<_>>();
        paths.sort();

        let mut failures = Vec::new();
        let official_output_stems = paths
            .iter()
            .filter_map(|path| path.file_stem().and_then(|value| value.to_str()))
            .collect::<BTreeSet<_>>();
        let mut output_pdfs = fs::read_dir(&output_dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(|extension| extension == "pdf")
            })
            .collect::<Vec<_>>();
        output_pdfs.sort();
        for pdf_path in &output_pdfs {
            let Some(stem) = pdf_path.file_stem().and_then(|value| value.to_str()) else {
                continue;
            };
            if !official_output_stems.contains(stem) {
                failures.push(format!(
                    "{}: unexpected auxiliary PDF output; only exact same-stem sample PDFs are official artifacts",
                    pdf_path.display()
                ));
            }
        }
        let mut checked_count = 0usize;
        for path in &paths {
            let Some(stem) = path.file_stem().and_then(|value| value.to_str()) else {
                continue;
            };
            let pdf_path = output_dir.join(format!("{stem}.pdf"));
            let pdf = match fs::read(&pdf_path) {
                Ok(pdf) => pdf,
                Err(error) => {
                    failures.push(format!("{}: {error}", pdf_path.display()));
                    continue;
                }
            };

            if !pdf.starts_with(b"%PDF-") {
                failures.push(format!("{}: missing PDF header", pdf_path.display()));
            }
            if count_pdf_eof_markers(&pdf) != 1 {
                failures.push(format!(
                    "{}: expected one EOF marker, got {}",
                    pdf_path.display(),
                    count_pdf_eof_markers(&pdf)
                ));
            }
            let form_xobject_count = pdf_byte_pattern_count(&pdf, b"/Subtype /Form");
            let form_type_count = pdf_byte_pattern_count(&pdf, b"/FormType 1");
            if form_xobject_count == 0 {
                failures.push(format!(
                    "{}: missing Form XObject wrapper",
                    pdf_path.display()
                ));
            }
            if form_type_count != form_xobject_count {
                failures.push(format!(
                    "{}: Form XObject /FormType coverage mismatch ({form_type_count}/{form_xobject_count})",
                    pdf_path.display()
                ));
            }
            let preview_safety_issues = pdf_preview_blocking_issues(&pdf);
            if !preview_safety_issues.is_empty() {
                failures.push(format!(
                    "{}: Preview/PDFKit risky PDF constructs: {}",
                    pdf_path.display(),
                    preview_safety_issues.join(", ")
                ));
            }
            let reference_pdf_path = sample_dir.join(format!("{stem}.pdf"));
            if reference_pdf_path.exists() && local_reference_pdf_page_count_is_trusted(stem) {
                let reference_pdf = match fs::read(&reference_pdf_path) {
                    Ok(reference_pdf) => reference_pdf,
                    Err(error) => {
                        failures.push(format!("{}: {error}", reference_pdf_path.display()));
                        continue;
                    }
                };
                let reference_page_count = pdf_page_object_count(&reference_pdf);
                let output_page_count = pdf_page_object_count(&pdf);
                let known_divergence = local_reference_known_divergence(stem);
                if reference_page_count == 0 {
                    failures.push(format!(
                        "{}: could not derive reference PDF page count",
                        reference_pdf_path.display()
                    ));
                } else if let Some(divergence) = known_divergence {
                    if reference_page_count != divergence.expected_reference_page_count
                        || output_page_count != divergence.expected_output_page_count
                    {
                        failures.push(format!(
                            "{}: known PDF page-count divergence lock changed ({reason}); expected reference/output {expected_reference}/{expected_output}, got {reference_page_count}/{output_page_count}; refresh the rjtd-export known-divergence lock",
                            pdf_path.display(),
                            reason = divergence.page_count_reason,
                            expected_reference = divergence.expected_reference_page_count,
                            expected_output = divergence.expected_output_page_count
                        ));
                    }
                } else if output_page_count != reference_page_count {
                    failures.push(format!(
                        "{}: expected {reference_page_count} PDF page objects to match {}, got {output_page_count}",
                        pdf_path.display(),
                        reference_pdf_path.display()
                    ));
                }

                let reference_media_boxes = pdf_media_box_sizes(&reference_pdf);
                let output_media_boxes = pdf_media_box_sizes(&pdf);
                let Some(reference_media_box) = reference_media_boxes.first().copied() else {
                    failures.push(format!(
                        "{}: could not derive reference MediaBox",
                        reference_pdf_path.display()
                    ));
                    continue;
                };
                if output_media_boxes.is_empty() {
                    failures.push(format!(
                        "{}: could not derive output MediaBox",
                        pdf_path.display()
                    ));
                }
                if output_page_count != 0 && output_media_boxes.len() != output_page_count {
                    failures.push(format!(
                        "{}: expected {output_page_count} MediaBox entries, got {}",
                        pdf_path.display(),
                        output_media_boxes.len()
                    ));
                }
                if let Some(media_box_divergence) =
                    known_divergence.and_then(|divergence| divergence.media_box_divergence)
                {
                    if !reference_media_box
                        .close_to(media_box_divergence.expected_reference_media_box)
                    {
                        failures.push(format!(
                            "{}: known reference MediaBox divergence lock changed ({reason}); expected {:.3}x{:.3}, got {:.3}x{:.3}; refresh the rjtd-export known-divergence lock",
                            reference_pdf_path.display(),
                            media_box_divergence.expected_reference_media_box.width,
                            media_box_divergence.expected_reference_media_box.height,
                            reference_media_box.width,
                            reference_media_box.height,
                            reason = media_box_divergence.reason
                        ));
                    }
                    for (page_index, output_media_box) in output_media_boxes.iter().enumerate() {
                        if !output_media_box
                            .close_to(media_box_divergence.expected_output_media_box)
                        {
                            failures.push(format!(
                                "{}: known output page {} MediaBox divergence lock changed ({reason}); expected {:.3}x{:.3}, got {:.3}x{:.3}; refresh the rjtd-export known-divergence lock",
                                pdf_path.display(),
                                page_index + 1,
                                media_box_divergence.expected_output_media_box.width,
                                media_box_divergence.expected_output_media_box.height,
                                output_media_box.width,
                                output_media_box.height,
                                reason = media_box_divergence.reason
                            ));
                        }
                    }
                } else {
                    for (page_index, output_media_box) in output_media_boxes.iter().enumerate() {
                        let expected_media_box = reference_media_boxes
                            .get(page_index)
                            .copied()
                            .unwrap_or(reference_media_box);
                        if !output_media_box.close_to(expected_media_box) {
                            failures.push(format!(
                                "{}: page {} MediaBox {:.3}x{:.3} does not match trusted reference {:.3}x{:.3}",
                                pdf_path.display(),
                                page_index + 1,
                                output_media_box.width,
                                output_media_box.height,
                                expected_media_box.width,
                                expected_media_box.height
                            ));
                        }
                    }
                }
            }
            checked_count += 1;
        }

        assert_eq!(failures, Vec::<String>::new());
        assert!(checked_count >= 1);
    }

    #[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
    #[test]
    fn local_pdf_output_artifacts_render_visible_content_with_macos_pdfkit_when_available() {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let output_dir = project_root.join("openjtd-samples/pdf-output");
        if !output_dir.exists() {
            return;
        }

        let mut failures = Vec::new();
        let mut rendered_count = 0usize;

        for fixture in LOCAL_PDF_SMOKE_FIXTURES {
            let sample = fixture.output_pdf_name;
            let pdf_path = fixture.output_pdf_path(&output_dir);
            if !pdf_path.exists() {
                continue;
            }

            let temp_dir = std::env::temp_dir().join(format!(
                "rjtd-output-pdfkit-smoke-{}-{sample}",
                std::process::id()
            ));
            if let Err(error) = fs::create_dir_all(&temp_dir) {
                failures.push(format!("{}: create temp dir failed: {error}", sample));
                continue;
            }
            let module_cache_path = temp_dir.join("swift-module-cache");
            if let Err(error) = fs::create_dir_all(&module_cache_path) {
                failures.push(format!(
                    "{}: create Swift module cache failed: {error}",
                    sample
                ));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }

            let mut command = Command::new("swift");
            command
                .env("CLANG_MODULE_CACHE_PATH", &module_cache_path)
                .arg("-e")
                .arg(PDFKIT_VISIBLE_CONTENT_SWIFT)
                .arg(&pdf_path);
            for page_check in fixture.page_checks {
                command.arg(page_check);
            }
            let output = match command.output() {
                Ok(output) => output,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
                Err(error) => {
                    failures.push(format!(
                        "{}: run Swift PDFKit check failed: {error}",
                        pdf_path.display()
                    ));
                    let _ = fs::remove_dir_all(&temp_dir);
                    continue;
                }
            };

            if !output.status.success() {
                failures.push(format!(
                    "{}: PDFKit visible-content check failed with status {:?}: stdout={} stderr={}",
                    pdf_path.display(),
                    output.status.code(),
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            } else {
                rendered_count += 1;
            }

            let _ = fs::remove_dir_all(&temp_dir);
        }

        assert_eq!(failures, Vec::<String>::new());
        assert!(rendered_count >= 1);
    }

    #[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
    #[test]
    fn local_pdf_output_success_data_test_title_rasterizes_with_macos_sips_when_available() {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let pdf_path = project_root
            .join("openjtd-samples/pdf-output")
            .join("ichitaro-20030228030923-success-002-success_data-test.pdf");
        if !pdf_path.exists() {
            return;
        }

        let temp_dir = std::env::temp_dir().join(format!(
            "rjtd-output-title-sips-smoke-{}",
            std::process::id()
        ));
        fs::create_dir_all(&temp_dir).unwrap();
        let png_path = temp_dir.join("success-data-test-page1.png");

        let output = match Command::new("sips")
            .arg("-s")
            .arg("format")
            .arg("png")
            .arg(&pdf_path)
            .arg("--out")
            .arg(&png_path)
            .output()
        {
            Ok(output) => output,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
            Err(error) => panic!("run sips failed: {error}"),
        };
        assert!(
            output.status.success(),
            "sips failed with status {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        );

        let title_non_white =
            png_non_white_count_in_ratio_region(&png_path, 0.05, 0.07, 0.92, 0.20)
                .expect("sips title region should be readable");
        let _ = fs::remove_dir_all(&temp_dir);

        assert!(
            title_non_white >= 3_000,
            "sips-rendered title region has too few non-white pixels: {title_non_white}"
        );
    }

    fn local_reference_pdf_page_count_is_trusted(stem: &str) -> bool {
        // The local 46.pdf reference is a known suspicious capture: it renders as
        // essentially blank/title-like while 46.jtd contains the Ginga body text.
        // Keep it out of full-document page-count gates until the sample is replaced.
        stem != "46"
    }

    fn local_reference_known_divergence(stem: &str) -> Option<LocalReferencePdfKnownDivergence> {
        const SEMINAR2004_REFERENCE_LANDSCAPE: PdfMediaBox = PdfMediaBox {
            width: 841.890,
            height: 595.276,
        };
        const SEMINAR2004_OUTPUT_PORTRAIT: PdfMediaBox = PdfMediaBox {
            width: 595.275,
            height: 841.875,
        };
        const SEMINAR2004_MEDIA_BOX_DIVERGENCE: LocalReferencePdfKnownMediaBoxDivergence =
            LocalReferencePdfKnownMediaBoxDivergence {
                expected_reference_media_box: SEMINAR2004_REFERENCE_LANDSCAPE,
                expected_output_media_box: SEMINAR2004_OUTPUT_PORTRAIT,
                reason: LOCAL_REFERENCE_PAPER_ORIENTATION_SOURCE_DECODE_UNPROVEN,
            };
        const KNOWN_DIVERGENCES: &[(&str, LocalReferencePdfKnownDivergence)] = &[
            (
                "ichitaro-20030316043238-success-001-success_data-iwata_file",
                LocalReferencePdfKnownDivergence::pagination(13, 24),
            ),
            (
                "ichitaro-20030316045013-success-002-success_data-resume",
                LocalReferencePdfKnownDivergence::pagination(3, 6),
            ),
            (
                "ichitaro-20030415170937-success-001-success_data-fujimoto_file",
                LocalReferencePdfKnownDivergence::pagination(3, 1),
            ),
            (
                "ichitaro-20030422193925-success-003-success_data-christmas_2001",
                LocalReferencePdfKnownDivergence::pagination(1, 2),
            ),
            (
                "ichitaro-20030422194039-success-003-success_data-syokuhin",
                LocalReferencePdfKnownDivergence::pagination(1, 26),
            ),
            (
                "ichitaro-20030422210439-success-002-success_data-natsu",
                LocalReferencePdfKnownDivergence::pagination(2, 1),
            ),
            (
                "ichitaro-20030706234132-success-004-success_data-asobinin_24",
                LocalReferencePdfKnownDivergence::pagination(1, 2),
            ),
            (
                "ichitaro-20041103142937-seminar2004-part2_1-img-shortcutkey1",
                LocalReferencePdfKnownDivergence::pagination_with_media_box(
                    1,
                    4,
                    SEMINAR2004_MEDIA_BOX_DIVERGENCE,
                ),
            ),
            (
                "ichitaro-20041103143104-seminar2004-part2_2-img-shortcutkey2",
                LocalReferencePdfKnownDivergence::pagination_with_media_box(
                    1,
                    4,
                    SEMINAR2004_MEDIA_BOX_DIVERGENCE,
                ),
            ),
            (
                "ichitaro-20050214114830-seminar2004-part2_3-img-toolbox",
                LocalReferencePdfKnownDivergence::pagination_with_media_box(
                    1,
                    2,
                    SEMINAR2004_MEDIA_BOX_DIVERGENCE,
                ),
            ),
            (
                "ichitaro-20050214115206-seminar2004-part2_3-img-shortcutkey3",
                LocalReferencePdfKnownDivergence::pagination_with_media_box(
                    1,
                    3,
                    SEMINAR2004_MEDIA_BOX_DIVERGENCE,
                ),
            ),
        ];

        // These trusted local reference PDFs expose source semantics the current
        // fallback renderer has not decoded yet. Keep exact counts locked so a
        // regenerated artifact or replaced reference must refresh this evidence.
        KNOWN_DIVERGENCES
            .iter()
            .find_map(|(known_stem, divergence)| (*known_stem == stem).then_some(*divergence))
    }

    fn pdf_page_object_count(pdf: &[u8]) -> usize {
        // Reference captures vary in name-token whitespace (`/Type /Page\n` from
        // this exporter, `/Type/Page` from external capture tools), so count
        // `/Type` + optional whitespace + `/Page` followed by a PDF delimiter.
        // The delimiter check keeps `/Type/Pages` tree nodes out of the count.
        let mut count = 0usize;
        let mut position = 0usize;
        while let Some(relative_offset) = find_subslice(&pdf[position..], b"/Type") {
            let type_offset = position + relative_offset;
            position = type_offset + b"/Type".len();
            let mut cursor = pdf_skip_whitespace(pdf, position);
            if !pdf
                .get(cursor..)
                .is_some_and(|tail| tail.starts_with(b"/Page"))
            {
                continue;
            }
            cursor += b"/Page".len();
            let next_is_delimiter = match pdf.get(cursor) {
                None => true,
                Some(byte) => matches!(
                    byte,
                    b'\0'
                        | b'\t'
                        | b'\n'
                        | b'\x0c'
                        | b'\r'
                        | b' '
                        | b'/'
                        | b'<'
                        | b'>'
                        | b'['
                        | b']'
                        | b'('
                        | b')'
                        | b'{'
                        | b'}'
                        | b'%'
                ),
            };
            if next_is_delimiter {
                count += 1;
            }
        }
        count
    }

    #[test]
    fn pdf_page_object_count_handles_reference_capture_serialization_variants() {
        assert_eq!(pdf_page_object_count(b"<< /Type /Page\n >>"), 1);
        assert_eq!(pdf_page_object_count(b"<</Type/Page/Parent 2 0 R>>"), 1);
        assert_eq!(pdf_page_object_count(b"<</Type/Pages/Count 3>>"), 0);
        assert_eq!(
            pdf_page_object_count(b"<</Type/Pages/Kids[3 0 R]>><</Type/Page>><< /Type /Page\n>>"),
            2
        );
        assert_eq!(pdf_page_object_count(b"/Type /PageLabels"), 0);
        assert_eq!(pdf_page_object_count(b"<</Type/Page%comment\n>>"), 1);
    }

    fn pdf_byte_pattern_count(pdf: &[u8], pattern: &[u8]) -> usize {
        pdf.windows(pattern.len())
            .filter(|window| *window == pattern)
            .count()
    }

    #[test]
    fn exports_json_from_document_model() {
        let paragraph = Paragraph::new(vec![Inline::Text(TextRun::new("hello\n\"", None))], None);
        let document = Document::new(
            Metadata::new(Some("sample".to_string())),
            vec![Block::Paragraph(paragraph)],
        );

        assert_eq!(
            to_json(&document),
            "{\"metadata\":{\"title\":\"sample\"},\"blocks\":[{\"type\":\"paragraph\",\"style\":null,\"inlines\":[{\"type\":\"text\",\"text\":\"hello\\n\\\"\",\"style\":null}]}],\"unknownStyles\":[],\"unknownObjects\":[],\"objectStreamCandidates\":[],\"objectFrameRecords\":[],\"objectEmbeddingFrames\":[],\"textCountRanges\":[],\"textControlBoundaries\":[],\"textBoundaryCandidates\":[],\"textParagraphBoundaryCandidates\":[],\"tableCandidates\":[],\"autoTextCandidates\":[],\"tocEntries\":[],\"pageMarks\":[],\"paperMarks\":[],\"rawStreams\":[],\"fonts\":[]}"
        );
    }

    #[test]
    fn exports_paragraph_style_reference_to_json() {
        let paragraph = Paragraph::new(
            vec![Inline::Text(TextRun::new("styled", None))],
            Some(StyleRef::new("1")),
        );
        let document = Document::new(Metadata::default(), vec![Block::Paragraph(paragraph)]);

        let json = to_json(&document);

        assert!(json.contains("\"style\":{\"id\":\"1\"}"));
    }

    #[test]
    fn exports_text_source_span_to_json_when_available() {
        let paragraph = Paragraph::new(
            vec![Inline::Text(TextRun::with_source_span(
                "銀河",
                None,
                Some(TextSourceSpan::new(10, 14, 5, 7)),
            ))],
            None,
        );
        let document = Document::new(Metadata::default(), vec![Block::Paragraph(paragraph)]);

        let json = to_json(&document);

        assert!(json.contains(
            "\"sourceSpan\":{\"byteStart\":10,\"byteEnd\":14,\"unitStart\":5,\"unitEnd\":7}"
        ));
    }

    #[test]
    fn exports_text_control_boundaries_to_json() {
        let mut document = Document::default();
        document.push_text_control_boundary(TextControlBoundary::new(
            0,
            0x001c,
            Some(TextSourceSpan::new(6, 8, 3, 4)),
        ));

        let json = to_json(&document);

        assert!(json.contains("\"textControlBoundaries\":[{"));
        assert!(json.contains("\"code\":28"));
        assert!(json.contains("\"codeHex\":\"0x001c\""));
        assert!(json.contains(
            "\"sourceSpan\":{\"byteStart\":6,\"byteEnd\":8,\"unitStart\":3,\"unitEnd\":4}"
        ));
        assert!(json.contains("\"decoded\":false"));
    }

    #[test]
    fn exports_ruby_inline_as_visible_base_with_preserved_annotation() {
        let annotation_source = UnknownObject::new(UnknownRecordKind::new(Some(0x001d)), vec![1]);
        let ruby = RubyAnnotation::new("午后", "ごご", 0x0082, annotation_source);
        let paragraph = Paragraph::new(
            vec![
                Inline::Text(TextRun::new("一、", None)),
                Inline::Ruby(ruby),
                Inline::Text(TextRun::new("の授業", None)),
            ],
            None,
        );
        let document = Document::new(Metadata::default(), vec![Block::Paragraph(paragraph)]);

        assert_eq!(to_plain_text(&document), "一、午后の授業\n");
        assert_eq!(to_markdown(&document), "一、午后の授業\n\n");

        let json = to_json(&document);
        assert!(json.contains("\"type\":\"ruby\""));
        assert!(json.contains("\"baseText\":\"午后\""));
        assert!(json.contains("\"annotationText\":\"ごご\""));
        assert!(json.contains("\"annotationSelector\":130"));
        assert!(json.contains("\"payloadHex\":\"01\""));
    }

    #[test]
    fn exports_unknown_blocks_to_json_without_dropping_payload() {
        let unknown = UnknownBlock::new(UnknownRecordKind::new(Some(7)), vec![1, 2, 255]);
        let document = Document::new(Metadata::default(), vec![Block::Unknown(unknown)]);

        assert!(to_json(&document).contains("\"payloadHex\":\"0102ff\""));
    }

    #[test]
    fn exports_unknown_style_stream_name_to_json() {
        let mut document = Document::from_plain_text("hello");
        document.push_unknown_style(UnknownStyle::from_stream("/TextLayoutStyle", vec![1, 2, 3]));

        let json = to_json(&document);

        assert!(json.contains("\"unknownStyles\":[{\"name\":\"/TextLayoutStyle\""));
        assert!(json.contains("\"family\":\"unknown\""));
        assert!(json.contains("\"headerU32Be\":[]"));
        assert!(json.contains("\"recordLayout\":\"none\""));
        assert!(json.contains("\"recordCount\":0"));
        assert!(json.contains("\"records\":[]"));
        assert!(json.contains("\"payloadHex\":\"010203\""));
    }

    #[test]
    fn exports_raw_stream_summary_to_json() {
        let mut document = Document::from_plain_text("hello");
        document.push_raw_stream(RawStream::new("/DocumentText", vec![1, 2, 3]));

        assert!(
            to_json(&document).contains("\"rawStreams\":[{\"name\":\"/DocumentText\",\"size\":3}]")
        );
    }

    #[test]
    fn exports_object_stream_candidates_to_json() {
        let mut document = Document::from_plain_text("hello");
        document.push_object_stream_candidate(ObjectStreamCandidate::new(
            "/EmbedItems/Embedding 1/Contents",
            12,
            ObjectStreamCandidateEvidence::new(
                vec![
                    ObjectStreamCandidateReason::ObjectPath,
                    ObjectStreamCandidateReason::ImageSignature,
                ],
                vec![ObjectImageSignatureHit::new("jpeg", 4)],
                vec![ObjectImagePayloadSpan::new(
                    "jpeg",
                    "image/jpeg",
                    ObjectImagePayloadLocation::new(4, 4, 11),
                    true,
                    b"\xff\xd8\xffda\xff\xd9".to_vec(),
                    ObjectImagePayloadEnvelope::new(
                        0,
                        4,
                        11,
                        12,
                        Some(ObjectImageDeclaredLengthCandidate::new(0, 7, "le32")),
                        vec![7, 0, 0, 0],
                        vec![0],
                    ),
                )],
                None,
                vec![],
                vec![8],
            ),
            vec![0x09, 0x00, 0x01, 0x00],
        ));
        document.push_object_stream_candidate(ObjectStreamCandidate::new(
            "/VisualList",
            19,
            ObjectStreamCandidateEvidence::new(
                vec![ObjectStreamCandidateReason::VisualListPath],
                vec![],
                vec![],
                None,
                vec![],
                vec![],
            ),
            b"BMDV visual payl".to_vec(),
        ));

        let json = to_json(&document);

        assert!(json.contains(
            "\"objectStreamCandidates\":[{\"path\":\"/EmbedItems/Embedding 1/Contents\""
        ));
        assert!(json.contains("\"reasons\":[\"object-path\",\"image-signature\"]"));
        assert!(json.contains("\"ownershipCandidate\":{\"basis\":\"stream-path\",\"family\":\"embed-items\",\"storagePath\":\"/EmbedItems/Embedding 1\",\"embeddingIndex\":1,\"streamRole\":\"contents\",\"decoded\":false}"));
        assert!(json.contains("\"ownershipReferences\":[]"));
        assert!(json.contains("\"frameReferenceRows\":[]"));
        assert!(json.contains("\"fdmIndexEntries\":[]"));
        assert!(json.contains("\"imageSignatures\":[{\"kind\":\"jpeg\",\"offset\":4}]"));
        assert!(json.contains("\"imagePayloads\":[{\"kind\":\"jpeg\",\"mime\":\"image/jpeg\",\"signatureOffset\":4,\"start\":4,\"end\":11,\"length\":7,\"complete\":true"));
        assert!(json.contains("\"objectEnvelope\":{\"headerStart\":0"));
        assert!(json.contains("\"headerEnd\":4"));
        assert!(json.contains("\"headerPrefixHex\":\"07000000\""));
        assert!(json.contains("\"headerFields\""));
        assert!(json.contains("\"u16LePrefix\":[{\"offset\":0,\"value\":7}"));
        assert!(json.contains("\"u32LePrefix\":[{\"offset\":0,\"value\":7}]"));
        assert!(json.contains("\"sourcePathCandidate\":null"));
        assert!(json.contains("\"trailerStart\":11"));
        assert!(json.contains("\"trailerPrefixHex\":\"00\""));
        assert!(json.contains("\"declaredPayloadLength\":7"));
        assert!(json.contains("\"declaredPayloadLengthOffset\":0"));
        assert!(json.contains("\"declaredPayloadLengthEndian\":\"le32\""));
        assert!(json.contains("\"payloadPrefixHex\":\"ffd8ff6461ffd9\",\"decoded\":false}]"));
        assert!(json.contains("\"soOffsets\":[8]"));
        assert!(json.contains("\"payloadPrefixHex\":\"09000100\""));
        assert!(
            json.contains(
                "{\"path\":\"/VisualList\",\"size\":19,\"reasons\":[\"visual-list-path\"]"
            )
        );
        assert!(json.contains("\"payloadPrefixHex\":\"424d44562076697375616c207061796c\""));
        assert!(json.contains("\"decoded\":false"));
    }

    #[test]
    fn local_fax02_exports_visual_list_metadata_to_json_when_reference_pdf_is_available() {
        let sample_dir = local_sample_dir();
        let sample_path = sample_dir.join("fax02.jtt");
        let reference_pdf_path = sample_dir.join("fax02.pdf");
        if !sample_path.exists() || !reference_pdf_path.exists() {
            return;
        }

        let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
        let json = to_json(&document);

        assert!(json.contains("\"path\":\"/VisualList\""));
        assert!(json.contains("\"reasons\":[\"visual-list-path\"]"));
        assert!(json.contains("\"visualList\":{\"format\":\"BMDV\""));
        assert!(json.contains("\"declaredSize\":2296"));
        assert!(json.contains("\"width\":120"));
        assert!(json.contains("\"height\":169"));
        assert!(json.contains("\"rleDataLength\":2216"));
        assert!(json.contains("\"pixelCount\":20280"));
        assert!(json.contains("\"rleEncoding\":\"bmp-rle8-like\""));
    }

    #[test]
    fn local_a5_exports_toc_page_label_candidates_when_reference_pdf_is_available() {
        let sample_dir = local_sample_dir();
        let sample_path = sample_dir.join("a5.jtd");
        let reference_pdf_path = sample_dir.join("a5.pdf");
        if !sample_path.exists() || !reference_pdf_path.exists() {
            return;
        }

        let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
        let json = to_json(&document);

        assert!(json.contains("\"tocEntries\":["));
        assert!(json.contains("\"title\":\"一、午后の授業\""));
        assert!(json.contains("\"pageLabel\":\"6\""));
        assert!(json.contains("\"title\":\"九、ジョバンニの切符\""));
        assert!(json.contains("\"pageLabel\":\"42\""));
        assert!(json.contains("\"pageMarks\":["));
        assert!(json.contains("\"sourceStream\":\"/PageMark\""));
        assert!(json.contains("\"family\":\"fixed84\""));
        assert!(json.contains("\"headerCount\":74"));
        assert!(json.contains("\"entryCount\":75"));
        assert!(json.contains("\"lineStart\":23"));
        assert!(json.contains("\"lineEnd\":40"));
        assert!(json.contains("\"paperMarks\":["));
        assert!(json.contains("\"sourceStream\":\"/PaperMark\""));
        assert!(json.contains("\"headerCount\":74"));
        assert!(json.contains("\"headerStride\":12"));
        assert!(json.contains("\"entryCount\":75"));
        assert!(json.contains("\"flagsHex\":\"0x00010010\""));
        assert!(json.contains("\"decoded\":false"));
    }

    #[test]
    fn local_tsaiten_exports_page_mark_u16_subrecord_candidates_when_reference_pdf_is_available() {
        let sample_dir = local_sample_dir();
        let sample_path = sample_dir.join("ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd");
        let reference_pdf_path = sample_dir.join("ichitaro-20030120132956-0007-sp-dat-tsaiten.pdf");
        if !sample_path.exists() || !reference_pdf_path.exists() {
            return;
        }

        let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
        let json = to_json(&document);

        assert!(json.contains("\"family\":\"count-plus-one-variable\""));
        assert!(json.contains(
            "\"u16SubrecordScan\":{\"source\":\"/PageMark raw u16 subrecord scan\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
        ));
        assert!(json.contains(
            "\"entryRelativeByteOffset\":162,\"streamByteOffset\":174,\"wordIndex\":81,\"words\":[2,5,768,0,85,0,140,0],\"wordsHex\":[\"0x0002\",\"0x0005\",\"0x0300\",\"0x0000\",\"0x0055\",\"0x0000\",\"0x008c\",\"0x0000\"]"
        ));
        assert!(json.contains(
            "\"entryRelativeByteOffset\":48,\"streamByteOffset\":334,\"wordIndex\":24,\"words\":[4,1,768,0,192,0,241,0],\"wordsHex\":[\"0x0004\",\"0x0001\",\"0x0300\",\"0x0000\",\"0x00c0\",\"0x0000\",\"0x00f1\",\"0x0000\"]"
        ));
    }

    #[test]
    fn local_success_data_test_exports_embedding_frame_candidates_when_reference_pdf_is_available()
    {
        let sample_dir = local_sample_dir();
        let sample_path =
            sample_dir.join("ichitaro-20030228030923-success-002-success_data-test.jtd");
        let reference_pdf_path =
            sample_dir.join("ichitaro-20030228030923-success-002-success_data-test.pdf");
        if !sample_path.exists() || !reference_pdf_path.exists() {
            return;
        }

        let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
        let json = to_json(&document);

        assert!(json.contains("\"pageMarks\":["));
        assert!(json.contains("\"rawLength\":84,\"rawHex\":\"00000000000100000000000000000027"));
        assert!(json.contains("\"u16Fields\":[0,0,1,0,0,0,0,39,0,0,370,0"));
        assert!(json.contains("\"u16FieldsHex\":[\"0x0000\",\"0x0000\",\"0x0001\",\"0x0000\""));
        assert!(json.contains("\"u16GeometryClass\":\"additive-boundary\""));
        assert!(json.contains("\"u32Fields\":[0,65536,0,39,0,24248320,370,12124160"));
        assert!(json.contains(
            "\"u32FieldsHex\":[\"0x00000000\",\"0x00010000\",\"0x00000000\",\"0x00000027\""
        ));
        assert!(json.contains(
            "\"u16GeometryHypotheses\":{\"source\":\"/PageMark\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"profile\":\"additive-boundary\""
        ));
        assert!(json.contains(
            "\"word20Is0x00ff\":true,\"word13PlusWord14\":555,\"word13PlusWord14EqualsWord21\":true,\"word21MinusWord13\":185,\"word21MinusWord13EqualsWord14\":true,\"word19EqualsWord13\":true,\"selectedFieldsAllZero\":false,\"nonZeroAdditiveUnitCandidate\":true,\"layoutComparisons\":null"
        ));
        assert!(json.contains("\"objectEmbeddingFrames\":["));
        assert!(json.contains("\"sourcePath\":\"/EmbedItems/EmbeddingInfo\""));
        assert!(json.contains("\"embeddingIndex\":24"));
        assert!(json.contains("\"className\":\"JSFart.Art.2\""));
        assert!(json.contains("\"frameRef\":1"));
        assert!(json.contains("\"frameSize\":{\"width\":13260,\"height\":1327}"));
        assert!(json.contains("\"embeddedPressSnapshot\":{\"format\":\"JSSnapShot32\""));
        assert!(json.contains("\"bodyLengthCandidate\":113332"));
        assert!(json.contains("\"width\":13260"));
        assert!(json.contains("\"height\":1327"));
        assert!(json.contains("\"textureBezierHeaderSummary\":{\"pathCount\":530,\"pointCount\":13,\"byteCount\":104,\"flags\":1,\"flagsHex\":\"0x00000001\",\"homogeneous\":true}"));
        assert!(json.contains("\"paintStateTransitions\":["));
        assert!(json.contains(
            "\"pathKind\":\"outline\",\"startPathIndex\":0,\"endPathIndex\":10,\"pathCount\":11"
        ));
        assert!(json.contains(
            "\"currentState\":{\"record48Word0\":\"0x00000001\",\"record70Word0\":\"0x0000002c\",\"record70Word3\":\"0x0000000a\",\"record82Word5\":\"0x0000002f\"}"
        ));
        assert!(json.contains(
            "\"pathKind\":\"texture\",\"startPathIndex\":11,\"endPathIndex\":540,\"pathCount\":530"
        ));
        assert!(json.contains(
            "\"pathKind\":\"outline\",\"startPathIndex\":541,\"endPathIndex\":551,\"pathCount\":11"
        ));
        assert!(json.contains("\"stateRecordSummary\":{\"pathCount\":"));
        assert!(json.contains("\"recordTypeHex\":\"0x00000082\""));
        assert!(json.contains("\"paintState82Preview\":[{"));
        assert!(json.contains("\"word3CandidateHex\":"));
        assert!(json.contains("\"word5CandidateHex\":"));
        assert!(json.contains("\"jsfartStreamProfile\":{\"format\":\"JSFart2Contents\""));
        assert!(json.contains("\"magicFamily\":\"mstudio-ocx-utf16le\""));
        assert!(json.contains("\"magicFamilyHex\":\"4d00\""));
        assert!(json.contains("\"structuredArtCandidatePresent\":true"));
        assert!(json.contains(
            "\"renderPromotionBlockedReason\":\"structured-jsfart-art-still-paint-authority-unproven\""
        ));
        assert!(json.contains("\"jsfartArt\":{\"format\":\"JSFart2Contents\""));
        assert!(json.contains("\"magic\":\"MSTUDIO.OCX\""));
        assert!(
            json.contains(
                "\"frameCandidate\":{\"left\":0,\"top\":0,\"right\":13260,\"bottom\":1327"
            )
        );
        assert!(json.contains(
            "\"contentLeft\":114,\"contentTop\":105,\"contentRight\":13145,\"contentBottom\":1159"
        ));
        assert!(json.contains("\"strokeWidthCandidate\":100"));
        assert!(json.contains(
            "\"paintCandidate\":{\"styleWord1\":34869296,\"styleWord1Hex\":\"0x02141030\""
        ));
        assert!(json.contains(
            "\"paintColorCandidate\":16777215,\"paintColorCandidateHex\":\"0x00ffffff\""
        ));
        assert!(
            json.contains("\"effectWordCandidate\":10,\"effectWordCandidateHex\":\"0x0000000a\"")
        );
        assert!(json.contains("\"embeddingIndex\":4"));
        assert!(json.contains("\"className\":\"JSEQ.Document.3\""));
        assert!(json.contains("\"jseq3Formula\":{\"format\":\"JSEQ3Contents\""));
        assert!(json.contains("\"magic\":\"MATH.VAF\""));
        assert!(json.contains("\"soTrailerOffset\":1658"));
        assert!(json.contains("\"soTrailerLength\":62"));
        assert!(json.contains("\"text\":\"Times New Roman\""));
        assert!(json.contains("\"path\":\"/FigureData/ExpandData/main_data/Link\""));
        assert!(json.contains("\"figureLink\":{\"headerWordsBe\":[11,1,0,15]"));
        assert!(json.contains("\"declaredRowCountCandidate\":15"));
        assert!(json.contains("\"rowStride\":14"));
        assert!(json.contains("\"rowCount\":15"));
        assert!(json.contains("\"relationKindCandidateHex\":\"0x0016\""));
        assert!(json.contains("\"path\":\"/FigureData/main_data/FDMVector\""));
        assert!(json.contains("\"fdmRawVectorSegmentCount\":5"));
        assert!(json.contains("\"fdmRawVectorCommandCount\":37"));
        assert!(json.contains("\"offsetFieldReferenceCandidates\":[{\"offsetField\":\"bbox.left\",\"offsetValue\":308,\"matchKind\":\"command-relative-offset-field\",\"referenceSource\":\"fdmRawVectorCommands.relativeOffset\",\"matchedCommandRelativeOffsets\":[308],\"decoded\":false}]"));
        assert!(json.contains("\"offsetFieldReferenceCandidates\":[{\"offsetField\":\"bbox.left\",\"offsetValue\":690,\"matchKind\":\"source-segment-relative-offset-field\",\"referenceSource\":\"fdmRawVectorCommands.sourceSegment.relativeOffset\",\"sourceSegmentRelativeOffset\":690,\"sourceSegmentBackedCommandCount\":1,\"matchedCommandRelativeOffsets\":[874],\"decoded\":false}]"));
        assert!(json.contains("\"offsetFieldReferenceCandidates\":[{\"offsetField\":\"bbox.left\",\"offsetValue\":1864,\"matchKind\":\"source-segment-relative-offset-field\",\"referenceSource\":\"fdmRawVectorCommands.sourceSegment.relativeOffset\",\"sourceSegmentRelativeOffset\":1864,\"sourceSegmentBackedCommandCount\":4,\"matchedCommandRelativeOffsets\":[1924,1958,1992,2024],\"decoded\":false}]"));
        assert!(json.contains("\"sourceVectorRelativeOffset\":208,\"sourceSegment\":null"));
        assert!(json.contains(
            "\"sourceVectorRelativeOffset\":1992,\"sourceSegment\":{\"relativeOffset\":1864,\"localOffset\":128,\"declaredLength\":236,\"commandCount\":4,\"commandIndex\":2,\"commandOffset\":128}"
        ));
        assert!(json.contains(
            "\"successDataTestFdmReferenceProjections\":[{\"role\":\"q4-angle-diagrams\""
        ));
        assert!(
            json.contains(
                "\"referenceTargetBboxPx\":{\"x\":93.300,\"y\":663.300,\"width\":491.400"
            )
        );
        assert!(json.contains(
            "\"commandRelativeOffsets\":[308,342,374,406,438,470,504,538,570,602,634,874,1048,1126,1158,1190,1430,1604,1730,1780]"
        ));
        assert!(
            json.contains("\"renderPromotionBlockedReason\":\"mixed-raw-and-segment-cohorts\"")
        );
        assert!(json.contains("\"primitiveOwnershipComparison\":{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":20,\"mainCircleAnchorCount\":3,\"lineCandidateCount\":11,\"radialLineCandidateCount\":0,\"chordCandidateCount\":0,\"arcCandidateCount\":6,\"connectorCandidateCount\":8,\"surfaceBoundaryCandidateCount\":2"));
        assert!(json.contains(
            "\"indexRowReferenceCandidateCount\":20,\"validVectorOffsetIndexRowReferenceCount\":0"
        ));
        assert_json_string_field_after(
            &json,
            "\"ownershipGate\":{",
            0,
            "renderOwnershipBlockedReason",
            "mixed-raw-and-segment-cohorts",
        );
        assert_json_string_array_field_after(
            &json,
            "\"ownershipGate\":{",
            0,
            "renderOwnershipBlockedReasons",
            &["mixed-raw-and-segment-cohorts"],
        );
        assert_json_number_field_after(&json, "\"ownershipGate\":{", 0, "commandCount", "20");
        assert_json_number_field_after(
            &json,
            "\"ownershipGate\":{",
            0,
            "rawSpanCommandCount",
            "18",
        );
        assert_json_number_field_after(
            &json,
            "\"ownershipGate\":{",
            0,
            "segmentBackedCommandCount",
            "2",
        );
        assert_json_bool_field_after(
            &json,
            "\"ownershipGate\":{",
            0,
            "oneToOneRowCommandReferenceCandidate",
            true,
        );
        assert_json_string_field_after(
            &json,
            "\"offsetFieldAuthorityGate\":{",
            0,
            "renderPromotionBlockedReason",
            "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
        );
        assert_json_number_field_after(
            &json,
            "\"offsetFieldAuthorityGate\":{",
            0,
            "commandRelativeOffsetFieldReferenceCount",
            "18",
        );
        assert_json_number_field_after(
            &json,
            "\"offsetFieldAuthorityGate\":{",
            0,
            "sourceSegmentRelativeOffsetFieldReferenceCount",
            "2",
        );
        assert_json_string_field_after(
            &json,
            "\"rowFanoutSegmentOwnerGate\":{",
            0,
            "renderPromotionBlockedReason",
            "fdm-index-row-fanout-segment-owner-offset-namespace-mixed",
        );
        assert_json_number_field_after(
            &json,
            "\"rowFanoutSegmentOwnerGate\":{",
            0,
            "maxRowFanout",
            "1",
        );
        assert_json_bool_field_after(
            &json,
            "\"rowFanoutSegmentOwnerGate\":{",
            0,
            "singleRowBacksMultipleCommandsCandidate",
            false,
        );
        assert_json_string_field_after(
            &json,
            "\"primitiveOwnershipAdmissionGate\":{",
            0,
            "renderPromotionBlockedReason",
            "mixed-raw-and-segment-cohorts",
        );
        assert_json_string_array_field_after(
            &json,
            "\"primitiveOwnershipAdmissionGate\":{",
            0,
            "renderPromotionBlockedReasons",
            &[
                "mixed-raw-and-segment-cohorts",
                "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
                "fdm-index-row-fanout-segment-owner-offset-namespace-mixed",
                "fdm-index-role-vector-offset-authority-valid-vector-offset-missing",
                "fdm-index-role-valid-vector-offset-missing",
                "role-paint-order-continuity-unproven",
            ],
        );
        assert_json_number_field_after(
            &json,
            "\"primitiveOwnershipAdmissionGate\":{",
            0,
            "rolePaintOrderBlockedGroupCount",
            "6",
        );
        assert_json_string_field_after(
            &json,
            "\"indexRowOrderPromotionGate\":{",
            0,
            "renderPromotionBlockedReason",
            "fdm-index-row-order-valid-vector-offset-missing",
        );
        assert_json_string_array_field_after(
            &json,
            "\"indexRowOrderPromotionGate\":{",
            0,
            "renderPromotionBlockedReasons",
            &[
                "fdm-index-row-order-valid-vector-offset-missing",
                "fdm-index-row-order-offset-namespace-mixed",
                "role-paint-order-continuity-unproven",
            ],
        );
        assert_json_number_field_after(
            &json,
            "\"indexRowOrderPromotionGate\":{",
            0,
            "uniqueRowIndexCount",
            "20",
        );
        assert!(json.contains("\"renderPaintOrderBasisCandidate\":\"fdm-index-row-command-pairs\",\"renderPaintOrderBasisDecoded\":false"));
        assert!(json.contains("\"roleCandidate\":\"main-circle-anchor\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":3,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":3,\"sourceSegmentRelativeOffsetFieldReferenceCount\":0,\"commandRelativeOffsets\":[308,470,504],\"rowIndexes\":[7,12,13],\"uniqueCommandRelativeOffsetCount\":3,\"uniqueRowIndexCount\":3,\"oneToOneRowCommandReferenceCandidate\":true,\"singleRowBacksMultipleCommandsCandidate\":false,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":7,\"commandRelativeOffset\":308,\"matchKind\":\"command-relative-offset-field\"}"));
        assert!(json.contains("\"paintOrderContinuityProfile\":{\"basis\":\"fdm-index-row-reference-role-command-span\",\"decoded\":false,\"sourceBacked\":true,\"paintOrderDecoded\":false,\"commandRelativeOffsetSpanMin\":308,\"commandRelativeOffsetSpanMax\":504,\"roleCommandCount\":3,\"commandCountInSpan\":7,\"interleavedNonRoleCommandCount\":4,\"hasInterleavedNonRoleCommands\":true,\"maxCommandOffsetGap\":162,\"commandOffsetContinuityScore\":0.429,\"spanContiguousCandidate\":false,\"paintOrderAuthorityPending\":false,\"continuityBlocked\":true,\"renderPromotionBlockedReason\":\"role-span-interleaved-non-role-commands\"}"));
        assert!(json.contains("\"roleCandidate\":\"radial-line-candidate\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":2,\"sourceSegmentRelativeOffsetFieldReferenceCount\":0,\"commandRelativeOffsets\":[342,406],\"rowIndexes\":[8,10],\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":2,\"oneToOneRowCommandReferenceCandidate\":true,\"singleRowBacksMultipleCommandsCandidate\":false,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":8,\"commandRelativeOffset\":342,\"matchKind\":\"command-relative-offset-field\"},{\"rowIndex\":10,\"commandRelativeOffset\":406,\"matchKind\":\"command-relative-offset-field\"}],\"roleVectorOffsetAuthorityGate\":"));
        assert!(json.contains("\"primitiveOwnershipComparison\":{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":7,\"mainCircleAnchorCount\":1,\"lineCandidateCount\":4,\"radialLineCandidateCount\":2,\"chordCandidateCount\":2,\"arcCandidateCount\":2,\"connectorCandidateCount\":2,\"surfaceBoundaryCandidateCount\":2"));
        assert!(json.contains("\"relativeOffset\":374,\"primitiveKind\":\"polyline\",\"markerHex\":\"01000160\",\"sourceSegmentBacked\":false,\"sourceSegmentRelativeOffset\":null,\"roleCandidates\":[\"line-candidate\",\"chord-candidate\",\"connector-candidate\"]"));
        assert!(json.contains("\"indexRowReferenceCandidates\":[{\"rowIndex\":9,\"indexOffset\":218,\"vectorOffset\":3663724543,\"validVectorOffset\":false,\"offsetField\":\"bbox.left\",\"offsetValue\":374,\"matchKind\":\"command-relative-offset-field\",\"decoded\":false}]"));
        assert!(json.contains("\"relativeOffset\":1430,\"primitiveKind\":\"ellipse\",\"markerHex\":\"ff000460\",\"sourceSegmentBacked\":true,\"sourceSegmentRelativeOffset\":1246,\"roleCandidates\":[\"arc-candidate\",\"control-ellipse-marker\"]"));
        assert!(json.contains("\"indexRowReferenceCandidates\":[{\"rowIndex\":32,\"indexOffset\":724,\"vectorOffset\":3671785471,\"validVectorOffset\":false,\"offsetField\":\"bbox.left\",\"offsetValue\":1246,\"matchKind\":\"source-segment-relative-offset-field\",\"decoded\":false}]"));
        assert!(json.contains(
            "\"subdiagrams\":[{\"index\":0,\"groupingSource\":\"nearest-main-circle-source-center\""
        ));
        assert!(json.contains("\"role\":\"q5-solid-diagram\""));
        assert!(json.contains(
            "\"referenceTargetBboxPx\":{\"x\":490.700,\"y\":795.000,\"width\":74.600,\"height\":110.000}"
        ));
        assert!(json.contains("\"commandRelativeOffsets\":[1830,1924,1958,1992,2024,2156,2190]"));
        assert!(json.contains("\"primitiveOwnershipComparison\":{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":7,\"mainCircleAnchorCount\":0,\"lineCandidateCount\":2,\"radialLineCandidateCount\":0,\"chordCandidateCount\":0,\"arcCandidateCount\":4,\"connectorCandidateCount\":3,\"surfaceBoundaryCandidateCount\":1"));
        assert!(json.contains(
            "\"indexRowReferenceCandidateCount\":7,\"validVectorOffsetIndexRowReferenceCount\":0"
        ));
        assert_json_string_field_after(
            &json,
            "\"ownershipGate\":{",
            1,
            "renderOwnershipBlockedReason",
            "multi-command-single-index-row",
        );
        assert_json_string_array_field_after(
            &json,
            "\"ownershipGate\":{",
            1,
            "renderOwnershipBlockedReasons",
            &[
                "multi-command-single-index-row",
                "mixed-raw-and-segment-cohorts",
                "row-command-reference-not-one-to-one",
            ],
        );
        assert_json_number_field_after(&json, "\"ownershipGate\":{", 1, "commandCount", "7");
        assert_json_number_field_after(&json, "\"ownershipGate\":{", 1, "rawSpanCommandCount", "1");
        assert_json_number_field_after(
            &json,
            "\"ownershipGate\":{",
            1,
            "segmentBackedCommandCount",
            "6",
        );
        assert_json_bool_field_after(
            &json,
            "\"ownershipGate\":{",
            1,
            "oneToOneRowCommandReferenceCandidate",
            false,
        );
        assert_json_string_field_after(
            &json,
            "\"offsetFieldAuthorityGate\":{",
            1,
            "renderPromotionBlockedReason",
            "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
        );
        assert_json_number_field_after(
            &json,
            "\"offsetFieldAuthorityGate\":{",
            1,
            "commandRelativeOffsetFieldReferenceCount",
            "1",
        );
        assert_json_number_field_after(
            &json,
            "\"offsetFieldAuthorityGate\":{",
            1,
            "sourceSegmentRelativeOffsetFieldReferenceCount",
            "6",
        );
        assert_json_string_field_after(
            &json,
            "\"rowFanoutSegmentOwnerGate\":{",
            1,
            "renderPromotionBlockedReason",
            "fdm-index-row-fanout-segment-owner-multi-command-single-row",
        );
        assert_json_number_field_after(
            &json,
            "\"rowFanoutSegmentOwnerGate\":{",
            1,
            "maxRowFanout",
            "4",
        );
        assert_json_bool_field_after(
            &json,
            "\"rowFanoutSegmentOwnerGate\":{",
            1,
            "singleRowBacksMultipleCommandsCandidate",
            true,
        );
        assert_json_string_field_after(
            &json,
            "\"primitiveOwnershipAdmissionGate\":{",
            1,
            "renderPromotionBlockedReason",
            "multi-command-single-index-row",
        );
        assert_json_string_array_field_after(
            &json,
            "\"primitiveOwnershipAdmissionGate\":{",
            1,
            "renderPromotionBlockedReasons",
            &[
                "multi-command-single-index-row",
                "mixed-raw-and-segment-cohorts",
                "row-command-reference-not-one-to-one",
                "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
                "fdm-index-row-fanout-segment-owner-multi-command-single-row",
                "fdm-index-role-row-fanout-multi-command-single-row",
                "fdm-index-role-vector-offset-authority-valid-vector-offset-missing",
                "fdm-index-role-valid-vector-offset-missing",
                "role-paint-order-continuity-unproven",
                "role-paint-order-authority-unproven",
            ],
        );
        assert_json_number_field_after(
            &json,
            "\"primitiveOwnershipAdmissionGate\":{",
            1,
            "rolePaintOrderBlockedGroupCount",
            "2",
        );
        assert_json_number_field_after(
            &json,
            "\"primitiveOwnershipAdmissionGate\":{",
            1,
            "rolePaintOrderAuthorityPendingGroupCount",
            "2",
        );
        assert_json_string_field_after(
            &json,
            "\"indexRowOrderPromotionGate\":{",
            1,
            "renderPromotionBlockedReason",
            "fdm-index-row-order-reference-not-one-to-one",
        );
        assert_json_string_array_field_after(
            &json,
            "\"indexRowOrderPromotionGate\":{",
            1,
            "renderPromotionBlockedReasons",
            &[
                "fdm-index-row-order-reference-not-one-to-one",
                "fdm-index-row-order-single-row-backs-multiple-commands",
                "fdm-index-row-order-valid-vector-offset-missing",
                "fdm-index-row-order-offset-namespace-mixed",
                "role-paint-order-continuity-unproven",
                "role-paint-order-authority-unproven",
            ],
        );
        assert_json_number_field_after(
            &json,
            "\"indexRowOrderPromotionGate\":{",
            1,
            "uniqueRowIndexCount",
            "3",
        );
        assert!(json.contains("\"roleCandidate\":\"line-candidate\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":0,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"commandRelativeOffsets\":[1992,2024],\"rowIndexes\":[40],\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":1,\"oneToOneRowCommandReferenceCandidate\":false,\"singleRowBacksMultipleCommandsCandidate\":true,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":40,\"commandRelativeOffset\":1992,\"matchKind\":\"source-segment-relative-offset-field\"},{\"rowIndex\":40,\"commandRelativeOffset\":2024,\"matchKind\":\"source-segment-relative-offset-field\"}],\"roleVectorOffsetAuthorityGate\":{\"basis\":\"fdm-index-role-vector-offset-authority-gate\",\"source\":\"FDMIndex.vectorOffset+FDMIndex role offset fields\",\"decoded\":false,\"sourceBacked\":true,\"roleCandidate\":\"line-candidate\",\"roleVectorOffsetAuthorityDecoded\":false,\"renderPromotionContribution\":\"fdm-index-role-vector-offset-authority-gate\",\"renderPromotionBlockedReason\":\"fdm-index-role-vector-offset-authority-valid-vector-offset-missing\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"invalidVectorOffsetReferenceCount\":2,\"commandRelativeOffsetFieldReferenceCount\":0,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"validCommandRelativeOffsetFieldReferenceCount\":0,\"validSourceSegmentRelativeOffsetFieldReferenceCount\":0,\"invalidCommandRelativeOffsetFieldReferenceCount\":0,\"invalidSourceSegmentRelativeOffsetFieldReferenceCount\":2,\"allValidReferencesUseCommandRelativeOffsetField\":false,\"allValidReferencesUseSourceSegmentRelativeOffsetField\":false,\"mixedOffsetNamespacesAmongValidReferences\":false,\"allReferencesHaveInvalidVectorOffset\":true},\"roleFanoutSegmentOwnerGate\":{\"basis\":\"fdm-index-role-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex role row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true,\"roleCandidate\":\"line-candidate\",\"roleOwnershipDecoded\":false,\"segmentOwnerDecoded\":false,\"renderPromotionContribution\":\"fdm-index-role-row-fanout-segment-owner-gate\",\"renderPromotionBlockedReason\":\"fdm-index-role-row-fanout-multi-command-single-row\",\"referenceCount\":2,\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":1,\"commandRelativeOffsetFieldReferenceCount\":0,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"fanoutRowCount\":1,\"fanoutReferenceCount\":2,\"fanoutCommandRelativeOffsetFieldReferenceCount\":0,\"fanoutSourceSegmentRelativeOffsetFieldReferenceCount\":2,\"maxRowFanout\":2,\"oneToOneRowCommandReferenceCandidate\":false,\"singleRowBacksMultipleCommandsCandidate\":true,\"mixedOffsetFieldNamespaces\":false,\"fanoutRowsUseCommandRelativeOffsetFields\":false,\"fanoutRowsUseSourceSegmentOffsetFields\":true,\"rowsWithMultipleCommandRefs\":[{\"rowIndex\":40,\"commandReferenceCount\":2,\"commandRelativeOffsets\":[1992,2024],\"matchKinds\":[\"source-segment-relative-offset-field\"]}]}"));
        assert!(json.contains("\"paintOrderContinuityProfile\":{\"basis\":\"fdm-index-row-reference-role-command-span\",\"decoded\":false,\"sourceBacked\":true,\"paintOrderDecoded\":false,\"commandRelativeOffsetSpanMin\":1992,\"commandRelativeOffsetSpanMax\":2024,\"roleCommandCount\":2,\"commandCountInSpan\":2,\"interleavedNonRoleCommandCount\":0,\"hasInterleavedNonRoleCommands\":false,\"maxCommandOffsetGap\":32,\"commandOffsetContinuityScore\":1.000,\"spanContiguousCandidate\":true,\"paintOrderAuthorityPending\":true,\"continuityBlocked\":false,\"renderPromotionBlockedReason\":\"role-paint-order-authority-unproven\"}"));
        assert!(json.contains("\"relativeOffset\":1992,\"primitiveKind\":\"polyline\",\"markerHex\":\"ff000160\",\"sourceSegmentBacked\":true,\"sourceSegmentRelativeOffset\":1864,\"roleCandidates\":[\"line-candidate\",\"connector-candidate\"]"));
        assert!(json.contains("\"indexRowReferenceCandidates\":[{\"rowIndex\":40,\"indexOffset\":900,\"vectorOffset\":3729719295,\"validVectorOffset\":false,\"offsetField\":\"bbox.left\",\"offsetValue\":1864,\"matchKind\":\"source-segment-relative-offset-field\",\"decoded\":false}]"));
        assert!(json.contains("\"primitiveKind\":\"cubicBezier\""));
        assert!(json.contains("\"primitiveKind\":\"ellipse\""));
        assert!(json.contains("\"curveSegmentCount\":1"));
        assert!(
            json.contains("\"ellipse\":{\"center\":{\"x\":-11280,\"y\":-10792},\"radiusX\":556")
        );
        assert!(json.contains("\"path\":\"/FigureData/ExpandData/main_data/Data/FDMText\""));
        assert!(json.contains("\"fdmTextCount\":15"));
        assert!(json.contains("\"fdmTextIndexEntries\":["));
        assert!(json.contains("\"text\":\"９㎝\""));
        assert!(json.contains("\"textRecordOffset\":6584"));
        assert!(json.contains("\"kind\":\"sparseDocumentTextControlRunTableCandidate\""));
        assert!(json.contains("\"rule\":\"sparse-document-text-001c-cells-with-000e-row-breaks\""));
        assert!(json.contains("\"textPreview\":\"\\t\\t\\t(1)表面積の比"));
        assert!(
            json.contains("\"sparseObservedTable\":{\"source\":\"sparseDocumentTextControlRows\"")
        );
        assert!(
            json.contains("\"topologyCandidate\":{\"source\":\"sparseDocumentTextControlRows\"")
        );
        assert!(
            json.contains(
                "\"sparseTopologyCandidate\":{\"source\":\"sparseDocumentTextControlRows\""
            )
        );
        assert!(json.contains("\"columns\":["));
        assert!(json.contains("\"firstNonEmptyColumnIndex\":3"));
        assert!(json.contains("\"emptyCellCountCandidate\":136"));
        assert!(json.contains("\"rows\":["));
        assert!(json.contains("\"cells\":["));
        assert!(json.contains("\"empty\":true"));
        assert!(json.contains("\"sourceStart\":2902"));
        assert!(json.contains("\"sourceEnd\":5419"));
        assert!(json.contains("\"geometryDecoded\":false"));
    }

    #[test]
    fn local_shanai_lan_exports_fdm_vector_command_diagnostics_when_reference_pdf_is_available() {
        let sample_dir = local_sample_dir();
        let sample_path =
            sample_dir.join("ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd");
        let reference_pdf_path =
            sample_dir.join("ichitaro-20030315134715-success-001-success_data-shanai_lan.pdf");
        if !sample_path.exists() || !reference_pdf_path.exists() {
            return;
        }

        let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
        let json = to_json(&document);

        assert!(json.contains("\"path\":\"/FigureData/main_data/FDMVector\""));
        assert!(json.contains("\"fdmIndexEntries\":["));
        assert!(json.contains("\"vectorCommandCount\":"));
        assert!(json.contains("\"vectorCommandBboxCount\":"));
        assert!(json.contains("\"vectorCommands\":[{"));
        assert!(json.contains("\"connectorCandidateCount\":"));
        assert!(json.contains("\"connectorCandidates\":[{"));
        assert!(json.contains("\"candidateBasis\":\"long-open-source-path\""));
        assert!(json.contains("\"sourceEndpoints\":{\"start\":{\"x\":"));
        assert!(json.contains("\"sourceSpan\":"));
        assert!(json.contains("\"endpointDistanceSquared\":"));
        assert!(json.contains("\"fillColor\":"));
        assert!(json.contains("\"strokeColor\":"));
        assert!(json.contains("\"pathSegmentCount\":"));
        assert!(json.contains("\"orthogonalSegmentCount\":"));
        assert!(json.contains("\"diagonalSegmentCount\":"));
        assert!(json.contains("\"compoundChildOffsetCount\":"));
        assert!(json.contains("\"axisAligned\":"));
        assert!(json.contains("\"orientation\":\"horizontal\""));
        assert!(json.contains("\"markerHex\":\"00000960\""));
        assert!(json.contains("\"primitiveKind\":\"cubicBezier\""));
        assert!(json.contains("\"pathPoints\":[{\"x\":"));
        assert!(json.contains("\"curveSegments\":[{\"control1\":"));
        assert!(json.contains("\"compoundChildOffsets\":["));
        assert!(json.contains("\"decoded\":false"));
    }

    #[test]
    fn local_200307_shanai_lan_exports_json_without_fdm_projection_overflow() {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let sample_path = project_root
            .join("rjtd-testdata/local-samples")
            .join("ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd");
        let pdf_output_path = project_root
            .join("openjtd-samples/pdf-output")
            .join("ichitaro-20030706232827-success-001-success_data-shanai_lan.pdf");
        if !sample_path.exists() || !pdf_output_path.exists() {
            return;
        }

        let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
        let json = to_json(&document);

        assert!(json.contains("\"objectStreamCandidates\":["));
        assert!(json.contains("\"path\":\"/FigureData/main_data/FDMVector\""));
        assert!(json.contains("\"successDataTestFdmReferenceProjections\":["));
    }
}
