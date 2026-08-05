use std::io::{self, Write};

use rjtd_core::container::StreamStorage;

use crate::BROKEN_PIPE_EXIT;

pub(crate) const FNV1A64_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV1A64_PRIME: u64 = 0x0000_0100_0000_01b3;

pub(crate) fn required_path(path: Option<String>, command: &str) -> Result<String, String> {
    path.ok_or_else(|| format!("missing path for `{command}`"))
}

pub(crate) fn required_page_index(
    page_index: Option<String>,
    command: &str,
) -> Result<u32, String> {
    let value = page_index.ok_or_else(|| format!("missing page index for `{command}`"))?;
    value
        .parse::<u32>()
        .map_err(|_| format!("invalid page index `{value}` for `{command}`"))
}

pub(crate) fn stream_chain_offset_basis(storage: StreamStorage) -> &'static str {
    match storage {
        StreamStorage::Mini => "mini-stream",
        StreamStorage::Regular => "file",
    }
}

pub(crate) fn write_stdout(text: &str) -> Result<(), String> {
    write_stdout_bytes(text.as_bytes())
}

pub(crate) fn write_stdout_line(line: &str) -> Result<(), String> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(line.as_bytes()).map_err(stdout_error)?;
    stdout.write_all(b"\n").map_err(stdout_error)
}

pub(crate) fn write_stdout_bytes(bytes: &[u8]) -> Result<(), String> {
    io::stdout().write_all(bytes).map_err(stdout_error)
}

pub(crate) fn stdout_error(error: io::Error) -> String {
    if error.kind() == io::ErrorKind::BrokenPipe {
        BROKEN_PIPE_EXIT.to_string()
    } else {
        format!("cannot write to stdout: {error}")
    }
}

pub(crate) fn escaped_path(path: &str) -> String {
    let mut escaped = String::new();
    for character in path.chars() {
        if character.is_ascii_control() {
            escaped.push_str(&format!("\\x{:02X}", character as u32));
        } else {
            escaped.push(character);
        }
    }
    escaped
}

pub(crate) fn escaped_text(text: &str) -> String {
    let mut escaped = String::new();
    for character in text.chars() {
        match character {
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_ascii_control() => {
                escaped.push_str(&format!("\\u{:04X}", character as u32));
            }
            character => escaped.push(character),
        }
    }
    escaped
}

pub(crate) fn escaped_text_preview(text: &str, max_chars: usize) -> String {
    let mut preview = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        preview.push_str("...");
    }
    escaped_text(&preview)
}

pub(crate) fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

pub(crate) fn format_hex_preview(bytes: &[u8], max_bytes: usize) -> String {
    if bytes.is_empty() {
        return "-".to_string();
    }

    let preview_len = bytes.len().min(max_bytes);
    let mut preview = bytes_to_hex(&bytes[..preview_len]);
    if bytes.len() > max_bytes {
        preview.push_str("...");
    }
    preview
}

pub(crate) fn fnv1a64(bytes: &[u8]) -> u64 {
    fnv1a64_update(FNV1A64_OFFSET, bytes)
}

pub(crate) fn fnv1a64_update(mut digest: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        digest ^= *byte as u64;
        digest = digest.wrapping_mul(FNV1A64_PRIME);
    }
    digest
}

pub(crate) fn format_fnv1a64_digest(digest: u64) -> String {
    format!("0x{digest:016x}")
}

pub(crate) fn ascii_text_runs(bytes: &[u8], min_chars: usize) -> Vec<(usize, String)> {
    let mut runs = Vec::new();
    let mut start = None;
    let mut text = String::new();

    for (offset, byte) in bytes.iter().copied().enumerate() {
        if byte.is_ascii_graphic() || byte == b' ' {
            start.get_or_insert(offset);
            text.push(byte as char);
            continue;
        }

        push_text_run(&mut runs, start.take(), &mut text, min_chars);
    }
    push_text_run(&mut runs, start, &mut text, min_chars);

    runs
}

#[derive(Clone, Copy)]
pub(crate) enum Utf16Endian {
    Little,
    Big,
}

