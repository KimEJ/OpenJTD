use crate::*;

/// The decoded page-size subrecord stores mm100 values shifted by eight bits.
pub(crate) const PAGE_LAYOUT_STYLE_MM100_SHIFT: u32 = 8;

pub(crate) const PAGE_LAYOUT_STYLE_MARGIN_MIN_MM100: u32 = 100;

pub(crate) const PAGE_LAYOUT_STYLE_MARGIN_MAX_MM100: u32 = 10_000;

pub(crate) const PAGE_LAYOUT_STYLE_MARGIN_QUAD_LEN: usize = 4;

/// Plausible `/PageMark` mm100 line-pitch range: 1mm to 20mm.
pub(crate) const PAGE_MARK_LINE_PITCH_MIN_MM100: u16 = 100;

pub(crate) const PAGE_MARK_LINE_PITCH_MAX_MM100: u16 = 2_000;

/// A pitch candidate must repeat inside the entry; a one-off word is not a grid
/// constant.
pub(crate) const PAGE_MARK_LINE_PITCH_MIN_REPEAT: usize = 3;

/// The geometry identity belongs to the first observed `/PageMark` page-record
/// block. Later words can belong to unrelated raw records and must not vote for
/// the page pitch.
pub(crate) const PAGE_MARK_LINE_PITCH_SEARCH_WORD_LIMIT: usize = 24;

/// One raw `/PageLayoutStyle` record-payload BE-u16 quad that fits the observed
/// `/PageMark` page-line gap count. The values and their field order remain
/// candidate-only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PageLayoutStyleMarginQuadCandidate {
    pub(crate) record_payload_offsets: Vec<usize>,
    pub(crate) values_mm100: Vec<u32>,
    pub(crate) best_page_fit_remainder_mm100: u32,
}

/// `/PageLayoutStyle` page-size subrecord fields read generically. Paper size is
/// already decoded elsewhere; the margin quad is candidate-only and its field
/// order is not proven.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PageLayoutStylePageGridFields {
    pub(crate) paper_width_mm100: u32,
    pub(crate) paper_height_mm100: u32,
    pub(crate) page_layout_style_record_offset: usize,
    pub(crate) margin_quad_mm100: Vec<u32>,
    pub(crate) margin_quad_payload_offsets: Vec<usize>,
    pub(crate) margin_quad_candidate_count: usize,
    pub(crate) margin_quad_selection_unique: bool,
    pub(crate) margin_quad_page_fit_remainder_mm100: Option<u32>,
}

/// `pitch == font + leading` identity found by searching the `/PageMark` entry's
/// u16 words. The pitch word is the repeated grid constant; the addend roles are
/// assigned by magnitude and are not proven.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PageMarkLinePitchIdentityCandidate {
    pub(crate) pitch_mm100: u16,
    pub(crate) pitch_word_indexes: Vec<usize>,
    pub(crate) font_mm100: u16,
    pub(crate) font_word_index: usize,
    pub(crate) leading_mm100: u16,
    pub(crate) leading_word_index: usize,
    pub(crate) addend_pair_count: usize,
}

/// One vertical reading of the unproven margin quad.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PageGridYAnchorVerticalPairing {
    pub(crate) label: &'static str,
    pub(crate) top_field_index: usize,
    pub(crate) bottom_field_index: usize,
    pub(crate) top_mm100: u32,
    pub(crate) bottom_mm100: u32,
    pub(crate) body_height_mm100: u32,
    pub(crate) body_top_px: f32,
    pub(crate) body_height_px: f32,
    pub(crate) line_capacity: u32,
    pub(crate) line_capacity_remainder_mm100: u32,
    pub(crate) page_line_row_tops_px: Vec<f32>,
    pub(crate) own_row_top_y_px: Vec<f32>,
}

impl PageGridYAnchorVerticalPairing {
    pub(crate) fn matches_page_record_line_count(&self, page_record_line_count: u32) -> bool {
        self.line_capacity == page_record_line_count
    }

