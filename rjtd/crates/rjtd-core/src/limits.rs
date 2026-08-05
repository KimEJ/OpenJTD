use crate::{Error, Result};

const MEBIBYTE: usize = 1024 * 1024;

/// Resource classes accounted for across one limits-aware document construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    Stream,
    StreamBytes,
    Record,
    RecordBytes,
    Image,
    ImageBytes,
    ImagePixels,
    Page,
    PageLine,
}

impl ResourceKind {
    pub const fn resource_name(self) -> &'static str {
        match self {
            Self::Stream => "document streams",
            Self::StreamBytes => "document stream bytes",
            Self::Record => "document records",
            Self::RecordBytes => "document record bytes",
            Self::Image => "embedded images",
            Self::ImageBytes => "embedded image bytes",
            Self::ImagePixels => "embedded image pixels",
            Self::Page => "document pages",
            Self::PageLine => "document page lines",
        }
    }
}

/// Resource limits applied to untrusted document input and LH5 payloads.
///
/// The per-member output limit applies to one LH5 member, and the total output limit applies to
/// every member sharing one [`DecompressionBudget`]. Input checks happen after a caller has
/// supplied a `&[u8]`; they prevent parser and decoder allocations, but cannot reclaim memory
/// already allocated by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseLimits {
    max_input_bytes: usize,
    max_decompressed_bytes: usize,
    max_total_decompressed_bytes: usize,
    max_decompression_ratio: usize,
    decompression_ratio_floor_bytes: usize,
    max_streams: usize,
    max_stream_bytes: usize,
    max_records: usize,
    max_record_bytes: usize,
    max_images: usize,
    max_image_bytes: usize,
    max_image_width: usize,
    max_image_height: usize,
    max_image_pixels: usize,
    max_pages: usize,
    max_page_lines: usize,
}

/// Shared state that accounts for total LH5 output during one document traversal.
///
/// Construct this from [`ParseLimits::decompression_budget`]. It is public only as compositional
/// plumbing between rjtd crates; downstream callers should use `*_with_limits` entry points
/// instead. This type is not a stable downstream API.
#[derive(Debug)]
pub struct DecompressionBudget {
    limits: ParseLimits,
    decompressed_bytes: usize,
}

/// Shared accounting state for a limits-aware document construction.
///
/// Construct this from [`ParseLimits::resource_budget`] and pass it through every cooperating
/// rjtd layer. This is compositional plumbing, not a stable downstream API.
#[derive(Debug)]
pub struct ResourceBudget {
    decompression: DecompressionBudget,
    limits: ParseLimits,
    streams: usize,
    stream_bytes: usize,
    records: usize,
    record_bytes: usize,
    images: usize,
    image_bytes: usize,
    image_pixels: usize,
    pages: usize,
    page_lines: usize,
}

impl ParseLimits {
    pub const DEFAULT: Self = Self {
        max_input_bytes: 64 * MEBIBYTE,
        max_decompressed_bytes: 256 * MEBIBYTE,
        max_total_decompressed_bytes: 256 * MEBIBYTE,
        max_decompression_ratio: 256,
        decompression_ratio_floor_bytes: MEBIBYTE,
        max_streams: 1_024,
        max_stream_bytes: 64 * MEBIBYTE,
        max_records: 65_536,
        max_record_bytes: 64 * MEBIBYTE,
        max_images: 1_024,
        max_image_bytes: 64 * MEBIBYTE,
        max_image_width: 16_384,
        max_image_height: 16_384,
        max_image_pixels: 64 * MEBIBYTE,
        max_pages: 65_536,
        max_page_lines: MEBIBYTE,
    };

    pub const fn max_input_bytes(self) -> usize {
        self.max_input_bytes
    }

    pub const fn with_max_input_bytes(mut self, max_input_bytes: usize) -> Self {
        self.max_input_bytes = max_input_bytes;
        self
    }

    /// Sets the maximum output bytes for each individual LH5 member.
    pub const fn with_max_decompressed_bytes(mut self, max_decompressed_bytes: usize) -> Self {
        self.max_decompressed_bytes = max_decompressed_bytes;
        self
    }

