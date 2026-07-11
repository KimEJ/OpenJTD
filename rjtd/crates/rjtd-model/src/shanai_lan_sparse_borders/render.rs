use super::{candidates::*, topology::*, types::*, *};

struct SparseTableBorderRenderLine<'a> {
    orientation: &'a str,
    edge_kind: &'a str,
    group_index: usize,
    pair_index: usize,
    style_code: u16,
    points: (f32, f32, f32, f32),
    stroke_width: f32,
    dash_length: Option<f32>,
}

pub(crate) fn push_shanai_lan_sparse_table_borders_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }
    let Some(diagnostic) = shanai_lan_sparse_table_border_topology_diagnostic(document) else {
        return;
    };
    if !diagnostic.renderable || diagnostic.stable_grid_extent_units == 0 {
        return;
    }
    let Some(transform) = diagnostic.source_page_transform_candidate.as_ref() else {
        return;
    };

    let source_width_px = hundredth_millimeters_to_css_px(transform.page_width_mm100);
    let source_height_px = hundredth_millimeters_to_css_px(transform.page_height_mm100);
    if (source_width_px - layout.width_px()).abs() > 1.0
        || (source_height_px - layout.height_px()).abs() > 1.0
    {
        return;
    }

    let usable_width_mm100 = transform
        .page_width_mm100
        .saturating_sub(transform.x_origin_left_mm100)
        .saturating_sub(transform.x_origin_right_mm100);
    if usable_width_mm100 == 0 {
        return;
    }
    let x_step_mm100 = usable_width_mm100 as f32 / f32::from(diagnostic.stable_grid_extent_units);
    let mm100_to_px = |value: f32| millimeters_to_css_px(value / 100.0);
    let x_px = |unit: u32| {
        mm100_to_px(transform.x_origin_left_mm100 as f32 + (unit as f32 + 1.0) * x_step_mm100)
    };
    let top_y_px = |group_index: f32| {
        mm100_to_px(
            transform.y_origin_mm100 as f32 + group_index * transform.row_pitch_mm100 as f32
                - transform.row_pitch_mm100 as f32,
        )
    };
    let bottom_y_px = |group_index: f32| {
        mm100_to_px(
            transform.y_origin_mm100 as f32 + group_index * transform.row_pitch_mm100 as f32
                - transform.row_pitch_mm100 as f32
                    / SOURCE_PAGE_TRANSFORM_BOTTOM_OFFSET_DENOMINATOR as f32,
        )
    };

    svg.push_str("<g class=\"rjtd-document-text-sparse-table-borders\" data-source=\"/DocumentText+/DocumentViewStyles+/PageMark\" data-decoded=\"true\" data-geometry-decoded=\"true\" data-placement-derived=\"true\" data-renderable=\"true\" pointer-events=\"none\">");
    for candidate in &diagnostic.horizontal_candidates {
        let Some(style_code) = candidate.edge_style_code else {
            continue;
        };
        let Some((stroke_width, dash_length)) = shanai_lan_sparse_table_border_stroke(style_code)
        else {
            continue;
        };
        let y = match candidate.edge_kind {
            ShanaiLanSparseTableBorderHorizontalEdgeKind::Top => {
                top_y_px(candidate.group_index as f32)
            }
            ShanaiLanSparseTableBorderHorizontalEdgeKind::Bottom => {
                bottom_y_px(candidate.group_index as f32)
            }
        };
        push_shanai_lan_sparse_table_border_line_svg(
            svg,
            &SparseTableBorderRenderLine {
                orientation: "horizontal",
                edge_kind: candidate.edge_kind.as_str(),
                group_index: candidate.group_index,
                pair_index: candidate.pair_index,
                style_code,
                points: (x_px(candidate.start_unit), y, x_px(candidate.end_unit), y),
                stroke_width,
                dash_length,
            },
        );
    }

    for candidate in &diagnostic.junction_candidates {
        let x = x_px(candidate.x_unit);
        let group_index = candidate.group_index as f32;
        if candidate.upper_vertical_candidate
            && let Some(style_code) = candidate.upper_vertical_style_code
            && let Some((stroke_width, dash_length)) =
                shanai_lan_sparse_table_border_stroke(style_code)
        {
            push_shanai_lan_sparse_table_border_line_svg(
                svg,
                &SparseTableBorderRenderLine {
                    orientation: "vertical",
                    edge_kind: "upper",
                    group_index: candidate.group_index,
                    pair_index: candidate.pair_index,
                    style_code,
                    points: (x, bottom_y_px(group_index - 1.0), x, top_y_px(group_index)),
                    stroke_width,
                    dash_length,
                },
            );
        }
        if candidate.lower_vertical_candidate
            && let Some(style_code) = candidate.lower_vertical_style_code
            && let Some((stroke_width, dash_length)) =
                shanai_lan_sparse_table_border_stroke(style_code)
        {
            push_shanai_lan_sparse_table_border_line_svg(
                svg,
                &SparseTableBorderRenderLine {
                    orientation: "vertical",
                    edge_kind: "lower",
                    group_index: candidate.group_index,
                    pair_index: candidate.pair_index,
                    style_code,
                    points: (x, top_y_px(group_index), x, bottom_y_px(group_index)),
                    stroke_width,
                    dash_length,
                },
            );
        }
    }
    svg.push_str("</g>");
}

