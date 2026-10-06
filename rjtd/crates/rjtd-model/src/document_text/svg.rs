use super::*;
use crate::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn render_text_page_svg(
    lines: &[PageTextLine],
    page_number: usize,
    _page_count: usize,
    layout: PageLayout,
    writing_mode: WritingMode,
    document: &Document,
    decoration: Option<&PageDecoration>,
    measured_widths: &BTreeMap<usize, f32>,
    print_date: Option<&str>,
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
    let style_resolver = document_text_style_resolver(document);
    let default_font_size = document_default_font_size_px(document);
    let fields = document.text_field_candidates();
    let shanai_lan_text_projection =
        shanai_lan_document_text_projection(document, layout, page_number);
    let form_projection = observed_form_text_projection(document, layout, page_number);
    let native_image = native_image_projection(
        document,
        layout,
        page_number,
        writing_mode,
        lines,
        measured_widths,
    )
    .filter(|_| shanai_lan_text_projection.is_none() && form_projection.is_none());
    let mut native_control_tables =
        native_control_table_text_projections(document, layout, page_number, writing_mode);
    let native_control_flow = native_control_flow_text_projection(
        document,
        layout,
        page_number,
        writing_mode,
        &native_control_tables,
    );
    let native_rule_flow = native_rule_flow_text_projection(
        document,
        layout,
        page_number,
        writing_mode,
        &native_control_tables,
        native_control_flow.as_ref(),
    )
    .filter(|_| shanai_lan_text_projection.is_none() && form_projection.is_none());
    let native_rule_borders =
        native_rule_border_projection(document, layout, page_number, writing_mode)
            .filter(|_| shanai_lan_text_projection.is_none() && form_projection.is_none());
    let native_figures = native_figure_projection(document, layout, page_number, writing_mode);
    if let Some(segments) = &native_rule_borders {
        for table in &mut native_control_tables {
            table.border = None;
        }
        push_native_rule_borders_svg(&mut svg, segments);
    }
    if native_image.is_none() && native_figures.is_none() {
        push_page_frame_projection_svg(&mut svg, layout, document, page_number);
    }
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
    let fdm_vector_primitives_rendered = if let Some(shapes) = &native_figures {
        push_native_figure_svg(&mut svg, shapes);
        true
    } else {
        push_fdm_vector_primitive_svg(&mut svg, layout, document, page_number)
    };

    if let Some(projection) = &shanai_lan_text_projection {
        push_shanai_lan_text_projection_svg(&mut svg, projection, &font_family);
    } else if let Some(projection) = &form_projection {
        push_observed_form_text_projection_svg(&mut svg, projection, &font_family);
    } else if native_figures.is_some() {
        // A fully source-bound drawing page does not need the no-text notice.
    } else if let Some(projection) = &native_image {
        push_native_image_svg(&mut svg, projection);
    } else if let Some(projection) =
        native_vertical_projection(document, layout, page_number, lines, writing_mode)
    {
        push_native_vertical_svg(&mut svg, &projection, &font_family);
    } else if writing_mode.is_vertical() {
        let placement = vertical_page_text_placement(layout, lines);
        svg.push_str("<g writing-mode=\"vertical-rl\" glyph-orientation-vertical=\"auto\">");
        for (index, line) in lines.iter().enumerate() {
            if line.text().is_empty() {
                continue;
            }

            let mut x =
                layout.width_px() - layout.margin_right_px() - (index as f32 * APP_LINE_HEIGHT_PX)
                    + placement.x_shift_px;
            let mut y = placement.y_start_px;
            if is_centered_ginga_title_page(page_number, line) {
                let line_extent = vertical_text_advance_px(line.text()) as f32;
                x = layout.width_px() / 2.0;
                y = ((layout.height_px() - line_extent) / 2.0).max(layout.margin_px());
            }

            for mut fragment in
                page_text_line_style_fragments(document, line, style_resolver.as_ref())
            {
                apply_print_date(&mut fragment, &fields, print_date);
                let field = fragment
                    .source_span
                    .as_ref()
                    .and_then(|span| field_for_span(&fields, span));
                let paint_span = field_paint_span(field, fragment.source_span.as_ref());
                if fragment.text.is_empty() {
                    continue;
                }
                let source_color = style_resolver
                    .as_ref()
                    .zip(paint_span.as_ref())
                    .and_then(|(resolver, span)| document_text_foreground_color(resolver, span));
                let fill_color = source_color
                    .as_deref()
                    .unwrap_or(fallback_text_fill_color());
                let font_size = style_resolver
                    .as_ref()
                    .zip(fragment.source_span.as_ref())
                    .and_then(|(resolver, span)| {
                        document_text_font_size(resolver, span, default_font_size)
                    })
                    .map(|size| size.px)
                    .unwrap_or(APP_FONT_SIZE_PX);
                let mut character_style = style_resolver
                    .as_ref()
                    .zip(paint_span.as_ref())
                    .map(|(resolver, span)| document_text_character_style(document, resolver, span))
                    .unwrap_or_default();
                character_style.script = None;
                character_style.source_unit_start = fragment
                    .source_span
                    .as_ref()
                    .map(TextSourceSpan::unit_start);
                let run_font_family = character_style
                    .font
                    .as_ref()
                    .map_or(font_family.as_str(), |(_, family)| family.as_str());

                push_svg_text_run(
                    &mut svg,
                    "rjtd-text",
                    x,
                    y,
                    run_font_family,
                    font_size,
                    fill_color,
                    &fragment.text,
                    Some("vertical-rl"),
                    Some(&character_style),
                    field,
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
                y += vertical_text_advance_px(&fragment.text) as f32 * font_size / APP_FONT_SIZE_PX;
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
                .or_else(|| native_paragraph_line_x(document, layout, line))
                .or_else(|| text_origin.map(|origin| origin.0))
                .unwrap_or_else(|| layout.margin_left_px());
            let y = frame_text_placement
                .map(|placement| placement.baseline as f32)
                .or_else(|| {
                    line.native_line_mark_index
                        .and_then(|record| native_rule_line_placement(document, layout, record))
                        .filter(|(page, _, _)| *page == page_number)
                        .map(|(_, top, _)| top + APP_FONT_SIZE_PX)
                })
                .unwrap_or_else(|| {
                    text_origin
                        .map(|origin| origin.1)
                        .unwrap_or_else(|| layout.margin_top_px())
                        + APP_FONT_SIZE_PX
                        + (index as f32 * APP_LINE_HEIGHT_PX)
                });
            let fragments = page_text_line_style_fragments(document, line, style_resolver.as_ref());
            let line_font_size =
                text_line_font_size(style_resolver.as_ref(), &fragments, default_font_size);
            for mut fragment in fragments {
                apply_print_date(&mut fragment, &fields, print_date);
                let field = fragment
                    .source_span
                    .as_ref()
                    .and_then(|span| field_for_span(&fields, span));
                let paint_span = field_paint_span(field, fragment.source_span.as_ref());
                if fragment.text.is_empty() {
                    continue;
                }
                if fragment_overlaps_rendered_table_projection(
                    layout,
                    document,
                    lines,
                    page_number,
                    &fragment,
                    &native_control_tables,
                    [native_rule_flow.as_ref(), native_control_flow.as_ref()],
                ) {
                    continue;
                }
                if fragment.source_span.as_ref().is_some_and(|span| {
                    native_control_table_text_projection_contains(&native_control_tables, span)
                        || native_control_flow_text_projection_contains(
                            native_control_flow.as_ref(),
                            span,
                        )
                        || native_control_flow_text_projection_contains(
                            native_rule_flow.as_ref(),
                            span,
                        )
                }) {
                    continue;
                }
                let unscaled_font_size = style_resolver
                    .as_ref()
                    .zip(fragment.source_span.as_ref())
                    .and_then(|(resolver, span)| {
                        document_text_font_size(resolver, span, default_font_size)
                    })
                    .map(|size| size.px)
                    .unwrap_or(APP_FONT_SIZE_PX);
                let mut character_style = style_resolver
                    .as_ref()
                    .zip(paint_span.as_ref())
                    .map(|(resolver, span)| document_text_character_style(document, resolver, span))
                    .unwrap_or_default();
                character_style.source_unit_start = fragment
                    .source_span
                    .as_ref()
                    .map(TextSourceSpan::unit_start);
                let run_font_family = character_style
                    .font
                    .as_ref()
                    .map_or(font_family.as_str(), |(_, family)| family.as_str());
                let font_size = unscaled_font_size * character_style.font_scale();
                let width = fragment
                    .source_span
                    .as_ref()
                    .and_then(|span| measured_widths.get(&span.unit_start()))
                    .copied()
                    .filter(|width| width.is_finite() && *width > 0.0)
                    .unwrap_or_else(|| {
                        text_width_px(layout, &fragment.text) as f32 * font_size / APP_FONT_SIZE_PX
                    });
                let baseline = fragment
                    .source_span
                    .as_ref()
                    .and_then(|span| {
                        if line.native_line_mark_index.is_some() {
                            return None;
                        }
                        native_rule_body_top_y(
                            document,
                            layout,
                            native_rule_flow.is_some()
                                || native_rule_borders.is_some()
                                || !native_control_tables.is_empty(),
                            span,
                        )
                    })
                    .map(|top| top + line_font_size)
                    .unwrap_or(y + line_font_size - APP_FONT_SIZE_PX)
                    + character_style.baseline_shift(unscaled_font_size);
                let source_color = style_resolver
                    .as_ref()
                    .zip(paint_span.as_ref())
                    .and_then(|(resolver, span)| document_text_foreground_color(resolver, span));
                let fill_color = source_color
                    .as_deref()
                    .unwrap_or(fallback_text_fill_color());
                push_svg_text_run(
                    &mut svg,
                    "rjtd-text",
                    x,
                    baseline,
                    run_font_family,
                    font_size,
                    fill_color,
                    &fragment.text,
                    None,
                    Some(&character_style),
                    field,
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
    push_native_control_flow_text_svg(
        &mut svg,
        native_control_flow.as_ref(),
        &font_family,
        measured_widths,
    );
    push_native_control_flow_text_svg(
        &mut svg,
        native_rule_flow.as_ref(),
        &font_family,
        measured_widths,
    );
    push_native_control_table_text_svg(&mut svg, &native_control_tables, &font_family);
    if let Some(projection) = layout_box_text_projection(document, layout, page_number) {
        push_layout_box_text_projection_svg(&mut svg, &projection, &font_family);
    }
    if let Some(decoration) = decoration {
        push_page_decoration_svg(&mut svg, layout, writing_mode, decoration, &font_family);
    }
    if let Some(items) = native_running_text(document, layout, writing_mode, page_number) {
        push_native_running_svg(&mut svg, &items, &font_family);
    }
    push_success_data_test_cone_diagram_projection_svg(
        &mut svg,
        layout,
        document,
        page_number,
        &font_family,
    );
    push_table_grid_candidate_svg(
        &mut svg,
        layout,
        document,
        lines,
        page_number,
        &native_control_tables,
        [native_rule_flow.as_ref(), native_control_flow.as_ref()],
    );
    if native_image.is_none() {
        push_image_payload_diagnostic_svg(&mut svg, layout, document, page_number);
    }
    if !fdm_vector_primitives_rendered && native_image.is_none() {
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
    character_style: Option<&DocumentTextCharacterStyle>,
    field: Option<&DocumentTextFieldCandidate>,
) {
    let visual_text = escape_xml(&svg_visual_text(text));
    let font_family = escape_xml(font_family);
    let writing_mode_attr = writing_mode
        .map(|mode| format!(" writing-mode=\"{mode}\""))
        .unwrap_or_default();
    let mut style_attrs = String::new();
    if let Some(field) = field {
        style_attrs.push_str(&format!(" data-field-kind-candidate=\"{}\" data-field-decoded=\"false\" data-cached-field-text=\"{}\"",field.kind().as_str(),escape_xml(field.cached_value())));
        if field.kind() == DocumentTextFieldKind::Hyperlink {
            svg.push_str(&format!(
                "<a href=\"{}\" target=\"_blank\" rel=\"noopener\">",
                escape_xml(field.argument())
            ));
        }
    }
    if let Some(style) = character_style {
        if let Some(unit) = style.source_unit_start {
            style_attrs.push_str(&format!(" id=\"rjtd-text-advance-{unit}\" data-source-unit-start=\"{unit}\" data-text-advance-candidate=\"true\""));
        }
        if style.bold {
            // Regular-only CJK faces have no bold/italic variant in the SVG
            // PDF backend. Use bounded synthetic paint in every backend;
            // its strength/angle are renderer approximations, not decoded units.
            style_attrs.push_str(&format!(" font-weight=\"normal\" stroke=\"{fill}\" stroke-width=\"{:.3}\" stroke-linejoin=\"round\" data-bold-paint-candidate=\"true\"", font_size * 0.025));
        }
        if style.italic {
            style_attrs.push_str(&format!(" font-style=\"normal\" transform=\"matrix(1 0 -0.25 1 {:.3} 0)\" data-italic-paint-candidate=\"true\"", y * 0.25));
        }
        if style.underline {
            style_attrs.push_str(" text-decoration=\"underline\"");
        }
        if let Some(flags) = style.flags {
            style_attrs.push_str(&format!(
                " data-style-flags=\"0x{flags:08x}\" data-style-decoded=\"false\""
            ));
        }
        if let Some(script) = style.script {
            style_attrs.push_str(&format!(" data-script-candidate=\"{script}\""));
        }
        if let Some((id, _)) = &style.font {
            style_attrs.push_str(&format!(
                " data-font-id-candidate=\"{id}\" data-font-decoded=\"false\""
            ));
        }
    }
    svg.push_str(&format!(
        "<text class=\"{class_name}\" x=\"{x:.1}\" y=\"{y:.1}\" font-family=\"{font_family}\" font-size=\"{font_size:.1}\" fill=\"{fill}\" letter-spacing=\"0\" xml:space=\"preserve\"{writing_mode_attr}{style_attrs}>{visual_text}</text>"
    ));
    if field.is_some_and(|field| field.kind() == DocumentTextFieldKind::Hyperlink) {
        svg.push_str("</a>");
    }
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