    /// Returns the maximum combined LH5 output for one shared decompression budget.
    pub const fn max_total_decompressed_bytes(self) -> usize {
        self.max_total_decompressed_bytes
    }

    /// Sets the maximum combined LH5 output for one shared decompression budget.
    pub const fn with_max_total_decompressed_bytes(
        mut self,
        max_total_decompressed_bytes: usize,
    ) -> Self {
        self.max_total_decompressed_bytes = max_total_decompressed_bytes;
        self
    }

    pub const fn with_max_decompression_ratio(mut self, max_decompression_ratio: usize) -> Self {
        self.max_decompression_ratio = max_decompression_ratio;
        self
    }

    pub const fn with_decompression_ratio_floor_bytes(
        mut self,
        decompression_ratio_floor_bytes: usize,
    ) -> Self {
        self.decompression_ratio_floor_bytes = decompression_ratio_floor_bytes;
        self
    }

    pub const fn with_max_streams(mut self, max_streams: usize) -> Self {
        self.max_streams = max_streams;
        self
    }

    /// Sets the maximum bytes represented by all readable container streams.
    ///
    /// Strict CFB inventories use stream declarations. Lenient recovery inventories use the
    /// bytes reachable through their sector chains because their declarations are not trusted.
    pub const fn with_max_stream_bytes(mut self, max_stream_bytes: usize) -> Self {
        self.max_stream_bytes = max_stream_bytes;
        self
    }

    pub const fn with_max_records(mut self, max_records: usize) -> Self {
        self.max_records = max_records;
        self
    }

    /// Sets the maximum bytes represented by retained frame and embedding records.
    pub const fn with_max_record_bytes(mut self, max_record_bytes: usize) -> Self {
        self.max_record_bytes = max_record_bytes;
        self
    }

    pub const fn with_max_images(mut self, max_images: usize) -> Self {
        self.max_images = max_images;
        self
    }

    /// Sets the maximum retained payload bytes across embedded images.
    pub const fn with_max_image_bytes(mut self, max_image_bytes: usize) -> Self {
        self.max_image_bytes = max_image_bytes;
        self
    }

    /// Sets the maximum width accepted from embedded-image metadata.
    pub const fn with_max_image_width(mut self, max_image_width: usize) -> Self {
        self.max_image_width = max_image_width;
        self
    }

    /// Sets the maximum height accepted from embedded-image metadata.
    pub const fn with_max_image_height(mut self, max_image_height: usize) -> Self {
        self.max_image_height = max_image_height;
        self
    }

    /// Sets the maximum pixel count accepted from embedded-image metadata.
    pub const fn with_max_image_pixels(mut self, max_image_pixels: usize) -> Self {
        self.max_image_pixels = max_image_pixels;
        self
    }

    pub const fn with_max_pages(mut self, max_pages: usize) -> Self {
        self.max_pages = max_pages;
        self
    }

    /// Sets the maximum number of page lines retained for document rendering.
    pub const fn with_max_page_lines(mut self, max_page_lines: usize) -> Self {
        self.max_page_lines = max_page_lines;
        self
    }

    pub fn check_input_size(self, actual: usize) -> Result<()> {
        check_resource("input bytes", self.max_input_bytes, actual)
    }

    pub fn check_lh5_output_size(self, packed_size: usize, original_size: usize) -> Result<()> {
        check_resource(
            "LH5 decompressed bytes",
            self.max_decompressed_bytes,
            original_size,
        )?;
        let ratio_limit = packed_size
            .saturating_mul(self.max_decompression_ratio)
            .max(self.decompression_ratio_floor_bytes);
        check_resource("LH5 expansion bytes", ratio_limit, original_size)
    }

    /// Creates the shared budget used to enforce this limit set's total LH5 output.
    ///
    /// This is compositional plumbing for `*_with_budget` functions, not a stable downstream API.
    pub const fn decompression_budget(self) -> DecompressionBudget {
        DecompressionBudget {
            limits: self,
            decompressed_bytes: 0,
        }
    }