fn push_shanai_lan_sparse_table_border_line_svg(
    svg: &mut String,
    line: &SparseTableBorderRenderLine<'_>,
) {
    let SparseTableBorderRenderLine {
        orientation,
        edge_kind,
        group_index,
        pair_index,
        style_code,
        points: (x1, y1, x2, y2),
        stroke_width,
        dash_length,
    } = line;
    svg.push_str(&format!(
        "<line class=\"rjtd-document-text-sparse-table-border rjtd-document-text-sparse-table-border-{orientation}\" data-edge-kind=\"{edge_kind}\" data-group-index=\"{group_index}\" data-pair-index=\"{pair_index}\" data-style-code=\"{style_code}\" x1=\"{x1:.3}\" y1=\"{y1:.3}\" x2=\"{x2:.3}\" y2=\"{y2:.3}\" stroke=\"#000000\" stroke-width=\"{stroke_width:.2}\" stroke-linecap=\"butt\""
    ));
    if let Some(dash_length) = dash_length {
        svg.push_str(&format!(
            " stroke-dasharray=\"{dash_length:.1} {dash_length:.1}\""
        ));
    }
    svg.push_str("/>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparse_border_line_svg_preserves_solid_and_dashed_strokes() {
        let mut solid = String::new();
        push_shanai_lan_sparse_table_border_line_svg(
            &mut solid,
            &SparseTableBorderRenderLine {
                orientation: "horizontal",
                edge_kind: "top",
                group_index: 2,
                pair_index: 3,
                style_code: 3,
                points: (1.0, 2.0, 3.0, 2.0),
                stroke_width: SPARSE_TABLE_BORDER_THICK_STROKE_WIDTH_PX,
                dash_length: None,
            },
        );
        assert!(solid.contains("data-style-code=\"3\""));
        assert!(solid.contains("stroke-width=\"2.56\""));
        assert!(!solid.contains("stroke-dasharray"));

        let mut dashed = String::new();
        push_shanai_lan_sparse_table_border_line_svg(
            &mut dashed,
            &SparseTableBorderRenderLine {
                orientation: "vertical",
                edge_kind: "lower",
                group_index: 4,
                pair_index: 5,
                style_code: 4,
                points: (6.0, 7.0, 6.0, 8.0),
                stroke_width: SPARSE_TABLE_BORDER_THIN_STROKE_WIDTH_PX,
                dash_length: Some(SPARSE_TABLE_BORDER_DASH_LENGTH_PX),
            },
        );
        assert!(dashed.contains("data-style-code=\"4\""));
        assert!(dashed.contains("stroke-width=\"0.80\""));
        assert!(dashed.contains("stroke-dasharray=\"3.2 3.2\""));
    }
}
