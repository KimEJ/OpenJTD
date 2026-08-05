use super::super::*;
use crate::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in super::super) enum LocalSampleCapability {
    UsesReferenceBackedColumnGridProjection,
}

#[derive(Debug, Clone, Copy)]
pub(in super::super) struct LocalSampleFixture {
    file_name: &'static str,
    capabilities: &'static [LocalSampleCapability],
}

pub(in super::super) const SHANAI_LAN_LOCAL_SAMPLE_CAPABILITIES: &[LocalSampleCapability] =
    &[LocalSampleCapability::UsesReferenceBackedColumnGridProjection];

pub(in super::super) const LOCAL_SAMPLE_FIXTURES: &[LocalSampleFixture] = &[LocalSampleFixture {
    file_name: "ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd",
    capabilities: SHANAI_LAN_LOCAL_SAMPLE_CAPABILITIES,
}];

impl LocalSampleFixture {
    fn has_capability(self, capability: LocalSampleCapability) -> bool {
        self.capabilities.contains(&capability)
    }
}

pub(in super::super) fn local_samples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("rjtd-testdata/local-samples")
}

pub(in super::super) fn local_sample_fixture_for_path(path: &Path) -> Option<LocalSampleFixture> {
    let file_name = path.file_name().and_then(|name| name.to_str())?;
    LOCAL_SAMPLE_FIXTURES
        .iter()
        .copied()
        .find(|fixture| fixture.file_name == file_name)
}

pub(in super::super) fn local_sample_has_capability(
    path: &Path,
    capability: LocalSampleCapability,
) -> bool {
    local_sample_fixture_for_path(path)
        .map(|fixture| fixture.has_capability(capability))
        .unwrap_or(false)
}

pub(in super::super) fn assert_local_ginga_sample_facing_page_decoration(
    sample_name: &str,
    expected_page_count: Option<u32>,
) {
    let samples_dir = local_samples_dir();
    let sample_path = samples_dir.join(format!("{sample_name}.jtd"));
    let reference_pdf_path = samples_dir.join(format!("{sample_name}.pdf"));
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let bytes = fs::read(&sample_path).unwrap();
    let document = parse_document(&bytes).unwrap();
    assert!(
        document
            .auto_texts()
            .iter()
            .any(|auto_text| auto_text.text() == "銀河鉄道の夜"),
        "{sample_name} should preserve running title text from /AutoTextInfo"
    );
    assert_eq!(
        document.toc_entries().first().unwrap().page_label(),
        "6",
        "{sample_name} first body chapter should start on visible page 6"
    );
    assert!(
        !document_page_decoration_paired_slot_pairs(&document).is_empty(),
        "{sample_name} should preserve active /PageLayoutStyle paired slots"
    );
    let has_page_paper_mark_pair =
        !document.page_marks().is_empty() && !document.paper_marks().is_empty();

    let mut renamed_core = DocumentCore::from_document(document.clone());
    renamed_core.set_file_name("renamed-ginga-layout.jtd");
    assert_eq!(renamed_core.writing_mode(), WritingMode::VerticalRl);
    if let Some(expected_page_count) = expected_page_count {
        assert_eq!(
            renamed_core.page_count(),
            expected_page_count,
            "{sample_name} should keep page count without relying on its file name"
        );
    }

    let mut core = DocumentCore::from_document(document);
    core.set_file_name(sample_path.to_string_lossy());
    if let Some(expected_page_count) = expected_page_count {
        assert_eq!(
            core.page_count(),
            expected_page_count,
            "{sample_name} should match the local reference PDF page count"
        );
    }
    assert!(
        core.page_count() >= 7,
        "{sample_name} needs enough pages for odd/even decoration checks"
    );

    let page_six = core.render_page_svg(5).unwrap();
    assert!(page_six.contains("class=\"rjtd-page-number\""));
    assert!(page_six.contains("data-side=\"left\""));
    assert!(page_six.contains(">6</text>"));
    assert!(page_six.contains("一、午后の授業"));

    let page_six_layer_tree = core.get_page_layer_tree(5).unwrap();
    assert_json_brackets_balanced(&page_six_layer_tree);
    assert!(page_six_layer_tree.contains("\"type\":\"pageDecoration\""));
    assert!(page_six_layer_tree.contains("\"sidePolicy\":\"facing-pages-odd-right-even-left\""));
    assert!(page_six_layer_tree.contains("\"sidePolicyDecoded\":false"));
    assert!(page_six_layer_tree.contains("\"facingPagesCandidate\":true"));
    assert!(
        page_six_layer_tree.contains(
            "\"pairedSlotPairs\":[\"0x32/0x33\",\"0x34/0x35\",\"0x36/0x37\",\"0x38/0x39\"]"
        )
    );
    assert!(page_six_layer_tree.contains("\"side\":\"left\""));
    assert!(page_six_layer_tree.contains("\"pageNumber\":6"));
    assert!(page_six_layer_tree.contains("\"headerText\":\"一、午后の授業\""));
    if has_page_paper_mark_pair {
        assert!(
            page_six_layer_tree
                .contains("\"layoutMarkEvidence\":{\"source\":\"/PageMark+/PaperMark\"")
        );
        assert!(page_six_layer_tree.contains("\"pageMarkEntryIndex\":5"));
        assert!(page_six_layer_tree.contains("\"paperMarkEntryIndex\":5"));
        assert!(page_six_layer_tree.contains("\"rowIndexAligned\":true"));
        assert!(page_six_layer_tree.contains("\"markIndexAligned\":true"));
        assert!(page_six_layer_tree.contains("\"entryCountAligned\":true"));
        assert!(
            page_six_layer_tree.contains(
                "\"renderPromotionBlockedReason\":\"paper-mark-flag-semantics-undecoded\""
            )
        );
    }

    let page_seven = core.render_page_svg(6).unwrap();
    assert!(page_seven.contains("class=\"rjtd-page-number\""));
    assert!(page_seven.contains("data-side=\"right\""));
    assert!(page_seven.contains(">7</text>"));
    assert!(page_seven.contains("銀河鉄道の夜"));

    let page_seven_layer_tree = core.get_page_layer_tree(6).unwrap();
    assert_json_brackets_balanced(&page_seven_layer_tree);
    assert!(page_seven_layer_tree.contains("\"type\":\"pageDecoration\""));
    assert!(page_seven_layer_tree.contains("\"side\":\"right\""));
    assert!(page_seven_layer_tree.contains("\"pageNumber\":7"));
    assert!(page_seven_layer_tree.contains("\"headerText\":\"銀河鉄道の夜\""));
}
