use super::*;
use crate::*;

pub(crate) fn push_visual_list_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    for diagnostic in visual_list_diagnostics(document) {
        let runs = visual_list_horizontal_runs(diagnostic.visual_list);
        if runs.is_empty() {
            continue;
        }
        let scale_x = layout.width_px() / diagnostic.visual_list.width() as f32;
        let scale_y = layout.height_px() / diagnostic.visual_list.height() as f32;
        svg.push_str(&format!(
            "<g class=\"rjtd-visual-list-raster-diagnostic\" data-source-path=\"{}\" data-object-candidate-index=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"true\" data-renderable=\"true\" data-format=\"BMDV\" data-projection=\"rle8-raster\" data-fallback-projection=\"horizontal-runs\" data-run-count=\"{}\">",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.candidate_index,
            runs.len()
        ));
        let suppress_dark_foreground =
            observed_form_text_projection(document, layout, page_number).is_some();
        let raster_data_uri =
            visual_list_svg_data_uri(diagnostic.visual_list, suppress_dark_foreground);
        if let Some(data_uri) = raster_data_uri.as_ref() {
            let width = layout.width_px();
            let height = layout.height_px();
            svg.push_str(&format!(
                "<image class=\"rjtd-visual-list-rle8-raster\" data-projection=\"visualListRle8RasterImage\" data-suppressed-dark-foreground=\"{suppress_dark_foreground}\" x=\"0\" y=\"0\" width=\"{width:.1}\" height=\"{height:.1}\" preserveAspectRatio=\"none\" href=\"{data_uri}\" xlink:href=\"{data_uri}\"/>"
            ));
        } else {
            if let Some(band) = visual_list_title_band(diagnostic.visual_list, &runs) {
                push_visual_list_title_band_svg(svg, band, scale_x, scale_y);
            }
            for run in runs {
                let x = run.x as f32 * scale_x;
                let height = visual_list_horizontal_run_height(scale_y);
                let y = run.y as f32 * scale_y + ((scale_y - height) / 2.0);
                let width = (run.width as f32 * scale_x).max(0.8);
                let fill = visual_list_svg_gray(run.value);
                svg.push_str(&format!(
                    "<rect class=\"rjtd-visual-list-horizontal-run\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" fill=\"{fill}\" opacity=\"0.82\"/>"
                ));
            }
        }
        svg.push_str("</g>");
    }
}

pub(crate) fn push_visual_list_title_band_svg(
    svg: &mut String,
    band: VisualListTitleBand,
    scale_x: f32,
    scale_y: f32,
) {
    let x = band.x * scale_x;
    let y = band.y * scale_y;
    let width = band.width * scale_x;
    let height = band.height * scale_y;
    svg.push_str(&format!(
        "<g class=\"rjtd-visual-list-fill-band\" data-projection=\"visualListTitleBandHatch\"><rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" fill=\"#eeeeee\" opacity=\"0.95\"/>"
    ));
    let stripe_pitch = scale_x.max(2.8);
    let stripe_width = (scale_x * 0.28).clamp(0.8, 1.6);
    let stripe_count = (width / stripe_pitch).ceil() as usize;
    for index in 0..stripe_count {
        let stripe_x = x + index as f32 * stripe_pitch;
        svg.push_str(&format!(
            "<rect x=\"{stripe_x:.1}\" y=\"{y:.1}\" width=\"{stripe_width:.1}\" height=\"{height:.1}\" fill=\"#d5d5d5\" opacity=\"0.72\"/>"
        ));
    }
    svg.push_str("</g>");
}

