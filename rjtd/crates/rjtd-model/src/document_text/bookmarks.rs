use crate::*;
use rjtd_core::mark_tag_stream::{
    MARK_TAG_PATH, MarkTagNameCandidate, parse_mark_tag_name_candidates,
};

impl Document {
    /// Read-only name candidates. Directory values and positions stay undecoded.
    pub fn bookmark_name_candidates(&self) -> Vec<MarkTagNameCandidate> {
        let mut streams = self
            .raw_streams()
            .iter()
            .filter(|s| s.name() == MARK_TAG_PATH);
        let Some(stream) = streams.next() else {
            return Vec::new();
        };
        if streams.next().is_some() {
            return Vec::new();
        }
        parse_mark_tag_name_candidates(stream.bytes()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bookmark_names_preserve_raw_bytes_and_reject_ambiguous_streams() {
        let mut bytes = b"MarkV.01".to_vec();
        bytes.extend([0, 1, 0, 0, 0, 0, 0, 1, 0, b'A']);
        let mut doc = Document::from_plain_text("BODY");
        doc.push_raw_stream(RawStream::new(MARK_TAG_PATH, bytes.clone()));
        assert_eq!(doc.bookmark_name_candidates()[0].name(), "A");
        assert_eq!(doc.raw_streams()[0].bytes(), bytes);
        doc.push_raw_stream(RawStream::new(MARK_TAG_PATH, bytes));
        assert!(doc.bookmark_name_candidates().is_empty());
    }
}
