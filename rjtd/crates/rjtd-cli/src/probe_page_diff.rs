use std::collections::{BTreeMap, BTreeSet};

use rjtd_core::layout_mark::read_page_mark;
use rjtd_model::page_mark_u16_geometry_profile;

use crate::probe_signals::JtdSignal;

const PAGE_MARK_U16_PROFILE_WORD_INDEXES: [usize; 8] = [10, 13, 14, 17, 18, 19, 20, 21];

#[derive(Debug, Clone)]
pub(crate) struct PageTuple {
    class_name: String,
    words: BTreeMap<usize, u16>,
}

pub(crate) fn page_tuples(bytes: &[u8]) -> Vec<PageTuple> {
    let Ok(page_mark) = read_page_mark(bytes) else {
        return Vec::new();
    };
    let mut by_class = BTreeMap::<String, PageTuple>::new();
    for entry in page_mark.entries() {
        let fields = be16_words(entry.raw()).collect::<Vec<_>>();
        let class_name = page_mark_u16_geometry_profile(&fields)
            .class_name()
            .to_string();
        by_class
            .entry(class_name.clone())
            .or_insert_with(|| PageTuple {
                class_name,
                words: PAGE_MARK_U16_PROFILE_WORD_INDEXES
                    .iter()
                    .filter_map(|index| fields.get(*index).map(|word| (*index, *word)))
                    .collect(),
            });
    }
    by_class.into_values().collect()
}

pub(crate) fn page_diff_lines(
    base: &[PageTuple],
    candidate: &[PageTuple],
    base_signal: &JtdSignal,
    candidate_signal: &JtdSignal,
) -> Vec<String> {
    let mut lines = vec![format!(
        "page-summary\tbaseFamily={}\tcandidateFamily={}\tbaseEntries={}\tcandidateEntries={}\tpageTupleSignatureSame={}",
        base_signal.page_family,
        candidate_signal.page_family,
        base_signal.page_entries,
        candidate_signal.page_entries,
        base_signal.page_tuple_signature == candidate_signal.page_tuple_signature
    )];
    for (entry_index, class_name) in page_classes(base, candidate).iter().enumerate() {
        let base_tuple = base.iter().find(|tuple| &tuple.class_name == class_name);
        let candidate_tuple = candidate
            .iter()
            .find(|tuple| &tuple.class_name == class_name);
        for word_index in PAGE_MARK_U16_PROFILE_WORD_INDEXES {
            let base_word = base_tuple.and_then(|tuple| tuple.words.get(&word_index));
            let candidate_word = candidate_tuple.and_then(|tuple| tuple.words.get(&word_index));
            if base_word != candidate_word {
                lines.push(format!(
                    "page-tuple-diff\tentry={entry_index}\tclass={class_name}\tword=w{word_index}\tbase={}\tcandidate={}\tstatus=changed",
                    format_page_word(base_word),
                    format_page_word(candidate_word)
                ));
            }
        }
    }
    if lines.len() == 1 {
        lines.push("page-tuple-diff\tstatus=none".to_string());
    }
    lines
}

fn page_classes(base: &[PageTuple], candidate: &[PageTuple]) -> Vec<String> {
    base.iter()
        .chain(candidate)
        .map(|tuple| tuple.class_name.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn format_page_word(word: Option<&u16>) -> String {
    word.map_or("-".to_string(), u16::to_string)
}

fn be16_words(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
}
