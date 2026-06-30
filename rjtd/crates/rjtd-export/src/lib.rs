//! Exporters that consume the document model.

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
    ObjectStreamCandidate, ObjectStreamOwnershipCandidate, ObjectStreamOwnershipReferenceCandidate,
    ObjectVisualListCandidate, StyleRef, TableCandidate, TableCandidateColumnSegment,
    TableCandidateInterval, TextBoundaryCandidate, TextControlBoundary,
    TextCountControlRangeOverlap, TextCountRange, TextCountRangeOverlap, TextLayoutExactEvidence,
    TextParagraphBoundaryCandidate, TextSourceSpan, UnknownObject, page_mark_u16_geometry_profile,
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
    let center_x = bbox.0 + (bbox.2 - bbox.0) / 2;
    let center_y = bbox.1 + (bbox.3 - bbox.1) / 2;
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
    Some((
        bbox.0 + (bbox.2 - bbox.0) / 2,
        bbox.1 + (bbox.3 - bbox.1) / 2,
    ))
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
    output.push_str("{\"basis\":\"fdm-index-row-reference-command-order\",\"decoded\":false,\"ownershipProven\":false,\"paintOrderDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-row-order-evidence-only\"");
    output
        .push_str(",\"renderPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\"");
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

#[derive(Debug, Default)]
struct SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup {
    role_candidate: &'static str,
    reference_count: usize,
    valid_vector_offset_reference_count: usize,
    command_relative_offset_field_reference_count: usize,
    source_segment_relative_offset_field_reference_count: usize,
    command_relative_offsets: BTreeSet<usize>,
    row_indexes: BTreeSet<usize>,
    row_command_pairs: BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SuccessDataTestFdmIndexRowCommandPair {
    row_index: usize,
    command_relative_offset: usize,
    match_kind: &'static str,
}

fn push_success_data_test_fdm_index_row_reference_role_candidate_groups_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
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
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
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
    use std::{collections::BTreeSet, fs, path::PathBuf, process::Command};

