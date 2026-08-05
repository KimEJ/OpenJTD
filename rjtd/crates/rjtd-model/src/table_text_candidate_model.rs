use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSourceSpan {
    pub(crate) byte_start: usize,
    pub(crate) byte_end: usize,
    pub(crate) unit_start: usize,
    pub(crate) unit_end: usize,
}

impl TextSourceSpan {
    pub fn new(byte_start: usize, byte_end: usize, unit_start: usize, unit_end: usize) -> Self {
        Self {
            byte_start,
            byte_end,
            unit_start,
            unit_end,
        }
    }

    pub(crate) fn from_document_text_entry(entry: &DocumentTextMapEntry) -> Self {
        Self::new(
            entry.byte_start(),
            entry.byte_end(),
            entry.unit_start(),
            entry.unit_end(),
        )
    }

    pub(crate) fn subspan_by_units(&self, start_units: usize, end_units: usize) -> Self {
        Self::new(
            self.byte_start + start_units * 2,
            self.byte_start + end_units * 2,
            self.unit_start + start_units,
            self.unit_start + end_units,
        )
    }

    pub fn byte_start(&self) -> usize {
        self.byte_start
    }

    pub fn byte_end(&self) -> usize {
        self.byte_end
    }

    pub fn unit_start(&self) -> usize {
        self.unit_start
    }

