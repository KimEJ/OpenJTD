use crate::lha::decompress_lh5_member_with_budget;
use crate::{DecompressionBudget, Error, ParseLimits, Result};

pub const JUST_COMPRESSED_DOCUMENT_MAGIC: &[u8] = b"\x26\0JustCompressedDocument";
const LH5_METHOD: &[u8; 5] = b"-lh5-";

pub fn is_just_compressed_document(data: &[u8]) -> bool {
    data.starts_with(JUST_COMPRESSED_DOCUMENT_MAGIC)
}

pub fn decompress_just_compressed_document(data: &[u8]) -> Result<Vec<u8>> {
    decompress_just_compressed_document_with_limits(data, ParseLimits::DEFAULT)
}

/// Decompresses an already allocated JustCompressedDocument with explicit resource limits.
///
/// Per-member LH5 output and every member reached by this call's total budget are limited. The
/// input check occurs after the caller has allocated `data`.
pub fn decompress_just_compressed_document_with_limits(
    data: &[u8],
    limits: ParseLimits,
) -> Result<Vec<u8>> {
    let mut budget = limits.decompression_budget();
    decompress_just_compressed_document_with_budget(data, &mut budget)
}

pub(crate) fn decompress_just_compressed_document_with_budget(
    data: &[u8],
    budget: &mut DecompressionBudget,
) -> Result<Vec<u8>> {
    budget.check_input_size(data.len())?;
    if !is_just_compressed_document(data) {
        return Err(Error::InvalidData(
            "missing JustCompressedDocument marker".into(),
        ));
    }

    let method_offset = data
        .windows(LH5_METHOD.len())
        .position(|window| window == LH5_METHOD)
        .ok_or_else(|| Error::InvalidData("missing -lh5- member marker".into()))?;
    let member_start = method_offset
        .checked_sub(2)
        .ok_or_else(|| Error::InvalidData("invalid -lh5- member marker offset".into()))?;
    Ok(decompress_lh5_member_with_budget(&data[member_start..], budget)?.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::{
        JUST_COMPRESSED_DOCUMENT_MAGIC, decompress_just_compressed_document_with_limits,
        is_just_compressed_document,
    };
    use crate::{Error, ParseLimits};

    #[test]
    fn detects_just_compressed_document_payload() {
        assert!(is_just_compressed_document(
            b"\x26\0JustCompressedDocument\0payload"
        ));
        assert!(!is_just_compressed_document(b"DocumentText"));
    }

    #[test]
    fn rejects_oversized_wrapper_before_scanning_for_lh5_member() {
        let data = [JUST_COMPRESSED_DOCUMENT_MAGIC, b"tail"].concat();
        let limit = data.len() - 1;

        let result = decompress_just_compressed_document_with_limits(
            &data,
            ParseLimits::DEFAULT.with_max_input_bytes(limit),
        );

        assert_eq!(
            result,
            Err(Error::ResourceLimit {
                resource: "input bytes",
                limit,
                actual: data.len(),
            })
        );
    }
}
