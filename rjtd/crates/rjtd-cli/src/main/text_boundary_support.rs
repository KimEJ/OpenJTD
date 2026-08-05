use std::collections::BTreeMap;

use rjtd_core::layout_mark::{PageMark, PaperMark};
use rjtd_model::{TableCandidate, TextBoundaryCandidate, TextCountRange, TextLayoutExactEvidence};

use super::line_mark_support::is_line_mark_tag;
use super::style_support::*;
use super::support::*;
use super::text_position_count_support::*;

pub(crate) const LAYOUT_MAP_DELTA_MIN: isize = -4096;
pub(crate) const LAYOUT_MAP_DELTA_MAX: isize = 4096;

pub(crate) fn format_boundary_candidate_interval_kind(interval_count: usize) -> &'static str {
    if interval_count == 1 {
        "single"
    } else {
        "multi"
    }
}

pub(crate) fn format_table_candidate_intervals(candidate: &TableCandidate) -> String {
    if candidate.intervals().is_empty() {
        return "-".to_string();
    }

    candidate
        .intervals()
        .iter()
        .map(|interval| {
            format!(
                "{}:source-interval={},source={}-{},line-breaks={},text={}",
                interval.index(),
                interval.source_interval_index(),
                interval.source_start(),
                interval.source_end(),
                interval.line_break_count(),
                escaped_text(interval.text_preview())
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

pub(crate) fn format_table_candidate_text_shape(
    candidate: &TableCandidate,
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    basis: RangeBasis,
) -> String {
    let mut non_empty = 0usize;
    let mut empty = 0usize;
    let mut total_chars = 0usize;
    let mut min_chars: Option<usize> = None;
    let mut max_chars: Option<usize> = None;
    let mut total_line_breaks = 0usize;

    for interval in candidate.intervals() {
        let text = range_visible_text(
            entries,
            interval.source_start(),
            interval.source_end(),
            basis,
        );
        let chars = text.chars().count();
        let line_breaks = text_line_break_count(&text);
        if chars == 0 {
            empty += 1;
        } else {
            non_empty += 1;
        }
        total_chars += chars;
        total_line_breaks += line_breaks;
        min_chars = Some(min_chars.map_or(chars, |value| value.min(chars)));
        max_chars = Some(max_chars.map_or(chars, |value| value.max(chars)));
    }

    format!(
        "non-empty={non_empty},empty={empty},min-chars={},max-chars={},total-chars={total_chars},line-breaks={total_line_breaks},cell-like={}",
        min_chars
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string()),
        max_chars
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string()),
        if non_empty > 1 && empty == 0 && total_line_breaks == 0 {
            "true"
        } else {
            "false"
        }
    )
}

pub(crate) fn is_table_candidate_cell_like(
    candidate: &TableCandidate,
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    basis: RangeBasis,
) -> bool {
    let mut non_empty = 0usize;
    let mut empty = 0usize;
    let mut line_breaks = 0usize;

    for interval in candidate.intervals() {
        let text = range_visible_text(
            entries,
            interval.source_start(),
            interval.source_end(),
            basis,
        );
        if text.is_empty() {
            empty += 1;
        } else {
            non_empty += 1;
        }
        line_breaks += text_line_break_count(&text);
    }

    non_empty > 1 && empty == 0 && line_breaks == 0
}

pub(crate) fn format_table_candidate_interval_texts(
    candidate: &TableCandidate,
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    basis: RangeBasis,
) -> String {
    if candidate.intervals().is_empty() {
        return "-".to_string();
    }

    candidate
        .intervals()
        .iter()
        .map(|interval| {
            let text = range_visible_text(
                entries,
                interval.source_start(),
                interval.source_end(),
                basis,
            );
            format!(
                "{}:source-interval={},source={}-{},chars={},text={}",
                interval.index(),
                interval.source_interval_index(),
                interval.source_start(),
                interval.source_end(),
                text.chars().count(),
                escaped_text_preview(&text, 80)
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

pub(crate) fn format_table_candidate_interval_column_segments(
    candidate: &TableCandidate,
) -> String {
    let interval_segments = candidate
        .intervals()
        .iter()
        .filter(|interval| !interval.column_segments().is_empty())
        .map(|interval| {
            let segments = interval
                .column_segments()
                .iter()
                .map(|segment| {
                    format!(
                        "{}:{}:{}-{}:{}",
                        segment.index(),
                        segment.kind().as_str(),
                        segment.char_start(),
                        segment.char_end(),
                        escaped_text(segment.text())
                    )
                })
                .collect::<Vec<_>>()
                .join("|");
            format!("{}={segments}", interval.index())
        })
        .collect::<Vec<_>>();

    if interval_segments.is_empty() {
        "-".to_string()
    } else {
        interval_segments.join(";")
    }
}

pub(crate) fn format_table_candidate_column_grid_shape(candidate: &TableCandidate) -> String {
    candidate
        .column_segment_grid_candidate()
        .map(|grid| format!("{}x{}", grid.row_count(), grid.column_count()))
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_table_candidate_column_grid_pattern(candidate: &TableCandidate) -> String {
    candidate
        .column_segment_grid_candidate()
        .map(|grid| {
            grid.pattern()
                .iter()
                .map(|kind| kind.as_str())
                .collect::<Vec<_>>()
                .join("|")
        })
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_table_candidate_interval_contexts(
    candidate: &TableCandidate,
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    basis: RangeBasis,
) -> String {
    if candidate.intervals().is_empty() {
        return "-".to_string();
    }

    candidate
        .intervals()
        .iter()
        .map(|interval| {
            let text = range_visible_text(
                entries,
                interval.source_start(),
                interval.source_end(),
                basis,
            );
            format!(
                "{}:source-interval={},source={}-{},chars={},line-breaks={},text={},edges={}",
                interval.index(),
                interval.source_interval_index(),
                interval.source_start(),
                interval.source_end(),
                text.chars().count(),
                text_line_break_count(&text),
                escaped_text_preview(&text, 80),
                format_candidate_range_boundaries(
                    entries,
                    interval.source_start(),
                    interval.source_end(),
                    basis
                )
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

pub(crate) fn range_basis_from_candidate(basis: &str) -> RangeBasis {
    match basis {
        "byte" => RangeBasis::Byte,
        "unit" => RangeBasis::Unit,
        _ => unreachable!("unexpected text boundary candidate basis"),
    }
}

pub(crate) fn format_candidate_range_preview(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    match basis {
        RangeBasis::Byte => format_byte_range_preview(entries, start, end),
        RangeBasis::Unit => format_unit_range_preview(entries, start, end),
    }
}

pub(crate) fn format_candidate_range_boundaries(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    match basis {
        RangeBasis::Byte => format_byte_range_boundaries(entries, start, end),
        RangeBasis::Unit => format_unit_range_boundaries(entries, start, end),
    }
}

pub(crate) fn range_line_break_count(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> usize {
    text_line_break_count(&range_visible_text(entries, start, end, basis))
}

pub(crate) fn text_line_break_count(text: &str) -> usize {
    text.chars()
        .filter(|character| matches!(character, '\n' | '\r'))
        .count()
}

pub(crate) fn range_visible_text(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    entries
        .iter()
        .filter(|entry| range_overlaps_entry(entry, start, end, basis))
        .map(|entry| range_text_overlap(entry, start, end, basis))
        .collect()
}

pub(crate) fn is_boundary_candidate_edge_good(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> bool {
    range_starts_after_control_gap(entries, start, basis)
        && range_ends_on_aligned_text(entries, end, basis)
}

pub(crate) fn is_strict_unit_paragraph_candidate(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> bool {
    let text = range_visible_text(entries, start, end, RangeBasis::Unit);
    is_boundary_candidate_edge_good(entries, start, end, RangeBasis::Unit)
        && !text.is_empty()
        && text_line_break_count(&text) <= 1
}

pub(crate) fn collect_unit_001c_single_layout_candidates(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    candidates: &[TextBoundaryCandidate],
) -> Vec<LayoutBoundaryCandidate> {
    candidates
        .iter()
        .filter(|candidate| {
            candidate.basis().as_str() == "unit"
                && candidate.delimiter_code() == 0x001c
                && candidate.interval_count() == 1
        })
        .map(|candidate| {
            let selected = is_strict_unit_paragraph_candidate(
                entries,
                candidate.source_start(),
                candidate.source_end(),
            );
            LayoutBoundaryCandidate::new(
                candidate.index(),
                candidate.text_count_range_index(),
                candidate.source_start(),
                candidate.source_end(),
                selected,
            )
        })
        .collect()
}

#[derive(Clone, Copy)]
pub(crate) struct LayoutBoundaryCandidate {
    pub(crate) index: usize,
    pub(crate) text_count_range_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) selected: bool,
}

impl LayoutBoundaryCandidate {
    pub(crate) fn new(
        index: usize,
        text_count_range_index: usize,
        source_start: usize,
        source_end: usize,
        selected: bool,
    ) -> Self {
        Self {
            index,
            text_count_range_index,
            source_start,
            source_end,
            selected,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum LayoutMapBase {
    Unit,
    UnitTimes2,
    UnitDiv2Floor,
    UnitDiv2Ceil,
}

impl LayoutMapBase {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Unit => "unit",
            Self::UnitTimes2 => "unit-times-2",
            Self::UnitDiv2Floor => "unit-div2-floor",
            Self::UnitDiv2Ceil => "unit-div2-ceil",
        }
    }

    pub(crate) fn apply(self, value: usize) -> i64 {
        match self {
            Self::Unit => value as i64,
            Self::UnitTimes2 => (value as i64) * 2,
            Self::UnitDiv2Floor => (value / 2) as i64,
            Self::UnitDiv2Ceil => value.div_ceil(2) as i64,
        }
    }
}

#[derive(Clone)]
pub(crate) struct LayoutMapTargetSet {
    pub(crate) name: &'static str,
    pub(crate) points: Vec<usize>,
}

impl LayoutMapTargetSet {
    pub(crate) fn new(name: &'static str, points: impl IntoIterator<Item = usize>) -> Self {
        Self {
            name,
            points: sorted_unique_usize(points),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct LayoutMapScore {
    pub(crate) candidates: usize,
    pub(crate) endpoints: usize,
    pub(crate) valid_endpoints: usize,
    pub(crate) exact_hits: usize,
    pub(crate) invalid_endpoints: usize,
    pub(crate) total_distance: Option<usize>,
    pub(crate) max_distance: Option<usize>,
}

#[derive(Clone)]
pub(crate) struct LayoutExactEvidence {
    pub(crate) target: &'static str,
    pub(crate) base: LayoutMapBase,
    pub(crate) delta: isize,
    pub(crate) start: String,
    pub(crate) end: String,
}

pub(crate) struct LayoutParagraphLikeEvidence {
    pub(crate) paragraph_like: bool,
    pub(crate) line_word_evidence: Option<LayoutExactEvidence>,
    pub(crate) page_field_evidence: Option<LayoutExactEvidence>,
}

#[derive(Default)]
pub(crate) struct ParagraphLikeBucketSummary {
    pub(crate) rows: usize,
    pub(crate) strict_selected: usize,
    pub(crate) line_word_exact2: usize,
    pub(crate) page_field_exact2: usize,
    pub(crate) dual_exact2: usize,
    pub(crate) text_style_hits: usize,
    pub(crate) page_style_hits: usize,
    pub(crate) view_style_group_hits: usize,
    pub(crate) missing_tcnt: usize,
    pub(crate) source_span_min: Option<usize>,
    pub(crate) source_span_max: Option<usize>,
    pub(crate) range_span_min: Option<usize>,
    pub(crate) range_span_max: Option<usize>,
    pub(crate) family_counts: BTreeMap<String, usize>,
    pub(crate) f0_counts: BTreeMap<u16, usize>,
    pub(crate) f4_counts: BTreeMap<u16, usize>,
    pub(crate) f7_counts: BTreeMap<u16, usize>,
    pub(crate) line_evidence_counts: BTreeMap<String, usize>,
    pub(crate) page_evidence_counts: BTreeMap<String, usize>,
}

impl ParagraphLikeBucketSummary {
    pub(crate) fn observe(
        &mut self,
        candidate: &LayoutBoundaryCandidate,
        evidence: &LayoutParagraphLikeEvidence,
        range: Option<&TextCountRange>,
        text_style_candidates: &[CliStyleCandidate],
        page_style_candidates: &[CliStyleCandidate],
        view_style_groups: &[DocumentViewStyleGroup],
    ) {
        self.rows += 1;
        if candidate.selected {
            self.strict_selected += 1;
        }
        if evidence.line_word_evidence.is_some() {
            self.line_word_exact2 += 1;
        }
        if evidence.page_field_evidence.is_some() {
            self.page_field_exact2 += 1;
        }
        if evidence.line_word_evidence.is_some() && evidence.page_field_evidence.is_some() {
            self.dual_exact2 += 1;
        }
        update_min_max(
            &mut self.source_span_min,
            &mut self.source_span_max,
            candidate.source_end.saturating_sub(candidate.source_start),
        );
        if let Some(evidence) = evidence.line_word_evidence.as_ref() {
            *self
                .line_evidence_counts
                .entry(format_layout_evidence_signature(evidence))
                .or_insert(0) += 1;
        }
        if let Some(evidence) = evidence.page_field_evidence.as_ref() {
            *self
                .page_evidence_counts
                .entry(format_layout_evidence_signature(evidence))
                .or_insert(0) += 1;
        }

        let Some(range) = range else {
            self.missing_tcnt += 1;
            return;
        };
        *self
            .family_counts
            .entry(range.family().to_string())
            .or_insert(0) += 1;
        update_min_max(
            &mut self.range_span_min,
            &mut self.range_span_max,
            range.span() as usize,
        );
        let tail_fields = range.tail_fields();
        count_tail_field(&mut self.f0_counts, tail_fields, 0);
        count_tail_field(&mut self.f4_counts, tail_fields, 4);
        count_tail_field(&mut self.f7_counts, tail_fields, 7);
        if has_style_hit(tail_fields, text_style_candidates) {
            self.text_style_hits += 1;
        }
        if has_style_hit(tail_fields, page_style_candidates) {
            self.page_style_hits += 1;
        }
        if has_view_style_group_hit(tail_fields, view_style_groups) {
            self.view_style_group_hits += 1;
        }
    }

    pub(crate) fn format_fields(&self) -> String {
        format!(
            "rows={}\tstrict-selected={}\tline-word-exact2={}\tpage-field-exact2={}\tdual-exact2={}\ttext-style-hit={}\tpage-style-hit={}\tview-style-group-hit={}\tmissing-tcnt={}\tsource-spans={}\trange-spans={}\tfamilies={}\tf0={}\tf4={}\tf7={}\tline-evidence={}\tpage-evidence={}\tdecoded=false",
            self.rows,
            self.strict_selected,
            self.line_word_exact2,
            self.page_field_exact2,
            self.dual_exact2,
            self.text_style_hits,
            self.page_style_hits,
            self.view_style_group_hits,
            self.missing_tcnt,
            format_min_max(self.source_span_min, self.source_span_max),
            format_min_max(self.range_span_min, self.range_span_max),
            format_string_counts(&self.family_counts),
            format_u16_value_counts(&self.f0_counts),
            format_u16_value_counts(&self.f4_counts),
            format_u16_value_counts(&self.f7_counts),
            format_string_counts(&self.line_evidence_counts),
            format_string_counts(&self.page_evidence_counts),
        )
    }
}

pub(crate) fn layout_map_bases() -> &'static [LayoutMapBase] {
    &[
        LayoutMapBase::Unit,
        LayoutMapBase::UnitTimes2,
        LayoutMapBase::UnitDiv2Floor,
        LayoutMapBase::UnitDiv2Ceil,
    ]
}

pub(crate) fn layout_map_target_sets(
    line_words: Option<&[u16]>,
    page_mark: Option<&PageMark>,
    paper_mark: Option<&PaperMark>,
) -> Vec<LayoutMapTargetSet> {
    vec![
        LayoutMapTargetSet::new(
            "line-tag-index",
            line_words
                .into_iter()
                .flat_map(|words| {
                    words
                        .iter()
                        .enumerate()
                        .filter(|(_, word)| is_line_mark_tag(**word))
                        .map(|(index, _)| index)
                })
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "line-tag-byte",
            line_words
                .into_iter()
                .flat_map(|words| {
                    words
                        .iter()
                        .enumerate()
                        .filter(|(_, word)| is_line_mark_tag(**word))
                        .map(|(index, _)| index * 2)
                })
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "line-word-value",
            line_words
                .into_iter()
                .flat_map(|words| words.iter().map(|word| *word as usize))
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "page-entry-index",
            page_mark
                .into_iter()
                .flat_map(|mark| mark.entries().iter().filter_map(|entry| entry.index()))
                .map(|value| value as usize)
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "page-entry-byte-boundary",
            page_mark
                .map(page_mark_entry_byte_boundaries)
                .unwrap_or_default(),
        ),
        LayoutMapTargetSet::new(
            "page-be32-field",
            page_mark
                .into_iter()
                .flat_map(|mark| {
                    mark.entries().iter().flat_map(|entry| {
                        entry.raw().chunks_exact(4).map(|chunk| {
                            u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as usize
                        })
                    })
                })
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "paper-entry-index",
            paper_mark
                .into_iter()
                .flat_map(|mark| mark.entries().iter().map(|entry| entry.index() as usize))
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "paper-entry-byte-boundary",
            paper_mark
                .map(paper_mark_entry_byte_boundaries)
                .unwrap_or_default(),
        ),
    ]
}

pub(crate) fn page_mark_entry_byte_boundaries(page_mark: &PageMark) -> Vec<usize> {
    let mut offset = 12usize;
    let mut points = vec![offset];
    for entry in page_mark.entries() {
        offset += entry.raw().len();
        points.push(offset);
    }
    points
}

pub(crate) fn paper_mark_entry_byte_boundaries(paper_mark: &PaperMark) -> Vec<usize> {
    let mut offset = 12usize;
    let mut points = vec![offset];
    for _ in paper_mark.entries() {
        offset += 8;
        points.push(offset);
    }
    points
}

pub(crate) fn sorted_unique_usize(points: impl IntoIterator<Item = usize>) -> Vec<usize> {
    let mut points = points.into_iter().collect::<Vec<_>>();
    points.sort_unstable();
    points.dedup();
    points
}

pub(crate) fn write_layout_map_best_rows(
    scope: &str,
    candidates: &[LayoutBoundaryCandidate],
    target_sets: &[LayoutMapTargetSet],
) -> Result<(), String> {
    for target_set in target_sets {
        for base in layout_map_bases() {
            let (delta, score) = best_layout_map_delta(candidates, target_set, *base);
            write_stdout_line(&format!(
                "best\tscope={}\ttarget={}\tbase={}\tdelta={}\tdelta-at-boundary={}\tpoints={}\tcandidates={}\tendpoints={}\tvalid={}\tinvalid={}\texact={}\ttotal-distance={}\tmax-distance={}\tdecoded=false",
                scope,
                target_set.name,
                base.name(),
                delta,
                delta == LAYOUT_MAP_DELTA_MIN || delta == LAYOUT_MAP_DELTA_MAX,
                target_set.points.len(),
                score.candidates,
                score.endpoints,
                score.valid_endpoints,
                score.invalid_endpoints,
                score.exact_hits,
                format_optional_usize(score.total_distance),
                format_optional_usize(score.max_distance),
            ))?;
        }
    }
    Ok(())
}

pub(crate) fn best_layout_exact2_evidence(
    candidate: &LayoutBoundaryCandidate,
    target_sets: &[LayoutMapTargetSet],
    target_name: &'static str,
) -> Option<LayoutExactEvidence> {
    let target_set = target_sets
        .iter()
        .find(|target_set| target_set.name == target_name)?;
    let mut best: Option<LayoutExactEvidence> = None;
    let single = [*candidate];
    for base in layout_map_bases() {
        let (delta, score) = best_layout_map_delta(&single, target_set, *base);
        let at_boundary = delta == LAYOUT_MAP_DELTA_MIN || delta == LAYOUT_MAP_DELTA_MAX;
        if at_boundary || score.exact_hits != 2 || score.total_distance != Some(0) {
            continue;
        }
        let evidence = LayoutExactEvidence {
            target: target_set.name,
            base: *base,
            delta,
            start: format_layout_map_endpoint(candidate.source_start, target_set, *base, delta),
            end: format_layout_map_endpoint(candidate.source_end, target_set, *base, delta),
        };
        let replace = best.as_ref().is_none_or(|best| {
            delta.unsigned_abs() < best.delta.unsigned_abs()
                || (delta.unsigned_abs() == best.delta.unsigned_abs()
                    && base.name() < best.base.name())
        });
        if replace {
            best = Some(evidence);
        }
    }
    best
}

pub(crate) fn layout_paragraph_like_evidence(
    candidate: &LayoutBoundaryCandidate,
    target_sets: &[LayoutMapTargetSet],
) -> LayoutParagraphLikeEvidence {
    let line_word_evidence = best_layout_exact2_evidence(candidate, target_sets, "line-word-value");
    let page_field_evidence =
        best_layout_exact2_evidence(candidate, target_sets, "page-be32-field");
    LayoutParagraphLikeEvidence {
        paragraph_like: candidate.selected
            && line_word_evidence.is_some()
            && page_field_evidence.is_some(),
        line_word_evidence,
        page_field_evidence,
    }
}

pub(crate) fn format_layout_evidence_signature(evidence: &LayoutExactEvidence) -> String {
    format!(
        "{}/{}/{}",
        evidence.target,
        evidence.base.name(),
        evidence.delta
    )
}

pub(crate) fn format_layout_exact_evidence(evidence: Option<&LayoutExactEvidence>) -> String {
    let Some(evidence) = evidence else {
        return "-".to_string();
    };
    format!(
        "{}:{}:{}:{}|{}",
        evidence.target,
        evidence.base.name(),
        evidence.delta,
        evidence.start,
        evidence.end
    )
}

pub(crate) fn format_model_layout_exact_evidence(evidence: &TextLayoutExactEvidence) -> String {
    format!(
        "{}:{}:{}",
        evidence.target(),
        evidence.base(),
        evidence.delta()
    )
}

pub(crate) fn layout_evidence_value(
    offset: usize,
    evidence: &TextLayoutExactEvidence,
) -> Option<usize> {
    let base = match evidence.base() {
        "unit" => offset as i64,
        "unit-times-2" => (offset as i64) * 2,
        "unit-div2-floor" => (offset / 2) as i64,
        "unit-div2-ceil" => offset.div_ceil(2) as i64,
        _ => return None,
    };
    let value = base + evidence.delta() as i64;
    usize::try_from(value).ok()
}

pub(crate) fn format_line_word_value_refs(
    line_words: Option<&[u16]>,
    value: Option<usize>,
) -> String {
    let Some(value) = value else {
        return "value=invalid,hits=0,refs=-".to_string();
    };
    let Some(line_words) = line_words else {
        return format!("value={value},hits=-,refs=missing");
    };
    let hits = line_words
        .iter()
        .enumerate()
        .filter(|(_, word)| **word as usize == value)
        .map(|(index, word)| format!("word{index}:0x{word:04x}"))
        .collect::<Vec<_>>();
    format_limited_hit_refs(value, &hits)
}

pub(crate) fn format_page_be32_field_value_refs(
    page_mark: Option<&PageMark>,
    value: Option<usize>,
) -> String {
    let Some(value) = value else {
        return "value=invalid,hits=0,refs=-".to_string();
    };
    let Some(page_mark) = page_mark else {
        return format!("value={value},hits=-,refs=missing");
    };
    let hits = page_mark
        .entries()
        .iter()
        .enumerate()
        .flat_map(|(row_index, entry)| {
            entry
                .raw()
                .chunks_exact(4)
                .enumerate()
                .filter_map(move |(field_index, chunk)| {
                    let field = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    (field as usize == value)
                        .then(|| format!("row{row_index}:f{field_index}:0x{field:08x}"))
                })
        })
        .collect::<Vec<_>>();
    format_limited_hit_refs(value, &hits)
}

pub(crate) fn format_limited_hit_refs(value: usize, hits: &[String]) -> String {
    if hits.is_empty() {
        return format!("value={value},hits=0,refs=-");
    }
    let mut refs = hits.iter().take(8).cloned().collect::<Vec<_>>();
    if hits.len() > refs.len() {
        refs.push(format!("+{}more", hits.len() - refs.len()));
    }
    format!("value={value},hits={},refs={}", hits.len(), refs.join(","))
}

pub(crate) fn format_layout_map_endpoint(
    offset: usize,
    target_set: &LayoutMapTargetSet,
    base: LayoutMapBase,
    delta: isize,
) -> String {
    let value = base.apply(offset) + delta as i64;
    if value < 0 {
        return format!("{}:{}->invalid", offset, value);
    }
    if target_set.points.is_empty() {
        return format!("{}:{}->missing", offset, value);
    }
    let value = value as usize;
    let (point, distance) = nearest_usize_point(&target_set.points, value);
    format!("{offset}:{value}->{point}:d={distance}")
}

pub(crate) fn format_text_count_range_summary(range: Option<&TextCountRange>) -> String {
    let Some(range) = range else {
        return "-".to_string();
    };
    format!(
        "index={},family={},start={},end={},span={},declared-start={},declared-end={},tail={}",
        range.index(),
        range.family(),
        range.start(),
        range.end(),
        range.span(),
        range.declared_start(),
        range.declared_end(),
        format_u16_values(range.tail_fields()),
    )
}

pub(crate) fn format_u16_values(values: &[u16]) -> String {
    if values.is_empty() {
        "-".to_string()
    } else {
        values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

pub(crate) fn best_layout_map_delta(
    candidates: &[LayoutBoundaryCandidate],
    target_set: &LayoutMapTargetSet,
    base: LayoutMapBase,
) -> (isize, LayoutMapScore) {
    let mut best_delta = 0isize;
    let mut best_score = score_layout_map_delta(candidates, target_set, base, best_delta);
    if candidates.is_empty() || target_set.points.is_empty() {
        return (best_delta, best_score);
    }

    for delta in LAYOUT_MAP_DELTA_MIN..=LAYOUT_MAP_DELTA_MAX {
        if delta == 0 {
            continue;
        }
        let score = score_layout_map_delta(candidates, target_set, base, delta);
        if is_better_layout_map_score(score, delta, best_score, best_delta) {
            best_delta = delta;
            best_score = score;
        }
    }

    (best_delta, best_score)
}

pub(crate) fn score_layout_map_delta(
    candidates: &[LayoutBoundaryCandidate],
    target_set: &LayoutMapTargetSet,
    base: LayoutMapBase,
    delta: isize,
) -> LayoutMapScore {
    let mut score = LayoutMapScore {
        candidates: candidates.len(),
        endpoints: candidates.len() * 2,
        valid_endpoints: 0,
        exact_hits: 0,
        invalid_endpoints: 0,
        total_distance: None,
        max_distance: None,
    };
    if target_set.points.is_empty() {
        score.invalid_endpoints = score.endpoints;
        return score;
    }

    let mut total_distance = 0usize;
    let mut max_distance = 0usize;
    for candidate in candidates {
        for offset in [candidate.source_start, candidate.source_end] {
            let value = base.apply(offset) + delta as i64;
            if value < 0 {
                score.invalid_endpoints += 1;
                continue;
            }
            score.valid_endpoints += 1;
            let distance = nearest_usize_distance(&target_set.points, value as usize);
            if distance == 0 {
                score.exact_hits += 1;
            }
            total_distance += distance;
            max_distance = max_distance.max(distance);
        }
    }
    if score.valid_endpoints > 0 {
        score.total_distance = Some(total_distance);
        score.max_distance = Some(max_distance);
    }
    score
}

pub(crate) fn nearest_usize_distance(points: &[usize], value: usize) -> usize {
    nearest_usize_point(points, value).1
}

pub(crate) fn nearest_usize_point(points: &[usize], value: usize) -> (usize, usize) {
    match points.binary_search(&value) {
        Ok(index) => (points[index], 0),
        Err(index) => {
            let mut best = (0usize, usize::MAX);
            if let Some(point) = points.get(index) {
                best = (*point, point.abs_diff(value));
            }
            if index > 0 {
                let point = points[index - 1];
                let distance = point.abs_diff(value);
                if distance < best.1 {
                    best = (point, distance);
                }
            }
            best
        }
    }
}

pub(crate) fn is_better_layout_map_score(
    candidate: LayoutMapScore,
    candidate_delta: isize,
    best: LayoutMapScore,
    best_delta: isize,
) -> bool {
    candidate.exact_hits > best.exact_hits
        || (candidate.exact_hits == best.exact_hits
            && (candidate.invalid_endpoints < best.invalid_endpoints
                || (candidate.invalid_endpoints == best.invalid_endpoints
                    && (is_better_optional_distance(
                        candidate.total_distance,
                        best.total_distance,
                    ) || (candidate.total_distance == best.total_distance
                        && (is_better_optional_distance(
                            candidate.max_distance,
                            best.max_distance,
                        ) || (candidate.max_distance == best.max_distance
                            && candidate_delta.unsigned_abs() < best_delta.unsigned_abs())))))))
}

pub(crate) fn is_better_optional_distance(candidate: Option<usize>, best: Option<usize>) -> bool {
    match (candidate, best) {
        (Some(candidate), Some(best)) => candidate < best,
        (Some(_), None) => true,
        _ => false,
    }
}

pub(crate) fn range_starts_after_control_gap(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> bool {
    let touches_entry = entries.iter().any(|entry| {
        let (entry_start, entry_end) = entry_range(entry, basis);
        entry_start == offset || (entry_start < offset && offset < entry_end)
    });
    !touches_entry
        && previous_range_entry(entries, offset, basis)
            .is_some_and(|entry| entry.kind().as_str() == "control")
}

pub(crate) fn range_ends_on_aligned_text(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> bool {
    entries.iter().any(|entry| {
        let (_, entry_end) = entry_range(entry, basis);
        entry_end == offset && entry.kind().as_str() == "text"
    })
}

pub(crate) fn range_text_overlap(
    entry: &rjtd_core::document_text::DocumentTextMapEntry,
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    if entry.kind().as_str() == "control" || start >= end {
        return String::new();
    }

    let (entry_start, entry_end) = entry_range(entry, basis);
    let overlap_start = entry_start.max(start);
    let overlap_end = entry_end.min(end);
    if overlap_start >= overlap_end {
        return String::new();
    }

    let (relative_start, relative_end) = match basis {
        RangeBasis::Byte => (
            overlap_start.saturating_sub(entry.byte_start()) / 2,
            overlap_end
                .saturating_sub(entry.byte_start())
                .saturating_add(1)
                / 2,
        ),
        RangeBasis::Unit => (
            overlap_start.saturating_sub(entry.unit_start()),
            overlap_end.saturating_sub(entry.unit_start()),
        ),
    };
    text_by_utf16_units(entry.text(), relative_start, relative_end)
}

pub(crate) fn text_by_utf16_units(text: &str, start: usize, end: usize) -> String {
    let mut output = String::new();
    let mut current = 0usize;
    for character in text.chars() {
        let next = current + character.len_utf16();
        if next > start && current < end {
            output.push(character);
        }
        current = next;
    }
    output
}

#[derive(Clone, Copy)]
pub(crate) enum RangeBasis {
    Byte,
    Unit,
}

pub(crate) struct ControlDelimitedRange {
    pub(crate) index: usize,
    pub(crate) previous_delimiter: Option<usize>,
    pub(crate) next_delimiter: Option<usize>,
    pub(crate) entry_start: usize,
    pub(crate) entry_end: usize,
    pub(crate) byte_start: usize,
    pub(crate) byte_end: usize,
    pub(crate) unit_start: usize,
    pub(crate) unit_end: usize,
}

pub(crate) fn build_control_delimited_ranges(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    filter: Option<u16>,
) -> Vec<ControlDelimitedRange> {
    let delimiters = entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| entry.kind().as_str() == "control")
        .filter(|(_, entry)| filter.is_none_or(|code| entry.code() == Some(code)))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    (0..=delimiters.len())
        .map(|range_index| {
            let previous_delimiter = range_index
                .checked_sub(1)
                .and_then(|index| delimiters.get(index).copied());
            let next_delimiter = delimiters.get(range_index).copied();
            let entry_start = previous_delimiter.map_or(0, |index| index + 1);
            let entry_end = next_delimiter.unwrap_or(entries.len());
            let range_entries = entries.get(entry_start..entry_end).unwrap_or(&[]);
            let byte_start = previous_delimiter
                .and_then(|index| entries.get(index))
                .map(|entry| entry.byte_end())
                .or_else(|| range_entries.first().map(|entry| entry.byte_start()))
                .or_else(|| {
                    next_delimiter
                        .and_then(|index| entries.get(index))
                        .map(|entry| entry.byte_start())
                })
                .unwrap_or(0);
            let byte_end = next_delimiter
                .and_then(|index| entries.get(index))
                .map(|entry| entry.byte_start())
                .or_else(|| range_entries.last().map(|entry| entry.byte_end()))
                .unwrap_or(byte_start);
            let unit_start = previous_delimiter
                .and_then(|index| entries.get(index))
                .map(|entry| entry.unit_end())
                .or_else(|| range_entries.first().map(|entry| entry.unit_start()))
                .or_else(|| {
                    next_delimiter
                        .and_then(|index| entries.get(index))
                        .map(|entry| entry.unit_start())
                })
                .unwrap_or(0);
            let unit_end = next_delimiter
                .and_then(|index| entries.get(index))
                .map(|entry| entry.unit_start())
                .or_else(|| range_entries.last().map(|entry| entry.unit_end()))
                .unwrap_or(unit_start);

            ControlDelimitedRange {
                index: range_index,
                previous_delimiter,
                next_delimiter,
                entry_start,
                entry_end,
                byte_start,
                byte_end,
                unit_start,
                unit_end,
            }
        })
        .collect()
}

pub(crate) fn format_control_range_hits(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    ranges: &[ControlDelimitedRange],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    let hits = ranges
        .iter()
        .filter(|range| control_range_overlaps(range, start, end, basis))
        .collect::<Vec<_>>();
    let Some(first) = hits.first() else {
        return "count=0,first=-,last=-,byte=-,unit=-,entry-ranges=-,controls=-,preview=-"
            .to_string();
    };
    let first = *first;
    let last = hits.last().copied().unwrap_or(first);
    let controls = format_range_control_counts(
        hits.iter()
            .flat_map(|range| entries[range.entry_start..range.entry_end].iter()),
    );
    let mut preview = String::new();
    for range in &hits {
        for entry in &entries[range.entry_start..range.entry_end] {
            if entry.kind().as_str() != "control" {
                preview.push_str(entry.text());
            }
        }
    }
    let preview = if preview.is_empty() {
        "-".to_string()
    } else {
        escaped_text_preview(&preview, 80)
    };

    format!(
        "count={},first={},last={},byte={}-{},unit={}-{},entry-ranges={},controls={},preview={}",
        hits.len(),
        first.index,
        last.index,
        first.byte_start,
        last.byte_end,
        first.unit_start,
        last.unit_end,
        format_control_range_hit_entry_spans(&hits),
        controls,
        preview
    )
}

pub(crate) fn control_range_overlaps(
    range: &ControlDelimitedRange,
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> bool {
    let (range_start, range_end) = control_range_basis_span(range, basis);
    if start == end {
        return range_start <= start && start <= range_end;
    }

    start < range_end && end > range_start
}

pub(crate) fn control_range_basis_span(
    range: &ControlDelimitedRange,
    basis: RangeBasis,
) -> (usize, usize) {
    match basis {
        RangeBasis::Byte => (range.byte_start, range.byte_end),
        RangeBasis::Unit => (range.unit_start, range.unit_end),
    }
}

pub(crate) fn format_control_range_hit_entry_spans(hits: &[&ControlDelimitedRange]) -> String {
    let spans = hits
        .iter()
        .map(|range| format_entry_index_span(range.entry_start, range.entry_end))
        .filter(|span| span != "-")
        .collect::<Vec<_>>();

    if spans.is_empty() {
        "-".to_string()
    } else {
        spans.join("+")
    }
}

pub(crate) fn format_byte_range_boundaries(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> String {
    format_range_boundaries(entries, start, end, RangeBasis::Byte)
}

pub(crate) fn format_unit_range_boundaries(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> String {
    format_range_boundaries(entries, start, end, RangeBasis::Unit)
}

pub(crate) fn format_range_boundaries(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    let overlapping = entries
        .iter()
        .filter(|entry| range_overlaps_entry(entry, start, end, basis))
        .collect::<Vec<_>>();
    let full_count = overlapping
        .iter()
        .filter(|entry| {
            let (entry_start, entry_end) = entry_range(entry, basis);
            start <= entry_start && entry_end <= end
        })
        .count();
    let controls = format_range_control_counts(overlapping.iter().copied());
    let first = overlapping
        .first()
        .map(|entry| summarize_map_entry(entry))
        .unwrap_or_else(|| "-".to_string());
    let last = overlapping
        .last()
        .map(|entry| summarize_map_entry(entry))
        .unwrap_or_else(|| "-".to_string());
    let previous = previous_range_entry(entries, start, basis)
        .map(summarize_map_entry)
        .unwrap_or_else(|| "-".to_string());
    let next = next_range_entry(entries, end, basis)
        .map(summarize_map_entry)
        .unwrap_or_else(|| "-".to_string());

    format!(
        "inside={},full={},partial={},start-edge={},end-edge={},first={},last={},prev={},next={},controls={}",
        overlapping.len(),
        full_count,
        overlapping.len().saturating_sub(full_count),
        format_range_start_edge(entries, start, basis),
        format_range_end_edge(entries, end, basis),
        first,
        last,
        previous,
        next,
        controls
    )
}

pub(crate) fn entry_range(
    entry: &rjtd_core::document_text::DocumentTextMapEntry,
    basis: RangeBasis,
) -> (usize, usize) {
    match basis {
        RangeBasis::Byte => (entry.byte_start(), entry.byte_end()),
        RangeBasis::Unit => (entry.unit_start(), entry.unit_end()),
    }
}

pub(crate) fn range_overlaps_entry(
    entry: &rjtd_core::document_text::DocumentTextMapEntry,
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> bool {
    if start >= end {
        return false;
    }
    let (entry_start, entry_end) = entry_range(entry, basis);
    entry_start < end && entry_end > start
}

pub(crate) fn previous_range_entry(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> Option<&rjtd_core::document_text::DocumentTextMapEntry> {
    entries
        .iter()
        .filter(|entry| entry_range(entry, basis).1 <= offset)
        .max_by_key(|entry| entry_range(entry, basis).1)
}

pub(crate) fn next_range_entry(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> Option<&rjtd_core::document_text::DocumentTextMapEntry> {
    entries
        .iter()
        .filter(|entry| entry_range(entry, basis).0 >= offset)
        .min_by_key(|entry| entry_range(entry, basis).0)
}

pub(crate) fn format_range_start_edge(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> String {
    if let Some(entry) = entries
        .iter()
        .find(|entry| entry_range(entry, basis).0 == offset)
    {
        return format!("aligned:{}", summarize_map_entry(entry));
    }

    if let Some(entry) = entries.iter().find(|entry| {
        let (entry_start, entry_end) = entry_range(entry, basis);
        entry_start < offset && offset < entry_end
    }) {
        return format!("inside:{}", summarize_map_entry(entry));
    }

    format!(
        "gap:{}|{}",
        previous_range_entry(entries, offset, basis)
            .map(summarize_map_entry)
            .unwrap_or_else(|| "-".to_string()),
        next_range_entry(entries, offset, basis)
            .map(summarize_map_entry)
            .unwrap_or_else(|| "-".to_string())
    )
}

pub(crate) fn format_range_end_edge(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> String {
    if let Some(entry) = entries
        .iter()
        .find(|entry| entry_range(entry, basis).1 == offset)
    {
        return format!("aligned:{}", summarize_map_entry(entry));
    }

    if let Some(entry) = entries.iter().find(|entry| {
        let (entry_start, entry_end) = entry_range(entry, basis);
        entry_start < offset && offset < entry_end
    }) {
        return format!("inside:{}", summarize_map_entry(entry));
    }

    format!(
        "gap:{}|{}",
        previous_range_entry(entries, offset, basis)
            .map(summarize_map_entry)
            .unwrap_or_else(|| "-".to_string()),
        next_range_entry(entries, offset, basis)
            .map(summarize_map_entry)
            .unwrap_or_else(|| "-".to_string())
    )
}

pub(crate) fn format_range_control_counts<'a>(
    entries: impl Iterator<Item = &'a rjtd_core::document_text::DocumentTextMapEntry>,
) -> String {
    let mut counts = BTreeMap::new();
    for entry in entries {
        if entry.kind().as_str() == "control"
            && let Some(code) = entry.code()
        {
            *counts.entry(code).or_insert(0usize) += 1;
        }
    }

    if counts.is_empty() {
        "-".to_string()
    } else {
        counts
            .into_iter()
            .map(|(code, count)| format!("0x{code:04x}:{count}"))
            .collect::<Vec<_>>()
            .join(",")
    }
}

pub(crate) fn format_mark_ids(ids: impl Iterator<Item = u16>) -> String {
    let values = ids.map(|id| id.to_string()).collect::<Vec<_>>();
    if values.is_empty() {
        "-".to_string()
    } else {
        values.join(",")
    }
}
