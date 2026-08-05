use super::*;

impl DocumentCore {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        Self::from_bytes_with_limits(data, ParseLimits::DEFAULT)
    }

    /// Creates a document core from already allocated input with explicit resource limits.
    ///
    /// Per-member and total LH5 output are bounded by `limits`; the input limit checks this
    /// `&[u8]` after the caller has allocated it, so it cannot reduce the caller's allocation.
    pub fn from_bytes_with_limits(data: &[u8], limits: ParseLimits) -> Result<Self> {
        let mut budget = limits.resource_budget();
        Self::from_bytes_with_budget(data, &mut budget)
    }

    /// Builds a document core with caller-owned shared resource accounting.
    pub fn from_bytes_with_budget(data: &[u8], budget: &mut ResourceBudget) -> Result<Self> {
        let document = parse_document_with_budget(data, budget)?;
        Self::from_document_with_budget(document, budget)
    }

    pub fn from_document(document: Document) -> Self {
        let mut core = Self::from_document_unpaginated(document);
        core.refresh_pages();
        core
    }

    pub fn from_document_with_limits(document: Document, limits: ParseLimits) -> Result<Self> {
        let mut budget = limits.resource_budget();
        Self::from_document_with_budget(document, &mut budget)
    }

    /// Builds page state with caller-owned shared resource accounting.
    pub fn from_document_with_budget(
        document: Document,
        budget: &mut ResourceBudget,
    ) -> Result<Self> {
        let mut core = Self::from_document_unpaginated(document);
        core.refresh_pages_with_budget(budget)?;
        Ok(core)
    }

    pub(crate) fn from_document_unpaginated(document: Document) -> Self {
        let decoded_page_layout = page_layout_from_document(&document);
        let hint = source_document_layout_hint(&document, decoded_page_layout);
        let mut page_layout = decoded_page_layout;
        let mut writing_mode = WritingMode::Horizontal;
        if let Some(hint) = hint {
            if hint.override_decoded_layout || page_layout == PageLayout::default() {
                page_layout = hint.fallback_layout;
            }
            if let Some(margin_px) = hint.margin_override_px {
                page_layout = page_layout.with_margin_px(margin_px);
            }
            if let Some(wrap_columns) = hint.vertical_wrap_columns_override {
                page_layout = page_layout.with_vertical_wrap_columns_override(wrap_columns);
            }
            writing_mode = hint.writing_mode;
        }
        Self {
            document,
            pages: Vec::new(),
            file_name: String::new(),
            dpi: APP_DEFAULT_DPI,
            page_layout,
            show_paragraph_marks: false,
            show_control_codes: false,
            show_transparent_borders: false,
            clip_enabled: true,
            writing_mode,
            next_snapshot_id: 1,
            snapshots: Vec::new(),
            caret_section: 0,
            caret_paragraph: 0,
            caret_char_offset: 0,
            clipboard_text: None,
        }
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub(crate) fn observed_table_candidate(&self, control_idx: u32) -> Option<&TableCandidate> {
        let candidate = self.document.table_candidates().get(control_idx as usize)?;
        candidate.is_row_like().then_some(candidate)
    }

    pub(crate) fn observed_table_cell(
        &self,
        control_idx: u32,
        cell_idx: u32,
    ) -> Option<&TableCandidateInterval> {
        self.observed_table_candidate(control_idx)?
            .intervals()
            .get(cell_idx as usize)
    }

    pub fn page_count(&self) -> u32 {
        self.pages.len().max(1) as u32
    }

    pub fn get_section_count(&self) -> u32 {
        1
    }

    pub fn get_document_info(&self) -> String {
        let style_candidates = text_style_candidates(self.document.unknown_styles());
        let font_names = document_font_names(&self.document);
        let fallback_font = primary_document_font_name(&font_names);
        let writing_mode_decision = writing_mode_decision_json(&self.document, self.writing_mode);
        let document_view_writing_mode_candidate =
            writing_mode_candidate_from_document_view_styles(self.document.unknown_styles());
        let document_view_writing_mode_candidate_str = document_view_writing_mode_candidate
            .as_ref()
            .map(|candidate| format!("\"{}\"", candidate.writing_mode.as_str()))
            .unwrap_or_else(|| "null".to_string());
        let document_view_writing_mode_first_code = document_view_writing_mode_candidate
            .as_ref()
            .map(|candidate| candidate.first_record_code.to_string())
            .unwrap_or_else(|| "null".to_string());
        let document_view_writing_mode_first_code_hex = document_view_writing_mode_candidate
            .as_ref()
            .map(|candidate| json_string(&format!("0x{:04x}", candidate.first_record_code)))
            .unwrap_or_else(|| "null".to_string());
        let paper_mark_writing_mode_diagnostics =
            paper_mark_writing_mode_diagnostics(self.document.paper_marks());
        let fdm_text_mirror_anchor_agreements =
            fdm_text_mirror_anchor_agreements(self.document.object_stream_candidates());
        let writing_mode_candidate_str = paper_mark_writing_mode_diagnostics
            .candidate
            .map(|m| format!("\"{}\"", m.as_str()))
            .unwrap_or_else(|| "null".to_string());
        format!(
            "{{\"version\":\"{APP_VERSION}\",\"format\":\"JTD\",\"engine\":\"rjtd\",\"sourceFormat\":\"{}\",\"fileName\":{},\"sectionCount\":1,\"pageCount\":{},\"encrypted\":false,\"hwp3Variant\":false,\"fallbackFont\":{},\"fontsUsed\":{},\"writingMode\":\"{}\",\"writingModeDecoded\":false,\"writingModeDecision\":{},\"writingModeCandidateFromDocumentViewStyles\":{},\"writingModeCandidateFromDocumentViewStylesDecoded\":false,\"writingModeCandidateFromDocumentViewStylesSourceBacked\":{},\"writingModeCandidateFromDocumentViewStylesFirstRecordCode\":{},\"writingModeCandidateFromDocumentViewStylesFirstRecordCodeHex\":{},\"writingModeCandidateFromPaperMark\":{},\"writingModeCandidateDecoded\":false,\"paperMarkFlagBit0VerticalCandidate\":{},\"paperMarkFlagBit17IndexStepCandidate\":{},\"paperMarkWritingModeCandidateEvidence\":{},\"paperMarkWritingModeCandidateBlockers\":{},\"blockCount\":{},\"rawStreamCount\":{},\"styleStreamCount\":{},\"styleCandidateCount\":{},\"styleCandidateNames\":{},\"styleStreams\":{},\"fontCount\":{},\"fontTable\":{},\"autoTextCount\":{},\"autoTextCandidates\":{},\"tocEntryCount\":{},\"tocEntries\":{},\"pageMarkCount\":{},\"pageMarks\":{},\"paperMarkCount\":{},\"paperMarks\":{},\"objectStreamCandidateCount\":{},\"objectStreamCandidates\":{},\"fdmTextMirrorAnchorAgreementCount\":{},\"fdmTextMirrorAnchorAgreements\":{},\"objectFrameRecordCount\":{},\"objectFrameRecords\":{},\"objectEmbeddingFrameCount\":{},\"objectEmbeddingFrames\":{},\"textCountRangeCount\":{},\"textCountRanges\":{},\"textControlBoundaryCount\":{},\"textControlBoundaries\":{},\"textBoundaryCandidateCount\":{},\"textBoundaryCandidates\":{},\"textParagraphBoundaryCandidateCount\":{},\"textParagraphBoundaryCandidates\":{},\"fdmOpenStrokeCohortSummary\":{},\"tableCandidateCount\":{},\"tableCandidates\":{}}}",
            APP_SOURCE_FORMAT,
            json_string(&self.file_name),
            self.page_count(),
            json_string(fallback_font),
            string_array_json(&font_names),
            self.writing_mode.as_str(),
            writing_mode_decision,
            document_view_writing_mode_candidate_str,
            if document_view_writing_mode_candidate.is_some() {
                "true"
            } else {
                "false"
            },
            document_view_writing_mode_first_code,
            document_view_writing_mode_first_code_hex,
            writing_mode_candidate_str,
            paper_mark_writing_mode_diagnostics.flag_bit0_vertical_candidate,
            paper_mark_writing_mode_diagnostics.flag_bit17_index_step_candidate,
            string_slice_array_json(&paper_mark_writing_mode_diagnostics.evidence),
            string_slice_array_json(&paper_mark_writing_mode_diagnostics.blockers),
            self.document.blocks().len(),
            self.document.raw_streams().len(),
            self.document.unknown_styles().len(),
            style_candidates.len(),
            style_candidate_names_json(&style_candidates),
            style_source_streams_json(self.document.unknown_styles()),
            self.document.fonts().len(),
            font_table_json(self.document.fonts()),
            self.document.auto_texts().len(),
            auto_texts_json(self.document.auto_texts()),
            self.document.toc_entries().len(),
            toc_entries_json(self.document.toc_entries()),
            self.document.page_marks().len(),
            page_marks_json(self.document.page_marks()),
            self.document.paper_marks().len(),
            paper_marks_json(self.document.paper_marks()),
            self.document.object_stream_candidates().len(),
            object_stream_candidates_json(self.document.object_stream_candidates()),
            fdm_text_mirror_anchor_agreements.len(),
            fdm_text_mirror_anchor_agreements_json(&fdm_text_mirror_anchor_agreements),
            self.document.object_frame_records().len(),
            object_frame_records_json(self.document.object_frame_records()),
            self.document.object_embedding_frames().len(),
            object_embedding_frames_json(self.document.object_embedding_frames()),
            self.document.text_count_ranges().len(),
            text_count_ranges_json(self.document.text_count_ranges()),
            self.document.text_control_boundaries().len(),
            text_control_boundaries_json(self.document.text_control_boundaries()),
            self.document.text_boundary_candidates().len(),
            text_boundary_candidates_json(self.document.text_boundary_candidates()),
            self.document.text_paragraph_boundary_candidates().len(),
            text_paragraph_boundary_candidates_json(
                self.document.text_paragraph_boundary_candidates()
            ),
            fdm_open_stroke_cohort_summary_json(self.page_layout, &self.document),
            self.document.table_candidates().len(),
            table_candidates_json(self.document.table_candidates())
        )
    }

    pub fn set_file_name(&mut self, name: impl Into<String>) {
        self.file_name = name.into();
        let decoded_page_layout = page_layout_from_document(&self.document);
        if let Some(hint) = source_document_layout_hint(&self.document, decoded_page_layout) {
            if hint.override_decoded_layout || self.page_layout == PageLayout::default() {
                self.page_layout = hint.fallback_layout;
            }
            if let Some(margin_px) = hint.margin_override_px {
                self.page_layout = self.page_layout.with_margin_px(margin_px);
            }
            if let Some(wrap_columns) = hint.vertical_wrap_columns_override {
                self.page_layout = self
                    .page_layout
                    .with_vertical_wrap_columns_override(wrap_columns);
            }
            self.writing_mode = hint.writing_mode;
        }
        self.refresh_pages();
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn get_source_format(&self) -> &'static str {
        APP_SOURCE_FORMAT
    }

    pub fn get_dpi(&self) -> f64 {
        self.dpi
    }

    pub fn set_dpi(&mut self, dpi: f64) {
        if dpi.is_finite() && dpi > 0.0 {
            self.dpi = dpi;
        }
    }

    pub fn writing_mode(&self) -> WritingMode {
        self.writing_mode
    }

    pub fn set_writing_mode(&mut self, writing_mode: WritingMode) {
        self.writing_mode = writing_mode;
        self.refresh_pages();
    }

    pub fn page_layout(&self) -> PageLayout {
        self.page_layout
    }

    pub fn get_page_def(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        let layout = self.page_layout;
        Ok(format!(
            "{{\"width\":{:.1},\"height\":{:.1},\"marginLeft\":{:.1},\"marginRight\":{:.1},\"marginTop\":{:.1},\"marginBottom\":{:.1},\"marginHeader\":0.0,\"marginFooter\":0.0,\"marginGutter\":0.0,\"landscape\":{},\"binding\":0}}",
            layout.width_px(),
            layout.height_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.landscape()
        ))
    }

    pub fn get_page_def_native(&self, section_idx: u32) -> Result<String> {
        self.get_page_def(section_idx)
    }

    pub fn set_page_def(&mut self, section_idx: u32, _page_def_json: &str) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(ok_page_count_json(self.page_count()))
    }

    pub fn set_page_def_native(&mut self, section_idx: u32, page_def_json: &str) -> Result<String> {
        self.set_page_def(section_idx, page_def_json)
    }

    pub fn get_section_def(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"pageNum\":1,\"pageNumType\":0,\"pictureNum\":1,\"tableNum\":1,\"equationNum\":1,\"columnSpacing\":0,\"defaultTabSpacing\":0,\"hideHeader\":false,\"hideFooter\":false,\"hideMasterPage\":false,\"hideBorder\":false,\"hideFill\":false,\"hideEmptyLine\":false}".to_string())
    }

    pub fn get_section_def_native(&self, section_idx: u32) -> Result<String> {
        self.get_section_def(section_idx)
    }

    pub fn set_section_def(&mut self, section_idx: u32, _section_def_json: &str) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(ok_page_count_json(self.page_count()))
    }

    pub fn set_section_def_native(
        &mut self,
        section_idx: u32,
        section_def_json: &str,
    ) -> Result<String> {
        self.set_section_def(section_idx, section_def_json)
    }

    pub fn set_section_def_all(&mut self, _section_def_json: &str) -> String {
        ok_page_count_json(self.page_count())
    }

    pub fn set_section_def_all_native(&mut self, section_def_json: &str) -> String {
        self.set_section_def_all(section_def_json)
    }

    pub fn get_page_border_fill(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        let border = "{\"type\":0,\"width\":0,\"color\":\"#000000\"}";
        Ok(format!(
            "{{\"attr\":0,\"basis\":\"paper\",\"spacingLeft\":0,\"spacingRight\":0,\"spacingTop\":0,\"spacingBottom\":0,\"borderFillId\":0,\"headerInside\":false,\"footerInside\":false,\"fillArea\":\"paper\",\"hideBorder\":true,\"hideFill\":true,\"borderLeft\":{border},\"borderRight\":{border},\"borderTop\":{border},\"borderBottom\":{border},\"fillType\":\"none\",\"fillColor\":\"#ffffff\",\"patternColor\":\"#000000\",\"patternType\":0,\"applyPage\":\"all\"}}"
        ))
    }

    pub fn get_page_border_fill_native(&self, section_idx: u32) -> Result<String> {
        self.get_page_border_fill(section_idx)
    }

    pub fn set_page_border_fill(
        &mut self,
        section_idx: u32,
        _settings_json: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(ok_page_count_json(self.page_count()))
    }

    pub fn set_page_border_fill_native(
        &mut self,
        section_idx: u32,
        settings_json: &str,
    ) -> Result<String> {
        self.set_page_border_fill(section_idx, settings_json)
    }

    pub fn plain_text(&self) -> String {
        document_plain_text(&self.document)
    }

    pub fn page_width_px(&self) -> f64 {
        self.page_layout.width_px() as f64
    }

    pub fn page_height_px(&self) -> f64 {
        self.page_layout.height_px() as f64
    }

    pub fn page_margin_px(&self) -> f64 {
        self.page_layout.margin_px() as f64
    }

    pub fn font_size_px(&self) -> f64 {
        APP_FONT_SIZE_PX as f64
    }

    pub fn line_height_px(&self) -> f64 {
        APP_LINE_HEIGHT_PX as f64
    }

    pub fn page_text_lines(&self, page_num: u32) -> Result<&[PageTextLine]> {
        self.page_lines(page_num)
    }

    pub(crate) fn page_decoration(&self, page_index: usize) -> Option<PageDecoration> {
        if !self.writing_mode.is_vertical() {
            return None;
        }
        let paired_slot_pairs = document_page_decoration_paired_slot_pairs(&self.document);
        if paired_slot_pairs.is_empty() {
            return None;
        }
        let slot_evidence = document_page_decoration_slot_evidence(&self.document);
        let document_title = document_auto_text_title(&self.document)?;
        let chapter_titles = document_chapter_title_candidates(&self.document);
        if chapter_titles.is_empty() {
            return None;
        }
        let body_start_page =
            running_body_start_page(&self.pages, document_title, &chapter_titles)?;
        if page_index < body_start_page {
            return None;
        }
        if page_index > body_start_page
            && self
                .pages
                .get(page_index)
                .is_some_and(|page| page_has_exact_text_line(page, document_title))
        {
            return None;
        }
        let chapter_title = running_chapter_title_for_page(
            &self.pages,
            body_start_page,
            page_index,
            &chapter_titles,
        )?;
        let page_number = page_index + 1;
        let side = if page_number.is_multiple_of(2) {
            PageDecorationSide::Left
        } else {
            PageDecorationSide::Right
        };
        let header_text = if side == PageDecorationSide::Left {
            chapter_title
        } else {
            document_title.to_string()
        };
        Some(PageDecoration {
            side,
            page_number,
            header_text,
            source: "autoTextInfo+pageLayoutStylePairedSlots+documentText",
            side_policy: "facing-pages-odd-right-even-left",
            side_policy_decoded: false,
            facing_pages_candidate: true,
            paired_slot_pairs,
            slot_evidence,
            mark_evidence: page_decoration_mark_evidence(&self.document, page_index),
        })
    }

    pub fn get_page_info(&self, page_num: u32) -> Result<String> {
        self.page_lines(page_num)?;
        let layout = self.page_layout;
        let body_x = layout.margin_px();
        let body_width = layout.body_width_px();
        let mark_evidence = page_decoration_mark_evidence(&self.document, page_num as usize);
        let mut mark_evidence_json = String::new();
        push_page_decoration_mark_evidence_json(
            &mut mark_evidence_json,
            layout,
            mark_evidence.as_ref(),
        );
        Ok(format!(
            "{{\"pageIndex\":{},\"pageNumber\":{},\"width\":{:.1},\"height\":{:.1},\"sectionIndex\":0,\"marginLeft\":{:.1},\"marginRight\":{:.1},\"marginTop\":{:.1},\"marginBottom\":{:.1},\"marginHeader\":0.0,\"marginFooter\":0.0,\"pageBorderLeft\":{:.1},\"pageBorderRight\":{:.1},\"pageBorderTop\":{:.1},\"pageBorderBottom\":{:.1},\"columns\":[{{\"x\":{:.1},\"width\":{:.1}}}],\"layoutMarkEvidence\":{}}}",
            page_num,
            page_num + 1,
            layout.width_px(),
            layout.height_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            body_x,
            body_width,
            mark_evidence_json
        ))
    }

    pub fn get_page_info_native(&self, page_num: u32) -> Result<String> {
        self.get_page_info(page_num)
    }

    pub fn get_page_layer_tree(&self, page_num: u32) -> Result<String> {
        self.get_page_layer_tree_with_profile(page_num, "screen")
    }

    pub fn get_page_layer_tree_native(&self, page_num: u32) -> Result<String> {
        self.get_page_layer_tree(page_num)
    }

    pub fn get_page_layer_tree_with_profile(&self, page_num: u32, profile: &str) -> Result<String> {
        let lines = self.page_lines(page_num)?;
        let profile = if profile.is_empty() {
            "screen"
        } else {
            profile
        };
        Ok(page_layer_tree_json(self, lines, profile, page_num))
    }

    pub fn get_page_layer_tree_with_profile_native(
        &self,
        page_num: u32,
        profile: &str,
    ) -> Result<String> {
        self.get_page_layer_tree_with_profile(page_num, profile)
    }

    pub fn get_page_overlay_images(&self, page_num: u32) -> Result<String> {
        self.page_lines(page_num)?;
        Ok(page_overlay_images_json(self))
    }

    pub fn get_page_overlay_images_native(&self, page_num: u32) -> Result<String> {
        self.get_page_overlay_images(page_num)
    }

    pub fn get_canvaskit_replay_plan(&self, page_num: u32, mode: &str) -> Result<String> {
        let lines = self.page_lines(page_num)?;
        let mode = canvaskit_replay_mode(mode)?;
        Ok(canvaskit_replay_plan_json(self, lines, mode))
    }

    pub fn get_canvaskit_replay_plan_native(&self, page_num: u32, mode: &str) -> Result<String> {
        self.get_canvaskit_replay_plan(page_num, mode)
    }

    pub fn convert_to_editable(&mut self) -> String {
        "{\"ok\":true,\"converted\":false}".to_string()
    }

    pub fn convert_to_editable_native(&mut self) -> String {
        self.convert_to_editable()
    }

    pub fn refresh_layout(&mut self) {
        self.refresh_pages();
    }

    pub fn get_validation_warnings(&self) -> String {
        jtd_validation_warnings_json(&jtd_validation_warnings(&self.document))
    }

    pub fn reflow_linesegs(&mut self) -> u32 {
        self.refresh_pages();
        0
    }

    pub fn get_external_image_basenames(&self) -> String {
        "[]".to_string()
    }

    pub fn inject_external_image(
        &mut self,
        _name: &str,
        _bytes: &[u8],
        _display_path: &str,
    ) -> u32 {
        0
    }

    pub fn get_page_control_layout(&self, page_num: u32) -> Result<String> {
        self.page_lines(page_num)?;
        let mut controls = Vec::new();
        for control in projected_text_controls(&self.document) {
            let Ok(rect) = self.cursor_rect_for(control.paragraph_index, control.char_offset)
            else {
                continue;
            };
            if rect.page_index != page_num as usize {
                continue;
            }
            controls.push(projected_control_layout_json(
                self.page_layout,
                &control,
                &rect,
            ));
        }
        Ok(format!("{{\"controls\":[{}]}}", controls.join(",")))
    }

    pub fn get_page_control_layout_native(&self, page_num: u32) -> Result<String> {
        self.get_page_control_layout(page_num)
    }
}
