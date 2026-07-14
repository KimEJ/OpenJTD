use rjtd_core::{Error, Result};

const MAX_CANVAS_DIMENSION: usize = 16_384;
const MAX_CANVAS_PIXELS: usize = 64 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CanvasLayout {
    width: u32,
    height: u32,
    scale: f64,
}

impl CanvasLayout {
    pub(crate) const fn width(self) -> u32 {
        self.width
    }

    pub(crate) const fn height(self) -> u32 {
        self.height
    }

    pub(crate) const fn scale(self) -> f64 {
        self.scale
    }
}

pub(crate) fn canvas_layout(width: f64, height: f64, scale: f64) -> Result<CanvasLayout> {
    let scale = normalize_scale(scale);
    let width = scaled_extent("canvas width pixels", width, scale)?;
    let height = scaled_extent("canvas height pixels", height, scale)?;
    let pixels = (width as usize).saturating_mul(height as usize);
    if pixels > MAX_CANVAS_PIXELS {
        return Err(Error::ResourceLimit {
            resource: "canvas pixels",
            limit: MAX_CANVAS_PIXELS,
            actual: pixels,
        });
    }
    Ok(CanvasLayout {
        width,
        height,
        scale,
    })
}

fn normalize_scale(scale: f64) -> f64 {
    if scale.is_finite() && scale > 0.0 {
        scale
    } else {
        1.0
    }
}

fn scaled_extent(resource: &'static str, extent: f64, scale: f64) -> Result<u32> {
    let scaled = (extent * scale).ceil();
    if !scaled.is_finite() || scaled <= 0.0 {
        return Ok(1);
    }
    if scaled > MAX_CANVAS_DIMENSION as f64 {
        return Err(Error::ResourceLimit {
            resource,
            limit: MAX_CANVAS_DIMENSION,
            actual: scaled.min(usize::MAX as f64) as usize,
        });
    }
    Ok(scaled as u32)
}

#[cfg(test)]
mod tests {
    use super::{MAX_CANVAS_PIXELS, canvas_layout};
    use rjtd_core::Error;

    #[test]
    fn preserves_unit_scale_fallback_for_invalid_scale() {
        // Given
        let scale = 0.0;

        // When
        let layout = canvas_layout(794.0, 1123.0, scale).unwrap();

        // Then
        assert_eq!(layout.width(), 794);
        assert_eq!(layout.height(), 1123);
        assert_eq!(layout.scale(), 1.0);
    }

    #[test]
    fn rejects_canvas_dimension_over_browser_limit() {
        // Given
        let scale = 32.0;

        // When
        let result = canvas_layout(800.0, 600.0, scale);

        // Then
        assert_eq!(
            result,
            Err(Error::ResourceLimit {
                resource: "canvas width pixels",
                limit: 16_384,
                actual: 25_600,
            })
        );
    }

    #[test]
    fn rejects_canvas_total_pixel_budget() {
        // Given
        let scale = 1.0;

        // When
        let result = canvas_layout(10_000.0, 10_000.0, scale);

        // Then
        assert_eq!(
            result,
            Err(Error::ResourceLimit {
                resource: "canvas pixels",
                limit: MAX_CANVAS_PIXELS,
                actual: 100_000_000,
            })
        );
    }
}
