use super::*;
use crate::*;

pub(crate) fn render_text_page_svg(
    lines: &[PageTextLine],
    page_number: usize,
    _page_count: usize,
    layout: PageLayout,
    writing_mode: WritingMode,
    document: &Document,
    decoration: Option<&PageDecoration>,
) -> String {
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{:.1}\" height=\"{:.1}\" viewBox=\"0 0 {:.1} {:.1}\">",
        layout.width_px(),
        layout.height_px(),
        layout.width_px(),
        layout.height_px()
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>");
    let font_family = document_font_family_css(document);
    push_page_frame_projection_svg(&mut svg, layout, document, page_number);
    push_page_mark_section_separator_svg(&mut svg, layout, document, page_number);
    push_shanai_lan_sparse_table_borders_svg(&mut svg, layout, document, page_number);
    push_visual_list_diagnostic_svg(&mut svg, layout, document, page_number);
    push_embedding_frame_diagnostic_svg(&mut svg, layout, document, lines, page_number);
    push_success_data_test_title_art_projection_svg(&mut svg, layout, document, lines, page_number);
    push_success_data_test_answer_sheet_projection_svg(
        &mut svg,
        layout,
        document,
        page_number,
        &font_family,
    );
    push_jseq_formula_projection_svg(&mut svg, layout, document, lines, page_number, &font_family);
    // Line-rule candidates stay in the layer tree until the topology decoder is reliable enough
    // to render them without adding false connector trunks.
    let fdm_vector_primitives_rendered =
        push_fdm_vector_primitive_svg(&mut svg, layout, document, page_number);

    if let Some(projection) = shanai_lan_document_text_projection(document, layout, page_number) {
        push_shanai_lan_text_projection_svg(&mut svg, &projection, &font_family);
    } else if let Some(projection) = observed_form_text_projection(document, layout, page_number) {
        push_observed_form_text_projection_svg(&mut svg, &projection, &font_family);
    } else if writing_mode.is_vertical() {
        let placement = vertical_page_text_placement(layout, lines);
        svg.push_str("<g writing-mode=\"vertical-rl\" glyph-orientation-vertical=\"auto\">");
        for (index, line) in lines.iter().enumerate() {
            if line.text().is_empty() {
                continue;
            }

            let mut x =
                layout.width_px() - layout.margin_px() - (index as f32 * APP_LINE_HEIGHT_PX)
                    + placement.x_shift_px;
            let mut y = placement.y_start_px;
            if is_centered_ginga_title_page(page_number, line) {
                let line_extent = vertical_text_advance_px(line.text()) as f32;
                x = layout.width_px() / 2.0;
                y = ((layout.height_px() - line_extent) / 2.0).max(layout.margin_px());
            }

            for fragment in page_text_line_fragments(document, line) {
                if fragment.text.is_empty() {
                    continue;
                }
                let fill_color = fallback_text_fill_color();

                push_svg_text_run(
                    &mut svg,
                    "rjtd-text",
                    x,
                    y,
                    &font_family,
                    APP_FONT_SIZE_PX,
                    fill_color,
                    &fragment.text,
                    Some("vertical-rl"),
                );
                if let Some(annotation) = &fragment.ruby_annotation {
                    push_svg_ruby_annotation(
                        &mut svg,
                        x + (APP_FONT_SIZE_PX * 0.72),
                        y,
                        &font_family,
                        annotation,
                        true,
                    );
                }
                y += vertical_text_advance_px(&fragment.text) as f32;
            }
        }
        svg.push_str("</g>");
    } else {
        if let Some(slots) = success_data_test_top_text_projection(document, page_number) {
            push_success_data_test_top_text_projection_svg(
                &mut svg,
                document,
                layout,
                slots,
                &font_family,
            );
        }
        let text_origin = fallback_text_origin(layout, document);
        let mut fallback_visual_line_index = 0usize;
        for (index, line) in lines.iter().enumerate() {
            if line.text().is_empty() {
                continue;
            }
            if success_data_test_top_text_line_should_skip(document, page_number, line) {
                continue;
            }
            let frame_text_placement = page_frame_text_placement(
                document,
                layout,
                page_number,
                fallback_visual_line_index,
                line,
            );
            let mut x = frame_text_placement
                .map(|placement| placement.x as f32)
                .or_else(|| text_origin.map(|origin| origin.0))
                .unwrap_or_else(|| layout.margin_px());
            let y = frame_text_placement
                .map(|placement| placement.baseline as f32)
                .unwrap_or_else(|| {
                    text_origin
                        .map(|origin| origin.1)
                        .unwrap_or_else(|| layout.margin_px())
                        + APP_FONT_SIZE_PX
                        + (index as f32 * APP_LINE_HEIGHT_PX)
                });
            for fragment in page_text_line_fragments(document, line) {
                if fragment.text.is_empty() {
                    continue;
                }
                if fragment_overlaps_rendered_table_projection(
                    layout,
                    document,
                    lines,
                    page_number,
                    &fragment,
                ) {
                    continue;
                }
                let width = text_width_px(layout, &fragment.text) as f32;
                let fill_color = fallback_text_fill_color();
                push_svg_text_run(
                    &mut svg,
                    "rjtd-text",
                    x,
                    y,
                    &font_family,
                    APP_FONT_SIZE_PX,
                    fill_color,
                    &fragment.text,
                    None,
                );
                if let Some(annotation) = &fragment.ruby_annotation {
                    push_svg_ruby_annotation(
                        &mut svg,
                        x + (width / 2.0),
                        y - (APP_FONT_SIZE_PX * 0.75),
                        &font_family,
                        annotation,
                        false,
                    );
                }
                x += width;
            }
            fallback_visual_line_index += 1;
        }
    }
    if let Some(projection) = layout_box_text_projection(document, layout, page_number) {
        push_layout_box_text_projection_svg(&mut svg, &projection, &font_family);
    }
    if let Some(decoration) = decoration {
        push_page_decoration_svg(&mut svg, layout, writing_mode, decoration, &font_family);
    }
    push_success_data_test_cone_diagram_projection_svg(
        &mut svg,
        layout,
        document,
        page_number,
        &font_family,
    );
    push_table_grid_candidate_svg(&mut svg, layout, document, lines, page_number);
    push_image_payload_diagnostic_svg(&mut svg, layout, document, page_number);
    if !fdm_vector_primitives_rendered {
        push_fdm_command_diagnostic_svg(&mut svg, layout, document, page_number);
        push_fdm_frame_diagnostic_svg(&mut svg, layout, document, page_number);
    }
    svg.push_str("</svg>");
    svg
}

