use super::source::{NativeFigureShapeCandidate, native_figure_bindings};
use crate::*;

pub(crate) struct NativeFigureProjection {
    shape: NativeFigureShapeCandidate,
    bbox: [f32; 4],
    points: Vec<[f32; 2]>,
    stroke: f32,
}

pub(crate) fn native_figure_projection(
    document: &Document,
    layout: PageLayout,
    page: usize,
    mode: WritingMode,
) -> Option<Vec<NativeFigureProjection>> {
    if page != 1 || mode.is_vertical() || !layout.has_source_margins() {
        return None;
    }
    let shapes = native_figure_bindings(document)?;
    let mut result = Vec::new();
    for shape in shapes {
        let (anchor_page, top, _) = native_rule_line_placement(document, layout, shape.line)?;
        if anchor_page != page {
            return None;
        }
        let bbox = [
            layout.margin_left_px() + hundredth_millimeters_to_css_px(u32::from(shape.frame[0])),
            top + hundredth_millimeters_to_css_px(u32::from(shape.frame[1])),
            hundredth_millimeters_to_css_px(u32::from(shape.frame[2])),
            hundredth_millimeters_to_css_px(u32::from(shape.frame[3])),
        ];
        if bbox[0] < layout.margin_left_px()
            || bbox[1] < layout.margin_top_px()
            || bbox[0] + bbox[2] > layout.width_px() - layout.margin_right_px()
            || bbox[1] + bbox[3] > layout.height_px() - layout.margin_bottom_px()
        {
            return None;
        }
        let sx = f64::from(bbox[2]) / (shape.bounds[2] - shape.bounds[0]);
        let sy = f64::from(bbox[3]) / (shape.bounds[3] - shape.bounds[1]);
        if (sx / sy - 1.0).abs() > 0.005 {
            return None;
        }
        let points = if shape.kind == "ellipse" {
            vec![
                [
                    bbox[0] + ((shape.points[0][0] - shape.bounds[0]) * sx) as f32,
                    bbox[1] + ((shape.points[0][1] - shape.bounds[1]) * sy) as f32,
                ],
                [
                    (shape.points[1][0] * sx) as f32,
                    (shape.points[1][1] * sy) as f32,
                ],
            ]
        } else {
            shape
                .points
                .iter()
                .map(|p| {
                    [
                        bbox[0] + ((p[0] - shape.bounds[0]) * sx) as f32,
                        bbox[1] + ((p[1] - shape.bounds[1]) * sy) as f32,
                    ]
                })
                .collect()
        };
        result.push(NativeFigureProjection {
            stroke: (f64::from(shape.stroke_units) * (sx + sy) / 2.0) as f32,
            shape,
            bbox,
            points,
        });
    }
    Some(result)
}

pub(crate) fn push_native_figure_svg(svg: &mut String, shapes: &[NativeFigureProjection]) {
    for s in shapes {
        let attrs = format!(
            "class=\"rjtd-native-figure\" data-object-id=\"{}\" data-paint-slot-candidate=\"{}\" data-primitive-kind=\"{}\" data-source=\"Frame+DocumentText+LineMark+FDMIndex+FDMVector\" data-decoded=\"false\" data-geometry-decoded=\"false\" fill=\"{}\" stroke=\"#000000\" stroke-width=\"{:.3}\"",
            s.shape.object_id,
            s.shape.slot,
            s.shape.kind,
            s.shape.fill.as_deref().unwrap_or("none"),
            s.stroke
        );
        if s.shape.kind == "ellipse" {
            svg.push_str(&format!(
                "<ellipse {attrs} cx=\"{:.3}\" cy=\"{:.3}\" rx=\"{:.3}\" ry=\"{:.3}\"/>",
                s.points[0][0], s.points[0][1], s.points[1][0], s.points[1][1]
            ));
        } else {
            let mut path = String::new();
            for (i, p) in s.points.iter().enumerate() {
                path.push_str(&format!(
                    "{} {:.3} {:.3} ",
                    if i == 0 { "M" } else { "L" },
                    p[0],
                    p[1]
                ));
            }
            if s.shape.kind == "rectangle" {
                path.push('Z');
            }
            svg.push_str(&format!("<path {attrs} d=\"{path}\"/>"));
        }
    }
}

pub(crate) fn push_native_figure_layer_json(out: &mut String, shapes: &[NativeFigureProjection]) {
    for s in shapes {
        let [x, y, width, height] = s.bbox;
        out.push_str(&format!(",{{\"type\":\"nativeFigureCandidate\",\"kind\":{},\"objectId\":{},\"paintSlotCandidate\":{},\"bbox\":{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}},\"fillColor\":{},\"strokeColor\":\"#000000\",\"strokeWidth\":{:.3},\"lineMarkRecordIndex\":{},\"vectorOffset\":{},\"commandOffset\":{},\"decoded\":false,\"geometryDecoded\":false,\"paintOrderDecoded\":false,\"referenceBacked\":false}}",json_string(s.shape.kind),s.shape.object_id,s.shape.slot,s.shape.fill.as_deref().map(json_string).unwrap_or_else(||"null".into()),s.stroke,s.shape.line,s.shape.vector_offset,s.shape.command_offset));
    }
}
