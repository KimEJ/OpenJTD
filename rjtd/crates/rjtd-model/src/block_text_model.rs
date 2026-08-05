use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Paragraph(Paragraph),
    Unknown(UnknownBlock),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    Text(TextRun),
    Ruby(RubyAnnotation),
    Unknown(UnknownObject),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextRun {
    pub(crate) text: String,
    pub(crate) style: Option<StyleRef>,
    pub(crate) source_span: Option<TextSourceSpan>,
}

impl TextRun {
    pub fn new(text: impl Into<String>, style: Option<StyleRef>) -> Self {
        Self::with_source_span(text, style, None)
    }

    pub fn with_source_span(
        text: impl Into<String>,
        style: Option<StyleRef>,
        source_span: Option<TextSourceSpan>,
    ) -> Self {
        Self {
            text: text.into(),
            style,
            source_span,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn style(&self) -> Option<&StyleRef> {
        self.style.as_ref()
    }

    pub fn source_span(&self) -> Option<&TextSourceSpan> {
        self.source_span.as_ref()
    }

    pub(crate) fn can_extend_source_span(&self, next: Option<&TextSourceSpan>) -> bool {
        match (self.source_span.as_ref(), next) {
            (None, None) => true,
            (Some(current), Some(next)) => {
                current.byte_end() == next.byte_start() && current.unit_end() == next.unit_start()
            }
            _ => false,
        }
    }

    pub(crate) fn push_text_with_span(&mut self, text: &str, next: Option<TextSourceSpan>) {
        self.text.push_str(text);
        match (self.source_span.as_mut(), next) {
            (Some(current), Some(next)) => {
                current.byte_end = next.byte_end();
                current.unit_end = next.unit_end();
            }
            (None, None) => {}
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleRef {
    pub(crate) id: String,
}

impl StyleRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownBlock {
    pub(crate) source: UnknownRecordKind,
    pub(crate) payload: Vec<u8>,
}

impl UnknownBlock {
    pub fn new(source: UnknownRecordKind, payload: Vec<u8>) -> Self {
        Self { source, payload }
    }

    pub fn source(&self) -> &UnknownRecordKind {
        &self.source
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownStyle {
    pub(crate) name: Option<String>,
    pub(crate) source: UnknownRecordKind,
    pub(crate) payload: Vec<u8>,
}

impl UnknownStyle {
    pub fn new(source: UnknownRecordKind, payload: Vec<u8>) -> Self {
        Self {
            name: None,
            source,
            payload,
        }
    }

    pub fn from_stream(name: impl Into<String>, payload: Vec<u8>) -> Self {
        Self {
            name: Some(name.into()),
            source: UnknownRecordKind::new(None),
            payload,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn source(&self) -> &UnknownRecordKind {
        &self.source
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownObject {
    pub(crate) source: UnknownRecordKind,
    pub(crate) payload: Vec<u8>,
}

impl UnknownObject {
    pub fn new(source: UnknownRecordKind, payload: Vec<u8>) -> Self {
        Self { source, payload }
    }

    pub fn source(&self) -> &UnknownRecordKind {
        &self.source
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(Debug, Default)]
pub(crate) struct DocumentTextModelBuilder {
    pub(crate) current_inlines: Vec<Inline>,
    pub(crate) blocks: Vec<Block>,
    pub(crate) unknown_objects: Vec<UnknownObject>,
    pub(crate) text_control_boundaries: Vec<TextControlBoundary>,
    pub(crate) can_merge_current_text_run: bool,
    pub(crate) pending_ruby_base_inline_index: Option<usize>,
}

impl DocumentTextModelBuilder {
    pub(crate) fn push_text_run(&mut self, text: &str) {
        self.push_text_run_with_span(text, None);
    }

    pub(crate) fn push_text_run_with_span(
        &mut self,
        text: &str,
        source_span: Option<TextSourceSpan>,
    ) {
        self.pending_ruby_base_inline_index = None;
        self.push_text(text, ModelTextSource::TextRun, source_span);
    }

    pub(crate) fn push_inline_text(&mut self, segment: &InlineTextSegment) {
        self.push_inline_text_with_span(segment, None);
    }

    pub(crate) fn push_inline_text_with_span(
        &mut self,
        segment: &InlineTextSegment,
        source_span: Option<TextSourceSpan>,
    ) {
        self.pending_ruby_base_inline_index = None;
        let previous_block_count = self.blocks.len();
        let previous_inline_count = self.current_inlines.len();

        self.push_text(segment.text(), ModelTextSource::Inline, source_span);

        if segment.selector() == DOCUMENT_TEXT_RUBY_BASE_SELECTOR
            && previous_block_count == self.blocks.len()
            && self.current_inlines.len() == previous_inline_count + 1
        {
            self.pending_ruby_base_inline_index = Some(previous_inline_count);
        }
    }

    pub(crate) fn push_skipped_inline(&mut self, segment: &SkippedInlineTextSegment) {
        self.push_skipped_inline_with_span(segment, None);
    }

    pub(crate) fn push_skipped_inline_with_span(
        &mut self,
        segment: &SkippedInlineTextSegment,
        _source_span: Option<TextSourceSpan>,
    ) {
        if self.promote_ruby_annotation(segment) {
            return;
        }

        self.pending_ruby_base_inline_index = None;
        self.unknown_objects
            .push(unknown_object_from_skipped_inline(segment));
        self.can_merge_current_text_run = false;
    }

    pub(crate) fn push_control_boundary(
        &mut self,
        control: &DocumentTextControl,
        source_span: Option<TextSourceSpan>,
    ) {
        self.can_merge_current_text_run = false;
        self.text_control_boundaries.push(TextControlBoundary::new(
            self.text_control_boundaries.len(),
            control.code(),
            source_span,
        ));
    }

    pub(crate) fn push_text(
        &mut self,
        text: &str,
        source: ModelTextSource,
        source_span: Option<TextSourceSpan>,
    ) {
        for part in source_text_parts(text, source_span.as_ref()) {
            if !part.text.is_empty() {
                self.push_text_part(&part.text, source, part.source_span);
            }

            if part.break_after {
                self.flush_paragraph();
            }
        }
    }

    pub(crate) fn finish(mut self) -> (Vec<Block>, Vec<UnknownObject>, Vec<TextControlBoundary>) {
        self.flush_paragraph();
        (
            self.blocks,
            self.unknown_objects,
            self.text_control_boundaries,
        )
    }

    pub(crate) fn flush_paragraph(&mut self) {
        if self.current_inlines.is_empty() {
            self.can_merge_current_text_run = false;
            self.pending_ruby_base_inline_index = None;
            return;
        }

        let inlines = std::mem::take(&mut self.current_inlines);
        self.blocks
            .push(Block::Paragraph(Paragraph::new(inlines, None)));
        self.can_merge_current_text_run = false;
        self.pending_ruby_base_inline_index = None;
    }

    pub(crate) fn push_text_part(
        &mut self,
        text: &str,
        source: ModelTextSource,
        source_span: Option<TextSourceSpan>,
    ) {
        if source == ModelTextSource::TextRun
            && self.can_merge_current_text_run
            && let Some(Inline::Text(run)) = self.current_inlines.last_mut()
            && run.can_extend_source_span(source_span.as_ref())
        {
            run.push_text_with_span(text, source_span);
            return;
        }

        self.current_inlines
            .push(Inline::Text(TextRun::with_source_span(
                text,
                None,
                source_span,
            )));
        self.can_merge_current_text_run = source == ModelTextSource::TextRun;
    }

    pub(crate) fn promote_ruby_annotation(&mut self, segment: &SkippedInlineTextSegment) -> bool {
        if segment.selector() != Some(DOCUMENT_TEXT_RUBY_TEXT_SELECTOR) {
            return false;
        }

        let Some(index) = self.pending_ruby_base_inline_index.take() else {
            return false;
        };

        let Some(inline) = self.current_inlines.get_mut(index) else {
            return false;
        };

        let Inline::Text(base_run) = inline else {
            return false;
        };

        let base_text = std::mem::take(&mut base_run.text);
        let annotation = RubyAnnotation::new(
            base_text,
            segment.text(),
            DOCUMENT_TEXT_RUBY_TEXT_SELECTOR,
            unknown_object_from_skipped_inline(segment),
        );
        *inline = Inline::Ruby(annotation);
        self.can_merge_current_text_run = false;
        true
    }
}

pub(crate) fn reserve_and_verify_cfb_streams(
    data: &[u8],
    budget: &mut ResourceBudget,
) -> Result<()> {
    let Ok((entries, mode)) = inspect_cfb_entries_with_mode(data) else {
        return Ok(());
    };
    let mut streams = BTreeMap::new();

    for entry in entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
    {
        streams
            .entry(entry.path())
            .and_modify(|size: &mut u64| *size = (*size).max(entry.size()))
            .or_insert(entry.size());
    }

    for (path, declared) in streams {
        let accounted = match mode {
            CfbEntryReadMode::Strict => cfb_stream_bytes_from_u64(declared)?,
            CfbEntryReadMode::Lenient => reachable_cfb_stream_bytes(data, path).unwrap_or(0),
        };
        budget.reserve_streams(1, accounted)?;

        let Ok(stream) = read_cfb_stream(data, path) else {
            continue;
        };
        budget.verify_stream_bytes(accounted, stream.len())?;
    }

    Ok(())
}

pub(crate) fn cfb_stream_bytes_from_u64(size: u64) -> Result<usize> {
    usize::try_from(size).map_err(|_| Error::ResourceLimit {
        resource: "document stream bytes",
        limit: usize::MAX,
        actual: usize::MAX,
    })
}

pub(crate) fn reachable_cfb_stream_bytes(data: &[u8], path: &str) -> Result<usize> {
    let chain = inspect_cfb_stream_chain(data, path)?;
    let capacity = u64::try_from(chain.capacity_bytes()).unwrap_or(u64::MAX);
    cfb_stream_bytes_from_u64(chain.location().size().min(capacity))
}

pub(crate) fn record_bytes_overflow() -> Error {
    Error::ResourceLimit {
        resource: "document record bytes",
        limit: usize::MAX,
        actual: usize::MAX,
    }
}

pub(crate) fn decode_utf16le_c_string(bytes: &[u8]) -> Option<String> {
    if !bytes.len().is_multiple_of(2) {
        return None;
    }
    let units = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .take_while(|unit| *unit != 0)
        .collect::<Vec<_>>();
    String::from_utf16(&units).ok()
}

pub(crate) fn permille(numerator: usize, denominator: usize) -> Option<usize> {
    numerator.saturating_mul(1000).checked_div(denominator)
}

pub(crate) fn stream_path_ends_with(path: &str, suffix: &str) -> bool {
    path.get(path.len().saturating_sub(suffix.len())..)
        .is_some_and(|tail| tail.eq_ignore_ascii_case(suffix))
}

pub(crate) fn utf16le_printable_preview(bytes: &[u8]) -> String {
    let mut preview = String::new();
    for chunk in bytes.chunks_exact(2) {
        let value = u16::from_le_bytes([chunk[0], chunk[1]]);
        if value == 0 {
            break;
        }
        let Some(ch) = char::from_u32(u32::from(value)) else {
            break;
        };
        if ch.is_control() {
            break;
        }
        preview.push(ch);
    }
    preview
}

pub(crate) fn utf16le_bytes(text: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}

pub(crate) fn embedded_stream_role(segment: &str) -> &'static str {
    match segment.trim_start_matches(|character: char| character.is_control()) {
        "Contents" => "contents",
        "EmbeddedPress" => "embedded-press",
        "CompObj" => "comp-obj",
        "OlePres000" => "ole-presentation",
        _ => "embedded-stream",
    }
}

pub(crate) fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

pub(crate) fn push_signature_hits(
    hits: &mut Vec<ObjectImageSignatureHit>,
    stream: &[u8],
    kind: &'static str,
    signature: &[u8],
    scan_anywhere: bool,
    budget: &mut ResourceBudget,
) -> Result<()> {
    if signature.is_empty() {
        return Ok(());
    }

    if scan_anywhere {
        for (offset, candidate) in stream.windows(signature.len()).enumerate() {
            if candidate == signature {
                reserve_image_signature_candidate(budget, kind)?;
                hits.push(ObjectImageSignatureHit::new(kind, offset));
            }
        }
    } else if stream.starts_with(signature) {
        reserve_image_signature_candidate(budget, kind)?;
        hits.push(ObjectImageSignatureHit::new(kind, 0));
    }
    Ok(())
}

pub(crate) fn reserve_image_signature_candidate(
    budget: &mut ResourceBudget,
    kind: &str,
) -> Result<()> {
    let bytes = std::mem::size_of::<ObjectImageSignatureHit>()
        .checked_add(kind.len())
        .ok_or(Error::ResourceLimit {
            resource: "document record bytes",
            limit: usize::MAX,
            actual: usize::MAX,
        })?;
    budget.reserve_record(bytes)
}

pub(crate) fn image_payload_retained_bytes(
    payload_len: usize,
    header_start: usize,
    payload_start: usize,
    payload_end: usize,
    trailer_end: usize,
) -> Result<usize> {
    let header_len = payload_start
        .checked_sub(header_start)
        .ok_or(Error::ResourceLimit {
            resource: "embedded image bytes",
            limit: usize::MAX,
            actual: usize::MAX,
        })?;
    let trailer_len = trailer_end
        .checked_sub(payload_end)
        .ok_or(Error::ResourceLimit {
            resource: "embedded image bytes",
            limit: usize::MAX,
            actual: usize::MAX,
        })?;
    payload_len
        .checked_add(header_len)
        .and_then(|bytes| bytes.checked_add(trailer_len))
        .ok_or(Error::ResourceLimit {
            resource: "embedded image bytes",
            limit: usize::MAX,
            actual: usize::MAX,
        })
}

pub(crate) fn png_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    if payload.get(..8)? != b"\x89PNG\r\n\x1a\n" || payload.get(12..16)? != b"IHDR" {
        return None;
    }
    let width = u32::from_be_bytes(payload.get(16..20)?.try_into().ok()?);
    let height = u32::from_be_bytes(payload.get(20..24)?.try_into().ok()?);
    (width != 0 && height != 0).then_some(ObjectImageDimensions::new(width, height))
}

pub(crate) fn gif_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    if !(payload.starts_with(b"GIF87a") || payload.starts_with(b"GIF89a")) {
        return None;
    }
    let width = u16::from_le_bytes(payload.get(6..8)?.try_into().ok()?) as u32;
    let height = u16::from_le_bytes(payload.get(8..10)?.try_into().ok()?) as u32;
    (width != 0 && height != 0).then_some(ObjectImageDimensions::new(width, height))
}

pub(crate) fn bmp_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    if payload.get(..2)? != b"BM" {
        return None;
    }
    let dib_header_size = u32::from_le_bytes(payload.get(14..18)?.try_into().ok()?);
    if dib_header_size < 40 {
        return None;
    }
    let width = i32::from_le_bytes(payload.get(18..22)?.try_into().ok()?);
    let height = i32::from_le_bytes(payload.get(22..26)?.try_into().ok()?);
    let width = u32::try_from(width).ok()?;
    let height = height.unsigned_abs();
    (width != 0 && height != 0).then_some(ObjectImageDimensions::new(width, height))
}

pub(crate) fn jpeg_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    if payload.get(0..2)? != b"\xff\xd8" {
        return None;
    }

    let mut cursor = 2usize;
    while cursor < payload.len() {
        while cursor < payload.len() && payload[cursor] != 0xff {
            cursor += 1;
        }
        while cursor < payload.len() && payload[cursor] == 0xff {
            cursor += 1;
        }

        let marker = *payload.get(cursor)?;
        cursor += 1;
        if marker == 0xda || marker == 0xd9 {
            return None;
        }
        if marker == 0x01 || (0xd0..=0xd8).contains(&marker) {
            continue;
        }

        let length_end = cursor.checked_add(2)?;
        let length_bytes = payload.get(cursor..length_end)?;
        let segment_len = u16::from_be_bytes([length_bytes[0], length_bytes[1]]) as usize;
        if segment_len < 2 {
            return None;
        }
        let data_start = length_end;
        let data_end = data_start.checked_add(segment_len - 2)?;
        let data = payload.get(data_start..data_end)?;

        if is_jpeg_sof_marker(marker) {
            if data.len() < 5 {
                return None;
            }
            let height = u16::from_be_bytes([data[1], data[2]]) as u32;
            let width = u16::from_be_bytes([data[3], data[4]]) as u32;
            return (width != 0 && height != 0)
                .then_some(ObjectImageDimensions::new(width, height));
        }

        cursor = data_end;
    }

    None
}

pub(crate) fn is_jpeg_sof_marker(marker: u8) -> bool {
    matches!(
        marker,
        0xc0 | 0xc1 | 0xc2 | 0xc3 | 0xc5 | 0xc6 | 0xc7 | 0xc9 | 0xca | 0xcb | 0xcd | 0xce | 0xcf
    )
}

pub(crate) fn looks_like_embedded_source_path(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .any(|byte| matches!(*byte, b'\\' | b'/' | b':' | b'.'))
}

pub(crate) fn jpeg_payload_end(stream: &[u8], offset: usize) -> Option<usize> {
    let search_start = jpeg_entropy_data_start(stream, offset)?;
    stream
        .get(search_start..)?
        .windows(2)
        .position(|window| window == [0xff, 0xd9])
        .map(|relative| search_start + relative + 2)
}

pub(crate) fn jpeg_entropy_data_start(stream: &[u8], offset: usize) -> Option<usize> {
    if stream.get(offset..offset.checked_add(2)?)? != b"\xff\xd8" {
        return None;
    }

    let mut cursor = offset.checked_add(2)?;
    let mut found_sof = false;
    while cursor < stream.len() {
        while cursor < stream.len() && stream[cursor] != 0xff {
            cursor += 1;
        }
        while cursor < stream.len() && stream[cursor] == 0xff {
            cursor += 1;
        }

        let marker = *stream.get(cursor)?;
        cursor += 1;
        if marker == 0xd9 {
            return None;
        }
        if marker == 0x01 || (0xd0..=0xd8).contains(&marker) {
            continue;
        }

        let length_end = cursor.checked_add(2)?;
        let length_bytes = stream.get(cursor..length_end)?;
        let segment_len = u16::from_be_bytes([length_bytes[0], length_bytes[1]]) as usize;
        if segment_len < 2 {
            return None;
        }
        let data_start = length_end;
        let data_end = data_start.checked_add(segment_len - 2)?;
        stream.get(data_start..data_end)?;

        if is_jpeg_sof_marker(marker) {
            found_sof = true;
        }
        if marker == 0xda {
            return found_sof.then_some(data_end);
        }

        cursor = data_end;
    }

    None
}

pub(crate) fn png_payload_end(stream: &[u8], offset: usize) -> Option<usize> {
    let signature_end = offset.checked_add(8)?;
    if stream.get(offset..signature_end)? != b"\x89PNG\r\n\x1a\n" {
        return None;
    }

    let mut cursor = signature_end;
    while cursor.checked_add(12)? <= stream.len() {
        let length = u32::from_be_bytes([
            stream[cursor],
            stream[cursor + 1],
            stream[cursor + 2],
            stream[cursor + 3],
        ]) as usize;
        let chunk_type_start = cursor + 4;
        let chunk_data_start = cursor + 8;
        let chunk_end = chunk_data_start.checked_add(length)?.checked_add(4)?;
        if chunk_end > stream.len() {
            return None;
        }
        let chunk_type = &stream[chunk_type_start..chunk_type_start + 4];
        if chunk_type == b"IEND" {
            return Some(chunk_end);
        }
        cursor = chunk_end;
    }
    None
}

pub(crate) fn gif_payload_end(stream: &[u8], offset: usize) -> Option<usize> {
    let search_start = offset.checked_add(6)?;
    stream
        .get(search_start..)?
        .iter()
        .position(|byte| *byte == 0x3b)
        .map(|relative| search_start + relative + 1)
}

pub(crate) fn bmp_payload_end(stream: &[u8], offset: usize) -> Option<usize> {
    if offset != 0 || stream.get(0..2)? != b"BM" || stream.len() < 6 {
        return None;
    }
    let size = u32::from_le_bytes([stream[2], stream[3], stream[4], stream[5]]) as usize;
    (size >= 14 && size <= stream.len()).then_some(size)
}

pub(crate) fn svg_signature_offsets(stream: &[u8]) -> Vec<usize> {
    let ascii_lower = stream
        .iter()
        .map(|byte| byte.to_ascii_lowercase())
        .collect::<Vec<_>>();
    find_subslice_offsets(&ascii_lower, b"<svg")
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
