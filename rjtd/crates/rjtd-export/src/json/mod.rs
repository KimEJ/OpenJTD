pub(crate) mod block;
pub(crate) mod document;
pub(crate) mod fdm;
pub(crate) mod image;
pub(crate) mod object;
pub(crate) mod primitives;
pub(crate) mod style;
pub(crate) mod table;
mod text_flow;
pub(crate) mod text_layout;

use rjtd_core::style_stream::summarize_style_stream;
use rjtd_model::Document;

use block::push_block_json;
use document::{
    push_document_auto_text_json, push_document_font_json, push_document_page_mark_json,
    push_document_paper_mark_json, push_document_toc_entry_json,
};
use object::{
    push_object_embedding_frame_candidate_json, push_object_frame_record_candidate_json,
    push_object_stream_candidate_json, push_unknown_object_json,
};
use primitives::{hex, push_json_string, push_u16_array_json, push_u32_array_json};
use style::{push_style_records_json, push_unknown_source_json};
use table::push_table_candidate_json;
use text_layout::{
    push_text_boundary_candidate_json, push_text_control_boundary_json, push_text_count_range_json,
    push_text_paragraph_boundary_candidate_json,
};

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
    output.push_str("],\"documentTextFlow\":");
    match document.document_text_flow() {
        Some(flow) => text_flow::push_document_text_flow_json(&mut output, flow),
        None => output.push_str("null"),
    }
    output.push_str(",\"englishJustificationCandidate\":");
    match document.english_justification_candidate() {
        Some(enabled) => output.push_str(&format!("{{\"enabled\":{enabled},\"source\":\"/DocumentViewStyles\",\"recordCode\":4107,\"decoded\":false}}")),
        None => output.push_str("null"),
    }
    output.push_str(",\"autoTextCandidates\":[");
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
    output.push(']');
    let equations = document.equation_candidates();
    if !equations.is_empty() {
        output.push_str(",\"equationCandidates\":[");
        for (index, equation) in equations.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str(&format!("{{\"source\":\"JSEQ3Contents+EmbeddedPress+Frame+DocumentText\",\"lineMarkRecordIndex\":{},\"recordSourceSpan\":",equation.line_mark_index()));
            text_layout::push_text_source_span_json(&mut output, equation.record_span());
            output.push_str(",\"glyphs\":[");
            for (i, glyph) in equation.glyphs().iter().enumerate() {
                if i > 0 {
                    output.push(',');
                }
                output.push_str("{\"text\":");
                push_json_string(&mut output, glyph.text());
                let [x, y] = glyph.position_mm100();
                let [start, end] = glyph.snapshot_range();
                output.push_str(&format!(",\"xMm100\":{x},\"yMm100\":{y},\"fontSizeMm100\":{},\"italic\":{},\"snapshotByteStart\":{start},\"snapshotByteEnd\":{end},\"contentsOffset\":{},\"fontFamily\":",glyph.font_size_mm100(),glyph.italic(),glyph.contents_offset()));
                push_json_string(&mut output, glyph.family());
                output.push('}');
            }
            output.push_str(
                "],\"decoded\":false,\"geometryDecoded\":false,\"baselineDecoded\":false}",
            );
        }
        output.push(']');
    }
    let figures = document.figure_shape_candidates();
    if !figures.is_empty() {
        output.push_str(",\"figureShapeCandidates\":[");
        for (index, shape) in figures.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str(&format!("{{\"objectId\":{},\"paintSlotCandidate\":{},\"lineMarkRecordIndex\":{},\"vectorOffset\":{},\"commandOffset\":{},\"strokeSourceUnits\":{},\"kind\":",shape.object_id(),shape.slot(),shape.line_mark_index(),shape.vector_offset(),shape.command_offset(),shape.stroke_units()));
            push_json_string(&mut output, shape.kind());
            output.push_str(",\"fillColor\":");
            if let Some(fill) = shape.fill() {
                push_json_string(&mut output, fill);
            } else {
                output.push_str("null");
            }
            let [x, y, width, height] = shape.frame_mm100();
            output.push_str(&format!(",\"frameMm100\":{{\"x\":{x},\"y\":{y},\"width\":{width},\"height\":{height}}},\"recordSourceSpan\":"));
            text_layout::push_text_source_span_json(&mut output, shape.record_span());
            output.push_str(
                ",\"decoded\":false,\"geometryDecoded\":false,\"paintOrderDecoded\":false}",
            );
        }
        output.push(']');
    }
    let image_frames = document.image_frame_candidates();
    if !image_frames.is_empty() {
        output.push_str(",\"imageFrameCandidates\":[");
        for (index, image) in image_frames.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            let [x, y, width, height] = image.geometry_mm100();
            output.push_str("{\"mode\":");
            push_json_string(&mut output, image.mode().as_str());
            output.push_str(&format!(",\"sourceFrame\":\"/Frame\",\"objectCandidateIndex\":{},\"payloadIndex\":{},\"geometryMm100\":{{\"x\":{x},\"y\":{y},\"width\":{width},\"height\":{height}}},\"wrapMarginMm100\":{},\"recordSourceSpan\":",image.image_index(),image.payload_index(),image.wrap_margin_mm100()));
            text_layout::push_text_source_span_json(&mut output, image.record_span());
            output.push_str(
                ",\"decoded\":false,\"geometryDecoded\":false,\"paintOrderDecoded\":false}",
            );
        }
        output.push(']');
    }
    let tatechuyoko = document.tatechuyoko_candidates();
    if !tatechuyoko.is_empty() {
        output.push_str(",\"tatechuyokoCandidates\":[");
        for (index, candidate) in tatechuyoko.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str("{\"text\":");
            push_json_string(&mut output, candidate.text());
            output.push_str(",\"recordSourceSpan\":");
            text_layout::push_text_source_span_json(&mut output, candidate.record_span());
            output.push_str(",\"valueSourceSpan\":");
            text_layout::push_text_source_span_json(&mut output, candidate.value_span());
            output.push_str(",\"decoded\":false,\"geometryDecoded\":false}");
        }
        output.push(']');
    }
    let headers = document.header_text_candidates();
    if !headers.is_empty() {
        output.push_str(",\"headerTextCandidates\":[");
        for (index, header) in headers.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str(&format!("{{\"sourceStream\":\"/Header\",\"slotId\":{},\"byteStart\":{},\"byteEnd\":{},\"text\":", header.slot_id(), header.byte_start(), header.byte_end()));
            push_json_string(&mut output, header.text());
            output.push_str(
                ",\"decoded\":false,\"slotRoleDecoded\":false,\"geometryDecoded\":false}",
            );
        }
        output.push(']');
    }
    let footnotes = document.footnote_text_candidates();
    if !footnotes.is_empty() {
        output.push_str(",\"footnoteTextCandidates\":[");
        for (index, note) in footnotes.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str("{\"sourceStream\":\"/Footnote\",\"marker\":");
            push_json_string(&mut output, note.marker());
            output.push_str(",\"text\":");
            push_json_string(&mut output, note.text());
            output.push_str(",\"sourceSpan\":");
            text_layout::push_text_source_span_json(&mut output, note.source_span());
            output.push_str(",\"bodyMarkerSourceSpan\":");
            text_layout::push_text_source_span_json(&mut output, note.body_marker_span());
            output.push_str(",\"noteMarkerSourceSpan\":");
            text_layout::push_text_source_span_json(&mut output, note.note_marker_span());
            output.push_str(&format!(",\"linkNoteAnchorOffset\":{},\"linkBodyRecordOffset\":{},\"decoded\":false,\"linkRolesDecoded\":false,\"placementDecoded\":false}}", note.note_anchor_offset(), note.body_record_offset()));
        }
        output.push(']');
    }
    let fields = document.text_field_candidates();
    if !fields.is_empty() {
        output.push_str(",\"textFieldCandidates\":[");
        for (index, field) in fields.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str("{\"kind\":");
            push_json_string(&mut output, field.kind().as_str());
            output.push_str(",\"cachedValue\":");
            push_json_string(&mut output, field.cached_value());
            output.push_str(",\"argument\":");
            push_json_string(&mut output, field.argument());
            output.push_str(",\"recordSourceSpan\":");
            text_layout::push_text_source_span_json(&mut output, field.record_span());
            output.push_str(",\"valueSourceSpan\":");
            text_layout::push_text_source_span_json(&mut output, field.value_span());
            output.push_str(",\"decoded\":false}");
        }
        output.push(']');
    }
    output.push_str(",\"pageMarks\":[");
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
