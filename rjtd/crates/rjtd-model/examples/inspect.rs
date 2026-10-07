//! Source-only model inspection; no page construction, fonts or rendering context.
use rjtd_model::{Block, Inline, parse_document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("usage: inspect <document.jtd>")?;
    let document = parse_document(&std::fs::read(path)?)?;
    println!(
        "blocks={} streams={} unknown_styles={} unknown_objects={}",
        document.blocks().len(),
        document.raw_streams().len(),
        document.unknown_styles().len(),
        document.unknown_objects().len()
    );
    println!(
        "source_units={:?}",
        document.document_text_flow().map(|flow| (
            flow.source_span().unit_start(),
            flow.source_span().unit_end(),
            flow.events().len()
        ))
    );
    println!(
        "size_mm100={:?} margins_mm100={:?} direction={:?}",
        document.page_size_mm100_candidate(),
        document.page_margins_mm100_candidate(),
        document.writing_mode_candidate()
    );
    println!(
        "section={:?} running={:?} rows={:?}",
        document.section_source_candidate(),
        document.running_text_source_candidate(),
        document.source_line_range_candidates()
    );
    println!(
        "fields={} notes={} images={} equations={} figures={}",
        document.text_field_candidates().len(),
        document.footnote_text_candidates().len(),
        document.image_frame_candidates().len(),
        document.equation_candidates().len(),
        document.figure_shape_candidates().len()
    );
    println!(
        "fixed_pitch={:?}",
        document.document_text_flow().map(|flow| flow
            .events()
            .iter()
            .filter_map(|event| event.fixed_pitch_mm100_candidate())
            .collect::<Vec<_>>())
    );
    for (index, block) in document.blocks().iter().enumerate() {
        if let Block::Paragraph(paragraph) = block {
            println!(
                "paragraph={index} attributes={:?}",
                document.paragraph_attribute_candidate(index)
            );
            for inline in paragraph.inlines() {
                if let Inline::Text(run) = inline
                    && let Some(span) = run.source_span()
                {
                    println!(
                        "span={:?} size={:?} style={:?} bgr={:?}",
                        span,
                        document.text_font_size_source_candidate(span),
                        document.text_character_style_source_candidate(span),
                        document.text_foreground_bgr24_candidate(span)
                    );
                }
            }
        }
    }
    Ok(())
}