    /// Creates the shared accounting budget used across limits-aware rjtd layers.
    pub const fn resource_budget(self) -> ResourceBudget {
        ResourceBudget {
            decompression: self.decompression_budget(),
            limits: self,
            streams: 0,
            stream_bytes: 0,
            records: 0,
            record_bytes: 0,
            images: 0,
            image_bytes: 0,
            image_pixels: 0,
            pages: 0,
            page_lines: 0,
        }
    }
}

impl DecompressionBudget {
    pub(crate) fn check_input_size(&self, actual: usize) -> Result<()> {
        self.limits.check_input_size(actual)
    }

    pub(crate) fn reserve_lh5_output(
        &mut self,
        packed_size: usize,
        original_size: usize,
    ) -> Result<()> {
        self.limits
            .check_lh5_output_size(packed_size, original_size)?;
        let actual =
            self.decompressed_bytes
                .checked_add(original_size)
                .ok_or(Error::ResourceLimit {
                    resource: "total LH5 decompressed bytes",
                    limit: self.limits.max_total_decompressed_bytes,
                    actual: usize::MAX,
                })?;
        check_resource(
            "total LH5 decompressed bytes",
            self.limits.max_total_decompressed_bytes,
            actual,
        )?;
        self.decompressed_bytes = actual;
        Ok(())
    }
}

impl ResourceBudget {
    pub fn check_input_size(&self, actual: usize) -> Result<()> {
        self.decompression.check_input_size(actual)
    }

    pub fn decompression_budget_mut(&mut self) -> &mut DecompressionBudget {
        &mut self.decompression
    }

    /// Reserves container streams before their bytes are retained by the model.
    pub fn reserve_streams(&mut self, count: usize, bytes: usize) -> Result<()> {
        self.reserve_pair(
            ResourceKind::Stream,
            count,
            ResourceKind::StreamBytes,
            bytes,
        )
    }

    /// Verifies that a stream read did not exceed its preflight accounting.
    pub fn verify_stream_bytes(&self, accounted: usize, actual: usize) -> Result<()> {
        check_resource(ResourceKind::StreamBytes.resource_name(), accounted, actual)
    }

    /// Reserves one retained frame or embedding record before it grows a collection.
    pub fn reserve_record(&mut self, bytes: usize) -> Result<()> {
        self.reserve_pair(ResourceKind::Record, 1, ResourceKind::RecordBytes, bytes)
    }

    /// Reserves one retained image payload before it is cloned into document evidence.
    pub fn reserve_image(&mut self, bytes: usize) -> Result<()> {
        self.reserve_pair(ResourceKind::Image, 1, ResourceKind::ImageBytes, bytes)
    }

    /// Checks embedded-image metadata without decoding a bitmap.
    pub fn check_image_dimensions(&mut self, width: u32, height: u32) -> Result<()> {
        check_resource(
            "embedded image width",
            self.limits.max_image_width,
            width as usize,
        )?;
        check_resource(
            "embedded image height",
            self.limits.max_image_height,
            height as usize,
        )?;
        let pixels = (width as usize)
            .checked_mul(height as usize)
            .ok_or(Error::ResourceLimit {
                resource: ResourceKind::ImagePixels.resource_name(),
                limit: self.limits.max_image_pixels,
                actual: usize::MAX,
            })?;
        self.reserve(ResourceKind::ImagePixels, pixels)
    }

    /// Reserves a page before its first line is pushed into the page vector.
    pub fn reserve_page(&mut self) -> Result<()> {
        self.reserve(ResourceKind::Page, 1)
    }

    /// Reserves a page line before its text and layout metadata are retained.
    pub fn reserve_page_line(&mut self) -> Result<()> {
        self.reserve(ResourceKind::PageLine, 1)
    }

    /// Reserves all page and page-line output before render preparation allocates it.
    pub fn reserve_page_output(&mut self, pages: usize, lines: usize) -> Result<()> {
        self.reserve_pair(ResourceKind::Page, pages, ResourceKind::PageLine, lines)
    }

    fn reserve_pair(
        &mut self,
        first_kind: ResourceKind,
        first_amount: usize,
        second_kind: ResourceKind,
        second_amount: usize,
    ) -> Result<()> {
        let first_actual = self.checked_total(first_kind, first_amount)?;
        let second_actual = self.checked_total(second_kind, second_amount)?;
        self.set_used(first_kind, first_actual);
        self.set_used(second_kind, second_actual);
        Ok(())
    }