pub(crate) fn utf16_text_runs(
    bytes: &[u8],
    endian: Utf16Endian,
    min_chars: usize,
) -> Vec<(usize, String)> {
    let mut runs = Vec::new();
    for alignment in 0..2 {
        let mut start = None;
        let mut text = String::new();
        let mut offset = alignment;
        while offset + 1 < bytes.len() {
            let unit = match endian {
                Utf16Endian::Little => u16::from_le_bytes([bytes[offset], bytes[offset + 1]]),
                Utf16Endian::Big => u16::from_be_bytes([bytes[offset], bytes[offset + 1]]),
            };
            if let Some(character) = char::from_u32(unit as u32)
                && is_probe_text_char(character)
            {
                start.get_or_insert(offset);
                text.push(character);
                offset += 2;
                continue;
            }

            push_text_run(&mut runs, start.take(), &mut text, min_chars);
            offset += 2;
        }
        push_text_run(&mut runs, start, &mut text, min_chars);
    }
    runs.sort_by_key(|(offset, _)| *offset);
    runs
}

pub(crate) fn push_text_run(
    runs: &mut Vec<(usize, String)>,
    start: Option<usize>,
    text: &mut String,
    min_chars: usize,
) {
    if let Some(start) = start
        && text.chars().count() >= min_chars
    {
        runs.push((start, std::mem::take(text)));
        return;
    }

    text.clear();
}

pub(crate) fn is_probe_text_char(character: char) -> bool {
    !character.is_control() && character != '\u{fffd}'
}

pub(crate) fn read_be32_candidate(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

pub(crate) fn format_optional_usize(value: Option<usize>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_model_table_source_index(value: usize) -> String {
    if value >= usize::MAX - 1 {
        "-".to_string()
    } else {
        value.to_string()
    }
}

pub(crate) fn format_optional_u32(value: Option<u32>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn be16_words(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    bytes
        .chunks_exact(2)
        .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
}

pub(crate) fn be32_dwords(bytes: &[u8]) -> impl Iterator<Item = u32> + '_ {
    bytes
        .chunks_exact(4)
        .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(crate) fn le32_dwords(bytes: &[u8]) -> impl Iterator<Item = u32> + '_ {
    bytes
        .chunks_exact(4)
        .map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(crate) fn stream_tail(stream: &[u8], offset: usize, byte_count: usize) -> &[u8] {
    let end = offset.saturating_add(byte_count).min(stream.len());
    &stream[offset..end]
}

pub(crate) fn find_subslice_offsets(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return Vec::new();
    }

    haystack
        .windows(needle.len())
        .enumerate()
        .filter_map(|(offset, candidate)| (candidate == needle).then_some(offset))
        .collect()
}

pub(crate) fn parse_hex_bytes(input: &str) -> Result<Vec<u8>, String> {
    let compact = input
        .chars()
        .filter(|character| !character.is_ascii_whitespace() && *character != '_')
        .collect::<String>()
        .replace("0x", "")
        .replace("0X", "");

    if compact.len() % 2 != 0 {
        return Err("hex bytes must contain an even number of digits".into());
    }

    let mut bytes = Vec::with_capacity(compact.len() / 2);
    let mut chars = compact.chars();
    while let (Some(high), Some(low)) = (chars.next(), chars.next()) {
        bytes.push(hex_pair(high, low)?);
    }
    Ok(bytes)
}

pub(crate) fn parse_u16_argument(input: &str) -> Result<u16, String> {
    let compact = input.replace('_', "");
    if let Some(hex) = compact
        .strip_prefix("0x")
        .or_else(|| compact.strip_prefix("0X"))
    {
        u16::from_str_radix(hex, 16).map_err(|_| format!("invalid u16 value: {input}"))
    } else {
        compact
            .parse::<u16>()
            .map_err(|_| format!("invalid u16 value: {input}"))
    }
}

pub(crate) fn unescaped_path(path: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut chars = path.chars().peekable();

    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }

        match chars.next() {
            Some('x') => {
                let high = chars
                    .next()
                    .ok_or_else(|| "incomplete \\x escape in stream path".to_string())?;
                let low = chars
                    .next()
                    .ok_or_else(|| "incomplete \\x escape in stream path".to_string())?;
                let byte = hex_pair(high, low)?;
                output.push(byte as char);
            }
            Some(other) => {
                output.push('\\');
                output.push(other);
            }
            None => output.push('\\'),
        }
    }

    Ok(output)
}

pub(crate) fn hex_pair(high: char, low: char) -> Result<u8, String> {
    let high = high
        .to_digit(16)
        .ok_or_else(|| format!("invalid hex escape digit: {high}"))?;
    let low = low
        .to_digit(16)
        .ok_or_else(|| format!("invalid hex escape digit: {low}"))?;
    Ok(((high << 4) | low) as u8)
}
