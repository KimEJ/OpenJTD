use crate::{Error, Result};

const MEBIBYTE: usize = 1024 * 1024;

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

impl ParseLimits {
    pub const DEFAULT: Self = Self {
        max_input_bytes: 64 * MEBIBYTE,
        max_decompressed_bytes: 256 * MEBIBYTE,
        max_total_decompressed_bytes: 256 * MEBIBYTE,
        max_decompression_ratio: 256,
        decompression_ratio_floor_bytes: MEBIBYTE,
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
}