pub(crate) fn push_embedding_frame_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    let diagnostics = embedding_frame_diagnostics(document);
    if diagnostics.is_empty() {
        return;
    }
    let renderable_diagnostics = diagnostics
        .into_iter()
        .filter_map(|diagnostic| {
            let bbox = embedding_frame_render_bbox(layout, lines, document, diagnostic)?;
            embedding_frame_snapshot_vector_renderable(diagnostic).then_some((diagnostic, bbox))
        })
        .collect::<Vec<_>>();
    if renderable_diagnostics.is_empty() {
        return;
    }

    svg.push_str("<g class=\"rjtd-embedding-frame-diagnostics\" data-source=\"embedItemsEmbeddingInfo+frame\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\">");
    for (diagnostic, (x, y, width, height)) in renderable_diagnostics {
        let linked_jseq3 = diagnostic.jseq3_formula.is_some();
        let snapshot_renderable = embedding_frame_snapshot_vector_renderable(diagnostic);
        svg.push_str(&format!(
            "<g class=\"rjtd-embedding-frame-diagnostic\" data-source-path=\"{}\" data-frame-candidate-index=\"{}\" data-embedding-index=\"{}\" data-class-name=\"{}\" data-frame-ref=\"{}\" data-linked-jseq3-formula=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"{}\">",
            escape_xml(diagnostic.frame.source_path()),
            diagnostic.frame_index,
            diagnostic.frame.embedding_index(),
            escape_xml(diagnostic.frame.class_name()),
            diagnostic.frame.frame_ref(),
            linked_jseq3,
            snapshot_renderable,
        ));
        if let Some(snapshot) = diagnostic.embedded_press_snapshot.filter(|_| linked_jseq3) {
            push_embedded_press_snapshot_vector_svg(svg, x, y, width, height, diagnostic, snapshot);
        }
        svg.push_str("</g>");
    }
    svg.push_str("</g>");
}

pub(crate) fn push_jseq_formula_projection_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    page_number: usize,
    font_family: &str,
) {
    if page_number != 1 {
        return;
    }

    for diagnostic in embedding_frame_diagnostics(document) {
        if diagnostic.jseq3_formula.is_none() {
            continue;
        }
        let Some(snapshot) = diagnostic.embedded_press_snapshot else {
            continue;
        };
        if snapshot.vector_paths().is_empty() || snapshot.width() == 0 || snapshot.height() == 0 {
            continue;
        }
        let Some((x, y, width, height)) =
            embedding_frame_render_bbox(layout, lines, document, diagnostic)
        else {
            continue;
        };
        let formula_y_anchor = success_data_test_jseq_formula_source_top_y(
            document,
            layout,
            diagnostic.frame.frame_ref(),
        );
        let formula_y_anchor_attrs = formula_y_anchor
            .as_ref()
            .map(|anchor| {
                format!(
                    " data-frame-y-basis=\"topTextSourceGrid\" data-frame-y-source-record-index=\"{}\" data-frame-y-source-top=\"{:.3}\" data-frame-y-top-offset=\"{:.3}\"",
                    anchor.source_record_index, anchor.source_top_y, anchor.top_offset
                )
            })
            .unwrap_or_else(|| " data-frame-y-basis=\"lineAnchorFallback\"".to_string());
        let scale_x = width / snapshot.width() as f32;
        let scale_y = height / snapshot.height() as f32;
        let vector_alignment = diagnostic
            .jseq3_formula
            .and_then(|formula| jseq_formula_vector_alignment(formula, scale_x, scale_y));
        let vector_dx = vector_alignment.map_or(0.0, |alignment| alignment.dx);
        let vector_dy = vector_alignment.map_or(0.0, |alignment| alignment.dy);
        let vector_cell_unit = vector_alignment.map_or(0.0, |alignment| alignment.cell_unit);
        let vector_path_stroke_source_unit =
            vector_alignment.map_or(0.0, |alignment| alignment.path_stroke_source_unit);
        let vector_path_stroke_width =
            vector_alignment.map_or(0.0, |alignment| alignment.path_stroke_width);
        let vector_x = x + vector_dx;
        let vector_y = y + vector_dy;
        let clip_width = width + vector_dx.max(0.0);
        let clip_height = height + vector_dy.max(0.0);
        let clip_id = format!(
            "rjtd-jseq-formula-clip-{}",
            diagnostic.frame.embedding_index()
        );
        svg.push_str(&format!(
            "<g class=\"rjtd-jseq-formula-projection\" data-source=\"jseq3EmbeddedPressSnapshot\" data-projection=\"jseqFormulaPathProjection\" data-embedding-index=\"{}\" data-frame-ref=\"{}\" data-vector-path-count=\"{}\" data-vector-segment-count=\"{}\"{formula_y_anchor_attrs} data-vector-bearing-source=\"jseq3TextRunContextCellMetric\" data-vector-bearing-cell-unit=\"{vector_cell_unit:.2}\" data-vector-bearing-dx=\"{vector_dx:.2}\" data-vector-bearing-dy=\"{vector_dy:.2}\" data-vector-path-stroke-source-unit=\"{vector_path_stroke_source_unit:.2}\" data-vector-path-stroke-width=\"{vector_path_stroke_width:.2}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"true\" data-reference-backed=\"true\">",
            diagnostic.frame.embedding_index(),
            diagnostic.frame.frame_ref(),
            snapshot.vector_paths().len(),
            snapshot.vector_segments().len()
        ));
        svg.push_str(&format!(
            "<defs><clipPath id=\"{}\"><rect x=\"{x:.2}\" y=\"{y:.2}\" width=\"{clip_width:.2}\" height=\"{clip_height:.2}\"/></clipPath></defs>",
            escape_xml(&clip_id)
        ));
        push_jseq_formula_vector_segment_svg(
            svg, snapshot, &clip_id, vector_x, vector_y, scale_x, scale_y,
        );
        svg.push_str(&format!(
            "<g class=\"rjtd-jseq-formula-paths\" clip-path=\"url(#{})\">",
            escape_xml(&clip_id)
        ));
        for path in snapshot.vector_paths() {
            push_embedded_press_vector_path_svg_with_stroke(
                svg,
                "rjtd-jseq-formula-path",
                path,
                EmbeddedPressPageContext {
                    x: vector_x,
                    y: vector_y,
                    scale_x,
                    scale_y,
                },
                "#111111",
                "evenodd",
                "#111111",
                vector_path_stroke_width,
                None,
            );
        }
        svg.push_str("</g></g>");
        if let Some(slots) = success_data_test_formula_text_slots(document, diagnostic) {
            svg.push_str(&format!(
                "<g class=\"rjtd-jseq-formula-text-projection\" data-source=\"jseq3ContentsTextTokens\" data-projection=\"jseqFormulaTextTokenProjection\" data-embedding-index=\"{}\" data-text-token-count=\"{}\" data-decoded=\"false\" data-placement-proven=\"false\" data-reference-backed=\"true\">",
                diagnostic.frame.embedding_index(),
                diagnostic
                    .jseq3_formula
                    .map(|formula| formula.text_tokens().len())
                    .unwrap_or_default()
            ));
            for slot in slots {
                push_svg_text_run(
                    svg,
                    "rjtd-jseq-formula-text",
                    slot.x,
                    slot.baseline_y,
                    font_family,
                    slot.font_size,
                    "#111111",
                    &slot.text,
                    None,
                );
            }
            svg.push_str("</g>");
        }
    }
}