    fn reserve(&mut self, kind: ResourceKind, amount: usize) -> Result<()> {
        let actual = self.checked_total(kind, amount)?;
        self.set_used(kind, actual);
        Ok(())
    }

    fn checked_total(&self, kind: ResourceKind, amount: usize) -> Result<usize> {
        let used = self.used(kind);
        let limit = self.limit(kind);
        let actual = used.checked_add(amount).ok_or(Error::ResourceLimit {
            resource: kind.resource_name(),
            limit,
            actual: usize::MAX,
        })?;
        check_resource(kind.resource_name(), limit, actual)?;
        Ok(actual)
    }

    fn used(&self, kind: ResourceKind) -> usize {
        match kind {
            ResourceKind::Stream => self.streams,
            ResourceKind::StreamBytes => self.stream_bytes,
            ResourceKind::Record => self.records,
            ResourceKind::RecordBytes => self.record_bytes,
            ResourceKind::Image => self.images,
            ResourceKind::ImageBytes => self.image_bytes,
            ResourceKind::ImagePixels => self.image_pixels,
            ResourceKind::Page => self.pages,
            ResourceKind::PageLine => self.page_lines,
        }
    }

    fn limit(&self, kind: ResourceKind) -> usize {
        match kind {
            ResourceKind::Stream => self.limits.max_streams,
            ResourceKind::StreamBytes => self.limits.max_stream_bytes,
            ResourceKind::Record => self.limits.max_records,
            ResourceKind::RecordBytes => self.limits.max_record_bytes,
            ResourceKind::Image => self.limits.max_images,
            ResourceKind::ImageBytes => self.limits.max_image_bytes,
            ResourceKind::ImagePixels => self.limits.max_image_pixels,
            ResourceKind::Page => self.limits.max_pages,
            ResourceKind::PageLine => self.limits.max_page_lines,
        }
    }

    fn set_used(&mut self, kind: ResourceKind, value: usize) {
        match kind {
            ResourceKind::Stream => self.streams = value,
            ResourceKind::StreamBytes => self.stream_bytes = value,
            ResourceKind::Record => self.records = value,
            ResourceKind::RecordBytes => self.record_bytes = value,
            ResourceKind::Image => self.images = value,
            ResourceKind::ImageBytes => self.image_bytes = value,
            ResourceKind::ImagePixels => self.image_pixels = value,
            ResourceKind::Page => self.pages = value,
            ResourceKind::PageLine => self.page_lines = value,
        }
    }
}

impl Default for ParseLimits {
    fn default() -> Self {
        Self::DEFAULT
    }
}