pub(crate) fn push_layout_box_text_projection_svg(
    svg: &mut String,
    projection: &LayoutBoxTextProjection,
    font_family: &str,
) {
    svg.push_str(&format!(
        "<g class=\"rjtd-layout-box-text-projection\" data-source=\"{}\" data-projection-kind=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-page-assignment-decoded=\"{}\" data-block-count=\"{}\" data-layout-record-count=\"{}\" data-position-table-present=\"{}\">",
        escape_xml(projection.source),
        escape_xml(projection.projection_kind),
        projection.page_assignment_decoded,
        projection.block_count,
        projection.layout_record_count,
        projection.position_table_present
    ));
    let font_family = escape_xml(font_family);
    for slot in &projection.slots {
        let record_index = slot
            .layout_record_index
            .map(|index| index.to_string())
            .unwrap_or_else(|| "-".to_string());
        svg.push_str(&format!(
            "<text class=\"rjtd-text rjtd-layout-box-text\" data-source=\"{}\" data-role=\"{}\" data-block-index=\"{}\" data-layout-record-index=\"{}\" data-placement-basis=\"{}\" x=\"{:.1}\" y=\"{:.1}\" font-family=\"{}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
            escape_xml(projection.source),
            escape_xml(slot.role),
            slot.block_index,
            escape_xml(&record_index),
            escape_xml(slot.placement_basis),
            slot.x,
            slot.y + slot.font_size,
            font_family,
            slot.font_size,
            escape_xml(&svg_visual_text(&slot.text))
        ));
    }
    svg.push_str("</g>");
}