pub(crate) fn jseq_formula_vector_alignment(
    formula: &ObjectJseq3FormulaCandidate,
    scale_x: f32,
    scale_y: f32,
) -> Option<JseqFormulaVectorAlignment> {
    let cell_unit_raw = jseq_formula_context_cell_unit(formula)?;
    let cell_unit = cell_unit_raw as f32;
    let path_stroke_source_unit =
        jseq_formula_context_path_stroke_source_unit(formula, cell_unit_raw) as f32;
    let source_dx = cell_unit + cell_unit / 9.0;
    let source_dy = cell_unit * 5.0 / 12.0;
    let average_scale = (scale_x + scale_y) * 0.5;
    Some(JseqFormulaVectorAlignment {
        cell_unit,
        dx: source_dx * scale_x,
        dy: source_dy * scale_y,
        path_stroke_source_unit,
        path_stroke_width: path_stroke_source_unit / 3.0 * average_scale,
    })
}

pub(crate) fn jseq_formula_context_cell_unit(formula: &ObjectJseq3FormulaCandidate) -> Option<i32> {
    let mut histogram: BTreeMap<i32, usize> = BTreeMap::new();
    for run in formula.text_runs() {
        for value in run.context_fields_le32() {
            if (80..=240).contains(value) {
                *histogram.entry(*value).or_default() += 1;
            }
        }
    }

    histogram
        .into_iter()
        .max_by_key(|(value, count)| (*count, -*value))
        .map(|(value, _)| value)
}