fn check_resource(resource: &'static str, limit: usize, actual: usize) -> Result<()> {
    if actual > limit {
        Err(Error::ResourceLimit {
            resource,
            limit,
            actual,
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ParseLimits;
    use crate::Error;

    #[test]
    fn rejects_input_when_actual_bytes_exceed_limit() {
        // Given
        let limits = ParseLimits::DEFAULT.with_max_input_bytes(3);

        // When
        let result = limits.check_input_size(4);

        // Then
        assert_eq!(
            result,
            Err(Error::ResourceLimit {
                resource: "input bytes",
                limit: 3,
                actual: 4,
            })
        );
    }

    #[test]
    fn rejects_lh5_output_when_expansion_ratio_exceeds_limit() {
        // Given
        let limits = ParseLimits::DEFAULT
            .with_max_decompressed_bytes(100)
            .with_max_decompression_ratio(2)
            .with_decompression_ratio_floor_bytes(0);

        // When
        let result = limits.check_lh5_output_size(4, 9);

        // Then
        assert_eq!(
            result,
            Err(Error::ResourceLimit {
                resource: "LH5 expansion bytes",
                limit: 8,
                actual: 9,
            })
        );
    }

    #[test]
    fn accepts_exact_limits_and_rejects_limit_plus_one_for_every_resource_dimension() {
        let mut streams = ParseLimits::DEFAULT.with_max_streams(2).resource_budget();
        streams.reserve_streams(2, 0).unwrap();
        assert_eq!(
            streams.reserve_streams(1, 0),
            Err(Error::ResourceLimit {
                resource: "document streams",
                limit: 2,
                actual: 3,
            })
        );

        let mut stream_bytes = ParseLimits::DEFAULT
            .with_max_stream_bytes(2)
            .resource_budget();
        stream_bytes.reserve_streams(0, 2).unwrap();
        assert_eq!(
            stream_bytes.reserve_streams(0, 1),
            Err(Error::ResourceLimit {
                resource: "document stream bytes",
                limit: 2,
                actual: 3,
            })
        );

        let mut records = ParseLimits::DEFAULT.with_max_records(2).resource_budget();
        records.reserve_record(0).unwrap();
        records.reserve_record(0).unwrap();
        assert_eq!(
            records.reserve_record(0),
            Err(Error::ResourceLimit {
                resource: "document records",
                limit: 2,
                actual: 3,
            })
        );

        let mut record_bytes = ParseLimits::DEFAULT
            .with_max_record_bytes(2)
            .resource_budget();
        record_bytes.reserve_record(2).unwrap();
        assert_eq!(
            record_bytes.reserve_record(1),
            Err(Error::ResourceLimit {
                resource: "document record bytes",
                limit: 2,
                actual: 3,
            })
        );

        let mut images = ParseLimits::DEFAULT.with_max_images(2).resource_budget();
        images.reserve_image(0).unwrap();
        images.reserve_image(0).unwrap();
        assert_eq!(
            images.reserve_image(0),
            Err(Error::ResourceLimit {
                resource: "embedded images",
                limit: 2,
                actual: 3,
            })
        );

        let mut image_bytes = ParseLimits::DEFAULT
            .with_max_image_bytes(2)
            .resource_budget();
        image_bytes.reserve_image(2).unwrap();
        assert_eq!(
            image_bytes.reserve_image(1),
            Err(Error::ResourceLimit {
                resource: "embedded image bytes",
                limit: 2,
                actual: 3,
            })
        );

        let mut pages = ParseLimits::DEFAULT.with_max_pages(2).resource_budget();
        pages.reserve_page_output(2, 0).unwrap();
        assert_eq!(
            pages.reserve_page_output(1, 0),
            Err(Error::ResourceLimit {
                resource: "document pages",
                limit: 2,
                actual: 3,
            })
        );

        let mut page_lines = ParseLimits::DEFAULT
            .with_max_page_lines(2)
            .resource_budget();
        page_lines.reserve_page_output(0, 2).unwrap();
        assert_eq!(
            page_lines.reserve_page_output(0, 1),
            Err(Error::ResourceLimit {
                resource: "document page lines",
                limit: 2,
                actual: 3,
            })
        );

        let mut dimensions = ParseLimits::DEFAULT
            .with_max_image_width(2)
            .with_max_image_height(2)
            .with_max_image_pixels(4)
            .resource_budget();
        dimensions.check_image_dimensions(2, 2).unwrap();
        assert_eq!(
            dimensions.check_image_dimensions(3, 1),
            Err(Error::ResourceLimit {
                resource: "embedded image width",
                limit: 2,
                actual: 3,
            })
        );
        assert_eq!(
            dimensions.check_image_dimensions(1, 3),
            Err(Error::ResourceLimit {
                resource: "embedded image height",
                limit: 2,
                actual: 3,
            })
        );
        assert_eq!(
            dimensions.check_image_dimensions(1, 1),
            Err(Error::ResourceLimit {
                resource: "embedded image pixels",
                limit: 4,
                actual: 5,
            })
        );
    }

    #[test]
    fn does_not_commit_a_count_when_the_paired_byte_reservation_fails() {
        let mut budget = ParseLimits::DEFAULT
            .with_max_streams(1)
            .with_max_stream_bytes(1)
            .resource_budget();

        assert_eq!(
            budget.reserve_streams(1, 2),
            Err(Error::ResourceLimit {
                resource: "document stream bytes",
                limit: 1,
                actual: 2,
            })
        );
        assert!(budget.reserve_streams(1, 1).is_ok());
    }
}
