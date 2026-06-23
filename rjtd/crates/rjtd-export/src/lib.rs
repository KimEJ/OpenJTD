//! Exporters that consume the document model.

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
        push_object_fdm_index_entry_candidate_json(output, entry);
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
    output.push_str("],\"fdmTextCount\":");
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

    Ok(pdf.finish())
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
    use std::{fs, path::PathBuf, process::Command};

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
        assert!(pdf_text.contains("/Producer (rjtd)"));
        assert!(pdf.ends_with(b"%%EOF"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn exports_pdf_with_file_name_layout_hint() {
        let document = Document::from_plain_text(&vec!["銀河鉄道の夜"; 80].join("\n"));
        let pdf = to_pdf_with_file_name(&document, "a5.jtd").unwrap();
        let pdf_text = String::from_utf8_lossy(&pdf);

        assert!(pdf.starts_with(b"%PDF-1.4"));
        assert!(pdf_text.contains("/MediaBox [0 0 419."));
        assert!(pdf_text.contains(" 595."));
        assert!(pdf_text.contains("1 1 1 rg\n0 0 419."));
        assert!(pdf_text.contains(" re\nf\nQ\nq\n"));
        assert!(pdf_text.contains("q\n419."));
        assert!(pdf_text.contains("/S1 Do\nQ"));
        assert!(!pdf_text.contains("/Group <<"));
        assert!(!pdf_text.contains("/S /Transparency"));
        assert!(pdf.ends_with(b"%%EOF"));
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
            "ichitaro-20030228030923-success-002-success_data-test",
            "ichitaro-20030315134715-success-001-success_data-shanai_lan",
        ];
        let mut failures = Vec::new();
        let mut rendered_count = 0usize;

        let any_sample_present = samples
            .iter()
            .any(|sample| sample_dir.join(format!("{sample}.jtd")).exists());
        if !any_sample_present {
            return;
        }

        for sample in samples {
            let sample_path = sample_dir.join(format!("{sample}.jtd"));
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

        let samples = [
            "ichitaro-20030228030923-success-002-success_data-test",
            "ichitaro-20030315134715-success-001-success_data-shanai_lan",
        ];
        let mut failures = Vec::new();
        let mut rendered_count = 0usize;

        let any_sample_present = samples
            .iter()
            .any(|sample| sample_dir.join(format!("{sample}.jtd")).exists());
        if !any_sample_present {
            return;
        }

        for sample in samples {
            let sample_path = sample_dir.join(format!("{sample}.jtd"));
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

            let output = match Command::new("swift")
                .env("CLANG_MODULE_CACHE_PATH", &module_cache_path)
                .arg("-e")
                .arg(PDFKIT_VISIBLE_CONTENT_SWIFT)
                .arg(&pdf_path)
                .output()
            {
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

var totalNonWhite = 0
for pageIndex in 0..<min(document.pageCount, 2) {
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
        exit(4)
    }
    context.setFillColor(CGColor(red: 1, green: 1, blue: 1, alpha: 1))
    context.fill(CGRect(x: 0, y: 0, width: width, height: height))
    page.draw(with: .mediaBox, to: context)

    var byteIndex = 0
    while byteIndex < bytes.count {
        if bytes[byteIndex] < 245 || bytes[byteIndex + 1] < 245 || bytes[byteIndex + 2] < 245 {
            totalNonWhite += 1
        }
        byteIndex += 4
    }
}

print("pages \(document.pageCount) nonWhiteFirst2 \(totalNonWhite)")
if totalNonWhite == 0 {
    fputs("PDFKit rendered no visible non-white pixels\n", stderr)
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
                    if pdf.windows(9).any(|window| window == b"/Group <<") {
                        failures.push(format!(
                            "{}: page-level transparency group can blank in Preview/PDFKit",
                            path.display()
                        ));
                    }
                    if pdf.windows(15).any(|window| window == b"/S /Transparency") {
                        failures.push(format!(
                            "{}: page-level transparency group can blank in Preview/PDFKit",
                            path.display()
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
        assert!(json.contains("\"sourceVectorRelativeOffset\":208,\"sourceSegment\":null"));
        assert!(json.contains(
            "\"sourceVectorRelativeOffset\":1992,\"sourceSegment\":{\"relativeOffset\":1864,\"localOffset\":128,\"declaredLength\":236,\"commandCount\":4,\"commandIndex\":2,\"commandOffset\":128}"
        ));
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