    /// The page record line range is inclusive, so a capacity derived from whole
    /// pitch steps can legitimately land on the gap count instead. Which of the
    /// two readings is correct is not proven.
    pub(crate) fn matches_page_record_line_gap_count(&self, page_record_line_count: u32) -> bool {
        self.line_capacity == page_record_line_count.saturating_sub(1)
    }

    pub(crate) fn fits_page_record(&self, page_record_line_count: u32) -> bool {
        self.matches_page_record_line_count(page_record_line_count)
            || self.matches_page_record_line_gap_count(page_record_line_count)
    }
}

/// Source-only page grid y anchor candidates for one table candidate.
#[derive(Debug, Clone)]
pub(crate) struct PageGridYAnchor {
    pub(crate) page_grid_fields: PageLayoutStylePageGridFields,
    pub(crate) pitch_identity: Option<PageMarkLinePitchIdentityCandidate>,
    pub(crate) page_mark_entry_index: usize,
    pub(crate) page_line_start: u32,
    pub(crate) page_line_end: u32,
    pub(crate) own_row_line_mark_record_indexes: Vec<usize>,
    pub(crate) pairings: Vec<PageGridYAnchorVerticalPairing>,
    pub(crate) implied_vertical_margin_from_page_line_gap_count_mm100: Option<i64>,
}

impl PageGridYAnchor {
    pub(crate) fn page_record_line_count(&self) -> u32 {
        self.page_line_end.saturating_sub(self.page_line_start) + 1
    }

    pub(crate) fn matched_pairing_count(&self) -> usize {
        let line_count = self.page_record_line_count();
        self.pairings
            .iter()
            .filter(|pairing| pairing.fits_page_record(line_count))
            .count()
    }

    /// True when the only page fit is against the gap count, so the inclusive
    /// line-range reading is still off by one.
    pub(crate) fn only_gap_count_fit(&self) -> bool {
        let line_count = self.page_record_line_count();
        self.matched_pairing_count() > 0
            && !self
                .pairings
                .iter()
                .any(|pairing| pairing.matches_page_record_line_count(line_count))
    }

    pub(crate) fn blocked_reasons(&self) -> Vec<&'static str> {
        let mut reasons = Vec::new();
        if self.margin_quad_complete() {
            reasons.push("page-layout-style-margin-quad-field-order-unproven");
            reasons.push("page-layout-style-margin-quad-selected-by-page-line-capacity");
        } else {
            reasons.push("page-layout-style-margin-quad-incomplete");
        }
        match self.pitch_identity.as_ref() {
            Some(identity) => {
                reasons.push("page-mark-font-leading-addend-roles-unproven");
                if identity.addend_pair_count > 1 {
                    reasons.push("page-mark-line-pitch-addend-pair-not-unique");
                }
            }
            None => reasons.push("page-mark-line-pitch-identity-absent"),
        }
        match self.matched_pairing_count() {
            0 => reasons.push("page-grid-line-capacity-does-not-match-page-record-line-count"),
            1 => {}
            _ => reasons.push("page-grid-line-capacity-page-fit-ambiguous"),
        }
        if self.only_gap_count_fit() {
            reasons.push("page-grid-line-capacity-fits-page-record-line-gap-count-not-line-count");
        }
        reasons.push("line-mark-record-index-to-page-line-index-mapping-unproven");
        reasons.push("page-grid-y-anchor-is-not-a-decoded-page-space-origin");
        reasons
    }

    pub(crate) fn margin_quad_complete(&self) -> bool {
        self.page_grid_fields.margin_quad_mm100.len() == PAGE_LAYOUT_STYLE_MARGIN_QUAD_LEN
    }
}

