use super::super::*;
use crate::*;

pub(in super::super) fn assert_embedding_frame_and_fdm_vector_candidates(document: &Document) {
    assert_eq!(document.object_embedding_frames().len(), 6);
    let first_art = document
        .object_embedding_frames()
        .iter()
        .find(|frame| frame.embedding_index() == 24)
        .expect("embedding 24 should be preserved from /EmbedItems/EmbeddingInfo");
    assert_eq!(first_art.source_path(), EMBEDDING_INFO_PATH);
    assert_eq!(first_art.class_name(), "JSFart.Art.2");
    assert_eq!(first_art.primary_width(), 13260);
    assert_eq!(first_art.primary_height(), 1327);
    assert_eq!(first_art.frame_ref(), 1);
    assert_eq!(first_art.frame_width(), 13260);
    assert_eq!(first_art.frame_height(), 1327);
    assert_eq!(
        success_data_test_title_art_frame_refs(document),
        vec![1, 16]
    );
    let title_pages = embedding_frame_diagnostics(document)
        .into_iter()
        .filter_map(|diagnostic| {
            success_data_test_title_art_page_number(document, diagnostic)
                .map(|page_number| (diagnostic.frame.frame_ref(), page_number))
        })
        .collect::<Vec<_>>();
    assert_eq!(title_pages, vec![(1, 1), (16, 2)]);

    let jseq = document
        .object_embedding_frames()
        .iter()
        .find(|frame| frame.embedding_index() == 4)
        .expect("JSEQ formula/document embedding should be preserved");
    assert_eq!(jseq.class_name(), "JSEQ.Document.3");
    assert_eq!(jseq.frame_ref(), 2);
    assert_eq!(jseq.frame_width(), 2590);
    assert_eq!(jseq.frame_height(), 460);

    let snapshot_candidates = document
        .object_stream_candidates()
        .iter()
        .filter(|candidate| candidate.embedded_press_snapshot_candidate().is_some())
        .collect::<Vec<_>>();
    assert_eq!(snapshot_candidates.len(), 6);
    let mut state_82_family_values = BTreeSet::new();
    let mut state_82_color_values = BTreeSet::new();
    let mut state_46_values = BTreeSet::new();
    let mut state_60_selector_values = BTreeSet::new();
    let mut state_65_selector_values = BTreeSet::new();
    for snapshot in snapshot_candidates
        .iter()
        .filter_map(|candidate| candidate.embedded_press_snapshot_candidate())
    {
        for path in snapshot.vector_paths() {
            for record in path.state_records() {
                let words = record.payload_le32_words();
                match record.record_type() {
                    0x82 => {
                        assert_eq!(words.get(3), Some(&0x00ff_ffff));
                        state_82_color_values.insert(words[3]);
                        state_82_family_values.insert(words[5]);
                    }
                    0x46 => {
                        assert_eq!(words.len(), 1);
                        assert!(matches!(words[0], 0 | 1));
                        state_46_values.insert(words[0]);
                    }
                    0x60 => {
                        assert_eq!(words.len(), 2);
                        assert!(matches!(words[0], 0x03 | 0x10 | 0x11));
                        assert_eq!(words[1], 0);
                        state_60_selector_values.insert(words[0]);
                    }
                    0x65 => {
                        assert_eq!(words.len(), 1);
                        assert!(matches!(words[0], 0x10 | 0x11));
                        state_65_selector_values.insert(words[0]);
                    }
                    _ => {}
                }
            }
        }
    }
    assert_eq!(
        state_82_color_values.into_iter().collect::<Vec<_>>(),
        vec![0x00ff_ffff]
    );
    assert_eq!(
        state_82_family_values.into_iter().collect::<Vec<_>>(),
        vec![0x10, 0x2f]
    );
    assert_eq!(state_46_values.into_iter().collect::<Vec<_>>(), vec![0, 1]);
    assert_eq!(
        state_60_selector_values.into_iter().collect::<Vec<_>>(),
        vec![0x03, 0x10, 0x11]
    );
    assert_eq!(
        state_65_selector_values.into_iter().collect::<Vec<_>>(),
        vec![0x10, 0x11]
    );
    let emb24_snapshot = snapshot_candidates
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 24/\x03EmbeddedPress")
        .and_then(|candidate| candidate.embedded_press_snapshot_candidate())
        .expect("Embedding 24 snapshot should expose JSSnapShot32 metadata");
    assert_eq!(emb24_snapshot.width(), 13260);
    assert_eq!(emb24_snapshot.height(), 1327);
    assert_eq!(emb24_snapshot.body_length_candidate(), 113332);
    assert_eq!(emb24_snapshot.vector_segments().len(), 9895);
    assert_eq!(emb24_snapshot.vector_paths().len(), 552);
    assert_eq!(
        embedded_press_snapshot_vector_path_kind_count(
            emb24_snapshot,
            ObjectEmbeddedPressVectorPathKind::Outline
        ),
        22
    );
    assert_eq!(
        embedded_press_snapshot_vector_path_kind_count(
            emb24_snapshot,
            ObjectEmbeddedPressVectorPathKind::Texture
        ),
        530
    );
    assert_eq!(
        embedded_press_snapshot_vector_path_state_record_count(emb24_snapshot),
        338
    );
    let state_record_counts = embedded_press_snapshot_state_record_type_counts(emb24_snapshot);
    assert_eq!(state_record_counts.get(&0x40), Some(&33));
    assert_eq!(state_record_counts.get(&0x46), Some(&76));
    assert_eq!(state_record_counts.get(&0x48), Some(&33));
    assert_eq!(state_record_counts.get(&0x60), Some(&66));
    assert_eq!(state_record_counts.get(&0x65), Some(&64));
    assert_eq!(state_record_counts.get(&0x70), Some(&33));
    assert_eq!(state_record_counts.get(&0x82), Some(&33));
    let first_shadow_outline = &emb24_snapshot.vector_paths()[0];
    assert_eq!(
        first_shadow_outline.kind(),
        ObjectEmbeddedPressVectorPathKind::Outline
    );
    assert_eq!(
        first_shadow_outline
            .state_records()
            .iter()
            .map(ObjectEmbeddedPressStateRecordCandidate::record_type)
            .collect::<Vec<_>>(),
        vec![0x40, 0x48, 0x46, 0x82, 0x70, 0x46, 0x60, 0x60]
    );
    assert_eq!(
        first_shadow_outline.state_records()[3].payload_le32_words()[5],
        0x2f
    );
    assert_eq!(
        embedded_press_state_record_payload_first_words(first_shadow_outline, 0x46),
        vec![0, 1]
    );
    let first_texture_path = &emb24_snapshot.vector_paths()[11];
    assert_eq!(
        first_texture_path.kind(),
        ObjectEmbeddedPressVectorPathKind::Texture
    );
    assert_eq!(
        first_texture_path.state_records()[6].payload_le32_words()[5],
        0x2f
    );
    assert_eq!(
        embedded_press_state_record_payload_first_words(first_texture_path, 0x46),
        vec![0, 0]
    );
    let later_texture_path = &emb24_snapshot.vector_paths()[27];
    assert_eq!(
        later_texture_path.kind(),
        ObjectEmbeddedPressVectorPathKind::Texture
    );
    assert_eq!(
        later_texture_path.state_records()[5].payload_le32_words()[5],
        0x2f
    );
    assert_eq!(
        embedded_press_state_record_payload_first_words(later_texture_path, 0x46),
        vec![0]
    );
    let first_main_outline = &emb24_snapshot.vector_paths()[541];
    assert_eq!(
        first_main_outline.kind(),
        ObjectEmbeddedPressVectorPathKind::Outline
    );
    assert_eq!(
        first_main_outline.state_records()[5].payload_le32_words()[5],
        0x10
    );
    assert_eq!(
        embedded_press_state_record_payload_first_words(first_main_outline, 0x46),
        vec![0, 1]
    );
    let texture_headers = emb24_snapshot
        .vector_paths()
        .iter()
        .filter_map(ObjectEmbeddedPressVectorPathCandidate::texture_bezier_header)
        .collect::<Vec<_>>();
    assert_eq!(texture_headers.len(), 530);
    assert!(texture_headers.iter().all(|header| {
        header.point_count() == 13 && header.byte_count() == 104 && header.flags() == 1
    }));
    assert!(success_data_test_title_art_rendered_segment_count(emb24_snapshot) > 0);
    assert_eq!(
        success_data_test_title_art_rendered_path_count(emb24_snapshot),
        22
    );
    assert_eq!(
        success_data_test_title_art_shadow_path_count(emb24_snapshot),
        11
    );
    let title_partition = embedded_press_title_art_shadow_path_partition(emb24_snapshot)
        .expect("title art outlines should be partitioned from EmbeddedPress paint state");
    assert_eq!(
        title_partition.strategy,
        "embedded-press-source-order-outline-texture-outline"
    );
    assert_eq!(title_partition.shadow_paths.len(), 11);
    assert_eq!(title_partition.main_paths.len(), 11);
    assert_eq!(title_partition.offset, (100, 100));
    let interstitial_texture_paths =
        success_data_test_title_art_interstitial_texture_paths(emb24_snapshot, &title_partition)
            .expect("title art texture block should sit between shadow and main outlines");
    assert_eq!(interstitial_texture_paths.len(), 530);
    let state_tagged_texture_paths =
        success_data_test_title_art_state_tagged_texture_paths(emb24_snapshot);
    assert_eq!(state_tagged_texture_paths.len(), 11);
    assert_eq!(
        embedded_press_vector_path_state_word5_values(&state_tagged_texture_paths),
        vec![EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5]
    );
    let shadow_texture_paths = success_data_test_title_art_shadow_texture_paths(emb24_snapshot);
    assert_eq!(shadow_texture_paths.len(), 11);
    assert_eq!(
        embedded_press_vector_path_state_word5_values(&shadow_texture_paths),
        vec![EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5]
    );
    let effective_shadow_texture_paths =
        success_data_test_title_art_effective_texture_paths_for_word5(
            emb24_snapshot,
            EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5,
        );
    assert_eq!(effective_shadow_texture_paths.len(), 530);
    let effective_front_texture_paths =
        success_data_test_title_art_effective_texture_paths_for_word5(
            emb24_snapshot,
            EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5,
        );
    let front_texture_paths = success_data_test_title_art_front_texture_paths(emb24_snapshot);
    assert_eq!(
        front_texture_paths.len(),
        effective_front_texture_paths.len(),
        "front texture selection must use inherited current paint-state ownership"
    );
    assert!(
        front_texture_paths.is_empty(),
        "shadow-state texture paths must not be replayed over the main title face"
    );
    assert_eq!(
        success_data_test_title_art_effective_texture_word5_values(emb24_snapshot),
        vec![EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5]
    );
    let emb4_snapshot = snapshot_candidates
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 4/\x03EmbeddedPress")
        .and_then(|candidate| candidate.embedded_press_snapshot_candidate())
        .expect("Embedding 4 snapshot should expose JSSnapShot32 metadata");
    assert_eq!(emb4_snapshot.width(), 2590);
    assert_eq!(emb4_snapshot.height(), 460);
    assert_eq!(emb4_snapshot.vector_segments().len(), 51);

    let jseq_candidates = document
        .object_stream_candidates()
        .iter()
        .filter(|candidate| candidate.jseq3_formula_candidate().is_some())
        .collect::<Vec<_>>();
    assert_eq!(jseq_candidates.len(), 4);
    let emb4_formula = jseq_candidates
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 4/JSEQ3Contents")
        .and_then(|candidate| candidate.jseq3_formula_candidate())
        .expect("Embedding 4 JSEQ3Contents should expose MATH.VAF metadata");
    assert_eq!(emb4_formula.magic(), "MATH.VAF");
    assert_eq!(emb4_formula.magic_offset(), 0);
    assert_eq!(emb4_formula.so_trailer_offset(), Some(1658));
    assert_eq!(emb4_formula.so_trailer_length(), Some(62));
    assert_eq!(emb4_formula.so_trailer_fields()[0], 0x0000_4f53);
    assert_eq!(emb4_formula.so_trailer_fields()[1], 0x200e_0a20);
    assert!(
        emb4_formula
            .text_markers()
            .iter()
            .any(|marker| marker.text() == "Times New Roman" && marker.offset() == 892)
    );
    assert_eq!(emb4_formula.text_tokens().len(), 4);
    assert_eq!(emb4_formula.text_tokens()[0].text(), "１");
    assert_eq!(emb4_formula.text_tokens()[1].text(), "２");
    assert_eq!(emb4_formula.text_tokens()[2].text(), "÷");
    assert_eq!(emb4_formula.text_tokens()[3].text(), "３");
    assert_eq!(emb4_formula.text_runs().len(), 3);
    assert_eq!(emb4_formula.text_runs()[0].text(), "１２");
    assert_eq!(emb4_formula.text_runs()[0].start_offset(), 556);
    assert_eq!(emb4_formula.text_runs()[0].end_offset(), 586);
    assert_eq!(emb4_formula.text_runs()[0].token_offsets(), &[556, 584]);
    assert_eq!(emb4_formula.text_runs()[0].context_start_offset(), 492);
    assert_eq!(emb4_formula.text_runs()[0].context_fields_le32()[14], 296);
    assert_eq!(emb4_formula.text_runs()[0].context_fields_le32()[15], 148);
    assert_eq!(emb4_formula.text_runs()[1].text(), "÷");
    assert_eq!(emb4_formula.text_runs()[2].text(), "３");

    let title_jsfart_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 24/JSFart2Contents")
        .expect("Embedding 24 JSFart2Contents should be preserved as object evidence");
    let title_jsfart_profile = title_jsfart_candidate
        .jsfart_stream_profile_candidate()
        .expect("Embedding 24 JSFart2Contents should expose a source stream profile");
    assert_eq!(title_jsfart_profile.magic_family(), "mstudio-ocx-utf16le");
    assert_eq!(title_jsfart_profile.magic_family_hex(), "4d00");
    assert_eq!(
        title_jsfart_profile.magic_ascii_or_utf16_preview(),
        "MSTUDIO."
    );
    assert!(title_jsfart_profile.structured_art_candidate_present());
    assert_eq!(
        title_jsfart_profile.render_promotion_blocked_reason(),
        "structured-jsfart-art-still-paint-authority-unproven"
    );
    let title_jsfart_art = title_jsfart_candidate
        .jsfart_art_candidate()
        .expect("Embedding 24 JSFart2Contents should expose title art metadata");
    assert_eq!(title_jsfart_art.magic(), "MSTUDIO.OCX");
    assert_eq!(title_jsfart_art.width(), 13260);
    assert_eq!(title_jsfart_art.height(), 1327);
    let title_frame = title_jsfart_art
        .frame_candidate()
        .expect("JSFart2Contents should expose title frame geometry");
    assert_eq!(title_frame.left(), 0);
    assert_eq!(title_frame.top(), 0);
    assert_eq!(title_frame.right(), 13260);
    assert_eq!(title_frame.bottom(), 1327);
    assert_eq!(title_frame.content_left(), 114);
    assert_eq!(title_frame.content_top(), 105);
    assert_eq!(title_frame.content_right(), 13145);
    assert_eq!(title_frame.content_bottom(), 1159);
    assert_eq!(title_frame.corner_radius_x(), 114);
    assert_eq!(title_frame.corner_radius_y(), 105);
    assert_eq!(title_frame.stroke_width_candidate(), Some(100));
    let title_paint = title_jsfart_art
        .paint_candidate()
        .expect("JSFart2Contents should expose observed paint/style header fields");
    assert_eq!(title_paint.style_word_1(), 0x0214_1030);
    assert_eq!(title_paint.style_word_2(), 0x0214_1018);
    assert_eq!(title_paint.paint_color_candidate(), 0x00ff_ffff);
    assert_eq!(title_paint.paint_flag_candidate(), 1);
    assert_eq!(title_paint.effect_word_candidate(), 0x0000_000a);

    let fdm_text = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMText")
        .expect("success_data test should expose FDMText labels");
    assert_eq!(fdm_text.fdm_text_candidates().len(), 15);
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "９㎝"
                && candidate.marker_offset() == 0
                && candidate.text_offset() == 0x00c4
                && candidate.raw_text() == [0x82, 0x58, 0x87, 0x70]
                && candidate.bbox()
                    == Some(ObjectFdmIndexBbox::new(-12004, -11540, -11692, -11336)))
    );
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "３㎝"
                && candidate.marker_offset() == 204
                && candidate.text_offset() == 0x0190
                && candidate.raw_text() == [0x82, 0x52, 0x87, 0x70]
                && candidate.bbox()
                    == Some(ObjectFdmIndexBbox::new(-11200, -10738, -10888, -10534)))
    );
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "110°"
                && candidate.marker_offset() == 2054
                && candidate.bbox() == Some(ObjectFdmIndexBbox::new(-15254, -9453, -14961, -9284)))
    );
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "30°"
                && candidate.bbox() == Some(ObjectFdmIndexBbox::new(-13488, -9410, -13251, -9241)))
    );
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "160°"
                && candidate.bbox() == Some(ObjectFdmIndexBbox::new(-11278, -9823, -10985, -9654)))
    );
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "Ａ"
                && candidate.marker_offset() == 2474
                && candidate.bbox()
                    == Some(ObjectFdmIndexBbox::new(-13494, -12097, -13380, -11928)))
    );
    let expanded_fdm_text = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/ExpandData/main_data/Data/FDMText")
        .expect("success_data answer sheet should expose expanded FDMText labels");
    assert_eq!(expanded_fdm_text.fdm_text_candidates().len(), 15);
    assert_eq!(
        expanded_fdm_text.fdm_text_index_entry_candidates().len(),
        15
    );
    assert!(
        expanded_fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "９㎝"
                && candidate.marker_offset() == 0
                && candidate.text_offset() == 0x00c6
                && candidate.raw_text() == [0xff, 0x19, 0x33, 0x9d]
                && candidate.bbox()
                    == Some(ObjectFdmIndexBbox::new(-12004, -11540, -11692, -11336)))
    );
    assert!(
        expanded_fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "110°"
                && candidate.marker_offset() == 0x1566
                && candidate.text_offset() == 0x1638
                && candidate.raw_text() == [0x00, 0x31, 0x00, 0x31, 0x00, 0x30, 0x00, 0xb0])
    );
    assert!(
        expanded_fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "Ａ"
                && candidate.marker_offset() == 0x19b8
                && candidate.raw_text() == [0xff, 0x21])
    );
    let first_text_index = &expanded_fdm_text.fdm_text_index_entry_candidates()[0];
    assert_eq!(first_text_index.index_offset(), 26);
    assert_eq!(first_text_index.text_record_offset(), 0);
    assert_eq!(first_text_index.kind(), 0x1600);
    assert_eq!(
        first_text_index.bbox(),
        ObjectFdmIndexBbox::new(-12104, -11592, -11640, -11236)
    );
    assert_eq!(
        first_text_index.text_record_bbox(),
        Some(ObjectFdmIndexBbox::new(-12004, -11540, -11692, -11336))
    );
    let last_text_index = expanded_fdm_text
        .fdm_text_index_entry_candidates()
        .last()
        .expect("expanded FDMText index must preserve the last text record link");
    assert_eq!(last_text_index.index_offset(), 334);
    assert_eq!(last_text_index.text_record_offset(), 0x1df8);
    assert_eq!(
        last_text_index.text_record_bbox(),
        Some(ObjectFdmIndexBbox::new(-15918, -10511, -15804, -10342))
    );

    let fdm_vector = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMVector")
        .expect("success_data test should expose FDMVector commands");
    assert_eq!(fdm_vector.fdm_raw_vector_segments().len(), 5);
    let first_fdm_segment = &fdm_vector.fdm_raw_vector_segments()[0];
    assert_eq!(first_fdm_segment.relative_offset(), 0);
    assert_eq!(first_fdm_segment.declared_len(), 176);
    assert_eq!(first_fdm_segment.command_count(), 2);
    assert_eq!(first_fdm_segment.command_offsets(), &[56, 90]);
    assert_eq!(
        first_fdm_segment.bbox(),
        Some(ObjectFdmIndexBbox::new(-11840, -12064, -10720, -10510))
    );
    assert_eq!(first_fdm_segment.source_width(), 1120);
    assert_eq!(first_fdm_segment.source_height(), 1554);
    assert!(
        fdm_vector
            .fdm_raw_vector_segments()
            .iter()
            .any(|segment| segment.relative_offset() == 1864
                && segment.declared_len() == 236
                && segment.command_count() == 4
                && segment.command_offsets() == [60, 94, 128, 160])
    );
    assert_eq!(fdm_vector.fdm_raw_vector_commands().len(), 37);
    assert!(
        fdm_vector
            .fdm_raw_vector_commands()
            .iter()
            .any(|command| command.marker() == b"\x01\x00\x04\x60")
    );
    assert!(
        fdm_vector
            .fdm_raw_vector_commands()
            .iter()
            .filter_map(fdm_vector_command_source_bbox)
            .map(normalize_fdm_bbox)
            .any(|bbox| bbox == (-15784, -10213, -14584, -9013))
    );
    let q3_dashed_curve = fdm_vector
        .fdm_raw_vector_commands()
        .iter()
        .find(|command| command.relative_offset() == 208)
        .expect("Q3 dashed guide curve command should be preserved from FDMVector");
    assert_eq!(q3_dashed_curve.marker(), b"\x01\x00\x09\x60");
    assert_eq!(q3_dashed_curve.style_word(), 0x0120);
    assert_eq!(q3_dashed_curve.path_points().len(), 2);
    assert_eq!(q3_dashed_curve.curve_segments().len(), 1);
    assert_eq!(q3_dashed_curve.source_vector_relative_offset(), Some(208));
    assert_eq!(q3_dashed_curve.source_segment(), None);
    let q5_segment_polyline = fdm_vector
        .fdm_raw_vector_commands()
        .iter()
        .find(|command| command.relative_offset() == 1992)
        .expect("Q5 segment-backed polyline command should preserve segment provenance");
    assert_eq!(
        q5_segment_polyline.source_vector_relative_offset(),
        Some(1992)
    );
    let q5_source_segment = q5_segment_polyline
        .source_segment()
        .expect("Q5 polyline should be linked to its FDMVector segment header");
    assert_eq!(q5_source_segment.relative_offset(), 1864);
    assert_eq!(q5_source_segment.local_offset(), 128);
    assert_eq!(q5_source_segment.declared_len(), 236);
    assert_eq!(q5_source_segment.command_count(), 4);
    assert_eq!(q5_source_segment.command_index(), 2);
    assert_eq!(q5_source_segment.command_offset(), 128);
}
