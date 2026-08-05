use super::*;

#[derive(Clone, Copy)]
pub(crate) struct SuccessDataTestTitleArtPathPlacement {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
}

#[derive(Clone, Copy)]
pub(crate) struct SuccessDataTestTitleArtFrontFill<'a> {
    pub(crate) rule: &'static str,
    pub(crate) attrs: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TitleArtFrontFillRenderColorGate<'a> {
    pub(crate) render_fill: &'static str,
    pub(crate) paint_color: Option<&'a str>,
    pub(crate) paint_source: Option<&'static str>,
    pub(crate) render_color_source: &'static str,
    pub(crate) render_color_source_backed: bool,
    pub(crate) source_paint_matches_render_fill: bool,
    pub(crate) render_color_blocked_reason: &'static str,
}

#[derive(Debug, Clone)]
pub(crate) struct TitleArtFrontFillWindingGate {
    pub(crate) path_count: usize,
    pub(crate) multi_contour_path_count: usize,
    pub(crate) opposite_signed_contour_path_count: usize,
    pub(crate) selected_fill_rule: &'static str,
    pub(crate) selected_fill_rule_source: &'static str,
    pub(crate) previous_fill_rule: &'static str,
    pub(crate) render_promoted: bool,
    pub(crate) reference_backed: bool,
    pub(crate) nonzero_title_tight_rms: f32,
    pub(crate) evenodd_title_tight_rms: f32,
}