    pub fn unit_end(&self) -> usize {
        self.unit_end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextControlBoundary {
    pub(crate) index: usize,
    pub(crate) code: u16,
    pub(crate) source_span: Option<TextSourceSpan>,
}

impl TextControlBoundary {
    pub fn new(index: usize, code: u16, source_span: Option<TextSourceSpan>) -> Self {
        Self {
            index,
            code,
            source_span,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn source_span(&self) -> Option<&TextSourceSpan> {
        self.source_span.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextCountRangeOverlapBasis {
    Byte,
    Unit,
}

impl TextCountRangeOverlapBasis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Byte => "byte",
            Self::Unit => "unit",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextCountRangeOverlap {
    pub(crate) basis: TextCountRangeOverlapBasis,
    pub(crate) block_index: usize,
    pub(crate) inline_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) text: String,
}

impl TextCountRangeOverlap {
    pub(crate) fn new(
        basis: TextCountRangeOverlapBasis,
        block_index: usize,
        inline_index: usize,
        source_start: usize,
        source_end: usize,
        text: String,
    ) -> Self {
        Self {
            basis,
            block_index,
            inline_index,
            source_start,
            source_end,
            text,
        }
    }

    pub fn basis(&self) -> TextCountRangeOverlapBasis {
        self.basis
    }

    pub fn block_index(&self) -> usize {
        self.block_index
    }

    pub fn inline_index(&self) -> usize {
        self.inline_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextCountControlRangeOverlap {
    pub(crate) basis: TextCountRangeOverlapBasis,
    pub(crate) delimiter_code: u16,
    pub(crate) range_count: usize,
    pub(crate) first_range_index: usize,
    pub(crate) last_range_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
}

impl TextCountControlRangeOverlap {
    pub(crate) fn new(
        basis: TextCountRangeOverlapBasis,
        delimiter_code: u16,
        range_count: usize,
        first_range_index: usize,
        last_range_index: usize,
        source_start: usize,
        source_end: usize,
    ) -> Self {
        Self {
            basis,
            delimiter_code,
            range_count,
            first_range_index,
            last_range_index,
            source_start,
            source_end,
        }
    }

    pub fn basis(&self) -> TextCountRangeOverlapBasis {
        self.basis
    }

    pub fn delimiter_code(&self) -> u16 {
        self.delimiter_code
    }

    pub fn range_count(&self) -> usize {
        self.range_count
    }

    pub fn first_range_index(&self) -> usize {
        self.first_range_index
    }

    pub fn last_range_index(&self) -> usize {
        self.last_range_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBoundaryCandidate {
    pub(crate) index: usize,
    pub(crate) text_count_range_index: usize,
    pub(crate) basis: TextCountRangeOverlapBasis,
    pub(crate) delimiter_code: u16,
    pub(crate) interval_count: usize,
    pub(crate) first_interval_index: usize,
    pub(crate) last_interval_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
}

impl TextBoundaryCandidate {
    pub(crate) fn from_control_range_overlap(
        index: usize,
        text_count_range_index: usize,
        overlap: &TextCountControlRangeOverlap,
    ) -> Self {
        Self {
            index,
            text_count_range_index,
            basis: overlap.basis(),
            delimiter_code: overlap.delimiter_code(),
            interval_count: overlap.range_count(),
            first_interval_index: overlap.first_range_index(),
            last_interval_index: overlap.last_range_index(),
            source_start: overlap.source_start(),
            source_end: overlap.source_end(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> &'static str {
        "controlDelimitedTextCountRange"
    }

    pub fn text_count_range_index(&self) -> usize {
        self.text_count_range_index
    }

    pub fn basis(&self) -> TextCountRangeOverlapBasis {
        self.basis
    }

    pub fn delimiter_code(&self) -> u16 {
        self.delimiter_code
    }

    pub fn interval_count(&self) -> usize {
        self.interval_count
    }

    pub fn first_interval_index(&self) -> usize {
        self.first_interval_index
    }

    pub fn last_interval_index(&self) -> usize {
        self.last_interval_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidate {
    pub(crate) index: usize,
    pub(crate) text_boundary_candidate_index: usize,
    pub(crate) text_count_range_index: usize,
    pub(crate) basis: TextCountRangeOverlapBasis,
    pub(crate) delimiter_code: u16,
    pub(crate) interval_count: usize,
    pub(crate) first_interval_index: usize,
    pub(crate) last_interval_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) intervals: Vec<TableCandidateInterval>,
}

impl TableCandidate {
    pub(crate) fn from_text_boundary_candidate(
        index: usize,
        candidate: &TextBoundaryCandidate,
        intervals: Vec<TableCandidateInterval>,
    ) -> Self {
        Self {
            index,
            text_boundary_candidate_index: candidate.index(),
            text_count_range_index: candidate.text_count_range_index(),
            basis: candidate.basis(),
            delimiter_code: candidate.delimiter_code(),
            interval_count: candidate.interval_count(),
            first_interval_index: candidate.first_interval_index(),
            last_interval_index: candidate.last_interval_index(),
            source_start: candidate.source_start(),
            source_end: candidate.source_end(),
            intervals,
        }
    }

    pub(crate) fn from_document_text_control_rows(
        index: usize,
        rows: &[DocumentTextControlTableRow],
    ) -> Self {
        let intervals = rows
            .iter()
            .enumerate()
            .map(|(row_index, row)| TableCandidateInterval::from_control_cells(row_index, row))
            .collect::<Vec<_>>();
        let first_interval_index = rows.first().map_or(0, |row| row.index);
        let last_interval_index = rows.last().map_or(0, |row| row.index);
        let source_start = rows.first().map_or(0, |row| row.source_start);
        let source_end = rows.last().map_or(source_start, |row| row.source_end);
        Self {
            index,
            text_boundary_candidate_index: DIRECT_TABLE_CANDIDATE_SENTINEL,
            text_count_range_index: DIRECT_TABLE_CANDIDATE_SENTINEL,
            basis: TextCountRangeOverlapBasis::Unit,
            delimiter_code: TABLE_ROW_DELIMITER_CONTROL,
            interval_count: intervals.len(),
            first_interval_index,
            last_interval_index,
            source_start,
            source_end,
            intervals,
        }
    }

    pub(crate) fn from_sparse_document_text_control_rows(
        index: usize,
        rows: &[DocumentTextControlTableRow],
    ) -> Self {
        let intervals = rows
            .iter()
            .enumerate()
            .map(|(row_index, row)| TableCandidateInterval::from_control_cells(row_index, row))
            .collect::<Vec<_>>();
        let first_interval_index = rows.first().map_or(0, |row| row.index);
        let last_interval_index = rows.last().map_or(0, |row| row.index);
        let source_start = rows.first().map_or(0, |row| row.source_start);
        let source_end = rows.last().map_or(source_start, |row| row.source_end);
        Self {
            index,
            text_boundary_candidate_index: SPARSE_TABLE_CANDIDATE_SENTINEL,
            text_count_range_index: SPARSE_TABLE_CANDIDATE_SENTINEL,
            basis: TextCountRangeOverlapBasis::Unit,
            delimiter_code: TABLE_ROW_DELIMITER_CONTROL,
            interval_count: intervals.len(),
            first_interval_index,
            last_interval_index,
            source_start,
            source_end,
            intervals,
        }
    }

    pub(crate) fn is_document_text_control_run_candidate(&self) -> bool {
        self.text_boundary_candidate_index == DIRECT_TABLE_CANDIDATE_SENTINEL
            && self.text_count_range_index == DIRECT_TABLE_CANDIDATE_SENTINEL
    }

    pub fn is_sparse_document_text_control_run_candidate(&self) -> bool {
        self.text_boundary_candidate_index == SPARSE_TABLE_CANDIDATE_SENTINEL
            && self.text_count_range_index == SPARSE_TABLE_CANDIDATE_SENTINEL
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> &'static str {
        if self.is_sparse_document_text_control_run_candidate() {
            "sparseDocumentTextControlRunTableCandidate"
        } else if self.is_document_text_control_run_candidate() {
            "documentTextControlRunTableCandidate"
        } else {
            "multiIntervalControlRangeTableCandidate"
        }
    }

    pub fn text_boundary_candidate_index(&self) -> usize {
        self.text_boundary_candidate_index
    }

    pub fn text_count_range_index(&self) -> usize {
        self.text_count_range_index
    }

    pub fn basis(&self) -> TextCountRangeOverlapBasis {
        self.basis
    }

    pub fn delimiter_code(&self) -> u16 {
        self.delimiter_code
    }

    pub fn interval_count(&self) -> usize {
        self.interval_count
    }

    pub fn first_interval_index(&self) -> usize {
        self.first_interval_index
    }

    pub fn last_interval_index(&self) -> usize {
        self.last_interval_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }

    pub fn intervals(&self) -> &[TableCandidateInterval] {
        &self.intervals
    }

    pub fn cell_count_candidate(&self) -> usize {
        self.intervals
            .iter()
            .map(|interval| interval.column_segments().len())
            .sum()
    }

    pub fn empty_cell_count_candidate(&self) -> usize {
        self.intervals
            .iter()
            .flat_map(|interval| interval.column_segments())
            .filter(|segment| segment.text().is_empty())
            .count()
    }

    pub fn non_empty_cell_count_candidate(&self) -> usize {
        self.cell_count_candidate()
            .saturating_sub(self.empty_cell_count_candidate())
    }

    pub fn is_row_like(&self) -> bool {
        if self.is_sparse_document_text_control_run_candidate() {
            return false;
        }

        let mut non_empty = 0usize;
        for interval in &self.intervals {
            if interval.line_break_count() != 0 {
                return false;
            }
            if interval.text_char_count() == 0 {
                return false;
            }
            non_empty += 1;
        }
        non_empty > 1
    }

    pub fn is_cell_like(&self) -> bool {
        self.is_row_like()
    }

    pub fn column_split_candidate_row_count(&self) -> usize {
        self.intervals
            .iter()
            .filter(|interval| !interval.column_segments().is_empty())
            .count()
    }

    pub fn max_column_segment_count(&self) -> usize {
        self.intervals
            .iter()
            .map(|interval| interval.column_segments().len())
            .max()
            .unwrap_or(0)
    }

    pub fn column_segment_pattern_consistent(&self) -> bool {
        self.column_split_candidate_row_count() > 0
            && self.column_segment_pattern_mismatch_rows() == 0
    }

    pub fn column_segment_pattern_mismatch_rows(&self) -> usize {
        if self.document_text_control_column_segments_are_compatible() {
            return 0;
        }

        let mut split_rows = 0usize;
        let mut signature_counts: BTreeMap<Vec<TableCandidateColumnSegmentKind>, usize> =
            BTreeMap::new();

        for interval in &self.intervals {
            if interval.column_segments().is_empty() {
                continue;
            }
            split_rows += 1;
            let signature = interval
                .column_segments()
                .iter()
                .map(|segment| segment.kind())
                .collect::<Vec<_>>();
            *signature_counts.entry(signature).or_insert(0) += 1;
        }

        if split_rows == 0 {
            return 0;
        }

        let dominant_rows = signature_counts.values().copied().max().unwrap_or(0);
        split_rows.saturating_sub(dominant_rows)
    }

    pub fn column_segment_grid_candidate(&self) -> Option<TableCandidateColumnGridCandidate> {
        if !self.is_row_like() || !self.column_segment_pattern_consistent() {
            return None;
        }

        let split_rows = self.column_split_candidate_row_count();
        if split_rows == 0 || split_rows != self.intervals.len() {
            return None;
        }

        let pattern_source = if self.document_text_control_column_segments_are_compatible() {
            self.intervals
                .iter()
                .max_by_key(|interval| interval.column_segments().len())
        } else {
            self.intervals
                .iter()
                .find(|interval| !interval.column_segments().is_empty())
        }?;
        let pattern = pattern_source
            .column_segments()
            .iter()
            .map(|segment| segment.kind())
            .collect::<Vec<_>>();

        if pattern.len() < 2 {
            return None;
        }

        Some(TableCandidateColumnGridCandidate::new(
            self.intervals.len(),
            pattern,
            split_rows,
        ))
    }

    pub(crate) fn document_text_control_column_segments_are_compatible(&self) -> bool {
        if !self.is_document_text_control_run_candidate() || self.intervals.len() < 3 {
            return false;
        }
        let mut min_columns = usize::MAX;
        let mut max_columns = 0usize;
        for interval in &self.intervals {
            let column_count = interval.column_segments().len();
            if column_count < 2 {
                return false;
            }
            if interval
                .column_segments()
                .iter()
                .any(|segment| segment.kind() != TableCandidateColumnSegmentKind::Label)
            {
                return false;
            }
            min_columns = min_columns.min(column_count);
            max_columns = max_columns.max(column_count);
        }
        max_columns >= 3 && max_columns.saturating_sub(min_columns) <= 1
    }

    pub fn sparse_topology_candidate(&self) -> Option<TableCandidateSparseTopologyCandidate> {
        if !self.is_sparse_document_text_control_run_candidate() {
            return None;
        }
        TableCandidateSparseTopologyCandidate::from_table_candidate(self)
    }

    pub fn rule(&self) -> &'static str {
        if self.is_sparse_document_text_control_run_candidate() {
            "sparse-document-text-001c-cells-with-000e-row-breaks"
        } else if self.is_document_text_control_run_candidate() {
            "document-text-001c-cells-with-000e-row-breaks"
        } else {
            "control-delimited-text-count-range-with-multiple-intervals"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateSparseTopologyCandidate {
    pub(crate) row_count: usize,
    pub(crate) max_column_count: usize,
    pub(crate) cell_count: usize,
    pub(crate) empty_cell_count: usize,
    pub(crate) non_empty_cell_count: usize,
    pub(crate) rows: Vec<TableCandidateSparseTopologyRow>,
    pub(crate) columns: Vec<TableCandidateSparseTopologyColumn>,
}

impl TableCandidateSparseTopologyCandidate {
    pub(crate) fn from_table_candidate(candidate: &TableCandidate) -> Option<Self> {
        let row_count = candidate.intervals().len();
        let max_column_count = candidate.max_column_segment_count();
        if row_count == 0 || max_column_count == 0 {
            return None;
        }

        let rows = candidate
            .intervals()
            .iter()
            .map(TableCandidateSparseTopologyRow::from_interval)
            .collect::<Vec<_>>();
        let columns = (0..max_column_count)
            .map(|column_index| {
                TableCandidateSparseTopologyColumn::from_candidate_column(candidate, column_index)
            })
            .collect::<Vec<_>>();

        Some(Self {
            row_count,
            max_column_count,
            cell_count: candidate.cell_count_candidate(),
            empty_cell_count: candidate.empty_cell_count_candidate(),
            non_empty_cell_count: candidate.non_empty_cell_count_candidate(),
            rows,
            columns,
        })
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn max_column_count(&self) -> usize {
        self.max_column_count
    }

    pub fn cell_count(&self) -> usize {
        self.cell_count
    }

    pub fn empty_cell_count(&self) -> usize {
        self.empty_cell_count
    }

    pub fn non_empty_cell_count(&self) -> usize {
        self.non_empty_cell_count
    }

    pub fn rows(&self) -> &[TableCandidateSparseTopologyRow] {
        &self.rows
    }

    pub fn columns(&self) -> &[TableCandidateSparseTopologyColumn] {
        &self.columns
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateSparseTopologyRow {
    pub(crate) index: usize,
    pub(crate) source_interval_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) cell_count: usize,
    pub(crate) empty_cell_count: usize,
    pub(crate) non_empty_cell_count: usize,
    pub(crate) first_non_empty_column_index: Option<usize>,
    pub(crate) last_non_empty_column_index: Option<usize>,
}

impl TableCandidateSparseTopologyRow {
    pub(crate) fn from_interval(interval: &TableCandidateInterval) -> Self {
        let mut first_non_empty_column_index = None;
        let mut last_non_empty_column_index = None;
        let mut empty_cell_count = 0usize;
        let mut non_empty_cell_count = 0usize;

        for segment in interval.column_segments() {
            if segment.text().is_empty() {
                empty_cell_count += 1;
            } else {
                non_empty_cell_count += 1;
                if first_non_empty_column_index.is_none() {
                    first_non_empty_column_index = Some(segment.index());
                }
                last_non_empty_column_index = Some(segment.index());
            }
        }

        Self {
            index: interval.index(),
            source_interval_index: interval.source_interval_index(),
            source_start: interval.source_start(),
            source_end: interval.source_end(),
            cell_count: interval.column_segments().len(),
            empty_cell_count,
            non_empty_cell_count,
            first_non_empty_column_index,
            last_non_empty_column_index,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn source_interval_index(&self) -> usize {
        self.source_interval_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }

    pub fn cell_count(&self) -> usize {
        self.cell_count
    }

    pub fn empty_cell_count(&self) -> usize {
        self.empty_cell_count
    }

    pub fn non_empty_cell_count(&self) -> usize {
        self.non_empty_cell_count
    }

    pub fn first_non_empty_column_index(&self) -> Option<usize> {
        self.first_non_empty_column_index
    }

    pub fn last_non_empty_column_index(&self) -> Option<usize> {
        self.last_non_empty_column_index
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateSparseTopologyColumn {
    pub(crate) index: usize,
    pub(crate) observed_cell_count: usize,
    pub(crate) empty_cell_count: usize,
    pub(crate) non_empty_cell_count: usize,
    pub(crate) first_non_empty_row_index: Option<usize>,
    pub(crate) last_non_empty_row_index: Option<usize>,
    pub(crate) source_start: Option<usize>,
    pub(crate) source_end: Option<usize>,
}

impl TableCandidateSparseTopologyColumn {
    pub(crate) fn from_candidate_column(candidate: &TableCandidate, column_index: usize) -> Self {
        let mut observed_cell_count = 0usize;
        let mut empty_cell_count = 0usize;
        let mut non_empty_cell_count = 0usize;
        let mut first_non_empty_row_index = None;
        let mut last_non_empty_row_index = None;
        let mut source_start = None;
        let mut source_end = None;

        for row in candidate.intervals() {
            let Some(segment) = row
                .column_segments()
                .iter()
                .find(|segment| segment.index() == column_index)
            else {
                continue;
            };

            observed_cell_count += 1;
            source_start = option_min_usize(source_start, segment.source_start());
            source_end = option_max_usize(source_end, segment.source_end());

            if segment.text().is_empty() {
                empty_cell_count += 1;
            } else {
                non_empty_cell_count += 1;
                if first_non_empty_row_index.is_none() {
                    first_non_empty_row_index = Some(row.index());
                }
                last_non_empty_row_index = Some(row.index());
            }
        }

        Self {
            index: column_index,
            observed_cell_count,
            empty_cell_count,
            non_empty_cell_count,
            first_non_empty_row_index,
            last_non_empty_row_index,
            source_start,
            source_end,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn observed_cell_count(&self) -> usize {
        self.observed_cell_count
    }

    pub fn empty_cell_count(&self) -> usize {
        self.empty_cell_count
    }

    pub fn non_empty_cell_count(&self) -> usize {
        self.non_empty_cell_count
    }

    pub fn first_non_empty_row_index(&self) -> Option<usize> {
        self.first_non_empty_row_index
    }

    pub fn last_non_empty_row_index(&self) -> Option<usize> {
        self.last_non_empty_row_index
    }

    pub fn source_start(&self) -> Option<usize> {
        self.source_start
    }

    pub fn source_end(&self) -> Option<usize> {
        self.source_end
    }
}

pub(crate) fn option_min_usize(left: Option<usize>, right: Option<usize>) -> Option<usize> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

pub(crate) fn option_max_usize(left: Option<usize>, right: Option<usize>) -> Option<usize> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateColumnGridCandidate {
    pub(crate) row_count: usize,
    pub(crate) column_count: usize,
    pub(crate) cell_count: usize,
    pub(crate) split_row_count: usize,
    pub(crate) pattern: Vec<TableCandidateColumnSegmentKind>,
}

impl TableCandidateColumnGridCandidate {
    pub(crate) fn new(
        row_count: usize,
        pattern: Vec<TableCandidateColumnSegmentKind>,
        split_row_count: usize,
    ) -> Self {
        let column_count = pattern.len();
        Self {
            row_count,
            column_count,
            cell_count: row_count.saturating_mul(column_count),
            split_row_count,
            pattern,
        }
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn column_count(&self) -> usize {
        self.column_count
    }

    pub fn cell_count(&self) -> usize {
        self.cell_count
    }

    pub fn split_row_count(&self) -> usize {
        self.split_row_count
    }

    pub fn pattern(&self) -> &[TableCandidateColumnSegmentKind] {
        &self.pattern
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateInterval {
    pub(crate) index: usize,
    pub(crate) source_interval_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) text_preview: String,
    pub(crate) text_char_count: usize,
    pub(crate) line_break_count: usize,
    pub(crate) column_segments: Vec<TableCandidateColumnSegment>,
}

impl TableCandidateInterval {
    pub(crate) fn new(
        index: usize,
        source_interval_index: usize,
        source_start: usize,
        source_end: usize,
        text: String,
    ) -> Self {
        let text_char_count = text.chars().count();
        let line_break_count = text_line_break_count(&text);
        let text_preview = preview_text(&text, 80);
        let column_segments = table_row_column_segments(&text);
        Self {
            index,
            source_interval_index,
            source_start,
            source_end,
            text_preview,
            text_char_count,
            line_break_count,
            column_segments,
        }
    }

    pub(crate) fn from_control_cells(index: usize, row: &DocumentTextControlTableRow) -> Self {
        let mut text = String::new();
        let mut column_segments = Vec::new();
        let mut char_offset = 0usize;
        for (cell_index, cell) in row.cells.iter().enumerate() {
            if cell_index > 0 {
                text.push('\t');
                char_offset += 1;
            }
            let cell_text = clean_table_control_cell_text(&cell.text);
            let char_start = char_offset;
            text.push_str(&cell_text);
            char_offset += cell_text.chars().count();
            column_segments.push(TableCandidateColumnSegment::new(
                cell_index,
                TableCandidateColumnSegmentKind::Label,
                char_start,
                char_offset,
                Some(cell.source_start),
                Some(cell.source_end),
                cell_text,
            ));
        }
        let text_char_count = text.chars().count();
        let text_preview = preview_text(&text, 80);
        Self {
            index,
            source_interval_index: row.index,
            source_start: row.source_start,
            source_end: row.source_end,
            text_preview,
            text_char_count,
            line_break_count: 0,
            column_segments,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn source_interval_index(&self) -> usize {
        self.source_interval_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }

    pub fn text_preview(&self) -> &str {
        &self.text_preview
    }

    pub fn text_char_count(&self) -> usize {
        self.text_char_count
    }

    pub fn line_break_count(&self) -> usize {
        self.line_break_count
    }

    pub fn column_segments(&self) -> &[TableCandidateColumnSegment] {
        &self.column_segments
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateColumnSegment {
    pub(crate) index: usize,
    pub(crate) kind: TableCandidateColumnSegmentKind,
    pub(crate) char_start: usize,
    pub(crate) char_end: usize,
    pub(crate) source_start: Option<usize>,
    pub(crate) source_end: Option<usize>,
    pub(crate) text: String,
}

impl TableCandidateColumnSegment {
    pub(crate) fn new(
        index: usize,
        kind: TableCandidateColumnSegmentKind,
        char_start: usize,
        char_end: usize,
        source_start: Option<usize>,
        source_end: Option<usize>,
        text: String,
    ) -> Self {
        Self {
            index,
            kind,
            char_start,
            char_end,
            source_start,
            source_end,
            text,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> TableCandidateColumnSegmentKind {
        self.kind
    }

    pub fn char_start(&self) -> usize {
        self.char_start
    }

    pub fn char_end(&self) -> usize {
        self.char_end
    }

    pub fn source_start(&self) -> Option<usize> {
        self.source_start
    }

    pub fn source_end(&self) -> Option<usize> {
        self.source_end
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableCandidateColumnSegmentKind {
    Label,
    Value,
}

impl TableCandidateColumnSegmentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Label => "label",
            Self::Value => "value",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextLayoutExactEvidence {
    pub(crate) target: &'static str,
    pub(crate) base: &'static str,
    pub(crate) delta: isize,
}

impl TextLayoutExactEvidence {
    pub(crate) fn new(target: &'static str, base: &'static str, delta: isize) -> Self {
        Self {
            target,
            base,
            delta,
        }
    }

    pub fn target(&self) -> &'static str {
        self.target
    }

    pub fn base(&self) -> &'static str {
        self.base
    }

    pub fn delta(&self) -> isize {
        self.delta
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextCountRange {
    pub(crate) index: usize,
    pub(crate) family: String,
    pub(crate) start: u32,
    pub(crate) end: u32,
    pub(crate) declared_start: u32,
    pub(crate) declared_end: u32,
    pub(crate) tail_fields: Vec<u16>,
    pub(crate) document_text_overlaps: Vec<TextCountRangeOverlap>,
    pub(crate) control_range_overlaps: Vec<TextCountControlRangeOverlap>,
    pub(crate) raw: Vec<u8>,
}

impl TextCountRange {
    pub(crate) fn from_entry(entry: &DocumentTextCountEntry) -> Self {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (start, end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        Self {
            index: entry.index(),
            family: family.to_string(),
            start,
            end,
            declared_start: entry.start_offset(),
            declared_end: entry.end_offset(),
            tail_fields: read_be16_fields(&raw[tail_offset..]),
            document_text_overlaps: Vec::new(),
            control_range_overlaps: Vec::new(),
            raw: raw.to_vec(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }

    pub fn span(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    pub fn declared_start(&self) -> u32 {
        self.declared_start
    }

    pub fn declared_end(&self) -> u32 {
        self.declared_end
    }

    pub fn tail_fields(&self) -> &[u16] {
        &self.tail_fields
    }

    pub fn document_text_overlaps(&self) -> &[TextCountRangeOverlap] {
        &self.document_text_overlaps
    }

    pub(crate) fn set_document_text_overlaps(&mut self, overlaps: Vec<TextCountRangeOverlap>) {
        self.document_text_overlaps = overlaps;
    }

    pub fn control_range_overlaps(&self) -> &[TextCountControlRangeOverlap] {
        &self.control_range_overlaps
    }

    pub(crate) fn set_control_range_overlaps(
        &mut self,
        overlaps: Vec<TextCountControlRangeOverlap>,
    ) {
        self.control_range_overlaps = overlaps;
    }

    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
}

pub(crate) fn read_be32_candidate(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

pub(crate) fn read_be16_at(bytes: &[u8], offset: usize) -> Option<u16> {
    let bytes = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_be_bytes([bytes[0], bytes[1]]))
}

pub(crate) fn read_be32_at(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(crate) fn read_le16_at(bytes: &[u8], offset: usize) -> Option<u16> {
    let bytes = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}

pub(crate) fn read_le32_at(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(crate) fn read_i32_le_at(bytes: &[u8], offset: usize) -> Option<i32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(crate) fn read_i32_be_at(bytes: &[u8], offset: usize) -> Option<i32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(crate) fn read_be16_fields(bytes: &[u8]) -> Vec<u16> {
    bytes
        .chunks_exact(2)
        .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
        .collect()
}