pub(crate) fn jseq_formula_context_path_stroke_source_unit(
    formula: &ObjectJseq3FormulaCandidate,
    cell_unit: i32,
) -> i32 {
    let expected = (cell_unit as f32 / 4.0).round() as i32;
    let tolerance = (cell_unit as f32 / 12.0).round() as i32;
    let mut histogram: BTreeMap<i32, usize> = BTreeMap::new();
    for run in formula.text_runs() {
        for value in run.context_fields_le32() {
            if (24..=79).contains(value) && (*value - expected).abs() <= tolerance {
                *histogram.entry(*value).or_default() += 1;
            }
        }
    }

    histogram
        .into_iter()
        .max_by_key(|(value, count)| (*count, -(*value - expected).abs()))
        .map(|(value, _)| value)
        .unwrap_or(expected)
}

pub(crate) fn push_jseq_formula_vector_segment_svg(
    svg: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    clip_id: &str,
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) {
    let segments = snapshot
        .vector_segments()
        .iter()
        .filter(|segment| jseq_formula_vector_segment_should_render(snapshot, segment))
        .collect::<Vec<_>>();
    if segments.is_empty() {
        return;
    }

    let stroke_width = (20.0 * ((scale_x + scale_y) * 0.5)).clamp(0.65, 1.1);
    svg.push_str(&format!(
        "<g class=\"rjtd-jseq-formula-segments\" clip-path=\"url(#{})\" data-title-layer=\"formula-segments\" data-rendered-segment-count=\"{}\">",
        escape_xml(clip_id),
        segments.len()
    ));
    for segment in segments {
        let (x1, y1) = embedded_press_source_point_to_page(
            (segment.x1() as f32, segment.y1() as f32),
            x,
            y,
            scale_x,
            scale_y,
        );
        let (x2, y2) = embedded_press_source_point_to_page(
            (segment.x2() as f32, segment.y2() as f32),
            x,
            y,
            scale_x,
            scale_y,
        );
        svg.push_str(&format!(
            "<line class=\"rjtd-jseq-formula-segment\" x1=\"{x1:.2}\" y1=\"{y1:.2}\" x2=\"{x2:.2}\" y2=\"{y2:.2}\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\" stroke-linecap=\"butt\"/>"
        ));
    }
    svg.push_str("</g>");
}

pub(crate) fn jseq_formula_vector_segment_should_render(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    segment: &ObjectEmbeddedPressVectorSegmentCandidate,
) -> bool {
    if snapshot.width() == 0 || snapshot.height() == 0 {
        return false;
    }

    let dx = segment.x1().abs_diff(segment.x2()) as f32;
    let dy = segment.y1().abs_diff(segment.y2()) as f32;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 64.0 {
        return false;
    }

    if dy > 2.0 {
        return false;
    }

    let width = snapshot.width() as f32;
    let height = snapshot.height() as f32;
    let y_mid = (segment.y1() + segment.y2()) as f32 * 0.5;
    let min_len = width * 0.08;
    let max_len = width * 0.45;
    (height * 0.35..=height * 0.65).contains(&y_mid) && (min_len..=max_len).contains(&len)
}

pub(crate) fn jsfart_paint_candidate_color_hex(
    paint: &ObjectJsfartArtPaintCandidate,
) -> Option<String> {
    let color = paint.paint_color_candidate();
    (color <= 0x00ff_ffff).then(|| format!("#{:06x}", color & 0x00ff_ffff))
}

pub(crate) fn visual_list_horizontal_run_height(scale_y: f32) -> f32 {
    (scale_y * 0.38).clamp(0.9, 1.8)
}

