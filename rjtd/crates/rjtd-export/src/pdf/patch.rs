use super::safety::{find_last_subslice, find_subslice, pdf_skip_whitespace};

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn ensure_pdf_form_xobject_form_types(bytes: &mut Vec<u8>) -> Result<(), String> {
    let xref_offset = pdf_startxref_offset(bytes)?;
    let mut body = bytes[..xref_offset].to_vec();
    if insert_pdf_form_xobject_form_types(&mut body) == 0 {
        return Ok(());
    }

    let root_ref = parse_pdf_trailer_ref(bytes, b"/Root")
        .ok_or_else(|| "generated PDF trailer is missing /Root".to_string())?;
    let info_ref = parse_pdf_trailer_ref(bytes, b"/Info");
    let offsets = collect_pdf_object_offsets(&body)?;

    let xref_offset = body.len();
    body.extend(b"xref\n0 ");
    let xref_len = offsets
        .last()
        .map(|(object_id, _)| object_id + 1)
        .unwrap_or(1);
    body.extend(xref_len.to_string().as_bytes());
    body.push(b'\n');
    body.extend(b"0000000000 65535 f\r\n");

    let mut next_offset = offsets.iter().peekable();
    for object_id in 1..xref_len {
        if next_offset
            .peek()
            .is_some_and(|(used_id, _)| *used_id == object_id)
        {
            let (_, offset) = next_offset.next().unwrap();
            body.extend(format!("{offset:010} 00000 n\r\n").as_bytes());
        } else {
            body.extend(b"0000000000 65535 f\r\n");
        }
    }

    body.extend(b"trailer\n<<\n  /Size ");
    body.extend(xref_len.to_string().as_bytes());
    body.extend(b"\n  /Root ");
    body.extend(root_ref.to_string().as_bytes());
    body.extend(b" 0 R");
    if let Some(info_ref) = info_ref {
        body.extend(b"\n  /Info ");
        body.extend(info_ref.to_string().as_bytes());
        body.extend(b" 0 R");
    }
    body.extend(b"\n>>\nstartxref\n");
    body.extend(xref_offset.to_string().as_bytes());
    body.extend(b"\n%%EOF");

    *bytes = body;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn insert_pdf_form_xobject_form_types(bytes: &mut Vec<u8>) -> usize {
    let mut inserted = 0usize;
    let mut position = 0usize;
    while let Some(relative_offset) = find_subslice(&bytes[position..], b"/Subtype /Form") {
        let subtype_offset = position + relative_offset;
        let Some(object_start) = find_pdf_object_start_before(bytes, subtype_offset) else {
            position = subtype_offset + b"/Subtype /Form".len();
            continue;
        };
        let Some(stream_offset) = find_pdf_stream_marker_after(bytes, subtype_offset) else {
            position = subtype_offset + b"/Subtype /Form".len();
            continue;
        };
        let dictionary = &bytes[object_start..stream_offset];
        if dictionary
            .windows(b"/FormType".len())
            .any(|w| w == b"/FormType")
        {
            position = subtype_offset + b"/Subtype /Form".len();
            continue;
        }

        let insert_offset = bytes[subtype_offset..stream_offset]
            .iter()
            .position(|byte| *byte == b'\n')
            .map(|newline| subtype_offset + newline + 1)
            .unwrap_or(subtype_offset + b"/Subtype /Form".len());
        bytes.splice(
            insert_offset..insert_offset,
            b"  /FormType 1\n".iter().copied(),
        );
        inserted += 1;
        position = insert_offset + b"  /FormType 1\n".len();
    }
    inserted
}

#[cfg(not(target_arch = "wasm32"))]
fn find_pdf_object_start_before(bytes: &[u8], offset: usize) -> Option<usize> {
    let object_marker = find_last_subslice(bytes.get(..offset)?, b" obj")?;
    let line_start = bytes[..object_marker]
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(0, |newline| newline + 1);
    Some(line_start)
}

#[cfg(not(target_arch = "wasm32"))]
fn find_pdf_stream_marker_after(bytes: &[u8], offset: usize) -> Option<usize> {
    let line_feed = find_subslice(bytes.get(offset..)?, b"\nstream")?;
    Some(offset + line_feed)
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_startxref_offset(bytes: &[u8]) -> Result<usize, String> {
    let marker_offset = find_last_subslice(bytes, b"startxref")
        .ok_or_else(|| "generated PDF is missing startxref".to_string())?;
    let mut position = marker_offset + b"startxref".len();
    position = pdf_skip_whitespace(bytes, position);
    let start = position;
    while position < bytes.len() && bytes[position].is_ascii_digit() {
        position += 1;
    }
    let value = std::str::from_utf8(&bytes[start..position])
        .ok()
        .and_then(|text| text.parse::<usize>().ok())
        .ok_or_else(|| "generated PDF has invalid startxref offset".to_string())?;
    if !bytes
        .get(value..)
        .is_some_and(|tail| tail.starts_with(b"xref"))
    {
        return Err("generated PDF startxref does not point to an xref table".to_string());
    }
    Ok(value)
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_pdf_trailer_ref(bytes: &[u8], key: &[u8]) -> Option<usize> {
    let key_offset = find_subslice(bytes, key)?;
    let mut position = pdf_skip_whitespace(bytes, key_offset + key.len());
    let start = position;
    while position < bytes.len() && bytes[position].is_ascii_digit() {
        position += 1;
    }
    let object_id = std::str::from_utf8(&bytes[start..position])
        .ok()?
        .parse::<usize>()
        .ok()?;
    position = pdf_skip_whitespace(bytes, position);
    if !bytes.get(position..)?.starts_with(b"0") {
        return None;
    }
    position = pdf_skip_whitespace(bytes, position + 1);
    if !bytes.get(position..)?.starts_with(b"R") {
        return None;
    }
    Some(object_id)
}

#[cfg(not(target_arch = "wasm32"))]
fn collect_pdf_object_offsets(bytes: &[u8]) -> Result<Vec<(usize, usize)>, String> {
    let mut offsets = Vec::new();
    let mut line_start = 0usize;
    while line_start < bytes.len() {
        if let Some(object_id) = parse_pdf_object_header(bytes, line_start) {
            offsets.push((object_id, line_start));
        }
        let Some(relative_newline) = bytes[line_start..].iter().position(|byte| *byte == b'\n')
        else {
            break;
        };
        line_start += relative_newline + 1;
    }
    offsets.sort_by_key(|(object_id, _)| *object_id);
    if offsets.windows(2).any(|window| window[0].0 == window[1].0) {
        return Err("generated PDF contains duplicate object ids".to_string());
    }
    Ok(offsets)
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_pdf_object_header(bytes: &[u8], offset: usize) -> Option<usize> {
    let mut position = offset;
    while position < bytes.len() && bytes[position].is_ascii_digit() {
        position += 1;
    }
    if position == offset {
        return None;
    }
    let object_id = std::str::from_utf8(&bytes[offset..position])
        .ok()?
        .parse::<usize>()
        .ok()?;
    position = pdf_skip_plain_spaces(bytes, position);
    if !bytes.get(position..)?.starts_with(b"0") {
        return None;
    }
    position = pdf_skip_plain_spaces(bytes, position + 1);
    if !bytes.get(position..)?.starts_with(b"obj") {
        return None;
    }
    Some(object_id)
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_skip_plain_spaces(bytes: &[u8], mut position: usize) -> usize {
    while position < bytes.len() && matches!(bytes[position], b'\t' | b' ') {
        position += 1;
    }
    position
}
