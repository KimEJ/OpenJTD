#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn scrub_embedded_pdf_eof_markers(bytes: &mut [u8]) {
    let Some(final_eof_offset) = find_last_subslice(bytes, b"%%EOF") else {
        return;
    };

    let mut position = 0usize;
    while position < final_eof_offset {
        let Some(relative_offset) = find_subslice(&bytes[position..final_eof_offset], b"%%EOF")
        else {
            break;
        };
        let marker_offset = position + relative_offset;
        if pdf_eof_marker_is_embedded_cmap_comment(bytes, marker_offset) {
            bytes[marker_offset + 4] = b'D';
        }
        position = marker_offset + b"%%EOF".len();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_eof_marker_is_embedded_cmap_comment(bytes: &[u8], marker_offset: usize) -> bool {
    let prefix_start = marker_offset.saturating_sub(96);
    let suffix_end = bytes.len().min(marker_offset + 64);
    let prefix = &bytes[prefix_start..marker_offset];
    let suffix = &bytes[marker_offset + b"%%EOF".len()..suffix_end];

    find_subslice(prefix, b"%%EndResource").is_some()
        && (suffix.starts_with(b"\nendstream") || suffix.starts_with(b"\r\nendstream"))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn find_last_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .rposition(|window| window == needle)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn validate_pdf_preview_safety(bytes: &[u8]) -> Result<(), String> {
    let issues = pdf_preview_blocking_issues(bytes);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "generated PDF contains Preview/PDFKit risky transparency constructs: {}",
            issues.join(", ")
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn pdf_preview_blocking_issues(bytes: &[u8]) -> Vec<&'static str> {
    pdf_preview_safety_issues(bytes)
        .into_iter()
        .filter(|issue| *issue != "soft-mask")
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn pdf_preview_safety_issues(bytes: &[u8]) -> Vec<&'static str> {
    let mut issues = Vec::new();
    if pdf_contains_token_sequence(bytes, &[b"/Group", b"<<"]) {
        issues.push("transparency-group-dictionary");
    }
    if pdf_contains_token_sequence(bytes, &[b"/S", b"/Transparency"]) {
        issues.push("transparency-group-subtype");
    }
    if pdf_contains_token_sequence(bytes, &[b"/SMask"]) {
        issues.push("soft-mask");
    }
    issues
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn pdf_contains_token_sequence(bytes: &[u8], tokens: &[&[u8]]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    for start in 0..bytes.len() {
        let Some(mut position) = pdf_match_token_at(bytes, start, tokens[0]) else {
            continue;
        };
        let mut matched = true;
        for token in &tokens[1..] {
            position = pdf_skip_whitespace(bytes, position);
            let Some(next_position) = pdf_match_token_at(bytes, position, token) else {
                matched = false;
                break;
            };
            position = next_position;
        }
        if matched {
            return true;
        }
    }
    false
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_match_token_at(bytes: &[u8], position: usize, token: &[u8]) -> Option<usize> {
    if token.is_empty() || !bytes.get(position..)?.starts_with(token) {
        return None;
    }
    let end = position + token.len();
    if token == b"<<" || token == b">>" {
        return Some(end);
    }
    if end < bytes.len() && !pdf_is_delimiter(bytes[end]) {
        return None;
    }
    Some(end)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn pdf_skip_whitespace(bytes: &[u8], mut position: usize) -> usize {
    while position < bytes.len() && matches!(bytes[position], 0 | b'\t' | b'\n' | 12 | b'\r' | b' ')
    {
        position += 1;
    }
    position
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_is_delimiter(byte: u8) -> bool {
    matches!(
        byte,
        0 | b'\t'
            | b'\n'
            | 12
            | b'\r'
            | b' '
            | b'('
            | b')'
            | b'<'
            | b'>'
            | b'['
            | b']'
            | b'{'
            | b'}'
            | b'/'
    )
}