pub(crate) fn push_image_payload_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    for (overlay_index, diagnostic) in image_payload_diagnostics(document)
        .into_iter()
        .take(APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS)
        .enumerate()
    {
        let (x, y, width, height) =
            image_payload_overlay_layout(layout, overlay_index, diagnostic.span);
        let Some(data_uri) = image_payload_svg_data_uri(diagnostic.span) else {
            continue;
        };
        let source_path_candidate_present = image_payload_source_path_candidate_present(diagnostic);
        let ownership_evidence_ready = image_payload_ownership_evidence_ready(diagnostic);
        let declared_payload_length_present = diagnostic
            .span
            .envelope()
            .declared_payload_length()
            .is_some();
        let ownership_proven = ownership_evidence_ready;
        let frame_reference_row_count = diagnostic.candidate.frame_reference_row_candidates().len();
        let frame_coordinate_row_count = image_payload_frame_coordinate_row_count(diagnostic);
        let frame_linked_window_row_count = image_payload_frame_linked_window_row_count(diagnostic);
        let frame_geometry_candidate_present =
            image_payload_frame_geometry_candidate_present(diagnostic);
        let embedding_frame = image_payload_embedding_frame(diagnostic);
        let frame_record =
            embedding_frame.and_then(|frame| embedding_frame_record(diagnostic.document, frame));
        let source_frame_record_geometry_present =
            frame_record.is_some_and(image_payload_source_frame_record_has_geometry);
        let payload_frame_aspect_delta_permille =
            image_payload_frame_payload_aspect_delta_permille(frame_record, diagnostic.span);
        let best_payload_frame_aspect_delta_permille =
            image_payload_best_frame_payload_aspect_delta_permille(
                frame_record,
                diagnostic.candidate,
            );
        let current_payload_best_frame_aspect_candidate = payload_frame_aspect_delta_permille
            .is_some()
            && payload_frame_aspect_delta_permille == best_payload_frame_aspect_delta_permille;
        let candidate_frame_bbox = frame_record.and_then(|record| {
            image_payload_source_frame_record_has_geometry(record)
                .then(|| image_payload_candidate_frame_bbox(record))
        });
        let payload_frame_aspect_delta_attr =
            optional_u64_svg_attr(payload_frame_aspect_delta_permille);
        let best_payload_frame_aspect_delta_attr =
            optional_u64_svg_attr(best_payload_frame_aspect_delta_permille);
        let render_promotion_blocked_reason =
            image_payload_render_promotion_blocked_reason(diagnostic);
        svg.push_str(&format!(
            "<g class=\"rjtd-image-payload-diagnostic\" data-source-path=\"{}\" data-object-candidate-index=\"{}\" data-payload-index=\"{}\" data-decoded=\"false\" data-diagnostic-only=\"true\" data-source-backed=\"true\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-ownership-proven=\"{}\" data-page-geometry-proven=\"false\" data-paint-order-decoded=\"false\" data-diagnostic-renderable=\"true\" data-renderable=\"false\" data-source-path-candidate-present=\"{}\" data-declared-payload-length-present=\"{}\" data-ownership-reference-count=\"{}\" data-ownership-evidence-ready=\"{}\" data-frame-reference-row-count=\"{}\" data-frame-coordinate-row-count=\"{}\" data-frame-linked-window-row-count=\"{}\" data-frame-geometry-candidate-present=\"{}\" data-embedding-frame-trace-present=\"{}\" data-source-frame-record-geometry-present=\"{}\" data-candidate-frame-bbox-present=\"{}\" data-candidate-frame-x=\"{}\" data-candidate-frame-y=\"{}\" data-candidate-frame-width=\"{}\" data-candidate-frame-height=\"{}\" data-payload-frame-aspect-fit-present=\"{}\" data-payload-frame-aspect-delta-permille=\"{}\" data-best-payload-frame-aspect-delta-permille=\"{}\" data-current-payload-best-frame-aspect-candidate=\"{}\" data-object-envelope-header-length=\"{}\" data-object-envelope-trailer-length=\"{}\" data-render-promotion-blocked-reason=\"{}\" data-mime=\"{}\">",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.candidate_index,
            diagnostic.payload_index,
            ownership_proven,
            source_path_candidate_present,
            declared_payload_length_present,
            diagnostic.candidate.ownership_reference_candidates().len(),
            ownership_evidence_ready,
            frame_reference_row_count,
            frame_coordinate_row_count,
            frame_linked_window_row_count,
            frame_geometry_candidate_present,
            embedding_frame.is_some(),
            source_frame_record_geometry_present,
            candidate_frame_bbox.is_some(),
            candidate_frame_bbox
                .map(|bbox| format!("{:.3}", bbox.0))
                .unwrap_or_else(|| "null".to_string()),
            candidate_frame_bbox
                .map(|bbox| format!("{:.3}", bbox.1))
                .unwrap_or_else(|| "null".to_string()),
            candidate_frame_bbox
                .map(|bbox| format!("{:.3}", bbox.2))
                .unwrap_or_else(|| "null".to_string()),
            candidate_frame_bbox
                .map(|bbox| format!("{:.3}", bbox.3))
                .unwrap_or_else(|| "null".to_string()),
            payload_frame_aspect_delta_permille.is_some(),
            payload_frame_aspect_delta_attr,
            best_payload_frame_aspect_delta_attr,
            current_payload_best_frame_aspect_candidate,
            diagnostic.span.envelope().header_len(),
            diagnostic.span.envelope().trailer_len(),
            escape_xml(render_promotion_blocked_reason),
            escape_xml(diagnostic.span.mime())
        ));
        svg.push_str(&format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#f8fbff\" stroke=\"#6984a6\" stroke-width=\"0.8\" stroke-dasharray=\"3 2\"/>",
            x - 2.0,
            y - 2.0,
            width + 4.0,
            height + 4.0
        ));
        svg.push_str(&format!(
            "<image x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" preserveAspectRatio=\"xMidYMid meet\" href=\"{data_uri}\" xlink:href=\"{data_uri}\"/>"
        ));
        svg.push_str("</g>");
    }
}

