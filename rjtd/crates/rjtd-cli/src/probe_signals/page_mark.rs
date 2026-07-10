use std::collections::BTreeMap;

use rjtd_core::layout_mark::{PageMark, read_page_mark};
use rjtd_model::page_mark_u16_geometry_profile;

const PAGE_MARK_U16_PROFILE_WORD_INDEXES: [usize; 8] = [10, 13, 14, 17, 18, 19, 20, 21];

pub(super) struct PageMarkSignal {
    pub(super) signature: String,
    pub(super) family: String,
    pub(super) entries: String,
    pub(super) tuple_signature: String,
}

pub(super) fn page_mark_signal(bytes: &[u8]) -> PageMarkSignal {
    match read_page_mark(bytes) {
        Ok(page_mark) => page_mark_signal_from_mark(&page_mark),
        Err(_) => PageMarkSignal {
            signature: "missing".to_string(),
            family: "missing".to_string(),
            entries: "missing".to_string(),
            tuple_signature: "missing".to_string(),
        },
    }
}

fn page_mark_signal_from_mark(page_mark: &PageMark) -> PageMarkSignal {
    let mut tuple_counts = BTreeMap::<String, usize>::new();
    for entry in page_mark.entries() {
        let fields = be16_words(entry.raw()).collect::<Vec<_>>();
        let class_name = page_mark_u16_geometry_profile(&fields).class_name();
        let tuple = PAGE_MARK_U16_PROFILE_WORD_INDEXES
            .iter()
            .map(|index| {
                fields
                    .get(*index)
                    .map(|word| format!("w{index}={word}"))
                    .unwrap_or_else(|| format!("w{index}=-"))
            })
            .collect::<Vec<_>>()
            .join(",");
        *tuple_counts
            .entry(format!("{class_name}:{tuple}"))
            .or_insert(0) += 1;
    }
    let tuple_signature = tuple_counts
        .iter()
        .map(|(tuple, count)| format!("{count}x{tuple}"))
        .collect::<Vec<_>>()
        .join("|");
    let family = page_mark.family().as_str().to_string();
    let entries = page_mark.entries().len().to_string();
    PageMarkSignal {
        signature: format!("family={family},entries={entries},tuples={tuple_signature}"),
        family,
        entries,
        tuple_signature,
    }
}

fn be16_words(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
}