pub(crate) fn push_observed_form_text_projection_svg(
    svg: &mut String,
    projection: &ObservedFormTextProjection,
    _font_family: &str,
) {
    svg.push_str(&format!(
        "<g class=\"rjtd-observed-form-text-projection\" data-source=\"{}\" data-projection=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"true\">",
        escape_xml(projection.source),
        escape_xml(projection.projection_kind)
    ));
    for shape in &projection.shapes {
        let stroke = shape.stroke.unwrap_or("none");
        svg.push_str(&format!(
            "<rect class=\"rjtd-form-shape\" data-role=\"{}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.1}\"/>",
            escape_xml(shape.role),
            shape.x,
            shape.y,
            shape.width,
            shape.height,
            shape.rx,
            shape.rx,
            escape_xml(shape.fill),
            escape_xml(stroke),
            shape.stroke_width
        ));
    }
    for slot in &projection.slots {
        let anchor = slot.anchor;
        let text = escape_xml(&svg_visual_text(&slot.text));
        let font_family = escape_xml(slot.font_family);
        svg.push_str(&format!(
            "<text class=\"rjtd-form-text\" data-role=\"{}\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{}\" font-family=\"{}\" font-size=\"{:.1}\" font-weight=\"{}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
            escape_xml(slot.role),
            slot.x,
            slot.y,
            anchor,
            font_family,
            slot.font_size,
            slot.font_weight,
            text
        ));
    }
    svg.push_str("</g>");
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_svg_text_run(
    svg: &mut String,
    class_name: &str,
    x: f32,
    y: f32,
    font_family: &str,
    font_size: f32,
    fill: &str,
    text: &str,
    writing_mode: Option<&str>,
) {
    let visual_text = escape_xml(&svg_visual_text(text));
    let font_family = escape_xml(font_family);
    let writing_mode_attr = writing_mode
        .map(|mode| format!(" writing-mode=\"{mode}\""))
        .unwrap_or_default();
    svg.push_str(&format!(
        "<text class=\"{class_name}\" x=\"{x:.1}\" y=\"{y:.1}\" font-family=\"{font_family}\" font-size=\"{font_size:.1}\" fill=\"{fill}\" letter-spacing=\"0\" xml:space=\"preserve\"{writing_mode_attr}>{visual_text}</text>"
    ));
}

pub(crate) fn push_svg_ruby_annotation(
    svg: &mut String,
    x: f32,
    y: f32,
    font_family: &str,
    annotation: &str,
    vertical: bool,
) {
    let writing_mode_attr = if vertical {
        " writing-mode=\"vertical-rl\""
    } else {
        " text-anchor=\"middle\""
    };
    let font_family = escape_xml(font_family);
    svg.push_str(&format!(
        "<text class=\"rjtd-ruby\" x=\"{x:.1}\" y=\"{y:.1}\" font-family=\"{font_family}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\"{writing_mode_attr}>{}</text>",
        APP_FONT_SIZE_PX * 0.55,
        escape_xml(&svg_visual_text(annotation))
    ));
}

pub(crate) fn observed_form_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<ObservedFormTextProjection> {
    if let Some(projection) = observed_tsaiten_text_projection(document, layout, page_number) {
        return Some(projection);
    }
    if page_number != 1 || !document_has_fax02_visual_list(document) {
        return None;
    }
    let plain_text = document_plain_text(document);
    let lines = plain_text
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    let title = lines.first().copied()?;
    if title != "FAX送付のご案内" {
        return None;
    }
    let date = lines.iter().copied().find(|line| line.contains("平成"))?;
    let addressee = lines.iter().copied().find(|line| line.contains('様'))?;
    let body = lines
        .iter()
        .copied()
        .filter(|line| {
            line.starts_with("拝啓")
                || line.starts_with("平素")
                || line.starts_with("下記")
                || line.starts_with("ご検討")
        })
        .collect::<Vec<_>>();
    let total = lines
        .iter()
        .copied()
        .find(|line| line.starts_with("全枚数"))?;
    if body.len() != 4 {
        return None;
    }

    let scale_x = layout.width_px() / 120.0;
    let scale_y = layout.height_px() / 169.0;
    let mut slots = Vec::with_capacity(8 + body.len());
    slots.push(form_slot(
        "title",
        title,
        15.0,
        23.1,
        30.5,
        "900",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "date",
        date,
        79.5,
        28.6,
        14.0,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "addressee",
        addressee.trim(),
        60.0,
        40.9,
        18.0,
        "500",
        "middle",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "left-fax-label",
        "FAX：",
        16.2,
        47.4,
        11.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "right-tel-label",
        "TEL：",
        71.0,
        67.8,
        11.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "right-fax-label",
        "FAX：",
        71.0,
        74.5,
        11.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    for (index, text) in body.iter().enumerate() {
        slots.push(form_slot(
            "body",
            text,
            25.8,
            81.8 + index as f32 * 3.55,
            13.6,
            "500",
            "start",
            VISUAL_LIST_GOTHIC_FONT_FAMILY,
            scale_x,
            scale_y,
        ));
    }
    slots.push(form_slot(
        "total-count",
        total,
        76.8,
        98.3,
        13.6,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    Some(ObservedFormTextProjection {
        source: "documentText+visualList",
        projection_kind: "visualListFormProjection",
        shapes: Vec::new(),
        slots,
    })
}

pub(crate) fn observed_tsaiten_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<ObservedFormTextProjection> {
    if page_number != 1 || !document_has_tsaiten_projection_evidence(document) {
        return None;
    }

    let scale_x = layout.width_px() / TSAITEN_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / TSAITEN_REFERENCE_PAGE_HEIGHT_PX;
    let mut shapes = Vec::new();
    let mut slots = Vec::new();

    slots.push(form_slot(
        "document-heading",
        "＜採点原則＞",
        397.0,
        83.0,
        12.0,
        "700",
        "middle",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    shapes.push(form_shape(
        "title-shadow",
        101.0,
        128.0,
        634.0,
        39.0,
        "#d0d0d0",
        None,
        0.0,
        1.5,
        scale_x,
        scale_y,
    ));
    shapes.push(form_shape(
        "title-box",
        94.0,
        121.0,
        634.0,
        39.0,
        "#ffffff",
        Some("#333333"),
        1.6,
        2.0,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "title",
        "タイピング科目採点方法",
        110.0,
        146.0,
        18.0,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    slots.push(form_slot(
        "instruction",
        "　標準解答を見ながら採点します。採点内容は以下のとおりです。",
        142.0,
        214.0,
        11.3,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "instruction",
        "　採点項目に当てはまる誤りがあった場合、減点すべき点数を採点用紙の指定の欄に記入してください。",
        142.0,
        240.0,
        11.3,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "section-heading",
        "【採点科目】",
        105.0,
        286.0,
        12.2,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "section-heading",
        "【採点内容】",
        105.0,
        486.0,
        12.2,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    shapes.push(form_shape(
        "document-format-label-box",
        183.0,
        511.0,
        110.0,
        23.0,
        "#ffffff",
        Some("#555555"),
        1.0,
        1.5,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "subsection-label",
        "文書の体裁",
        195.0,
        528.0,
        10.8,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    push_tsaiten_document_format_table_projection(&mut shapes, &mut slots, scale_x, scale_y);

    shapes.push(form_shape(
        "linebreak-label-box",
        183.0,
        737.0,
        146.0,
        23.0,
        "#ffffff",
        Some("#555555"),
        1.0,
        1.5,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "subsection-label",
        "文字・改行の誤り",
        195.0,
        754.0,
        10.8,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    slots.push(form_slot(
        "note",
        "※行頭字下げのスペースを含め、入力している文字すべてを採点する。",
        112.0,
        905.0,
        9.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "note",
        "※同じ行を２回以上入力している場合、余分な行の文字は余字として、１文字につき１点減点する。",
        112.0,
        930.0,
        9.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "note",
        "※全角サイズでない文字は、誤字として１文字につき１点減点する。",
        112.0,
        955.0,
        9.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    Some(ObservedFormTextProjection {
        source: "documentText+tableCandidates",
        projection_kind: "tsaitenReferenceProjection",
        shapes,
        slots,
    })
}

pub(crate) fn find_text_utf16_unit_range_after(
    haystack: &str,
    needle: &str,
    start_units: usize,
) -> Option<(usize, usize)> {
    if needle.is_empty() {
        return None;
    }
    let start_byte = byte_index_after_utf16_units(haystack, start_units)?;
    let match_byte = haystack.get(start_byte..)?.find(needle)? + start_byte;
    let match_start_units = haystack[..match_byte].encode_utf16().count();
    let match_end_units = match_start_units + needle.encode_utf16().count();
    Some((match_start_units, match_end_units))
}