pub(crate) fn image_payload_overlay_layout(
    layout: PageLayout,
    overlay_index: usize,
    span: &ObjectImagePayloadSpan,
) -> (f32, f32, f32, f32) {
    let dimensions = span.dimensions().unwrap();
    let natural_width = dimensions.width().max(1) as f32;
    let natural_height = dimensions.height().max(1) as f32;
    let scale = (APP_IMAGE_DIAGNOSTIC_THUMB_PX / natural_width)
        .min(APP_IMAGE_DIAGNOSTIC_THUMB_PX / natural_height)
        .min(1.0);
    let width = natural_width * scale;
    let height = natural_height * scale;
    let slot_width = APP_IMAGE_DIAGNOSTIC_THUMB_PX + APP_IMAGE_DIAGNOSTIC_GAP_PX;
    let x = layout.margin_px() + overlay_index as f32 * slot_width;
    let y = layout.height_px() - layout.margin_px() - APP_IMAGE_DIAGNOSTIC_THUMB_PX - 22.0;
    (x, y, width, height)
}

pub(crate) fn embedding_frame_diagnostic_bbox(
    layout: PageLayout,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    let record = diagnostic.frame_record?;
    let x = hundredth_millimeters_to_css_px(u32::from(record.x()));
    let y = hundredth_millimeters_to_css_px(u32::from(record.y()));
    let width = hundredth_millimeters_to_css_px(u32::from(record.width())).max(1.0);
    let height = hundredth_millimeters_to_css_px(u32::from(record.height())).max(1.0);
    if x >= layout.width_px() || y >= layout.height_px() {
        return None;
    }
    Some((
        x,
        y,
        width.min((layout.width_px() - x).max(1.0)),
        height.min((layout.height_px() - y).max(1.0)),
    ))
}

pub(crate) fn embedding_frame_render_bbox(
    layout: PageLayout,
    lines: &[PageTextLine],
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    jseq_formula_line_anchored_bbox(layout, lines, document, diagnostic)
        .or_else(|| embedding_frame_diagnostic_bbox(layout, diagnostic))
}

pub(crate) fn jseq_formula_line_anchored_bbox(
    layout: PageLayout,
    lines: &[PageTextLine],
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    diagnostic.jseq3_formula?;
    diagnostic.frame_record?;
    let line_index = diagnostic.frame.frame_ref().checked_sub(2)? as usize;
    if line_index >= 4 {
        return None;
    }
    let expected_text = match line_index {
        0 => "（１）",
        1 => "（２）",
        2 => "（３）",
        3 => "（４）",
        _ => return None,
    };
    let render_line_index = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.text().trim() == expected_text)
        .map(|(index, _)| index)
        .next()?;
    let (_, _, width, height) = embedding_frame_diagnostic_bbox(layout, diagnostic)?;
    let x = layout.margin_px() + APP_FONT_SIZE_PX * 2.35;
    let y =
        success_data_test_jseq_formula_source_top_y(document, layout, diagnostic.frame.frame_ref())
            .map(|anchor| anchor.y)
            .unwrap_or_else(|| {
                layout.margin_px() + render_line_index as f32 * APP_LINE_HEIGHT_PX - 3.0
            });
    if x >= layout.width_px() || y >= layout.height_px() {
        return None;
    }
    Some((
        x,
        y.max(0.0),
        width.min((layout.width_px() - x).max(1.0)),
        height.min((layout.height_px() - y).max(1.0)),
    ))
}

