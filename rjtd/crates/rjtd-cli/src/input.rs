use std::fs::File;
use std::io::Read;
use std::path::Path;

use rjtd_core::ParseLimits;

pub(crate) fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>, String> {
    read_file_with_limits(path, ParseLimits::DEFAULT)
}

fn read_file_with_limits(path: impl AsRef<Path>, limits: ParseLimits) -> Result<Vec<u8>, String> {
    let path = path.as_ref();
    let file =
        File::open(path).map_err(|error| format!("cannot open `{}`: {error}", path.display()))?;
    let metadata = file
        .metadata()
        .map_err(|error| format!("cannot inspect `{}`: {error}", path.display()))?;
    let metadata_len = usize::try_from(metadata.len())
        .map_err(|_| format!("`{}` is too large for this platform", path.display()))?;
    limits
        .check_input_size(metadata_len)
        .map_err(|error| error.to_string())?;

    let mut bytes = Vec::new();
    bytes.try_reserve_exact(metadata_len).map_err(|error| {
        format!(
            "cannot reserve {metadata_len} bytes to read `{}`: {error}",
            path.display()
        )
    })?;
    let read_limit = u64::try_from(limits.max_input_bytes().saturating_add(1)).unwrap_or(u64::MAX);
    file.take(read_limit)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read `{}`: {error}", path.display()))?;
    limits
        .check_input_size(bytes.len())
        .map_err(|error| error.to_string())?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::read_file_with_limits;
    use rjtd_core::ParseLimits;

    static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn rejects_file_before_reading_past_configured_limit() {
        // Given
        let counter = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rjtd-input-limit-{}-{counter}.jtd",
            std::process::id()
        ));
        fs::write(&path, b"four").unwrap();
        let limits = ParseLimits::DEFAULT.with_max_input_bytes(3);

        // When
        let result = read_file_with_limits(&path, limits);
        fs::remove_file(&path).unwrap();

        // Then
        assert_eq!(
            result.unwrap_err(),
            "resource limit exceeded: input bytes is 4, limit is 3"
        );
    }
}