impl TitleArtFrontFillWindingGate {
    pub(crate) fn svg_attrs(&self) -> String {
        format!(
            " data-title-front-fill-winding-gate-source=\"embeddedPressContourWinding+popplerTitleCropAB\" data-title-front-fill-winding-source-backed=\"{}\" data-title-front-fill-winding-reference-backed=\"{}\" data-title-front-fill-winding-render-promoted=\"{}\" data-title-front-fill-selected-rule=\"{}\" data-title-front-fill-selected-rule-source=\"{}\" data-title-front-fill-previous-rule=\"{}\" data-title-front-fill-path-count=\"{}\" data-title-front-fill-multi-contour-path-count=\"{}\" data-title-front-fill-opposite-signed-contour-path-count=\"{}\" data-title-front-fill-nonzero-title-tight-rms=\"{:.3}\" data-title-front-fill-evenodd-title-tight-rms=\"{:.3}\" data-title-front-fill-rms-improvement=\"{:.3}\"",
            self.opposite_signed_contour_path_count > 0,
            self.reference_backed,
            self.render_promoted,
            escape_xml(self.selected_fill_rule),
            escape_xml(self.selected_fill_rule_source),
            escape_xml(self.previous_fill_rule),
            self.path_count,
            self.multi_contour_path_count,
            self.opposite_signed_contour_path_count,
            self.nonzero_title_tight_rms,
            self.evenodd_title_tight_rms,
            self.nonzero_title_tight_rms - self.evenodd_title_tight_rms
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TitleArtShadowPathPartition<'a> {
    pub(crate) main_paths: Vec<&'a ObjectEmbeddedPressVectorPathCandidate>,
    pub(crate) shadow_paths: Vec<&'a ObjectEmbeddedPressVectorPathCandidate>,
    pub(crate) offset: (i32, i32),
    pub(crate) strategy: &'static str,
}

#[derive(Debug, Clone)]
pub(crate) struct EmbeddedPressTitleArtShadowEffect {
    pub(crate) opacity: f32,
    pub(crate) word0: u32,
    pub(crate) fill_color: String,
}

impl EmbeddedPressTitleArtShadowEffect {
    pub(crate) fn svg_attrs(&self) -> String {
        format!(
            " data-title-shadow-effect-opacity=\"{:.3}\" data-title-shadow-effect-word0=\"0x{:02x}\" data-title-shadow-fill-source=\"embedded-press-0x70-word0-percent-black-on-white\"",
            self.opacity, self.word0
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EmbeddedPressTitleArtTextureEffect {
    pub(crate) opacity: f32,
    pub(crate) word0: u32,
    pub(crate) base_fill_color: String,
    pub(crate) fill_color: String,
}

impl EmbeddedPressTitleArtTextureEffect {
    pub(crate) fn svg_attrs(&self) -> String {
        format!(
            " data-title-texture-effect-candidate-opacity=\"{:.3}\" data-title-texture-effect-candidate-word0=\"0x{:02x}\" data-title-texture-effect-candidate-base-fill=\"{}\" data-title-texture-effect-candidate-fill=\"{}\" data-title-texture-effect-candidate-source=\"embedded-press-interstitial-0x70-word0-percent-black-over-shadow\" data-title-texture-effect-render-promoted=\"false\" data-title-texture-effect-render-promotion-blocked-reason=\"record70-separates-shadow-but-not-interstitial-texture-from-main\"",
            self.opacity,
            self.word0,
            escape_xml(&self.base_fill_color),
            escape_xml(&self.fill_color)
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TitleArtTextureGeometryRoleGate {
    pub(crate) partition_present: bool,
    pub(crate) texture_path_count: usize,
    pub(crate) shadow_outline_path_count: usize,
    pub(crate) main_outline_path_count: usize,
    pub(crate) texture_bbox: Option<(i32, i32, i32, i32)>,
    pub(crate) shadow_bbox: Option<(i32, i32, i32, i32)>,
    pub(crate) main_bbox: Option<(i32, i32, i32, i32)>,
    pub(crate) side_sweep_bbox: Option<(i32, i32, i32, i32)>,
    pub(crate) texture_area: i64,
    pub(crate) texture_main_overlap_area: i64,
    pub(crate) texture_shadow_overlap_area: i64,
    pub(crate) texture_side_sweep_overlap_area: i64,
    pub(crate) texture_main_overlap_ratio: f32,
    pub(crate) texture_shadow_overlap_ratio: f32,
    pub(crate) texture_side_sweep_overlap_ratio: f32,
    pub(crate) texture_contained_by_main_bbox: bool,
    pub(crate) texture_contained_by_shadow_bbox: bool,
    pub(crate) texture_contained_by_side_sweep_bbox: bool,
    pub(crate) role_conclusion: &'static str,
    pub(crate) render_promotion_blocked_reason: &'static str,
}

pub(crate) fn blend_css_hex_colors(
    foreground: &str,
    background: &str,
    alpha: f32,
) -> Option<String> {
    let foreground = parse_css_hex_rgb(foreground)?;
    let background = parse_css_hex_rgb(background)?;
    let blend_channel = |fg: u8, bg: u8| -> u8 {
        (fg as f32 * alpha + bg as f32 * (1.0 - alpha))
            .round()
            .clamp(0.0, 255.0) as u8
    };
    Some(format!(
        "#{:02x}{:02x}{:02x}",
        blend_channel(foreground.0, background.0),
        blend_channel(foreground.1, background.1),
        blend_channel(foreground.2, background.2)
    ))
}

pub(crate) fn parse_css_hex_rgb(color: &str) -> Option<(u8, u8, u8)> {
    let hex = color.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((red, green, blue))
}

#[derive(Debug, Clone)]
pub(crate) struct EmbeddedPressTitleArtTextureStateSpan {
    pub(crate) state_path_index: usize,
    pub(crate) inherited_span_end_path_index: usize,
    pub(crate) path_count: usize,
    pub(crate) texture_path_count: usize,
    pub(crate) record48_word0_values: Vec<u32>,
    pub(crate) record70_word0_values: Vec<u32>,
    pub(crate) record82_word3_values: Vec<u32>,
    pub(crate) record82_word5_values: Vec<u32>,
}

#[derive(Debug, Clone)]
pub(crate) struct TitleArtFrontErasePaintTransitionGate {
    pub(crate) partition_present: bool,
    pub(crate) interstitial_texture_path_count: usize,
    pub(crate) explicit_state_texture_path_count: usize,
    pub(crate) inherited_texture_path_count: usize,
    pub(crate) span_count: usize,
    pub(crate) span_path_counts: Vec<usize>,
    pub(crate) shadow_last_path_index: Option<usize>,
    pub(crate) interstitial_first_path_index: Option<usize>,
    pub(crate) interstitial_last_path_index: Option<usize>,
    pub(crate) main_first_path_index: Option<usize>,
    pub(crate) shadow_to_interstitial_boundary_adjacent: bool,
    pub(crate) interstitial_to_main_boundary_adjacent: bool,
    pub(crate) record48_separates_shadow_from_texture_and_main: bool,
    pub(crate) record48_separates_texture_from_main: bool,
    pub(crate) record70_word0_separates_texture_from_main: bool,
    pub(crate) record82_word5_separates_texture_from_main: bool,
    pub(crate) record82_word5_matches_shadow: bool,
    pub(crate) record82_word3_is_white_paint_candidate: bool,
    pub(crate) paint_intent_inference: &'static str,
    pub(crate) transition_boundary_class: &'static str,
    pub(crate) render_promotion_blocked_reason: &'static str,
}

impl TitleArtFrontErasePaintTransitionGate {
    pub(crate) fn promotion_ready(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy)]
pub(crate) struct EmbeddedPressPageContext {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
}

pub(crate) fn push_title_rounded_frame_svg(svg: &mut String, shape: &PageFrameShape) {
    let inset = (shape.corner_radius * 0.65).clamp(2.5, 5.0);
    let inner_x = shape.x + inset;
    let inner_y = shape.y + inset;
    let inner_width = (shape.width - inset * 2.0).max(0.0);
    let inner_height = (shape.height - inset * 2.0).max(0.0);
    let inner_radius = (shape.corner_radius - inset * 0.5).max(0.0);
    svg.push_str(&format!(
        "<g class=\"rjtd-page-frame-shape rjtd-title-rounded-frame\" data-role=\"{}\" data-row-index=\"{}\" data-object-id=\"{}\" data-object-type=\"0x{:04x}\" data-source-x=\"{}\" data-source-y=\"{}\" data-source-width=\"{}\" data-source-height=\"{}\" data-source-corner-radius=\"{}\" data-source-style-id=\"{}\" data-placement-basis=\"{}\" data-style-basis=\"{}\">",
        escape_xml(shape.role),
        shape.row_index,
        shape.object_id,
        shape.object_type,
        shape.source_x,
        shape.source_y,
        shape.source_width,
        shape.source_height,
        shape.source_corner_radius,
        shape.source_style_id,
        escape_xml(shape.placement_basis),
        escape_xml(shape.style_basis)
    ));
    svg.push_str(&format!(
        "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" rx=\"{:.2}\" ry=\"{:.2}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"1.35\"/>",
        shape.x, shape.y, shape.width, shape.height, shape.corner_radius, shape.corner_radius
    ));
    svg.push_str(&format!(
        "<rect x=\"{inner_x:.2}\" y=\"{inner_y:.2}\" width=\"{inner_width:.2}\" height=\"{inner_height:.2}\" rx=\"{inner_radius:.2}\" ry=\"{inner_radius:.2}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"1.05\"/>"
    ));
    svg.push_str("</g>");
}

pub(crate) fn push_horizontal_pattern_bar_svg(
    svg: &mut String,
    shape: &PageFrameShape,
    pattern_id: &str,
) {
    svg.push_str(&format!(
        "<rect class=\"rjtd-page-frame-shape rjtd-horizontal-pattern-bar\" data-role=\"{}\" data-row-index=\"{}\" data-object-id=\"{}\" data-object-type=\"0x{:04x}\" data-source-x=\"{}\" data-source-y=\"{}\" data-source-width=\"{}\" data-source-height=\"{}\" data-source-corner-radius=\"{}\" data-source-style-id=\"{}\" data-placement-basis=\"{}\" data-style-basis=\"{}\" x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" rx=\"{:.2}\" ry=\"{:.2}\" fill=\"url(#{})\" stroke=\"#111111\" stroke-width=\"1.15\"/>",
        escape_xml(shape.role),
        shape.row_index,
        shape.object_id,
        shape.object_type,
        shape.source_x,
        shape.source_y,
        shape.source_width,
        shape.source_height,
        shape.source_corner_radius,
        shape.source_style_id,
        escape_xml(shape.placement_basis),
        escape_xml(shape.style_basis),
        shape.x,
        shape.y,
        shape.width,
        shape.height,
        shape.corner_radius,
        shape.corner_radius,
        escape_xml(pattern_id)
    ));
}