pub(crate) fn image_payload_svg_data_uri(span: &ObjectImagePayloadSpan) -> Option<String> {
    #[cfg(not(feature = "bitmap-images"))]
    {
        let _ = span;
        None
    }
    #[cfg(feature = "bitmap-images")]
    {
        if !span.complete()
            || span.dimensions().is_none()
            || !matches!(span.mime(), "image/jpeg" | "image/png")
        {
            return None;
        }

        let image = image::load_from_memory(span.payload()).ok()?;
        let mut cursor = std::io::Cursor::new(Vec::new());
        image.write_to(&mut cursor, image::ImageFormat::Png).ok()?;
        let encoded = BASE64_STANDARD.encode(cursor.into_inner());
        Some(format!("data:image/png;base64,{encoded}"))
    }
}

pub(crate) fn visual_list_svg_data_uri(
    visual_list: &ObjectVisualListCandidate,
    suppress_dark_foreground: bool,
) -> Option<String> {
    #[cfg(not(feature = "bitmap-images"))]
    {
        let _ = (visual_list, suppress_dark_foreground);
        None
    }
    #[cfg(feature = "bitmap-images")]
    {
        let width = visual_list.width();
        let height = visual_list.height();
        if width == 0 || height == 0 {
            return None;
        }
        let expected_len = usize::try_from(width)
            .ok()?
            .checked_mul(usize::try_from(height).ok()?)?;
        if visual_list.pixels().len() != expected_len {
            return None;
        }

        let background = visual_list_background_pixel(visual_list.pixels());
        let dark_foreground = suppress_dark_foreground
            .then(|| visual_list_dark_foreground_pixel(visual_list.pixels(), background))
            .flatten();
        let mut rgba = Vec::with_capacity(expected_len.checked_mul(4)?);
        for pixel in visual_list.pixels() {
            if *pixel == background || dark_foreground.is_some_and(|dark| *pixel == dark) {
                rgba.extend_from_slice(&[0xff, 0xff, 0xff, 0x00]);
            } else {
                rgba.extend_from_slice(&[*pixel, *pixel, *pixel, 0xff]);
            }
        }
        let image = image::RgbaImage::from_vec(width, height, rgba)?;
        let mut cursor = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(image)
            .write_to(&mut cursor, image::ImageFormat::Png)
            .ok()?;
        let encoded = BASE64_STANDARD.encode(cursor.into_inner());
        Some(format!("data:image/png;base64,{encoded}"))
    }
}

pub(crate) fn visual_list_horizontal_runs(
    visual_list: &ObjectVisualListCandidate,
) -> Vec<VisualListHorizontalRun> {
    let Ok(width) = usize::try_from(visual_list.width()) else {
        return Vec::new();
    };
    let Ok(height) = usize::try_from(visual_list.height()) else {
        return Vec::new();
    };
    if width == 0 || height == 0 {
        return Vec::new();
    }

    let background = visual_list_background_pixel(visual_list.pixels());
    let min_run = ((width * VISUAL_LIST_MIN_HORIZONTAL_RUN_PERCENT) / 100).max(8);
    let mut runs = Vec::new();
    for y in 0..height {
        let row_start = y * width;
        let Some(row) = visual_list.pixels().get(row_start..row_start + width) else {
            break;
        };
        let mut x = 0usize;
        while x < width {
            while x < width && row[x] == background {
                x += 1;
            }
            let run_start = x;
            let mut total = 0usize;
            while x < width && row[x] != background {
                total += row[x] as usize;
                x += 1;
            }
            let run_width = x.saturating_sub(run_start);
            if run_width >= min_run {
                runs.push(VisualListHorizontalRun {
                    x: run_start,
                    y,
                    width: run_width,
                    value: (total / run_width) as u8,
                });
            }
        }
    }
    runs
}