    #[cfg(not(target_arch = "wasm32"))]
    fn count_pdf_eof_markers(pdf: &[u8]) -> usize {
        pdf.windows(b"%%EOF".len())
            .filter(|window| *window == b"%%EOF")
            .count()
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
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
        if !sample_dir.exists() {
            return;
        }

        let samples = [
            "ichitaro-20030228030923-success-002-success_data-test.jtd",
            "ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd",
            "a5.jtd",
            "fax02.jtt",
        ];
        let mut failures = Vec::new();
        let mut rendered_count = 0usize;

        let any_sample_present = samples.iter().any(|sample| {
            let sample_path = sample_dir.join(sample);
            sample_path.exists() && sample_path.with_extension("pdf").exists()
        });
        if !any_sample_present {
            return;
        }

        for sample in samples {
            let sample_path = sample_dir.join(sample);
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
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
        if !sample_dir.exists() {
            return;
        }

        let samples: [(&str, &[&str]); 4] = [
            (
                "ichitaro-20030228030923-success-002-success_data-test.jtd",
                &["1:10000", "2:500"],
            ),
            (
                "ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd",
                &["1:5000"],
            ),
            ("a5.jtd", &["1:300", "6:3000"]),
            ("fax02.jtt", &["1:10000"]),
        ];
        let mut failures = Vec::new();
        let mut rendered_count = 0usize;

        let any_sample_present = samples.iter().any(|(sample, _)| {
            let sample_path = sample_dir.join(sample);
            sample_path.exists() && sample_path.with_extension("pdf").exists()
        });
        if !any_sample_present {
            return;
        }

        for (sample, page_checks) in samples {
            let sample_path = sample_dir.join(sample);
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
            for page_check in page_checks {
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
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
        if !sample_dir.exists() {
            return;
        }

        let samples: [(&str, &[&str]); 4] = [
            (
                "ichitaro-20030228030923-success-002-success_data-test.jtd",
                &["1:10000", "2:500"],
            ),
            (
                "ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd",
                &["1:5000"],
            ),
            ("a5.jtd", &["1:300", "6:3000"]),
            ("fax02.jtt", &["1:10000"]),
        ];
        let mut failures = Vec::new();
        let mut rendered_count = 0usize;

        let any_sample_present = samples.iter().any(|(sample, _)| {
            let sample_path = sample_dir.join(sample);
            sample_path.exists() && sample_path.with_extension("pdf").exists()
        });
        if !any_sample_present {
            return;
        }

        for (sample, page_checks) in samples {
            let sample_path = sample_dir.join(sample);
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
            for page_check in page_checks {
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
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
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
                if reference_page_count == 0 {
                    failures.push(format!(
                        "{}: could not derive reference PDF page count",
                        reference_pdf_path.display()
                    ));
                } else if output_page_count != reference_page_count {
                    failures.push(format!(
                        "{}: expected {reference_page_count} PDF page objects to match {}, got {output_page_count}",
                        pdf_path.display(),
                        reference_pdf_path.display()
                    ));
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

        let samples: [(&str, &[&str]); 4] = [
            (
                "ichitaro-20030228030923-success-002-success_data-test.pdf",
                &["1:10000", "2:500"],
            ),
            (
                "ichitaro-20030315134715-success-001-success_data-shanai_lan.pdf",
                &["1:5000"],
            ),
            ("a5.pdf", &["1:300", "6:3000"]),
            ("fax02.pdf", &["1:10000"]),
        ];
        let mut failures = Vec::new();
        let mut rendered_count = 0usize;

        for (sample, page_checks) in samples {
            let pdf_path = output_dir.join(sample);
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
            for page_check in page_checks {
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

    fn local_reference_pdf_page_count_is_trusted(stem: &str) -> bool {
        // The local 46.pdf reference is a known suspicious capture: it renders as
        // essentially blank/title-like while 46.jtd contains the Ginga body text.
        // Keep it out of full-document page-count gates until the sample is replaced.
        stem != "46"
    }

    fn pdf_page_object_count(pdf: &[u8]) -> usize {
        pdf_byte_pattern_count(pdf, b"/Type /Page\n")
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
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
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
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
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
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
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
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
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
        assert!(json.contains("\"indexRowOrderPromotionGate\":{\"basis\":\"fdm-index-row-reference-command-order\",\"decoded\":false,\"ownershipProven\":false,\"paintOrderDecoded\":false,\"renderPromotionContribution\":\"fdm-index-row-order-evidence-only\",\"renderPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":20,\"referencedCommandCount\":20,\"unreferencedCommandCount\":0,\"uniqueRowIndexCount\":20,\"referenceCount\":20,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":18,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"allCommandsReferencedByIndexRowsCandidate\":true,\"oneToOneRowCommandReferenceCandidate\":true,\"singleRowBacksMultipleCommandsCandidate\":false,\"rowOrderMatchesCommandOrderCandidate\":true"));
        assert!(json.contains("\"renderPaintOrderBasisCandidate\":\"fdm-index-row-command-pairs\",\"renderPaintOrderBasisDecoded\":false"));
        assert!(json.contains("\"roleCandidate\":\"main-circle-anchor\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":3,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":3,\"sourceSegmentRelativeOffsetFieldReferenceCount\":0,\"commandRelativeOffsets\":[308,470,504],\"rowIndexes\":[7,12,13],\"uniqueCommandRelativeOffsetCount\":3,\"uniqueRowIndexCount\":3,\"oneToOneRowCommandReferenceCandidate\":true,\"singleRowBacksMultipleCommandsCandidate\":false,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":7,\"commandRelativeOffset\":308,\"matchKind\":\"command-relative-offset-field\"}"));
        assert!(json.contains("\"roleCandidate\":\"radial-line-candidate\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":2,\"sourceSegmentRelativeOffsetFieldReferenceCount\":0,\"commandRelativeOffsets\":[342,406],\"rowIndexes\":[8,10],\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":2,\"oneToOneRowCommandReferenceCandidate\":true,\"singleRowBacksMultipleCommandsCandidate\":false,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":8,\"commandRelativeOffset\":342,\"matchKind\":\"command-relative-offset-field\"},{\"rowIndex\":10,\"commandRelativeOffset\":406,\"matchKind\":\"command-relative-offset-field\"}],\"decoded\":false"));
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
        assert!(json.contains("\"indexRowOrderPromotionGate\":{\"basis\":\"fdm-index-row-reference-command-order\",\"decoded\":false,\"ownershipProven\":false,\"paintOrderDecoded\":false,\"renderPromotionContribution\":\"fdm-index-row-order-evidence-only\",\"renderPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":7,\"referencedCommandCount\":7,\"unreferencedCommandCount\":0,\"uniqueRowIndexCount\":3,\"referenceCount\":7,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":1,\"sourceSegmentRelativeOffsetFieldReferenceCount\":6,\"allCommandsReferencedByIndexRowsCandidate\":true,\"oneToOneRowCommandReferenceCandidate\":false,\"singleRowBacksMultipleCommandsCandidate\":true,\"rowOrderMatchesCommandOrderCandidate\":true"));
        assert!(json.contains("\"roleCandidate\":\"line-candidate\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":0,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"commandRelativeOffsets\":[1992,2024],\"rowIndexes\":[40],\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":1,\"oneToOneRowCommandReferenceCandidate\":false,\"singleRowBacksMultipleCommandsCandidate\":true,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":40,\"commandRelativeOffset\":1992,\"matchKind\":\"source-segment-relative-offset-field\"},{\"rowIndex\":40,\"commandRelativeOffset\":2024,\"matchKind\":\"source-segment-relative-offset-field\"}],\"decoded\":false"));
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
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
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
}
