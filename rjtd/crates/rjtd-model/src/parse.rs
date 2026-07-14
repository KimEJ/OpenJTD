use rjtd_core::{Error, ParseLimits, Result};

use crate::{Document, IchitaroParser};

pub fn parse_document(data: &[u8]) -> Result<Document> {
    parse_document_with_limits(data, ParseLimits::DEFAULT)
}

/// Parses an already allocated document with explicit resource limits.
///
/// `max_decompressed_bytes` applies to each LH5 member, while the budget created here applies
/// `max_total_decompressed_bytes` across all members reached during this parse. Input limits
/// validate `data` after the caller has allocated it and therefore cannot reclaim that memory.
/// Resource-limit failures from optional style and font streams are returned instead of being
/// treated as malformed optional content.
pub fn parse_document_with_limits(data: &[u8], limits: ParseLimits) -> Result<Document> {
    limits.check_input_size(data.len())?;
    let mut budget = limits.decompression_budget();
    IchitaroParser.parse_with_budget(data, &mut budget)
}

pub(crate) fn optional_stream<T>(result: Result<T>) -> Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(error @ Error::ResourceLimit { .. }) => Err(error),
        Err(Error::InvalidData(_) | Error::NotFound(_) | Error::Unsupported(_) | Error::Io(_)) => {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::optional_stream;
    use rjtd_core::Error;

    #[test]
    fn propagates_resource_limit_from_optional_stream() {
        // Given
        let error = Error::ResourceLimit {
            resource: "LH5 decompressed bytes",
            limit: 1,
            actual: 2,
        };

        // When
        let result = optional_stream::<()>(Err(error.clone()));

        // Then
        assert_eq!(result, Err(error));
    }

    #[test]
    fn tolerates_malformed_optional_stream() {
        // Given
        let error = Error::InvalidData("truncated optional stream".to_owned());

        // When
        let result = optional_stream::<()>(Err(error));

        // Then
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn tolerates_unsupported_optional_stream() {
        // Given
        let error = Error::Unsupported("optional stream revision");

        // When
        let result = optional_stream::<()>(Err(error));

        // Then
        assert_eq!(result, Ok(None));
    }
}