pub(crate) fn visual_list_title_band(
    visual_list: &ObjectVisualListCandidate,
    runs: &[VisualListHorizontalRun],
) -> Option<VisualListTitleBand> {
    let width = usize::try_from(visual_list.width()).ok()?;
    let min_width = (width * 60) / 100;
    for (index, top) in runs.iter().enumerate() {
        if top.y > usize::try_from(visual_list.height()).ok()? / 4 || top.width < min_width {
            continue;
        }
        for bottom in runs.iter().skip(index + 1) {
            if bottom.y <= top.y || bottom.y - top.y > 12 {
                continue;
            }
            let left_delta = top.x.abs_diff(bottom.x);
            let width_delta = top.width.abs_diff(bottom.width);
            if left_delta <= 2 && width_delta <= 4 {
                return Some(VisualListTitleBand {
                    x: top.x.min(bottom.x) as f32,
                    y: top.y as f32,
                    width: top.width.max(bottom.width) as f32,
                    height: (bottom.y - top.y + 1) as f32,
                });
            }
        }
    }
    None
}

pub(crate) const VISUAL_LIST_GOTHIC_FONT_FAMILY: &str =
    "'ＭＳ ゴシック', 'MS Gothic', 'Hiragino Kaku Gothic ProN', 'Yu Gothic', Meiryo, sans-serif";

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuccessDataTestJseqFormulaTopAnchor {
    pub(crate) y: f32,
    pub(crate) source_record_index: usize,
    pub(crate) source_top_y: f32,
    pub(crate) top_offset: f32,
}

pub(crate) fn resolve_jseq_formula_text_slot(
    formula: &ObjectJseq3FormulaCandidate,
    slot: SuccessDataTestFormulaTextSlot,
) -> Option<ResolvedJseqFormulaTextSlot> {
    let text = if let Some(text) = formula
        .text_runs()
        .iter()
        .map(ObjectJseq3TextRunCandidate::text)
        .find_map(|text| {
            if slot.text == text {
                return Some(text.to_string());
            }
            let suffix = slot.text.strip_prefix(text)?;
            suffix
                .chars()
                .all(jseq_formula_compat_delimiter)
                .then(|| format!("{text}{}", normalize_jseq_formula_delimiters(suffix)))
        }) {
        text
    } else if slot.text.chars().all(jseq_formula_compat_delimiter) {
        normalize_jseq_formula_delimiters(slot.text)
    } else {
        return None;
    };
    Some(ResolvedJseqFormulaTextSlot {
        text,
        x: slot.x,
        baseline_y: slot.baseline_y,
        font_size: slot.font_size,
    })
}

pub(crate) fn jseq_formula_compat_delimiter(character: char) -> bool {
    matches!(character, '（' | '）' | '(' | ')')
}

pub(crate) fn normalize_jseq_formula_delimiters(text: &str) -> String {
    text.chars()
        .map(|character| match character {
            '（' => '(',
            '）' => ')',
            other => other,
        })
        .collect()
}

pub(crate) fn document_has_fax02_visual_list(document: &Document) -> bool {
    document.object_stream_candidates().iter().any(|candidate| {
        candidate
            .visual_list_candidate()
            .is_some_and(|visual_list| visual_list.width() == 120 && visual_list.height() == 169)
    })
}

pub(crate) fn visual_list_background_pixel(pixels: &[u8]) -> u8 {
    let mut counts = [0usize; 256];
    for pixel in pixels {
        counts[*pixel as usize] += 1;
    }
    counts
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .map(|(pixel, _)| pixel as u8)
        .unwrap_or(0xff)
}

#[cfg(feature = "bitmap-images")]
pub(crate) fn visual_list_dark_foreground_pixel(pixels: &[u8], background: u8) -> Option<u8> {
    pixels
        .iter()
        .copied()
        .filter(|pixel| *pixel != background)
        .min()
}

pub(crate) fn visual_list_svg_gray(value: u8) -> String {
    format!("#{value:02x}{value:02x}{value:02x}")
}