/// Reads the `/PageLayoutStyle` page-size subrecord generically, then scans the
/// owning style record's raw payload for a BE-u16 margin quad candidate. A quad
/// is retained only when one of its two field pairings fits the source-only
/// `/PageMark` page-line gap count; the field order remains unproven.
pub(crate) fn page_layout_style_page_grid_fields(
    document: &Document,
    pitch_mm100: Option<u16>,
    page_line_gap_count: u32,
) -> Option<PageLayoutStylePageGridFields> {
    let payload = document
        .unknown_styles()
        .iter()
        .find(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
        .map(|style| style.payload())?;
    let summary = summarize_style_stream(payload);
    let record = summary
        .records()
        .iter()
        .filter(|record| record.code() == PAGE_LAYOUT_STYLE_RECORD_CODE)
        .find(|record| {
            record
                .subrecords()
                .iter()
                .any(|subrecord| subrecord.code() == PAGE_LAYOUT_STYLE_PAGE_SIZE_SUBRECORD_CODE)
        })?;
    let subrecord = record
        .subrecords()
        .iter()
        .find(|subrecord| subrecord.code() == PAGE_LAYOUT_STYLE_PAGE_SIZE_SUBRECORD_CODE)?;
    let bytes = subrecord.payload();
    let paper_width_mm100 = read_be32_at(bytes, PAGE_LAYOUT_STYLE_PAGE_SIZE_WIDTH_OFFSET)?
        >> PAGE_LAYOUT_STYLE_MM100_SHIFT;
    let paper_height_mm100 = read_be32_at(bytes, PAGE_LAYOUT_STYLE_PAGE_SIZE_HEIGHT_OFFSET)?
        >> PAGE_LAYOUT_STYLE_MM100_SHIFT;
    if !paper_size_mm100_is_plausible(paper_width_mm100)
        || !paper_size_mm100_is_plausible(paper_height_mm100)
    {
        return None;
    }

    let record_payload_start = record.offset().checked_add(4)?;
    let record_payload_end = record_payload_start.checked_add(record.payload_len())?;
    let record_payload = payload.get(record_payload_start..record_payload_end)?;
    let margin_candidates = pitch_mm100
        .filter(|pitch| *pitch > 0)
        .map(|pitch| {
            page_layout_style_margin_quad_candidates(
                record_payload,
                paper_width_mm100,
                paper_height_mm100,
                pitch,
                page_line_gap_count,
            )
        })
        .unwrap_or_default();
    let margin_quad_selection_unique = match (margin_candidates.first(), margin_candidates.get(1)) {
        (Some(_), None) => true,
        (Some(best), Some(runner_up)) => {
            best.best_page_fit_remainder_mm100 != runner_up.best_page_fit_remainder_mm100
        }
        (None, _) => false,
    };
    let selected = margin_quad_selection_unique
        .then(|| margin_candidates.first())
        .flatten();
    Some(PageLayoutStylePageGridFields {
        paper_width_mm100,
        paper_height_mm100,
        page_layout_style_record_offset: record.offset(),
        margin_quad_mm100: selected
            .map(|candidate| candidate.values_mm100.clone())
            .unwrap_or_default(),
        margin_quad_payload_offsets: selected
            .map(|candidate| candidate.record_payload_offsets.clone())
            .unwrap_or_default(),
        margin_quad_candidate_count: margin_candidates.len(),
        margin_quad_selection_unique,
        margin_quad_page_fit_remainder_mm100: selected
            .map(|candidate| candidate.best_page_fit_remainder_mm100),
    })
}

/// Finds raw BE-u16 margin quads whose first or second field pair can describe a
/// vertical body with the observed page-line gap count. Candidates are ordered
/// by the smallest pitch remainder and then by source offset. Pure so the raw
/// grammar and selection stay unit-testable.
pub(crate) fn page_layout_style_margin_quad_candidates(
    bytes: &[u8],
    paper_width_mm100: u32,
    paper_height_mm100: u32,
    pitch_mm100: u16,
    page_line_gap_count: u32,
) -> Vec<PageLayoutStyleMarginQuadCandidate> {
    let pitch_mm100 = u32::from(pitch_mm100);
    if pitch_mm100 == 0 || bytes.len() < PAGE_LAYOUT_STYLE_MARGIN_QUAD_LEN * 2 {
        return Vec::new();
    }

    let mut candidates = Vec::new();
    for offset in (0..=bytes.len() - PAGE_LAYOUT_STYLE_MARGIN_QUAD_LEN * 2).step_by(2) {
        let values = (0..PAGE_LAYOUT_STYLE_MARGIN_QUAD_LEN)
            .filter_map(|index| read_be16_at(bytes, offset + index * 2).map(u32::from))
            .collect::<Vec<_>>();
        if values.len() != PAGE_LAYOUT_STYLE_MARGIN_QUAD_LEN
            || values.iter().any(|value| {
                !(PAGE_LAYOUT_STYLE_MARGIN_MIN_MM100..=PAGE_LAYOUT_STYLE_MARGIN_MAX_MM100)
                    .contains(value)
                    || value % 100 != 0
            })
            || values[0].saturating_add(values[1]) >= paper_height_mm100
            || values[2].saturating_add(values[3]) >= paper_width_mm100
        {
            continue;
        }

        let best_page_fit_remainder_mm100 = [(0usize, 1usize), (2usize, 3usize)]
            .into_iter()
            .filter_map(|(top, bottom)| {
                let body_height = paper_height_mm100
                    .checked_sub(values[top])?
                    .checked_sub(values[bottom])?;
                (body_height / pitch_mm100 == page_line_gap_count)
                    .then_some(body_height % pitch_mm100)
            })
            .min();
        let Some(best_page_fit_remainder_mm100) = best_page_fit_remainder_mm100 else {
            continue;
        };
        candidates.push(PageLayoutStyleMarginQuadCandidate {
            record_payload_offsets: (0..PAGE_LAYOUT_STYLE_MARGIN_QUAD_LEN)
                .map(|index| offset + index * 2)
                .collect(),
            values_mm100: values,
            best_page_fit_remainder_mm100,
        });
    }
    candidates.sort_by_key(|candidate| {
        (
            candidate.best_page_fit_remainder_mm100,
            candidate.record_payload_offsets[0],
        )
    });
    candidates
}

/// Finds the repeated `/PageMark` grid pitch word and the single addend pair that
/// sums to it. Pure so the identity stays unit-testable without a sample.
pub(crate) fn page_mark_line_pitch_identity_candidate(
    words: &[u16],
) -> Option<PageMarkLinePitchIdentityCandidate> {
    let mut repeats: BTreeMap<u16, Vec<usize>> = BTreeMap::new();
    for (index, value) in words.iter().copied().enumerate() {
        if (PAGE_MARK_LINE_PITCH_MIN_MM100..=PAGE_MARK_LINE_PITCH_MAX_MM100).contains(&value) {
            repeats.entry(value).or_default().push(index);
        }
    }
    let (pitch_mm100, pitch_word_indexes) = repeats
        .into_iter()
        .filter(|(_, indexes)| indexes.len() >= PAGE_MARK_LINE_PITCH_MIN_REPEAT)
        .max_by_key(|(value, indexes)| (indexes.len(), *value))?;

    let mut pairs = Vec::new();
    for (left_index, left) in words.iter().copied().enumerate() {
        for (right_index, right) in words.iter().copied().enumerate().skip(left_index + 1) {
            if left == 0 || right == 0 || left.checked_add(right) != Some(pitch_mm100) {
                continue;
            }
            if pitch_word_indexes.contains(&left_index) || pitch_word_indexes.contains(&right_index)
            {
                continue;
            }
            pairs.push(if left >= right {
                (left, left_index, right, right_index)
            } else {
                (right, right_index, left, left_index)
            });
        }
    }
    let (font_mm100, font_word_index, leading_mm100, leading_word_index) =
        pairs.first().copied()?;
    Some(PageMarkLinePitchIdentityCandidate {
        pitch_mm100,
        pitch_word_indexes,
        font_mm100,
        font_word_index,
        leading_mm100,
        leading_word_index,
        addend_pair_count: pairs.len(),
    })
}

/// Vertical readings of the unproven margin quad: fields `0/1` and fields `2/3`.
pub(crate) fn page_grid_y_anchor_vertical_pairings(
    paper_height_mm100: u32,
    margin_quad_mm100: &[u32],
    pitch_mm100: Option<u16>,
    page_line_start: u32,
    own_row_line_mark_record_indexes: &[usize],
    page_line_sample_count: usize,
) -> Vec<PageGridYAnchorVerticalPairing> {
    const PAIRINGS: &[(&str, usize, usize)] = &[
        ("margin-quad-fields-0-1-as-top-bottom", 0, 1),
        ("margin-quad-fields-2-3-as-top-bottom", 2, 3),
    ];
    let mut pairings = Vec::new();
    for (label, top_field_index, bottom_field_index) in PAIRINGS.iter().copied() {
        let (Some(top_mm100), Some(bottom_mm100)) = (
            margin_quad_mm100.get(top_field_index).copied(),
            margin_quad_mm100.get(bottom_field_index).copied(),
        ) else {
            continue;
        };
        let body_height_mm100 = paper_height_mm100
            .saturating_sub(top_mm100)
            .saturating_sub(bottom_mm100);
        let pitch = pitch_mm100.map(u32::from).filter(|pitch| *pitch > 0);
        let line_capacity = pitch.map(|pitch| body_height_mm100 / pitch).unwrap_or(0);
        let line_capacity_remainder_mm100 =
            pitch.map(|pitch| body_height_mm100 % pitch).unwrap_or(0);
        let body_top_px = hundredth_millimeters_to_css_px(top_mm100);
        let pitch_px = pitch.map(hundredth_millimeters_to_css_px).unwrap_or(0.0);
        let page_line_row_tops_px = (0..page_line_sample_count)
            .map(|line| body_top_px + line as f32 * pitch_px)
            .collect();
        let own_row_top_y_px = own_row_line_mark_record_indexes
            .iter()
            .copied()
            .filter_map(|record_index| {
                let page_line_offset = u32::try_from(record_index)
                    .ok()?
                    .checked_sub(page_line_start)?;
                Some(body_top_px + page_line_offset as f32 * pitch_px)
            })
            .collect();
        pairings.push(PageGridYAnchorVerticalPairing {
            label,
            top_field_index,
            bottom_field_index,
            top_mm100,
            bottom_mm100,
            body_height_mm100,
            body_top_px,
            body_height_px: hundredth_millimeters_to_css_px(body_height_mm100),
            line_capacity,
            line_capacity_remainder_mm100,
            page_line_row_tops_px,
            own_row_top_y_px,
        });
    }
    pairings
}

/// Number of projected page-line row tops emitted per pairing. Enough to compare
/// against the first table rows without inflating the JSON.
pub(crate) const PAGE_GRID_Y_ANCHOR_PAGE_LINE_SAMPLE_COUNT: usize = 4;

pub(crate) fn table_grid_page_grid_y_anchor(
    document: &Document,
    candidate: &TableCandidate,
) -> Option<PageGridYAnchor> {
    let own_row_line_mark_record_indexes =
        table_grid_previous_row_span_line_mark_record_indexes(document, candidate);
    let context = table_grid_page_mark_context_for_line_mark_record_indexes(
        document,
        &own_row_line_mark_record_indexes,
    )?;
    let pitch_search_word_count = context
        .page_mark_u16_fields
        .len()
        .min(PAGE_MARK_LINE_PITCH_SEARCH_WORD_LIMIT);
    let pitch_identity = page_mark_line_pitch_identity_candidate(
        &context.page_mark_u16_fields[..pitch_search_word_count],
    );
    let page_line_start = context.page_line_start as u32;
    let page_line_end = context.page_line_end as u32;
    let page_line_gap_count = page_line_end.saturating_sub(page_line_start);
    let page_grid_fields = page_layout_style_page_grid_fields(
        document,
        pitch_identity.as_ref().map(|identity| identity.pitch_mm100),
        page_line_gap_count,
    )?;
    let pairings = page_grid_y_anchor_vertical_pairings(
        page_grid_fields.paper_height_mm100,
        &page_grid_fields.margin_quad_mm100,
        pitch_identity.as_ref().map(|identity| identity.pitch_mm100),
        page_line_start,
        &own_row_line_mark_record_indexes,
        PAGE_GRID_Y_ANCHOR_PAGE_LINE_SAMPLE_COUNT,
    );
    let implied_vertical_margin_from_page_line_gap_count_mm100 =
        pitch_identity.as_ref().map(|identity| {
            i64::from(page_grid_fields.paper_height_mm100)
                - i64::from(page_line_gap_count) * i64::from(identity.pitch_mm100)
        });

    Some(PageGridYAnchor {
        page_grid_fields,
        pitch_identity,
        page_mark_entry_index: context.page_mark_entry_index,
        page_line_start,
        page_line_end,
        own_row_line_mark_record_indexes,
        pairings,
        implied_vertical_margin_from_page_line_gap_count_mm100,
    })
}

pub(crate) fn push_table_grid_page_grid_y_anchor_gate_json(
    output: &mut String,
    anchor: Option<&PageGridYAnchor>,
) {
    let Some(anchor) = anchor else {
        output.push_str("null");
        return;
    };
    let fields = &anchor.page_grid_fields;
    let page_record_line_count = anchor.page_record_line_count();
    let blocked_reasons = anchor.blocked_reasons();

    output.push_str("{\"source\":\"/PageLayoutStyle+/PageMark+/LineMark source page grid fields\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"paperWidthMm100\":");
    output.push_str(&fields.paper_width_mm100.to_string());
    output.push_str(",\"paperHeightMm100\":");
    output.push_str(&fields.paper_height_mm100.to_string());
    output.push_str(",\"paperWidthPx\":");
    output.push_str(&format!(
        "{:.3}",
        hundredth_millimeters_to_css_px(fields.paper_width_mm100)
    ));
    output.push_str(",\"paperHeightPx\":");
    output.push_str(&format!(
        "{:.3}",
        hundredth_millimeters_to_css_px(fields.paper_height_mm100)
    ));
    output.push_str(",\"pageLayoutStyleRecordOffset\":");
    output.push_str(&fields.page_layout_style_record_offset.to_string());
    output.push_str(",\"marginQuadMm100\":");
    push_u32_array_json(output, &fields.margin_quad_mm100);
    output.push_str(",\"marginQuadPayloadOffsets\":");
    push_usize_array_json(output, &fields.margin_quad_payload_offsets);
    output.push_str(",\"marginQuadComplete\":");
    output.push_str(json_bool(anchor.margin_quad_complete()));
    output.push_str(",\"marginQuadCandidateCount\":");
    output.push_str(&fields.margin_quad_candidate_count.to_string());
    output.push_str(",\"marginQuadSelectionUnique\":");
    output.push_str(json_bool(fields.margin_quad_selection_unique));
    output.push_str(",\"marginQuadPageFitRemainderMm100\":");
    match fields.margin_quad_page_fit_remainder_mm100 {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"marginQuadFieldOrderProven\":false");
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&anchor.page_mark_entry_index.to_string());
    output.push_str(",\"pageLineStart\":");
    output.push_str(&anchor.page_line_start.to_string());
    output.push_str(",\"pageLineEnd\":");
    output.push_str(&anchor.page_line_end.to_string());
    output.push_str(",\"pageRecordLineCount\":");
    output.push_str(&page_record_line_count.to_string());
    output.push_str(",\"ownRowLineMarkRecordIndexes\":");
    push_usize_array_json(output, &anchor.own_row_line_mark_record_indexes);
    output.push_str(",\"linePitchIdentity\":");
    match anchor.pitch_identity.as_ref() {
        Some(identity) => {
            output.push_str("{\"identity\":\"pitchMm100 == fontMm100 + leadingMm100\"");
            output.push_str(",\"pitchMm100\":");
            output.push_str(&identity.pitch_mm100.to_string());
            output.push_str(",\"pitchWordIndexes\":");
            push_usize_array_json(output, &identity.pitch_word_indexes);
            output.push_str(",\"pitchSearchWordLimit\":");
            output.push_str(&PAGE_MARK_LINE_PITCH_SEARCH_WORD_LIMIT.to_string());
            output.push_str(",\"fontMm100\":");
            output.push_str(&identity.font_mm100.to_string());
            output.push_str(",\"fontWordIndex\":");
            output.push_str(&identity.font_word_index.to_string());
            output.push_str(",\"leadingMm100\":");
            output.push_str(&identity.leading_mm100.to_string());
            output.push_str(",\"leadingWordIndex\":");
            output.push_str(&identity.leading_word_index.to_string());
            output.push_str(",\"addendPairCount\":");
            output.push_str(&identity.addend_pair_count.to_string());
            output.push_str(",\"pitchPx\":");
            output.push_str(&format!(
                "{:.3}",
                hundredth_millimeters_to_css_px(u32::from(identity.pitch_mm100))
            ));
            output.push_str(",\"fontPx\":");
            output.push_str(&format!(
                "{:.3}",
                hundredth_millimeters_to_css_px(u32::from(identity.font_mm100))
            ));
            output.push_str(",\"addendRolesProven\":false}");
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"impliedVerticalMarginFromPageLineGapCountMm100\":");
    match anchor.implied_vertical_margin_from_page_line_gap_count_mm100 {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"verticalPairings\":[");
    for (index, pairing) in anchor.pairings.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"label\":");
        output.push_str(&json_string(pairing.label));
        output.push_str(",\"topFieldIndex\":");
        output.push_str(&pairing.top_field_index.to_string());
        output.push_str(",\"bottomFieldIndex\":");
        output.push_str(&pairing.bottom_field_index.to_string());
        output.push_str(",\"topMm100\":");
        output.push_str(&pairing.top_mm100.to_string());
        output.push_str(",\"bottomMm100\":");
        output.push_str(&pairing.bottom_mm100.to_string());
        output.push_str(",\"bodyHeightMm100\":");
        output.push_str(&pairing.body_height_mm100.to_string());
        output.push_str(",\"bodyTopPx\":");
        output.push_str(&format!("{:.3}", pairing.body_top_px));
        output.push_str(",\"bodyHeightPx\":");
        output.push_str(&format!("{:.3}", pairing.body_height_px));
        output.push_str(",\"lineCapacity\":");
        output.push_str(&pairing.line_capacity.to_string());
        output.push_str(",\"lineCapacityRemainderMm100\":");
        output.push_str(&pairing.line_capacity_remainder_mm100.to_string());
        output.push_str(",\"matchesPageRecordLineCount\":");
        output.push_str(json_bool(
            pairing.matches_page_record_line_count(page_record_line_count),
        ));
        output.push_str(",\"matchesPageRecordLineGapCount\":");
        output.push_str(json_bool(
            pairing.matches_page_record_line_gap_count(page_record_line_count),
        ));
        output.push_str(",\"pageLineRowTopsPx\":");
        push_f32_array_json(output, &pairing.page_line_row_tops_px);
        output.push_str(",\"ownRowTopYPx\":");
        push_f32_array_json(output, &pairing.own_row_top_y_px);
        output.push('}');
    }
    output.push(']');
    output.push_str(",\"matchedVerticalPairingCount\":");
    output.push_str(&anchor.matched_pairing_count().to_string());
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-only-page-grid-y-anchor-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-grid-y-anchor-is-not-a-decoded-page-space-origin",
    ));
    output.push('}');
}
