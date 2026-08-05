use super::*;
use crate::json::fdm::{fdm_bbox_center, push_json_string_slice_array};
use crate::json::primitives::push_json_string;
#[cfg(not(target_arch = "wasm32"))]
use crate::pdf::convert::svgs_to_pdf;
#[cfg(not(target_arch = "wasm32"))]
use crate::pdf::safety::{
    find_subslice, pdf_contains_token_sequence, pdf_preview_blocking_issues,
    pdf_preview_safety_issues, pdf_skip_whitespace, scrub_embedded_pdf_eof_markers,
};
use rjtd_core::record::UnknownRecordKind;
use rjtd_model::{
    Block, Document, Inline, Metadata, ObjectImageDeclaredLengthCandidate,
    ObjectImagePayloadEnvelope, ObjectImagePayloadLocation, ObjectImagePayloadSpan,
    ObjectImageSignatureHit, ObjectStreamCandidate, ObjectStreamCandidateEvidence,
    ObjectStreamCandidateReason, Paragraph, RawStream, RubyAnnotation, StyleRef,
    TextControlBoundary, TextRun, TextSourceSpan, UnknownBlock, UnknownObject, UnknownStyle,
    parse_document,
};
use std::{collections::BTreeSet, fs, path::PathBuf};
#[cfg(target_os = "macos")]
use std::{path::Path, process::Command};

#[cfg(not(target_arch = "wasm32"))]
fn count_pdf_eof_markers(pdf: &[u8]) -> usize {
    pdf.windows(b"%%EOF".len())
        .filter(|window| *window == b"%%EOF")
        .count()
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy)]
struct PdfMediaBox {
    width: f32,
    height: f32,
}

#[cfg(not(target_arch = "wasm32"))]
impl PdfMediaBox {
    fn close_to(self, other: Self) -> bool {
        const MEDIA_BOX_TOLERANCE_PT: f32 = 1.0;
        (self.width - other.width).abs() <= MEDIA_BOX_TOLERANCE_PT
            && (self.height - other.height).abs() <= MEDIA_BOX_TOLERANCE_PT
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy)]
struct LocalReferencePdfKnownDivergence {
    expected_reference_page_count: usize,
    expected_output_page_count: usize,
    page_count_reason: &'static str,
    media_box_divergence: Option<LocalReferencePdfKnownMediaBoxDivergence>,
}

#[cfg(not(target_arch = "wasm32"))]
impl LocalReferencePdfKnownDivergence {
    const fn pagination(
        expected_reference_page_count: usize,
        expected_output_page_count: usize,
    ) -> Self {
        Self {
            expected_reference_page_count,
            expected_output_page_count,
            page_count_reason: LOCAL_REFERENCE_FALLBACK_PAGINATION_DIVERGES_FROM_REFERENCE,
            media_box_divergence: None,
        }
    }

    const fn pagination_with_media_box(
        expected_reference_page_count: usize,
        expected_output_page_count: usize,
        media_box_divergence: LocalReferencePdfKnownMediaBoxDivergence,
    ) -> Self {
        Self {
            expected_reference_page_count,
            expected_output_page_count,
            page_count_reason: LOCAL_REFERENCE_FALLBACK_PAGINATION_DIVERGES_FROM_REFERENCE,
            media_box_divergence: Some(media_box_divergence),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy)]
struct LocalReferencePdfKnownMediaBoxDivergence {
    expected_reference_media_box: PdfMediaBox,
    expected_output_media_box: PdfMediaBox,
    reason: &'static str,
}

#[cfg(not(target_arch = "wasm32"))]
const LOCAL_REFERENCE_FALLBACK_PAGINATION_DIVERGES_FROM_REFERENCE: &str =
    "fallback-pagination-diverges-from-reference";
#[cfg(not(target_arch = "wasm32"))]
const LOCAL_REFERENCE_PAPER_ORIENTATION_SOURCE_DECODE_UNPROVEN: &str =
    "paper-orientation-source-decode-unproven";

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
#[derive(Debug, Clone, Copy)]
struct PngRatioRegionCheck {
    label: &'static str,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    min_non_white: usize,
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
#[derive(Debug, Clone, Copy)]
struct LocalPdfSmokeFixture {
    source_name: &'static str,
    output_pdf_name: &'static str,
    page_checks: &'static [&'static str],
    sips_region_check: Option<PngRatioRegionCheck>,
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
impl LocalPdfSmokeFixture {
    fn source_path(self, sample_dir: &Path) -> PathBuf {
        sample_dir.join(self.source_name)
    }

    fn output_pdf_path(self, output_dir: &Path) -> PathBuf {
        output_dir.join(self.output_pdf_name)
    }

    fn source_with_reference_pdf_exists(self, sample_dir: &Path) -> bool {
        let sample_path = self.source_path(sample_dir);
        sample_path.exists() && sample_path.with_extension("pdf").exists()
    }
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
const SUCCESS_DATA_TEST_PAGE_CHECKS: &[&str] = &["1:10000", "2:500"];
#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
const SHANAI_LAN_PAGE_CHECKS: &[&str] = &["1:5000"];
#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
const A5_PAGE_CHECKS: &[&str] = &["1:300", "6:3000"];
#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
const FAX02_PAGE_CHECKS: &[&str] = &["1:10000"];

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
const LOCAL_PDF_SMOKE_FIXTURES: &[LocalPdfSmokeFixture] = &[
    LocalPdfSmokeFixture {
        source_name: "ichitaro-20030228030923-success-002-success_data-test.jtd",
        output_pdf_name: "ichitaro-20030228030923-success-002-success_data-test.pdf",
        page_checks: SUCCESS_DATA_TEST_PAGE_CHECKS,
        sips_region_check: Some(PngRatioRegionCheck {
            label: "title region",
            left: 0.05,
            top: 0.07,
            right: 0.92,
            bottom: 0.20,
            min_non_white: 3_000,
        }),
    },
    LocalPdfSmokeFixture {
        source_name: "ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd",
        output_pdf_name: "ichitaro-20030315134715-success-001-success_data-shanai_lan.pdf",
        page_checks: SHANAI_LAN_PAGE_CHECKS,
        sips_region_check: None,
    },
    LocalPdfSmokeFixture {
        source_name: "a5.jtd",
        output_pdf_name: "a5.pdf",
        page_checks: A5_PAGE_CHECKS,
        sips_region_check: None,
    },
    LocalPdfSmokeFixture {
        source_name: "fax02.jtt",
        output_pdf_name: "fax02.pdf",
        page_checks: FAX02_PAGE_CHECKS,
        sips_region_check: None,
    },
];

#[cfg(not(target_arch = "wasm32"))]
fn local_sample_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("rjtd-testdata/local-samples")
}

fn test_json_string(value: &str) -> String {
    let mut output = String::new();
    push_json_string(&mut output, value);
    output
}

fn test_json_string_array(values: &[&str]) -> String {
    let mut output = String::new();
    push_json_string_slice_array(&mut output, values);
    output
}

fn tail_after_occurrence<'a>(haystack: &'a str, marker: &str, occurrence: usize) -> &'a str {
    let mut tail = haystack;
    for index in 0..=occurrence {
        let Some((_, next_tail)) = tail.split_once(marker) else {
            panic!("missing JSON marker occurrence {index} for {marker}");
        };
        tail = next_tail;
    }
    tail
}

fn assert_json_string_field_after(
    haystack: &str,
    marker: &str,
    occurrence: usize,
    field: &str,
    expected: &str,
) {
    let fragment = format!("\"{field}\":{}", test_json_string(expected));
    let tail = tail_after_occurrence(haystack, marker, occurrence);
    assert!(
        tail.contains(&fragment),
        "missing JSON field {field}={expected:?} after marker {marker}"
    );
}

fn assert_json_number_field_after(
    haystack: &str,
    marker: &str,
    occurrence: usize,
    field: &str,
    expected: &str,
) {
    let fragment = format!("\"{field}\":{expected}");
    let tail = tail_after_occurrence(haystack, marker, occurrence);
    assert!(
        tail.contains(&fragment),
        "missing JSON field {field}={expected} after marker {marker}"
    );
}

fn assert_json_bool_field_after(
    haystack: &str,
    marker: &str,
    occurrence: usize,
    field: &str,
    expected: bool,
) {
    let fragment = format!("\"{field}\":{}", if expected { "true" } else { "false" });
    let tail = tail_after_occurrence(haystack, marker, occurrence);
    assert!(
        tail.contains(&fragment),
        "missing JSON field {field}={expected} after marker {marker}"
    );
}

fn assert_json_string_array_field_after(
    haystack: &str,
    marker: &str,
    occurrence: usize,
    field: &str,
    expected: &[&str],
) {
    let fragment = format!("\"{field}\":{}", test_json_string_array(expected));
    let tail = tail_after_occurrence(haystack, marker, occurrence);
    assert!(
        tail.contains(&fragment),
        "missing JSON string array field {field}={expected:?} after marker {marker}"
    );
}

#[test]
fn fdm_bbox_center_handles_extreme_bounds_without_overflow() {
    assert_eq!(
        fdm_bbox_center((i32::MIN, i32::MIN, i32::MAX, i32::MAX)),
        (-1, -1)
    );
    assert_eq!(fdm_bbox_center((-3, -3, -2, -2)), (-3, -3));
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_media_box_sizes(pdf: &[u8]) -> Vec<PdfMediaBox> {
    let mut sizes = Vec::new();
    let mut position = 0usize;
    while let Some(relative_offset) = find_subslice(&pdf[position..], b"/MediaBox") {
        let media_box_offset = position + relative_offset;
        let mut cursor = pdf_skip_whitespace(pdf, media_box_offset + b"/MediaBox".len());
        if !pdf.get(cursor..).is_some_and(|tail| tail.starts_with(b"[")) {
            position = media_box_offset + b"/MediaBox".len();
            continue;
        }
        cursor += 1;

        let Some(x0) = pdf_parse_number(pdf, &mut cursor) else {
            position = media_box_offset + b"/MediaBox".len();
            continue;
        };
        let Some(y0) = pdf_parse_number(pdf, &mut cursor) else {
            position = media_box_offset + b"/MediaBox".len();
            continue;
        };
        let Some(x1) = pdf_parse_number(pdf, &mut cursor) else {
            position = media_box_offset + b"/MediaBox".len();
            continue;
        };
        let Some(y1) = pdf_parse_number(pdf, &mut cursor) else {
            position = media_box_offset + b"/MediaBox".len();
            continue;
        };
        sizes.push(PdfMediaBox {
            width: (x1 - x0).abs(),
            height: (y1 - y0).abs(),
        });
        position = cursor;
    }
    sizes
}

#[cfg(not(target_arch = "wasm32"))]
fn pdf_parse_number(bytes: &[u8], position: &mut usize) -> Option<f32> {
    *position = pdf_skip_whitespace(bytes, *position);
    let start = *position;
    if bytes
        .get(*position)
        .is_some_and(|byte| matches!(*byte, b'+' | b'-'))
    {
        *position += 1;
    }
    let mut saw_digit = false;
    while bytes.get(*position).is_some_and(|byte| {
        let numeric = byte.is_ascii_digit() || *byte == b'.';
        saw_digit |= byte.is_ascii_digit();
        numeric
    }) {
        *position += 1;
    }
    if !saw_digit {
        return None;
    }
    std::str::from_utf8(&bytes[start..*position])
        .ok()?
        .parse::<f32>()
        .ok()
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
fn png_non_white_count_in_ratio_region(
    path: &Path,
    left_ratio: f32,
    top_ratio: f32,
    right_ratio: f32,
    bottom_ratio: f32,
) -> Result<usize, String> {
    let image = image::ImageReader::open(path)
        .map_err(|error| error.to_string())?
        .decode()
        .map_err(|error| error.to_string())?
        .to_rgb8();
    let width = image.width();
    let height = image.height();
    if width == 0 || height == 0 {
        return Err("PNG image has zero size".to_string());
    }

    let left = ((width as f32) * left_ratio).floor().max(0.0) as u32;
    let top = ((height as f32) * top_ratio).floor().max(0.0) as u32;
    let right = ((width as f32) * right_ratio).ceil().min(width as f32) as u32;
    let bottom = ((height as f32) * bottom_ratio).ceil().min(height as f32) as u32;
    if left >= right || top >= bottom {
        return Err(format!(
            "invalid PNG region {left},{top}..{right},{bottom} for {width}x{height}"
        ));
    }

    let mut non_white = 0usize;
    for y in top..bottom {
        for x in left..right {
            let pixel = image.get_pixel(x, y);
            if pixel[0] < 245 || pixel[1] < 245 || pixel[2] < 245 {
                non_white += 1;
            }
        }
    }
    Ok(non_white)
}

#[test]
fn exports_markdown_from_document_model() {
    let paragraph = Paragraph::new(vec![Inline::Text(TextRun::new("hello", None))], None);
    let document = Document::new(
        Metadata::new(Some("sample".to_string())),
        vec![Block::Paragraph(paragraph)],
    );

    assert_eq!(to_markdown(&document), "hello\n\n");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn exports_pdf_from_document_model() {
    let document = Document::from_plain_text("銀河鉄道\n午后の授業");
    let pdf = to_pdf(&document).unwrap();
    let pdf_text = String::from_utf8_lossy(&pdf);

    assert!(pdf.starts_with(b"%PDF-1.4"));
    assert!(pdf.windows(5).any(|window| window == b"/Page"));
    assert!(pdf_text.contains("/MediaBox [0 0 "));
    assert!(pdf_text.contains("1 1 1 rg\n0 0 "));
    assert!(pdf_text.contains(" re\nf\nQ\nq\n"));
    assert!(pdf_text.contains("/S1 Do"));
    assert!(pdf_text.contains("/Subtype /Form"));
    assert!(pdf_text.contains("/FormType 1"));
    assert!(pdf_text.contains("/Producer (rjtd)"));
    assert!(!pdf_text.contains("/SMask"));
    assert!(pdf_preview_safety_issues(&pdf).is_empty());
    assert_eq!(count_pdf_eof_markers(&pdf), 1);
    assert!(pdf.ends_with(b"%%EOF"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn pdf_media_box_parser_extracts_page_sizes() {
    let pdf = b"1 0 obj\n<< /Type /Page\n/MediaBox [0 0 419.55 595.275]\n>>\nendobj\n2 0 obj\n<< /Type /Page /MediaBox [-10 -20 90 180] >>\nendobj\n";

    let sizes = pdf_media_box_sizes(pdf);

    assert_eq!(sizes.len(), 2);
    assert!(sizes[0].close_to(PdfMediaBox {
        width: 419.55,
        height: 595.275,
    }));
    assert!(sizes[1].close_to(PdfMediaBox {
        width: 100.0,
        height: 200.0,
    }));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn exports_pdf_does_not_apply_layout_hints_from_filename_only() {
    let document = Document::from_plain_text(&vec!["銀河鉄道の夜"; 80].join("\n"));
    let pdf = to_pdf_with_file_name(&document, "a5.jtd").unwrap();
    let pdf_text = String::from_utf8_lossy(&pdf);

    assert!(pdf.starts_with(b"%PDF-1.4"));
    assert!(pdf_text.contains("/MediaBox [0 0 595.5 842.25]"));
    assert!(pdf_text.contains("1 1 1 rg\n0 0 595.5 842.25"));
    assert!(pdf_text.contains(" re\nf\nQ\nq\n"));
    assert!(pdf_text.contains("q\n595.5 0 0 842.25 0 0 cm"));
    assert!(pdf_text.contains("/S1 Do\nQ"));
    assert!(pdf_text.contains("/FormType 1"));
    assert!(!pdf_text.contains("/Group <<"));
    assert!(!pdf_text.contains("/S /Transparency"));
    assert!(!pdf_text.contains("/SMask"));
    assert!(pdf_preview_safety_issues(&pdf).is_empty());
    assert_eq!(count_pdf_eof_markers(&pdf), 1);
    assert!(pdf.ends_with(b"%%EOF"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn embeds_svg_chunk_with_preview_safe_page_wrapper_contract() {
    let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="80" viewBox="0 0 120 80"><rect width="120" height="80" fill="#fff"/><circle cx="60" cy="40" r="24" fill="#123456"/></svg>"##;
    let pdf = svgs_to_pdf(&[svg.to_string()]).unwrap();
    let pdf_text = String::from_utf8_lossy(&pdf);

    assert!(pdf.starts_with(b"%PDF-1.4"));
    assert!(pdf_text.contains("/MediaBox [0 0 90 60]"));
    assert!(pdf_text.contains("1 1 1 rg\n0 0 90 60 re\nf\nQ\nq\n"));
    assert!(pdf_text.contains("90 0 0 60 0 0 cm\n/S1 Do\nQ"));
    assert!(pdf_text.contains("/Subtype /Form"));
    assert!(pdf_text.contains("/FormType 1"));
    assert!(pdf_text.contains("/BBox [0 0 120 80]"));
    assert!(pdf_text.contains("/Matrix [0.008333334 0 0 0.0125 0 0]"));
    assert!(!pdf_text.contains("/Group <<"));
    assert!(!pdf_text.contains("/S /Transparency"));
    assert!(!pdf_text.contains("/SMask"));
    assert!(pdf_preview_safety_issues(&pdf).is_empty());
    assert_eq!(count_pdf_eof_markers(&pdf), 1);
    assert!(pdf.ends_with(b"%%EOF"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn scrubs_embedded_cmap_eof_markers_but_keeps_file_eof() {
    let mut pdf = b"%PDF-1.4\n1 0 obj\n<< /Length 45 >>\nstream\n%%EndResource\n%%EOF\nendstream\nendobj\nstartxref\n0\n%%EOF"
            .to_vec();

    scrub_embedded_pdf_eof_markers(&mut pdf);

    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(pdf_text.contains("%%EndResource\n%%EOD\nendstream"));
    assert!(pdf.ends_with(b"%%EOF"));
    assert_eq!(count_pdf_eof_markers(&pdf), 1);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn preview_safety_scanner_catches_flexible_pdf_token_spacing() {
    let pdf = b"%PDF-1.4\n1 0 obj\n<< /Group\n  << /S\t/Transparency >> /SMask 2 0 R >>\nendobj";
    assert_eq!(
        pdf_preview_safety_issues(pdf),
        vec![
            "transparency-group-dictionary",
            "transparency-group-subtype",
            "soft-mask"
        ]
    );
    assert_eq!(
        pdf_preview_blocking_issues(pdf),
        vec![
            "transparency-group-dictionary",
            "transparency-group-subtype"
        ]
    );

    assert!(!pdf_contains_token_sequence(
        b"<< /Subtype /Form >>",
        &[b"/S"]
    ));
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
#[test]
fn local_complex_pdfs_rasterize_with_macos_sips_when_available() {
    let sample_dir = local_sample_dir();
    if !sample_dir.exists() {
        return;
    }

    let mut failures = Vec::new();
    let mut rendered_count = 0usize;

    let any_sample_present = LOCAL_PDF_SMOKE_FIXTURES
        .iter()
        .any(|fixture| fixture.source_with_reference_pdf_exists(&sample_dir));
    if !any_sample_present {
        return;
    }

    for fixture in LOCAL_PDF_SMOKE_FIXTURES {
        let sample = fixture.source_name;
        let sample_path = fixture.source_path(&sample_dir);
        if !sample_path.exists() || !sample_path.with_extension("pdf").exists() {
            continue;
        }

        let result = fs::read(&sample_path)
            .map_err(|error| error.to_string())
            .and_then(|bytes| parse_document(&bytes).map_err(|error| error.to_string()))
            .and_then(|document| to_pdf_with_file_name(&document, &sample_path.to_string_lossy()));
        let pdf = match result {
            Ok(pdf) => pdf,
            Err(error) => {
                failures.push(format!("{}: {error}", sample_path.display()));
                continue;
            }
        };

        let temp_dir =
            std::env::temp_dir().join(format!("rjtd-sips-smoke-{}-{sample}", std::process::id()));
        if let Err(error) = fs::create_dir_all(&temp_dir) {
            failures.push(format!("{}: create temp dir failed: {error}", sample));
            continue;
        }
        let pdf_path = temp_dir.join("sample.pdf");
        let png_path = temp_dir.join("sample.png");
        let module_cache_path = temp_dir.join("swift-module-cache");
        if let Err(error) = fs::create_dir_all(&module_cache_path) {
            failures.push(format!(
                "{}: create Swift module cache failed: {error}",
                sample
            ));
            let _ = fs::remove_dir_all(&temp_dir);
            continue;
        }
        if let Err(error) = fs::write(&pdf_path, &pdf) {
            failures.push(format!("{}: write temp pdf failed: {error}", sample));
            let _ = fs::remove_dir_all(&temp_dir);
            continue;
        }

        let output = match Command::new("sips")
            .arg("-s")
            .arg("format")
            .arg("png")
            .arg(&pdf_path)
            .arg("--out")
            .arg(&png_path)
            .output()
        {
            Ok(output) => output,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
            Err(error) => {
                failures.push(format!("{}: run sips failed: {error}", sample));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }
        };

        if !output.status.success() {
            failures.push(format!(
                "{}: sips failed with status {:?}: {}",
                sample,
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            ));
        } else if fs::metadata(&png_path)
            .map(|metadata| metadata.len() == 0)
            .unwrap_or(true)
        {
            failures.push(format!("{}: sips did not create a non-empty PNG", sample));
        } else {
            let png_output = match Command::new("swift")
                .env("CLANG_MODULE_CACHE_PATH", &module_cache_path)
                .arg("-e")
                .arg(PNG_VISIBLE_CONTENT_SWIFT)
                .arg(&png_path)
                .output()
            {
                Ok(output) => output,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
                Err(error) => {
                    failures.push(format!("{}: run Swift PNG check failed: {error}", sample));
                    let _ = fs::remove_dir_all(&temp_dir);
                    continue;
                }
            };
            if !png_output.status.success() {
                failures.push(format!(
                        "{}: sips PNG visible-content check failed with status {:?}: stdout={} stderr={}",
                        sample,
                        png_output.status.code(),
                        String::from_utf8_lossy(&png_output.stdout),
                        String::from_utf8_lossy(&png_output.stderr)
                    ));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }
            if let Some(check) = fixture.sips_region_check {
                match png_non_white_count_in_ratio_region(
                    &png_path,
                    check.left,
                    check.top,
                    check.right,
                    check.bottom,
                ) {
                    Ok(non_white) if non_white >= check.min_non_white => {}
                    Ok(non_white) => failures.push(format!(
                        "{}: sips {} rendered too few non-white pixels ({non_white})",
                        sample, check.label
                    )),
                    Err(error) => failures.push(format!(
                        "{}: sips {} check failed: {error}",
                        sample, check.label
                    )),
                }
            }
            rendered_count += 1;
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }

    assert_eq!(failures, Vec::<String>::new());
    assert!(rendered_count >= 1);
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
#[test]
fn local_complex_pdfs_render_visible_content_with_macos_pdfkit_when_available() {
    let sample_dir = local_sample_dir();
    if !sample_dir.exists() {
        return;
    }

    let mut failures = Vec::new();
    let mut rendered_count = 0usize;

    let any_sample_present = LOCAL_PDF_SMOKE_FIXTURES
        .iter()
        .any(|fixture| fixture.source_with_reference_pdf_exists(&sample_dir));
    if !any_sample_present {
        return;
    }

    for fixture in LOCAL_PDF_SMOKE_FIXTURES {
        let sample = fixture.source_name;
        let sample_path = fixture.source_path(&sample_dir);
        if !sample_path.exists() || !sample_path.with_extension("pdf").exists() {
            continue;
        }

        let result = fs::read(&sample_path)
            .map_err(|error| error.to_string())
            .and_then(|bytes| parse_document(&bytes).map_err(|error| error.to_string()))
            .and_then(|document| to_pdf_with_file_name(&document, &sample_path.to_string_lossy()));
        let pdf = match result {
            Ok(pdf) => pdf,
            Err(error) => {
                failures.push(format!("{}: {error}", sample_path.display()));
                continue;
            }
        };

        let temp_dir =
            std::env::temp_dir().join(format!("rjtd-pdfkit-smoke-{}-{sample}", std::process::id()));
        if let Err(error) = fs::create_dir_all(&temp_dir) {
            failures.push(format!("{}: create temp dir failed: {error}", sample));
            continue;
        }
        let pdf_path = temp_dir.join("sample.pdf");
        let module_cache_path = temp_dir.join("swift-module-cache");
        if let Err(error) = fs::create_dir_all(&module_cache_path) {
            failures.push(format!(
                "{}: create Swift module cache failed: {error}",
                sample
            ));
            let _ = fs::remove_dir_all(&temp_dir);
            continue;
        }
        if let Err(error) = fs::write(&pdf_path, &pdf) {
            failures.push(format!("{}: write temp pdf failed: {error}", sample));
            let _ = fs::remove_dir_all(&temp_dir);
            continue;
        }

        let mut command = Command::new("swift");
        command
            .env("CLANG_MODULE_CACHE_PATH", &module_cache_path)
            .arg("-e")
            .arg(PDFKIT_VISIBLE_CONTENT_SWIFT)
            .arg(&pdf_path);
        for page_check in fixture.page_checks {
            command.arg(page_check);
        }
        let output = match command.output() {
            Ok(output) => output,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
            Err(error) => {
                failures.push(format!(
                    "{}: run Swift PDFKit check failed: {error}",
                    sample
                ));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }
        };

        if !output.status.success() {
            failures.push(format!(
                "{}: PDFKit visible-content check failed with status {:?}: stdout={} stderr={}",
                sample,
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        } else {
            rendered_count += 1;
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }

    assert_eq!(failures, Vec::<String>::new());
    assert!(rendered_count >= 1);
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
#[test]
fn local_complex_pdfs_render_visible_content_with_macos_coregraphics_when_available() {
    let sample_dir = local_sample_dir();
    if !sample_dir.exists() {
        return;
    }

    let mut failures = Vec::new();
    let mut rendered_count = 0usize;

    let any_sample_present = LOCAL_PDF_SMOKE_FIXTURES
        .iter()
        .any(|fixture| fixture.source_with_reference_pdf_exists(&sample_dir));
    if !any_sample_present {
        return;
    }

    for fixture in LOCAL_PDF_SMOKE_FIXTURES {
        let sample = fixture.source_name;
        let sample_path = fixture.source_path(&sample_dir);
        if !sample_path.exists() || !sample_path.with_extension("pdf").exists() {
            continue;
        }

        let result = fs::read(&sample_path)
            .map_err(|error| error.to_string())
            .and_then(|bytes| parse_document(&bytes).map_err(|error| error.to_string()))
            .and_then(|document| to_pdf_with_file_name(&document, &sample_path.to_string_lossy()));
        let pdf = match result {
            Ok(pdf) => pdf,
            Err(error) => {
                failures.push(format!("{}: {error}", sample_path.display()));
                continue;
            }
        };

        let temp_dir = std::env::temp_dir().join(format!(
            "rjtd-coregraphics-smoke-{}-{sample}",
            std::process::id()
        ));
        if let Err(error) = fs::create_dir_all(&temp_dir) {
            failures.push(format!("{}: create temp dir failed: {error}", sample));
            continue;
        }
        let pdf_path = temp_dir.join("sample.pdf");
        let module_cache_path = temp_dir.join("swift-module-cache");
        if let Err(error) = fs::create_dir_all(&module_cache_path) {
            failures.push(format!(
                "{}: create Swift module cache failed: {error}",
                sample
            ));
            let _ = fs::remove_dir_all(&temp_dir);
            continue;
        }
        if let Err(error) = fs::write(&pdf_path, &pdf) {
            failures.push(format!("{}: write temp pdf failed: {error}", sample));
            let _ = fs::remove_dir_all(&temp_dir);
            continue;
        }

        let mut command = Command::new("swift");
        command
            .env("CLANG_MODULE_CACHE_PATH", &module_cache_path)
            .arg("-e")
            .arg(COREGRAPHICS_VISIBLE_CONTENT_SWIFT)
            .arg(&pdf_path);
        for page_check in fixture.page_checks {
            command.arg(page_check);
        }
        let output = match command.output() {
            Ok(output) => output,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
            Err(error) => {
                failures.push(format!(
                    "{}: run Swift CoreGraphics check failed: {error}",
                    sample
                ));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }
        };

        if !output.status.success() {
            failures.push(format!(
                    "{}: CoreGraphics visible-content check failed with status {:?}: stdout={} stderr={}",
                    sample,
                    output.status.code(),
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
        } else {
            rendered_count += 1;
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }

    assert_eq!(failures, Vec::<String>::new());
    assert!(rendered_count >= 1);
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
const PDFKIT_VISIBLE_CONTENT_SWIFT: &str = r#"
import CoreGraphics
import Foundation
import PDFKit

let path = CommandLine.arguments[1]
guard let document = PDFDocument(url: URL(fileURLWithPath: path)) else {
    fputs("PDFKit could not load document\n", stderr)
    exit(2)
}
if document.pageCount == 0 {
    fputs("PDFKit loaded zero pages\n", stderr)
    exit(3)
}

let requestedSpecs = Array(CommandLine.arguments.dropFirst(2))
var pageChecks: [(page: Int, minNonWhite: Int)] = []
if requestedSpecs.isEmpty {
    pageChecks = Array(1...min(document.pageCount, 2)).map { (page: $0, minNonWhite: 1) }
} else {
    for spec in requestedSpecs {
        let parts = spec.split(separator: ":", maxSplits: 1).map(String.init)
        guard let page = Int(parts[0]), page > 0 else {
            fputs("PDFKit invalid page check spec \(spec)\n", stderr)
            exit(4)
        }
        var minNonWhite = 1
        if parts.count == 2 {
            guard let parsedMinNonWhite = Int(parts[1]), parsedMinNonWhite > 0 else {
                fputs("PDFKit invalid minimum non-white spec \(spec)\n", stderr)
                exit(4)
            }
            minNonWhite = parsedMinNonWhite
        }
        pageChecks.append((page: page, minNonWhite: minNonWhite))
    }
}
var totalNonWhite = 0
var pageSummaries: [String] = []
for check in pageChecks {
    let oneBasedPageIndex = check.page
    if oneBasedPageIndex < 1 || oneBasedPageIndex > document.pageCount {
        fputs("PDFKit requested page \(oneBasedPageIndex) outside 1...\(document.pageCount)\n", stderr)
        exit(5)
    }
    let pageIndex = oneBasedPageIndex - 1
    guard let page = document.page(at: pageIndex) else {
        continue
    }
    let box = page.bounds(for: .mediaBox)
    let width = max(1, Int(box.width.rounded(.up)))
    let height = max(1, Int(box.height.rounded(.up)))
    var bytes = [UInt8](repeating: 255, count: width * height * 4)
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    guard let context = CGContext(
        data: &bytes,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: width * 4,
        space: colorSpace,
        bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
    ) else {
        fputs("Could not create CGContext\n", stderr)
        exit(6)
    }
    context.setFillColor(CGColor(red: 1, green: 1, blue: 1, alpha: 1))
    context.fill(CGRect(x: 0, y: 0, width: width, height: height))
    page.draw(with: .mediaBox, to: context)

    var pageNonWhite = 0
    var byteIndex = 0
    while byteIndex < bytes.count {
        if bytes[byteIndex] < 245 || bytes[byteIndex + 1] < 245 || bytes[byteIndex + 2] < 245 {
            pageNonWhite += 1
        }
        byteIndex += 4
    }
    if pageNonWhite < check.minNonWhite {
        fputs("PDFKit rendered \(pageNonWhite) non-white pixels on page \(pageIndex + 1), below minimum \(check.minNonWhite)\n", stderr)
        exit(7)
    }
    totalNonWhite += pageNonWhite
    pageSummaries.append("\(oneBasedPageIndex):\(pageNonWhite)")
}

let checkedSummary = pageSummaries.joined(separator: ",")
print("pages \(document.pageCount) checked \(checkedSummary) nonWhite \(totalNonWhite)")
"#;

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
const COREGRAPHICS_VISIBLE_CONTENT_SWIFT: &str = r#"
import CoreGraphics
import Foundation

let path = CommandLine.arguments[1]
let url = URL(fileURLWithPath: path) as CFURL
guard let document = CGPDFDocument(url) else {
    fputs("CGPDFDocument could not load document\n", stderr)
    exit(2)
}
let pageCount = document.numberOfPages
if pageCount == 0 {
    fputs("CGPDFDocument loaded zero pages\n", stderr)
    exit(3)
}

let requestedSpecs = Array(CommandLine.arguments.dropFirst(2))
var pageChecks: [(page: Int, minNonWhite: Int)] = []
if requestedSpecs.isEmpty {
    pageChecks = Array(1...min(pageCount, 2)).map { (page: $0, minNonWhite: 1) }
} else {
    for spec in requestedSpecs {
        let parts = spec.split(separator: ":", maxSplits: 1).map(String.init)
        guard let page = Int(parts[0]), page > 0 else {
            fputs("CoreGraphics invalid page check spec \(spec)\n", stderr)
            exit(4)
        }
        var minNonWhite = 1
        if parts.count == 2 {
            guard let parsedMinNonWhite = Int(parts[1]), parsedMinNonWhite > 0 else {
                fputs("CoreGraphics invalid minimum non-white spec \(spec)\n", stderr)
                exit(4)
            }
            minNonWhite = parsedMinNonWhite
        }
        pageChecks.append((page: page, minNonWhite: minNonWhite))
    }
}
var totalNonWhite = 0
var pageSummaries: [String] = []
for check in pageChecks {
    let pageIndex = check.page
    if pageIndex < 1 || pageIndex > pageCount {
        fputs("CoreGraphics requested page \(pageIndex) outside 1...\(pageCount)\n", stderr)
        exit(5)
    }
    guard let page = document.page(at: pageIndex) else {
        continue
    }
    let box = page.getBoxRect(.mediaBox)
    let width = max(1, Int(box.width.rounded(.up)))
    let height = max(1, Int(box.height.rounded(.up)))
    var bytes = [UInt8](repeating: 255, count: width * height * 4)
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    guard let context = CGContext(
        data: &bytes,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: width * 4,
        space: colorSpace,
        bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
    ) else {
        fputs("Could not create CGContext\n", stderr)
        exit(6)
    }
    context.setFillColor(CGColor(red: 1, green: 1, blue: 1, alpha: 1))
    context.fill(CGRect(x: 0, y: 0, width: width, height: height))
    context.drawPDFPage(page)

    var pageNonWhite = 0
    var byteIndex = 0
    while byteIndex < bytes.count {
        if bytes[byteIndex] < 245 || bytes[byteIndex + 1] < 245 || bytes[byteIndex + 2] < 245 {
            pageNonWhite += 1
        }
        byteIndex += 4
    }
    if pageNonWhite < check.minNonWhite {
        fputs("CoreGraphics rendered \(pageNonWhite) non-white pixels on page \(pageIndex), below minimum \(check.minNonWhite)\n", stderr)
        exit(7)
    }
    totalNonWhite += pageNonWhite
    pageSummaries.append("\(pageIndex):\(pageNonWhite)")
}

let checkedSummary = pageSummaries.joined(separator: ",")
print("pages \(pageCount) checked \(checkedSummary) nonWhite \(totalNonWhite)")
"#;

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
const PNG_VISIBLE_CONTENT_SWIFT: &str = r#"
import CoreGraphics
import Foundation
import ImageIO

let path = CommandLine.arguments[1]
let url = URL(fileURLWithPath: path) as CFURL
guard let source = CGImageSourceCreateWithURL(url, nil),
      let image = CGImageSourceCreateImageAtIndex(source, 0, nil) else {
    fputs("Could not load PNG image\n", stderr)
    exit(2)
}
let width = image.width
let height = image.height
if width == 0 || height == 0 {
    fputs("PNG image has zero size\n", stderr)
    exit(3)
}
var bytes = [UInt8](repeating: 255, count: width * height * 4)
let colorSpace = CGColorSpaceCreateDeviceRGB()
guard let context = CGContext(
    data: &bytes,
    width: width,
    height: height,
    bitsPerComponent: 8,
    bytesPerRow: width * 4,
    space: colorSpace,
    bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
) else {
    fputs("Could not create CGContext\n", stderr)
    exit(4)
}
context.setFillColor(CGColor(red: 1, green: 1, blue: 1, alpha: 1))
context.fill(CGRect(x: 0, y: 0, width: width, height: height))
context.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))

var nonWhite = 0
var byteIndex = 0
while byteIndex < bytes.count {
    if bytes[byteIndex] < 245 || bytes[byteIndex + 1] < 245 || bytes[byteIndex + 2] < 245 {
        nonWhite += 1
    }
    byteIndex += 4
}
print("png \(width)x\(height) nonWhite \(nonWhite)")
if nonWhite == 0 {
    fputs("PNG rendered no visible non-white pixels\n", stderr)
    exit(5)
}
"#;

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn local_samples_export_to_valid_pdf_when_available() {
    let sample_dir = local_sample_dir();
    if !sample_dir.exists() {
        return;
    }

    let mut paths = fs::read_dir(&sample_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .and_then(|value| value.to_str())
                .is_some_and(|extension| matches!(extension, "jtd" | "jtt" | "jttc"))
                && path.with_extension("pdf").exists()
        })
        .collect::<Vec<_>>();
    paths.sort();
    if paths.is_empty() {
        return;
    }

    let mut failures = Vec::new();
    let mut pdf_count = 0usize;
    let mut total_pdf_bytes = 0usize;

    for path in &paths {
        let result = fs::read(path)
            .map_err(|error| error.to_string())
            .and_then(|bytes| parse_document(&bytes).map_err(|error| error.to_string()))
            .and_then(|document| to_pdf_with_file_name(&document, &path.to_string_lossy()));

        match result {
            Ok(pdf) => {
                if !pdf.starts_with(b"%PDF-") {
                    failures.push(format!("{}: missing PDF header", path.display()));
                }
                if !pdf.windows(5).any(|window| window == b"/Page") {
                    failures.push(format!("{}: missing /Page marker", path.display()));
                }
                if !pdf.windows(5).any(|window| window == b"%%EOF") {
                    failures.push(format!("{}: missing EOF marker", path.display()));
                }
                if pdf.len() < 512 {
                    failures.push(format!("{}: suspiciously small PDF", path.display()));
                }
                if !pdf.windows(10).any(|window| window == b"/ToUnicode") {
                    failures.push(format!("{}: missing ToUnicode text map", path.display()));
                }
                if !pdf.windows(12).any(|window| window == b"/CIDFontType") {
                    failures.push(format!("{}: missing CID font resource", path.display()));
                }
                let form_xobject_count = pdf_byte_pattern_count(&pdf, b"/Subtype /Form");
                let form_type_count = pdf_byte_pattern_count(&pdf, b"/FormType 1");
                if form_xobject_count == 0 {
                    failures.push(format!("{}: missing Form XObject wrapper", path.display()));
                }
                if form_type_count != form_xobject_count {
                    failures.push(format!(
                            "{}: Form XObject /FormType coverage mismatch ({form_type_count}/{form_xobject_count})",
                            path.display()
                        ));
                }
                let preview_safety_issues = pdf_preview_blocking_issues(&pdf);
                if !preview_safety_issues.is_empty() {
                    failures.push(format!(
                        "{}: Preview/PDFKit risky PDF constructs: {}",
                        path.display(),
                        preview_safety_issues.join(", ")
                    ));
                }
                if path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .is_some_and(|file_name| file_name == "a6.jtd")
                {
                    let page_object_count = pdf_page_object_count(&pdf);
                    if page_object_count != 114 {
                        failures.push(format!(
                            "{}: expected 114 PDF page objects, got {page_object_count}",
                            path.display()
                        ));
                    }
                    if !pdf.windows(10).any(|window| window == b"/Count 114") {
                        failures.push(format!("{}: missing /Count 114", path.display()));
                    }
                    if pdf_byte_pattern_count(&pdf, b"/MediaBox [0 0 297.675") != 114 {
                        failures.push(format!(
                            "{}: A6 portrait MediaBox does not cover all pages",
                            path.display()
                        ));
                    }
                }
                pdf_count += 1;
                total_pdf_bytes += pdf.len();
            }
            Err(error) => failures.push(format!("{}: {error}", path.display())),
        }
    }

    assert_eq!(failures, Vec::<String>::new());
    assert!(pdf_count >= 1);
    assert!(total_pdf_bytes > pdf_count * 512);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn local_pdf_output_artifacts_have_preview_compatible_form_xobjects_when_available() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let sample_dir = project_root.join("rjtd-testdata/local-samples");
    let output_dir = project_root.join("openjtd-samples/pdf-output");
    if !sample_dir.exists() || !output_dir.exists() {
        return;
    }

    let mut paths = fs::read_dir(&sample_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .and_then(|value| value.to_str())
                .is_some_and(|extension| matches!(extension, "jtd" | "jtt" | "jttc"))
        })
        .collect::<Vec<_>>();
    paths.sort();

    let mut failures = Vec::new();
    let official_output_stems = paths
        .iter()
        .filter_map(|path| path.file_stem().and_then(|value| value.to_str()))
        .collect::<BTreeSet<_>>();
    let mut output_pdfs = fs::read_dir(&output_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .and_then(|value| value.to_str())
                .is_some_and(|extension| extension == "pdf")
        })
        .collect::<Vec<_>>();
    output_pdfs.sort();
    for pdf_path in &output_pdfs {
        let Some(stem) = pdf_path.file_stem().and_then(|value| value.to_str()) else {
            continue;
        };
        if !official_output_stems.contains(stem) {
            failures.push(format!(
                    "{}: unexpected auxiliary PDF output; only exact same-stem sample PDFs are official artifacts",
                    pdf_path.display()
                ));
        }
    }
    let mut checked_count = 0usize;
    for path in &paths {
        let Some(stem) = path.file_stem().and_then(|value| value.to_str()) else {
            continue;
        };
        let pdf_path = output_dir.join(format!("{stem}.pdf"));
        let pdf = match fs::read(&pdf_path) {
            Ok(pdf) => pdf,
            Err(error) => {
                failures.push(format!("{}: {error}", pdf_path.display()));
                continue;
            }
        };

        if !pdf.starts_with(b"%PDF-") {
            failures.push(format!("{}: missing PDF header", pdf_path.display()));
        }
        if count_pdf_eof_markers(&pdf) != 1 {
            failures.push(format!(
                "{}: expected one EOF marker, got {}",
                pdf_path.display(),
                count_pdf_eof_markers(&pdf)
            ));
        }
        let form_xobject_count = pdf_byte_pattern_count(&pdf, b"/Subtype /Form");
        let form_type_count = pdf_byte_pattern_count(&pdf, b"/FormType 1");
        if form_xobject_count == 0 {
            failures.push(format!(
                "{}: missing Form XObject wrapper",
                pdf_path.display()
            ));
        }
        if form_type_count != form_xobject_count {
            failures.push(format!(
                    "{}: Form XObject /FormType coverage mismatch ({form_type_count}/{form_xobject_count})",
                    pdf_path.display()
                ));
        }
        let preview_safety_issues = pdf_preview_blocking_issues(&pdf);
        if !preview_safety_issues.is_empty() {
            failures.push(format!(
                "{}: Preview/PDFKit risky PDF constructs: {}",
                pdf_path.display(),
                preview_safety_issues.join(", ")
            ));
        }
        let reference_pdf_path = sample_dir.join(format!("{stem}.pdf"));
        if reference_pdf_path.exists() && local_reference_pdf_page_count_is_trusted(stem) {
            let reference_pdf = match fs::read(&reference_pdf_path) {
                Ok(reference_pdf) => reference_pdf,
                Err(error) => {
                    failures.push(format!("{}: {error}", reference_pdf_path.display()));
                    continue;
                }
            };
            let reference_page_count = pdf_page_object_count(&reference_pdf);
            let output_page_count = pdf_page_object_count(&pdf);
            let known_divergence = local_reference_known_divergence(stem);
            if reference_page_count == 0 {
                failures.push(format!(
                    "{}: could not derive reference PDF page count",
                    reference_pdf_path.display()
                ));
            } else if let Some(divergence) = known_divergence {
                if reference_page_count != divergence.expected_reference_page_count
                    || output_page_count != divergence.expected_output_page_count
                {
                    failures.push(format!(
                            "{}: known PDF page-count divergence lock changed ({reason}); expected reference/output {expected_reference}/{expected_output}, got {reference_page_count}/{output_page_count}; refresh the rjtd-export known-divergence lock",
                            pdf_path.display(),
                            reason = divergence.page_count_reason,
                            expected_reference = divergence.expected_reference_page_count,
                            expected_output = divergence.expected_output_page_count
                        ));
                }
            } else if output_page_count != reference_page_count {
                failures.push(format!(
                        "{}: expected {reference_page_count} PDF page objects to match {}, got {output_page_count}",
                        pdf_path.display(),
                        reference_pdf_path.display()
                    ));
            }

            let reference_media_boxes = pdf_media_box_sizes(&reference_pdf);
            let output_media_boxes = pdf_media_box_sizes(&pdf);
            let Some(reference_media_box) = reference_media_boxes.first().copied() else {
                failures.push(format!(
                    "{}: could not derive reference MediaBox",
                    reference_pdf_path.display()
                ));
                continue;
            };
            if output_media_boxes.is_empty() {
                failures.push(format!(
                    "{}: could not derive output MediaBox",
                    pdf_path.display()
                ));
            }
            if output_page_count != 0 && output_media_boxes.len() != output_page_count {
                failures.push(format!(
                    "{}: expected {output_page_count} MediaBox entries, got {}",
                    pdf_path.display(),
                    output_media_boxes.len()
                ));
            }
            if let Some(media_box_divergence) =
                known_divergence.and_then(|divergence| divergence.media_box_divergence)
            {
                if !reference_media_box.close_to(media_box_divergence.expected_reference_media_box)
                {
                    failures.push(format!(
                            "{}: known reference MediaBox divergence lock changed ({reason}); expected {:.3}x{:.3}, got {:.3}x{:.3}; refresh the rjtd-export known-divergence lock",
                            reference_pdf_path.display(),
                            media_box_divergence.expected_reference_media_box.width,
                            media_box_divergence.expected_reference_media_box.height,
                            reference_media_box.width,
                            reference_media_box.height,
                            reason = media_box_divergence.reason
                        ));
                }
                for (page_index, output_media_box) in output_media_boxes.iter().enumerate() {
                    if !output_media_box.close_to(media_box_divergence.expected_output_media_box) {
                        failures.push(format!(
                                "{}: known output page {} MediaBox divergence lock changed ({reason}); expected {:.3}x{:.3}, got {:.3}x{:.3}; refresh the rjtd-export known-divergence lock",
                                pdf_path.display(),
                                page_index + 1,
                                media_box_divergence.expected_output_media_box.width,
                                media_box_divergence.expected_output_media_box.height,
                                output_media_box.width,
                                output_media_box.height,
                                reason = media_box_divergence.reason
                            ));
                    }
                }
            } else {
                for (page_index, output_media_box) in output_media_boxes.iter().enumerate() {
                    let expected_media_box = reference_media_boxes
                        .get(page_index)
                        .copied()
                        .unwrap_or(reference_media_box);
                    if !output_media_box.close_to(expected_media_box) {
                        failures.push(format!(
                                "{}: page {} MediaBox {:.3}x{:.3} does not match trusted reference {:.3}x{:.3}",
                                pdf_path.display(),
                                page_index + 1,
                                output_media_box.width,
                                output_media_box.height,
                                expected_media_box.width,
                                expected_media_box.height
                            ));
                    }
                }
            }
        }
        checked_count += 1;
    }

    assert_eq!(failures, Vec::<String>::new());
    assert!(checked_count >= 1);
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
#[test]
fn local_pdf_output_artifacts_render_visible_content_with_macos_pdfkit_when_available() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let output_dir = project_root.join("openjtd-samples/pdf-output");
    if !output_dir.exists() {
        return;
    }

    let mut failures = Vec::new();
    let mut rendered_count = 0usize;

    for fixture in LOCAL_PDF_SMOKE_FIXTURES {
        let sample = fixture.output_pdf_name;
        let pdf_path = fixture.output_pdf_path(&output_dir);
        if !pdf_path.exists() {
            continue;
        }

        let temp_dir = std::env::temp_dir().join(format!(
            "rjtd-output-pdfkit-smoke-{}-{sample}",
            std::process::id()
        ));
        if let Err(error) = fs::create_dir_all(&temp_dir) {
            failures.push(format!("{}: create temp dir failed: {error}", sample));
            continue;
        }
        let module_cache_path = temp_dir.join("swift-module-cache");
        if let Err(error) = fs::create_dir_all(&module_cache_path) {
            failures.push(format!(
                "{}: create Swift module cache failed: {error}",
                sample
            ));
            let _ = fs::remove_dir_all(&temp_dir);
            continue;
        }

        let mut command = Command::new("swift");
        command
            .env("CLANG_MODULE_CACHE_PATH", &module_cache_path)
            .arg("-e")
            .arg(PDFKIT_VISIBLE_CONTENT_SWIFT)
            .arg(&pdf_path);
        for page_check in fixture.page_checks {
            command.arg(page_check);
        }
        let output = match command.output() {
            Ok(output) => output,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
            Err(error) => {
                failures.push(format!(
                    "{}: run Swift PDFKit check failed: {error}",
                    pdf_path.display()
                ));
                let _ = fs::remove_dir_all(&temp_dir);
                continue;
            }
        };

        if !output.status.success() {
            failures.push(format!(
                "{}: PDFKit visible-content check failed with status {:?}: stdout={} stderr={}",
                pdf_path.display(),
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        } else {
            rendered_count += 1;
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }

    assert_eq!(failures, Vec::<String>::new());
    assert!(rendered_count >= 1);
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
#[test]
fn local_pdf_output_success_data_test_title_rasterizes_with_macos_sips_when_available() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let pdf_path = project_root
        .join("openjtd-samples/pdf-output")
        .join("ichitaro-20030228030923-success-002-success_data-test.pdf");
    if !pdf_path.exists() {
        return;
    }

    let temp_dir = std::env::temp_dir().join(format!(
        "rjtd-output-title-sips-smoke-{}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_dir).unwrap();
    let png_path = temp_dir.join("success-data-test-page1.png");

    let output = match Command::new("sips")
        .arg("-s")
        .arg("format")
        .arg("png")
        .arg(&pdf_path)
        .arg("--out")
        .arg(&png_path)
        .output()
    {
        Ok(output) => output,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
        Err(error) => panic!("run sips failed: {error}"),
    };
    assert!(
        output.status.success(),
        "sips failed with status {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let title_non_white = png_non_white_count_in_ratio_region(&png_path, 0.05, 0.07, 0.92, 0.20)
        .expect("sips title region should be readable");
    let _ = fs::remove_dir_all(&temp_dir);

    assert!(
        title_non_white >= 3_000,
        "sips-rendered title region has too few non-white pixels: {title_non_white}"
    );
}

fn local_reference_pdf_page_count_is_trusted(stem: &str) -> bool {
    // The local 46.pdf reference is a known suspicious capture: it renders as
    // essentially blank/title-like while 46.jtd contains the Ginga body text.
    // Keep it out of full-document page-count gates until the sample is replaced.
    stem != "46"
}

fn local_reference_known_divergence(stem: &str) -> Option<LocalReferencePdfKnownDivergence> {
    const SEMINAR2004_REFERENCE_LANDSCAPE: PdfMediaBox = PdfMediaBox {
        width: 841.890,
        height: 595.276,
    };
    const SEMINAR2004_OUTPUT_PORTRAIT: PdfMediaBox = PdfMediaBox {
        width: 595.275,
        height: 841.875,
    };
    const SEMINAR2004_MEDIA_BOX_DIVERGENCE: LocalReferencePdfKnownMediaBoxDivergence =
        LocalReferencePdfKnownMediaBoxDivergence {
            expected_reference_media_box: SEMINAR2004_REFERENCE_LANDSCAPE,
            expected_output_media_box: SEMINAR2004_OUTPUT_PORTRAIT,
            reason: LOCAL_REFERENCE_PAPER_ORIENTATION_SOURCE_DECODE_UNPROVEN,
        };
    const KNOWN_DIVERGENCES: &[(&str, LocalReferencePdfKnownDivergence)] = &[
        (
            "ichitaro-20030316043238-success-001-success_data-iwata_file",
            LocalReferencePdfKnownDivergence::pagination(13, 24),
        ),
        (
            "ichitaro-20030316045013-success-002-success_data-resume",
            LocalReferencePdfKnownDivergence::pagination(3, 6),
        ),
        (
            "ichitaro-20030415170937-success-001-success_data-fujimoto_file",
            LocalReferencePdfKnownDivergence::pagination(3, 1),
        ),
        (
            "ichitaro-20030422193925-success-003-success_data-christmas_2001",
            LocalReferencePdfKnownDivergence::pagination(1, 2),
        ),
        (
            "ichitaro-20030422194039-success-003-success_data-syokuhin",
            LocalReferencePdfKnownDivergence::pagination(1, 26),
        ),
        (
            "ichitaro-20030422210439-success-002-success_data-natsu",
            LocalReferencePdfKnownDivergence::pagination(2, 1),
        ),
        (
            "ichitaro-20030706234132-success-004-success_data-asobinin_24",
            LocalReferencePdfKnownDivergence::pagination(1, 2),
        ),
        (
            "ichitaro-20041103142937-seminar2004-part2_1-img-shortcutkey1",
            LocalReferencePdfKnownDivergence::pagination_with_media_box(
                1,
                4,
                SEMINAR2004_MEDIA_BOX_DIVERGENCE,
            ),
        ),
        (
            "ichitaro-20041103143104-seminar2004-part2_2-img-shortcutkey2",
            LocalReferencePdfKnownDivergence::pagination_with_media_box(
                1,
                4,
                SEMINAR2004_MEDIA_BOX_DIVERGENCE,
            ),
        ),
        (
            "ichitaro-20050214114830-seminar2004-part2_3-img-toolbox",
            LocalReferencePdfKnownDivergence::pagination_with_media_box(
                1,
                2,
                SEMINAR2004_MEDIA_BOX_DIVERGENCE,
            ),
        ),
        (
            "ichitaro-20050214115206-seminar2004-part2_3-img-shortcutkey3",
            LocalReferencePdfKnownDivergence::pagination_with_media_box(
                1,
                3,
                SEMINAR2004_MEDIA_BOX_DIVERGENCE,
            ),
        ),
    ];

    // These trusted local reference PDFs expose source semantics the current
    // fallback renderer has not decoded yet. Keep exact counts locked so a
    // regenerated artifact or replaced reference must refresh this evidence.
    KNOWN_DIVERGENCES
        .iter()
        .find_map(|(known_stem, divergence)| (*known_stem == stem).then_some(*divergence))
}

fn pdf_page_object_count(pdf: &[u8]) -> usize {
    // Reference captures vary in name-token whitespace (`/Type /Page\n` from
    // this exporter, `/Type/Page` from external capture tools), so count
    // `/Type` + optional whitespace + `/Page` followed by a PDF delimiter.
    // The delimiter check keeps `/Type/Pages` tree nodes out of the count.
    let mut count = 0usize;
    let mut position = 0usize;
    while let Some(relative_offset) = find_subslice(&pdf[position..], b"/Type") {
        let type_offset = position + relative_offset;
        position = type_offset + b"/Type".len();
        let mut cursor = pdf_skip_whitespace(pdf, position);
        if !pdf
            .get(cursor..)
            .is_some_and(|tail| tail.starts_with(b"/Page"))
        {
            continue;
        }
        cursor += b"/Page".len();
        let next_is_delimiter = match pdf.get(cursor) {
            None => true,
            Some(byte) => matches!(
                byte,
                b'\0'
                    | b'\t'
                    | b'\n'
                    | b'\x0c'
                    | b'\r'
                    | b' '
                    | b'/'
                    | b'<'
                    | b'>'
                    | b'['
                    | b']'
                    | b'('
                    | b')'
                    | b'{'
                    | b'}'
                    | b'%'
            ),
        };
        if next_is_delimiter {
            count += 1;
        }
    }
    count
}

#[test]
fn pdf_page_object_count_handles_reference_capture_serialization_variants() {
    assert_eq!(pdf_page_object_count(b"<< /Type /Page\n >>"), 1);
    assert_eq!(pdf_page_object_count(b"<</Type/Page/Parent 2 0 R>>"), 1);
    assert_eq!(pdf_page_object_count(b"<</Type/Pages/Count 3>>"), 0);
    assert_eq!(
        pdf_page_object_count(b"<</Type/Pages/Kids[3 0 R]>><</Type/Page>><< /Type /Page\n>>"),
        2
    );
    assert_eq!(pdf_page_object_count(b"/Type /PageLabels"), 0);
    assert_eq!(pdf_page_object_count(b"<</Type/Page%comment\n>>"), 1);
}

fn pdf_byte_pattern_count(pdf: &[u8], pattern: &[u8]) -> usize {
    pdf.windows(pattern.len())
        .filter(|window| *window == pattern)
        .count()
}

#[test]
fn exports_json_from_document_model() {
    let paragraph = Paragraph::new(vec![Inline::Text(TextRun::new("hello\n\"", None))], None);
    let document = Document::new(
        Metadata::new(Some("sample".to_string())),
        vec![Block::Paragraph(paragraph)],
    );

    assert_eq!(
        to_json(&document),
        "{\"metadata\":{\"title\":\"sample\"},\"blocks\":[{\"type\":\"paragraph\",\"style\":null,\"inlines\":[{\"type\":\"text\",\"text\":\"hello\\n\\\"\",\"style\":null}]}],\"unknownStyles\":[],\"unknownObjects\":[],\"objectStreamCandidates\":[],\"objectFrameRecords\":[],\"objectEmbeddingFrames\":[],\"textCountRanges\":[],\"textControlBoundaries\":[],\"textBoundaryCandidates\":[],\"textParagraphBoundaryCandidates\":[],\"tableCandidates\":[],\"autoTextCandidates\":[],\"tocEntries\":[],\"pageMarks\":[],\"paperMarks\":[],\"rawStreams\":[],\"fonts\":[]}"
    );
}

#[test]
fn exports_paragraph_style_reference_to_json() {
    let paragraph = Paragraph::new(
        vec![Inline::Text(TextRun::new("styled", None))],
        Some(StyleRef::new("1")),
    );
    let document = Document::new(Metadata::default(), vec![Block::Paragraph(paragraph)]);

    let json = to_json(&document);

    assert!(json.contains("\"style\":{\"id\":\"1\"}"));
}

#[test]
fn exports_text_source_span_to_json_when_available() {
    let paragraph = Paragraph::new(
        vec![Inline::Text(TextRun::with_source_span(
            "銀河",
            None,
            Some(TextSourceSpan::new(10, 14, 5, 7)),
        ))],
        None,
    );
    let document = Document::new(Metadata::default(), vec![Block::Paragraph(paragraph)]);

    let json = to_json(&document);

    assert!(json.contains(
        "\"sourceSpan\":{\"byteStart\":10,\"byteEnd\":14,\"unitStart\":5,\"unitEnd\":7}"
    ));
}

#[test]
fn exports_text_control_boundaries_to_json() {
    let mut document = Document::default();
    document.push_text_control_boundary(TextControlBoundary::new(
        0,
        0x001c,
        Some(TextSourceSpan::new(6, 8, 3, 4)),
    ));

    let json = to_json(&document);

    assert!(json.contains("\"textControlBoundaries\":[{"));
    assert!(json.contains("\"code\":28"));
    assert!(json.contains("\"codeHex\":\"0x001c\""));
    assert!(
        json.contains(
            "\"sourceSpan\":{\"byteStart\":6,\"byteEnd\":8,\"unitStart\":3,\"unitEnd\":4}"
        )
    );
    assert!(json.contains("\"decoded\":false"));
}

#[test]
fn exports_ruby_inline_as_visible_base_with_preserved_annotation() {
    let annotation_source = UnknownObject::new(UnknownRecordKind::new(Some(0x001d)), vec![1]);
    let ruby = RubyAnnotation::new("午后", "ごご", 0x0082, annotation_source);
    let paragraph = Paragraph::new(
        vec![
            Inline::Text(TextRun::new("一、", None)),
            Inline::Ruby(ruby),
            Inline::Text(TextRun::new("の授業", None)),
        ],
        None,
    );
    let document = Document::new(Metadata::default(), vec![Block::Paragraph(paragraph)]);

    assert_eq!(to_plain_text(&document), "一、午后の授業\n");
    assert_eq!(to_markdown(&document), "一、午后の授業\n\n");

    let json = to_json(&document);
    assert!(json.contains("\"type\":\"ruby\""));
    assert!(json.contains("\"baseText\":\"午后\""));
    assert!(json.contains("\"annotationText\":\"ごご\""));
    assert!(json.contains("\"annotationSelector\":130"));
    assert!(json.contains("\"payloadHex\":\"01\""));
}

#[test]
fn exports_unknown_blocks_to_json_without_dropping_payload() {
    let unknown = UnknownBlock::new(UnknownRecordKind::new(Some(7)), vec![1, 2, 255]);
    let document = Document::new(Metadata::default(), vec![Block::Unknown(unknown)]);

    assert!(to_json(&document).contains("\"payloadHex\":\"0102ff\""));
}

#[test]
fn exports_unknown_style_stream_name_to_json() {
    let mut document = Document::from_plain_text("hello");
    document.push_unknown_style(UnknownStyle::from_stream("/TextLayoutStyle", vec![1, 2, 3]));

    let json = to_json(&document);

    assert!(json.contains("\"unknownStyles\":[{\"name\":\"/TextLayoutStyle\""));
    assert!(json.contains("\"family\":\"unknown\""));
    assert!(json.contains("\"headerU32Be\":[]"));
    assert!(json.contains("\"recordLayout\":\"none\""));
    assert!(json.contains("\"recordCount\":0"));
    assert!(json.contains("\"records\":[]"));
    assert!(json.contains("\"payloadHex\":\"010203\""));
}

#[test]
fn exports_raw_stream_summary_to_json() {
    let mut document = Document::from_plain_text("hello");
    document.push_raw_stream(RawStream::new("/DocumentText", vec![1, 2, 3]));

    assert!(
        to_json(&document).contains("\"rawStreams\":[{\"name\":\"/DocumentText\",\"size\":3}]")
    );
}

#[test]
fn exports_object_stream_candidates_to_json() {
    let mut document = Document::from_plain_text("hello");
    document.push_object_stream_candidate(ObjectStreamCandidate::new(
        "/EmbedItems/Embedding 1/Contents",
        12,
        ObjectStreamCandidateEvidence::new(
            vec![
                ObjectStreamCandidateReason::ObjectPath,
                ObjectStreamCandidateReason::ImageSignature,
            ],
            vec![ObjectImageSignatureHit::new("jpeg", 4)],
            vec![ObjectImagePayloadSpan::new(
                "jpeg",
                "image/jpeg",
                ObjectImagePayloadLocation::new(4, 4, 11),
                true,
                b"\xff\xd8\xffda\xff\xd9".to_vec(),
                ObjectImagePayloadEnvelope::new(
                    0,
                    4,
                    11,
                    12,
                    Some(ObjectImageDeclaredLengthCandidate::new(0, 7, "le32")),
                    vec![7, 0, 0, 0],
                    vec![0],
                ),
            )],
            None,
            vec![],
            vec![8],
        ),
        vec![0x09, 0x00, 0x01, 0x00],
    ));
    document.push_object_stream_candidate(ObjectStreamCandidate::new(
        "/VisualList",
        19,
        ObjectStreamCandidateEvidence::new(
            vec![ObjectStreamCandidateReason::VisualListPath],
            vec![],
            vec![],
            None,
            vec![],
            vec![],
        ),
        b"BMDV visual payl".to_vec(),
    ));

    let json = to_json(&document);

    assert!(
        json.contains("\"objectStreamCandidates\":[{\"path\":\"/EmbedItems/Embedding 1/Contents\"")
    );
    assert!(json.contains("\"reasons\":[\"object-path\",\"image-signature\"]"));
    assert!(json.contains("\"ownershipCandidate\":{\"basis\":\"stream-path\",\"family\":\"embed-items\",\"storagePath\":\"/EmbedItems/Embedding 1\",\"embeddingIndex\":1,\"streamRole\":\"contents\",\"decoded\":false}"));
    assert!(json.contains("\"ownershipReferences\":[]"));
    assert!(json.contains("\"frameReferenceRows\":[]"));
    assert!(json.contains("\"fdmIndexEntries\":[]"));
    assert!(json.contains("\"imageSignatures\":[{\"kind\":\"jpeg\",\"offset\":4}]"));
    assert!(json.contains("\"imagePayloads\":[{\"kind\":\"jpeg\",\"mime\":\"image/jpeg\",\"signatureOffset\":4,\"start\":4,\"end\":11,\"length\":7,\"complete\":true"));
    assert!(json.contains("\"objectEnvelope\":{\"headerStart\":0"));
    assert!(json.contains("\"headerEnd\":4"));
    assert!(json.contains("\"headerPrefixHex\":\"07000000\""));
    assert!(json.contains("\"headerFields\""));
    assert!(json.contains("\"u16LePrefix\":[{\"offset\":0,\"value\":7}"));
    assert!(json.contains("\"u32LePrefix\":[{\"offset\":0,\"value\":7}]"));
    assert!(json.contains("\"sourcePathCandidate\":null"));
    assert!(json.contains("\"trailerStart\":11"));
    assert!(json.contains("\"trailerPrefixHex\":\"00\""));
    assert!(json.contains("\"declaredPayloadLength\":7"));
    assert!(json.contains("\"declaredPayloadLengthOffset\":0"));
    assert!(json.contains("\"declaredPayloadLengthEndian\":\"le32\""));
    assert!(json.contains("\"payloadPrefixHex\":\"ffd8ff6461ffd9\",\"decoded\":false}]"));
    assert!(json.contains("\"soOffsets\":[8]"));
    assert!(json.contains("\"payloadPrefixHex\":\"09000100\""));
    assert!(
        json.contains("{\"path\":\"/VisualList\",\"size\":19,\"reasons\":[\"visual-list-path\"]")
    );
    assert!(json.contains("\"payloadPrefixHex\":\"424d44562076697375616c207061796c\""));
    assert!(json.contains("\"decoded\":false"));
}

#[test]
fn local_fax02_exports_visual_list_metadata_to_json_when_reference_pdf_is_available() {
    let sample_dir = local_sample_dir();
    let sample_path = sample_dir.join("fax02.jtt");
    let reference_pdf_path = sample_dir.join("fax02.pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
    let json = to_json(&document);

    assert!(json.contains("\"path\":\"/VisualList\""));
    assert!(json.contains("\"reasons\":[\"visual-list-path\"]"));
    assert!(json.contains("\"visualList\":{\"format\":\"BMDV\""));
    assert!(json.contains("\"declaredSize\":2296"));
    assert!(json.contains("\"width\":120"));
    assert!(json.contains("\"height\":169"));
    assert!(json.contains("\"rleDataLength\":2216"));
    assert!(json.contains("\"pixelCount\":20280"));
    assert!(json.contains("\"rleEncoding\":\"bmp-rle8-like\""));
}

#[test]
fn local_a5_exports_toc_page_label_candidates_when_reference_pdf_is_available() {
    let sample_dir = local_sample_dir();
    let sample_path = sample_dir.join("a5.jtd");
    let reference_pdf_path = sample_dir.join("a5.pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
    let json = to_json(&document);

    assert!(json.contains("\"tocEntries\":["));
    assert!(json.contains("\"title\":\"一、午后の授業\""));
    assert!(json.contains("\"pageLabel\":\"6\""));
    assert!(json.contains("\"title\":\"九、ジョバンニの切符\""));
    assert!(json.contains("\"pageLabel\":\"42\""));
    assert!(json.contains("\"pageMarks\":["));
    assert!(json.contains("\"sourceStream\":\"/PageMark\""));
    assert!(json.contains("\"family\":\"fixed84\""));
    assert!(json.contains("\"headerCount\":74"));
    assert!(json.contains("\"entryCount\":75"));
    assert!(json.contains("\"lineStart\":23"));
    assert!(json.contains("\"lineEnd\":40"));
    assert!(json.contains("\"paperMarks\":["));
    assert!(json.contains("\"sourceStream\":\"/PaperMark\""));
    assert!(json.contains("\"headerCount\":74"));
    assert!(json.contains("\"headerStride\":12"));
    assert!(json.contains("\"entryCount\":75"));
    assert!(json.contains("\"flagsHex\":\"0x00010010\""));
    assert!(json.contains("\"decoded\":false"));
}

#[test]
fn local_tsaiten_exports_page_mark_u16_subrecord_candidates_when_reference_pdf_is_available() {
    let sample_dir = local_sample_dir();
    let sample_path = sample_dir.join("ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd");
    let reference_pdf_path = sample_dir.join("ichitaro-20030120132956-0007-sp-dat-tsaiten.pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
    let json = to_json(&document);

    assert!(json.contains("\"family\":\"count-plus-one-variable\""));
    assert!(json.contains(
            "\"u16SubrecordScan\":{\"source\":\"/PageMark raw u16 subrecord scan\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
        ));
    assert!(json.contains(
            "\"entryRelativeByteOffset\":162,\"streamByteOffset\":174,\"wordIndex\":81,\"words\":[2,5,768,0,85,0,140,0],\"wordsHex\":[\"0x0002\",\"0x0005\",\"0x0300\",\"0x0000\",\"0x0055\",\"0x0000\",\"0x008c\",\"0x0000\"]"
        ));
    assert!(json.contains(
            "\"entryRelativeByteOffset\":48,\"streamByteOffset\":334,\"wordIndex\":24,\"words\":[4,1,768,0,192,0,241,0],\"wordsHex\":[\"0x0004\",\"0x0001\",\"0x0300\",\"0x0000\",\"0x00c0\",\"0x0000\",\"0x00f1\",\"0x0000\"]"
        ));
}

#[test]
fn local_success_data_test_exports_embedding_frame_candidates_when_reference_pdf_is_available() {
    let sample_dir = local_sample_dir();
    let sample_path = sample_dir.join("ichitaro-20030228030923-success-002-success_data-test.jtd");
    let reference_pdf_path =
        sample_dir.join("ichitaro-20030228030923-success-002-success_data-test.pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
    let json = to_json(&document);

    assert!(json.contains("\"pageMarks\":["));
    assert!(json.contains("\"rawLength\":84,\"rawHex\":\"00000000000100000000000000000027"));
    assert!(json.contains("\"u16Fields\":[0,0,1,0,0,0,0,39,0,0,370,0"));
    assert!(json.contains("\"u16FieldsHex\":[\"0x0000\",\"0x0000\",\"0x0001\",\"0x0000\""));
    assert!(json.contains("\"u16GeometryClass\":\"additive-boundary\""));
    assert!(json.contains("\"u32Fields\":[0,65536,0,39,0,24248320,370,12124160"));
    assert!(
        json.contains(
            "\"u32FieldsHex\":[\"0x00000000\",\"0x00010000\",\"0x00000000\",\"0x00000027\""
        )
    );
    assert!(json.contains(
            "\"u16GeometryHypotheses\":{\"source\":\"/PageMark\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"profile\":\"additive-boundary\""
        ));
    assert!(json.contains(
            "\"word20Is0x00ff\":true,\"word13PlusWord14\":555,\"word13PlusWord14EqualsWord21\":true,\"word21MinusWord13\":185,\"word21MinusWord13EqualsWord14\":true,\"word19EqualsWord13\":true,\"selectedFieldsAllZero\":false,\"nonZeroAdditiveUnitCandidate\":true,\"layoutComparisons\":null"
        ));
    assert!(json.contains("\"objectEmbeddingFrames\":["));
    assert!(json.contains("\"sourcePath\":\"/EmbedItems/EmbeddingInfo\""));
    assert!(json.contains("\"embeddingIndex\":24"));
    assert!(json.contains("\"className\":\"JSFart.Art.2\""));
    assert!(json.contains("\"frameRef\":1"));
    assert!(json.contains("\"frameSize\":{\"width\":13260,\"height\":1327}"));
    assert!(json.contains("\"embeddedPressSnapshot\":{\"format\":\"JSSnapShot32\""));
    assert!(json.contains("\"bodyLengthCandidate\":113332"));
    assert!(json.contains("\"width\":13260"));
    assert!(json.contains("\"height\":1327"));
    assert!(json.contains("\"textureBezierHeaderSummary\":{\"pathCount\":530,\"pointCount\":13,\"byteCount\":104,\"flags\":1,\"flagsHex\":\"0x00000001\",\"homogeneous\":true}"));
    assert!(json.contains("\"paintStateTransitions\":["));
    assert!(json.contains(
        "\"pathKind\":\"outline\",\"startPathIndex\":0,\"endPathIndex\":10,\"pathCount\":11"
    ));
    assert!(json.contains(
            "\"currentState\":{\"record48Word0\":\"0x00000001\",\"record70Word0\":\"0x0000002c\",\"record70Word3\":\"0x0000000a\",\"record82Word5\":\"0x0000002f\"}"
        ));
    assert!(json.contains(
        "\"pathKind\":\"texture\",\"startPathIndex\":11,\"endPathIndex\":540,\"pathCount\":530"
    ));
    assert!(json.contains(
        "\"pathKind\":\"outline\",\"startPathIndex\":541,\"endPathIndex\":551,\"pathCount\":11"
    ));
    assert!(json.contains("\"stateRecordSummary\":{\"pathCount\":"));
    assert!(json.contains("\"recordTypeHex\":\"0x00000082\""));
    assert!(json.contains("\"paintState82Preview\":[{"));
    assert!(json.contains("\"word3CandidateHex\":"));
    assert!(json.contains("\"word5CandidateHex\":"));
    assert!(json.contains("\"jsfartStreamProfile\":{\"format\":\"JSFart2Contents\""));
    assert!(json.contains("\"magicFamily\":\"mstudio-ocx-utf16le\""));
    assert!(json.contains("\"magicFamilyHex\":\"4d00\""));
    assert!(json.contains("\"structuredArtCandidatePresent\":true"));
    assert!(json.contains(
        "\"renderPromotionBlockedReason\":\"structured-jsfart-art-still-paint-authority-unproven\""
    ));
    assert!(json.contains("\"jsfartArt\":{\"format\":\"JSFart2Contents\""));
    assert!(json.contains("\"magic\":\"MSTUDIO.OCX\""));
    assert!(
        json.contains("\"frameCandidate\":{\"left\":0,\"top\":0,\"right\":13260,\"bottom\":1327")
    );
    assert!(json.contains(
        "\"contentLeft\":114,\"contentTop\":105,\"contentRight\":13145,\"contentBottom\":1159"
    ));
    assert!(json.contains("\"strokeWidthCandidate\":100"));
    assert!(
        json.contains(
            "\"paintCandidate\":{\"styleWord1\":34869296,\"styleWord1Hex\":\"0x02141030\""
        )
    );
    assert!(
        json.contains("\"paintColorCandidate\":16777215,\"paintColorCandidateHex\":\"0x00ffffff\"")
    );
    assert!(json.contains("\"effectWordCandidate\":10,\"effectWordCandidateHex\":\"0x0000000a\""));
    assert!(json.contains("\"embeddingIndex\":4"));
    assert!(json.contains("\"className\":\"JSEQ.Document.3\""));
    assert!(json.contains("\"jseq3Formula\":{\"format\":\"JSEQ3Contents\""));
    assert!(json.contains("\"magic\":\"MATH.VAF\""));
    assert!(json.contains("\"soTrailerOffset\":1658"));
    assert!(json.contains("\"soTrailerLength\":62"));
    assert!(json.contains("\"text\":\"Times New Roman\""));
    assert!(json.contains("\"path\":\"/FigureData/ExpandData/main_data/Link\""));
    assert!(json.contains("\"figureLink\":{\"headerWordsBe\":[11,1,0,15]"));
    assert!(json.contains("\"declaredRowCountCandidate\":15"));
    assert!(json.contains("\"rowStride\":14"));
    assert!(json.contains("\"rowCount\":15"));
    assert!(json.contains("\"relationKindCandidateHex\":\"0x0016\""));
    assert!(json.contains("\"path\":\"/FigureData/main_data/FDMVector\""));
    assert!(json.contains("\"fdmRawVectorSegmentCount\":5"));
    assert!(json.contains("\"fdmRawVectorCommandCount\":37"));
    assert!(json.contains("\"offsetFieldReferenceCandidates\":[{\"offsetField\":\"bbox.left\",\"offsetValue\":308,\"matchKind\":\"command-relative-offset-field\",\"referenceSource\":\"fdmRawVectorCommands.relativeOffset\",\"matchedCommandRelativeOffsets\":[308],\"decoded\":false}]"));
    assert!(json.contains("\"offsetFieldReferenceCandidates\":[{\"offsetField\":\"bbox.left\",\"offsetValue\":690,\"matchKind\":\"source-segment-relative-offset-field\",\"referenceSource\":\"fdmRawVectorCommands.sourceSegment.relativeOffset\",\"sourceSegmentRelativeOffset\":690,\"sourceSegmentBackedCommandCount\":1,\"matchedCommandRelativeOffsets\":[874],\"decoded\":false}]"));
    assert!(json.contains("\"offsetFieldReferenceCandidates\":[{\"offsetField\":\"bbox.left\",\"offsetValue\":1864,\"matchKind\":\"source-segment-relative-offset-field\",\"referenceSource\":\"fdmRawVectorCommands.sourceSegment.relativeOffset\",\"sourceSegmentRelativeOffset\":1864,\"sourceSegmentBackedCommandCount\":4,\"matchedCommandRelativeOffsets\":[1924,1958,1992,2024],\"decoded\":false}]"));
    assert!(json.contains("\"sourceVectorRelativeOffset\":208,\"sourceSegment\":null"));
    assert!(json.contains(
            "\"sourceVectorRelativeOffset\":1992,\"sourceSegment\":{\"relativeOffset\":1864,\"localOffset\":128,\"declaredLength\":236,\"commandCount\":4,\"commandIndex\":2,\"commandOffset\":128}"
        ));
    assert!(
        json.contains(
            "\"successDataTestFdmReferenceProjections\":[{\"role\":\"q4-angle-diagrams\""
        )
    );
    assert!(
        json.contains("\"referenceTargetBboxPx\":{\"x\":93.300,\"y\":663.300,\"width\":491.400")
    );
    assert!(json.contains(
            "\"commandRelativeOffsets\":[308,342,374,406,438,470,504,538,570,602,634,874,1048,1126,1158,1190,1430,1604,1730,1780]"
        ));
    assert!(json.contains("\"renderPromotionBlockedReason\":\"mixed-raw-and-segment-cohorts\""));
    assert!(json.contains("\"primitiveOwnershipComparison\":{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":20,\"mainCircleAnchorCount\":3,\"lineCandidateCount\":11,\"radialLineCandidateCount\":0,\"chordCandidateCount\":0,\"arcCandidateCount\":6,\"connectorCandidateCount\":8,\"surfaceBoundaryCandidateCount\":2"));
    assert!(json.contains(
        "\"indexRowReferenceCandidateCount\":20,\"validVectorOffsetIndexRowReferenceCount\":0"
    ));
    assert_json_string_field_after(
        &json,
        "\"ownershipGate\":{",
        0,
        "renderOwnershipBlockedReason",
        "mixed-raw-and-segment-cohorts",
    );
    assert_json_string_array_field_after(
        &json,
        "\"ownershipGate\":{",
        0,
        "renderOwnershipBlockedReasons",
        &["mixed-raw-and-segment-cohorts"],
    );
    assert_json_number_field_after(&json, "\"ownershipGate\":{", 0, "commandCount", "20");
    assert_json_number_field_after(&json, "\"ownershipGate\":{", 0, "rawSpanCommandCount", "18");
    assert_json_number_field_after(
        &json,
        "\"ownershipGate\":{",
        0,
        "segmentBackedCommandCount",
        "2",
    );
    assert_json_bool_field_after(
        &json,
        "\"ownershipGate\":{",
        0,
        "oneToOneRowCommandReferenceCandidate",
        true,
    );
    assert_json_string_field_after(
        &json,
        "\"offsetFieldAuthorityGate\":{",
        0,
        "renderPromotionBlockedReason",
        "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
    );
    assert_json_number_field_after(
        &json,
        "\"offsetFieldAuthorityGate\":{",
        0,
        "commandRelativeOffsetFieldReferenceCount",
        "18",
    );
    assert_json_number_field_after(
        &json,
        "\"offsetFieldAuthorityGate\":{",
        0,
        "sourceSegmentRelativeOffsetFieldReferenceCount",
        "2",
    );
    assert_json_string_field_after(
        &json,
        "\"rowFanoutSegmentOwnerGate\":{",
        0,
        "renderPromotionBlockedReason",
        "fdm-index-row-fanout-segment-owner-offset-namespace-mixed",
    );
    assert_json_number_field_after(
        &json,
        "\"rowFanoutSegmentOwnerGate\":{",
        0,
        "maxRowFanout",
        "1",
    );
    assert_json_bool_field_after(
        &json,
        "\"rowFanoutSegmentOwnerGate\":{",
        0,
        "singleRowBacksMultipleCommandsCandidate",
        false,
    );
    assert_json_string_field_after(
        &json,
        "\"primitiveOwnershipAdmissionGate\":{",
        0,
        "renderPromotionBlockedReason",
        "mixed-raw-and-segment-cohorts",
    );
    assert_json_string_array_field_after(
        &json,
        "\"primitiveOwnershipAdmissionGate\":{",
        0,
        "renderPromotionBlockedReasons",
        &[
            "mixed-raw-and-segment-cohorts",
            "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
            "fdm-index-row-fanout-segment-owner-offset-namespace-mixed",
            "fdm-index-role-vector-offset-authority-valid-vector-offset-missing",
            "fdm-index-role-valid-vector-offset-missing",
            "role-paint-order-continuity-unproven",
        ],
    );
    assert_json_number_field_after(
        &json,
        "\"primitiveOwnershipAdmissionGate\":{",
        0,
        "rolePaintOrderBlockedGroupCount",
        "6",
    );
    assert_json_string_field_after(
        &json,
        "\"indexRowOrderPromotionGate\":{",
        0,
        "renderPromotionBlockedReason",
        "fdm-index-row-order-valid-vector-offset-missing",
    );
    assert_json_string_array_field_after(
        &json,
        "\"indexRowOrderPromotionGate\":{",
        0,
        "renderPromotionBlockedReasons",
        &[
            "fdm-index-row-order-valid-vector-offset-missing",
            "fdm-index-row-order-offset-namespace-mixed",
            "role-paint-order-continuity-unproven",
        ],
    );
    assert_json_number_field_after(
        &json,
        "\"indexRowOrderPromotionGate\":{",
        0,
        "uniqueRowIndexCount",
        "20",
    );
    assert!(json.contains("\"renderPaintOrderBasisCandidate\":\"fdm-index-row-command-pairs\",\"renderPaintOrderBasisDecoded\":false"));
    assert!(json.contains("\"roleCandidate\":\"main-circle-anchor\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":3,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":3,\"sourceSegmentRelativeOffsetFieldReferenceCount\":0,\"commandRelativeOffsets\":[308,470,504],\"rowIndexes\":[7,12,13],\"uniqueCommandRelativeOffsetCount\":3,\"uniqueRowIndexCount\":3,\"oneToOneRowCommandReferenceCandidate\":true,\"singleRowBacksMultipleCommandsCandidate\":false,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":7,\"commandRelativeOffset\":308,\"matchKind\":\"command-relative-offset-field\"}"));
    assert!(json.contains("\"paintOrderContinuityProfile\":{\"basis\":\"fdm-index-row-reference-role-command-span\",\"decoded\":false,\"sourceBacked\":true,\"paintOrderDecoded\":false,\"commandRelativeOffsetSpanMin\":308,\"commandRelativeOffsetSpanMax\":504,\"roleCommandCount\":3,\"commandCountInSpan\":7,\"interleavedNonRoleCommandCount\":4,\"hasInterleavedNonRoleCommands\":true,\"maxCommandOffsetGap\":162,\"commandOffsetContinuityScore\":0.429,\"spanContiguousCandidate\":false,\"paintOrderAuthorityPending\":false,\"continuityBlocked\":true,\"renderPromotionBlockedReason\":\"role-span-interleaved-non-role-commands\"}"));
    assert!(json.contains("\"roleCandidate\":\"radial-line-candidate\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":2,\"sourceSegmentRelativeOffsetFieldReferenceCount\":0,\"commandRelativeOffsets\":[342,406],\"rowIndexes\":[8,10],\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":2,\"oneToOneRowCommandReferenceCandidate\":true,\"singleRowBacksMultipleCommandsCandidate\":false,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":8,\"commandRelativeOffset\":342,\"matchKind\":\"command-relative-offset-field\"},{\"rowIndex\":10,\"commandRelativeOffset\":406,\"matchKind\":\"command-relative-offset-field\"}],\"roleVectorOffsetAuthorityGate\":"));
    assert!(json.contains("\"primitiveOwnershipComparison\":{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":7,\"mainCircleAnchorCount\":1,\"lineCandidateCount\":4,\"radialLineCandidateCount\":2,\"chordCandidateCount\":2,\"arcCandidateCount\":2,\"connectorCandidateCount\":2,\"surfaceBoundaryCandidateCount\":2"));
    assert!(json.contains("\"relativeOffset\":374,\"primitiveKind\":\"polyline\",\"markerHex\":\"01000160\",\"sourceSegmentBacked\":false,\"sourceSegmentRelativeOffset\":null,\"roleCandidates\":[\"line-candidate\",\"chord-candidate\",\"connector-candidate\"]"));
    assert!(json.contains("\"indexRowReferenceCandidates\":[{\"rowIndex\":9,\"indexOffset\":218,\"vectorOffset\":3663724543,\"validVectorOffset\":false,\"offsetField\":\"bbox.left\",\"offsetValue\":374,\"matchKind\":\"command-relative-offset-field\",\"decoded\":false}]"));
    assert!(json.contains("\"relativeOffset\":1430,\"primitiveKind\":\"ellipse\",\"markerHex\":\"ff000460\",\"sourceSegmentBacked\":true,\"sourceSegmentRelativeOffset\":1246,\"roleCandidates\":[\"arc-candidate\",\"control-ellipse-marker\"]"));
    assert!(json.contains("\"indexRowReferenceCandidates\":[{\"rowIndex\":32,\"indexOffset\":724,\"vectorOffset\":3671785471,\"validVectorOffset\":false,\"offsetField\":\"bbox.left\",\"offsetValue\":1246,\"matchKind\":\"source-segment-relative-offset-field\",\"decoded\":false}]"));
    assert!(json.contains(
        "\"subdiagrams\":[{\"index\":0,\"groupingSource\":\"nearest-main-circle-source-center\""
    ));
    assert!(json.contains("\"role\":\"q5-solid-diagram\""));
    assert!(json.contains(
            "\"referenceTargetBboxPx\":{\"x\":490.700,\"y\":795.000,\"width\":74.600,\"height\":110.000}"
        ));
    assert!(json.contains("\"commandRelativeOffsets\":[1830,1924,1958,1992,2024,2156,2190]"));
    assert!(json.contains("\"primitiveOwnershipComparison\":{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":7,\"mainCircleAnchorCount\":0,\"lineCandidateCount\":2,\"radialLineCandidateCount\":0,\"chordCandidateCount\":0,\"arcCandidateCount\":4,\"connectorCandidateCount\":3,\"surfaceBoundaryCandidateCount\":1"));
    assert!(json.contains(
        "\"indexRowReferenceCandidateCount\":7,\"validVectorOffsetIndexRowReferenceCount\":0"
    ));
    assert_json_string_field_after(
        &json,
        "\"ownershipGate\":{",
        1,
        "renderOwnershipBlockedReason",
        "multi-command-single-index-row",
    );
    assert_json_string_array_field_after(
        &json,
        "\"ownershipGate\":{",
        1,
        "renderOwnershipBlockedReasons",
        &[
            "multi-command-single-index-row",
            "mixed-raw-and-segment-cohorts",
            "row-command-reference-not-one-to-one",
        ],
    );
    assert_json_number_field_after(&json, "\"ownershipGate\":{", 1, "commandCount", "7");
    assert_json_number_field_after(&json, "\"ownershipGate\":{", 1, "rawSpanCommandCount", "1");
    assert_json_number_field_after(
        &json,
        "\"ownershipGate\":{",
        1,
        "segmentBackedCommandCount",
        "6",
    );
    assert_json_bool_field_after(
        &json,
        "\"ownershipGate\":{",
        1,
        "oneToOneRowCommandReferenceCandidate",
        false,
    );
    assert_json_string_field_after(
        &json,
        "\"offsetFieldAuthorityGate\":{",
        1,
        "renderPromotionBlockedReason",
        "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
    );
    assert_json_number_field_after(
        &json,
        "\"offsetFieldAuthorityGate\":{",
        1,
        "commandRelativeOffsetFieldReferenceCount",
        "1",
    );
    assert_json_number_field_after(
        &json,
        "\"offsetFieldAuthorityGate\":{",
        1,
        "sourceSegmentRelativeOffsetFieldReferenceCount",
        "6",
    );
    assert_json_string_field_after(
        &json,
        "\"rowFanoutSegmentOwnerGate\":{",
        1,
        "renderPromotionBlockedReason",
        "fdm-index-row-fanout-segment-owner-multi-command-single-row",
    );
    assert_json_number_field_after(
        &json,
        "\"rowFanoutSegmentOwnerGate\":{",
        1,
        "maxRowFanout",
        "4",
    );
    assert_json_bool_field_after(
        &json,
        "\"rowFanoutSegmentOwnerGate\":{",
        1,
        "singleRowBacksMultipleCommandsCandidate",
        true,
    );
    assert_json_string_field_after(
        &json,
        "\"primitiveOwnershipAdmissionGate\":{",
        1,
        "renderPromotionBlockedReason",
        "multi-command-single-index-row",
    );
    assert_json_string_array_field_after(
        &json,
        "\"primitiveOwnershipAdmissionGate\":{",
        1,
        "renderPromotionBlockedReasons",
        &[
            "multi-command-single-index-row",
            "mixed-raw-and-segment-cohorts",
            "row-command-reference-not-one-to-one",
            "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
            "fdm-index-row-fanout-segment-owner-multi-command-single-row",
            "fdm-index-role-row-fanout-multi-command-single-row",
            "fdm-index-role-vector-offset-authority-valid-vector-offset-missing",
            "fdm-index-role-valid-vector-offset-missing",
            "role-paint-order-continuity-unproven",
            "role-paint-order-authority-unproven",
        ],
    );
    assert_json_number_field_after(
        &json,
        "\"primitiveOwnershipAdmissionGate\":{",
        1,
        "rolePaintOrderBlockedGroupCount",
        "2",
    );
    assert_json_number_field_after(
        &json,
        "\"primitiveOwnershipAdmissionGate\":{",
        1,
        "rolePaintOrderAuthorityPendingGroupCount",
        "2",
    );
    assert_json_string_field_after(
        &json,
        "\"indexRowOrderPromotionGate\":{",
        1,
        "renderPromotionBlockedReason",
        "fdm-index-row-order-reference-not-one-to-one",
    );
    assert_json_string_array_field_after(
        &json,
        "\"indexRowOrderPromotionGate\":{",
        1,
        "renderPromotionBlockedReasons",
        &[
            "fdm-index-row-order-reference-not-one-to-one",
            "fdm-index-row-order-single-row-backs-multiple-commands",
            "fdm-index-row-order-valid-vector-offset-missing",
            "fdm-index-row-order-offset-namespace-mixed",
            "role-paint-order-continuity-unproven",
            "role-paint-order-authority-unproven",
        ],
    );
    assert_json_number_field_after(
        &json,
        "\"indexRowOrderPromotionGate\":{",
        1,
        "uniqueRowIndexCount",
        "3",
    );
    assert!(json.contains("\"roleCandidate\":\"line-candidate\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":0,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"commandRelativeOffsets\":[1992,2024],\"rowIndexes\":[40],\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":1,\"oneToOneRowCommandReferenceCandidate\":false,\"singleRowBacksMultipleCommandsCandidate\":true,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":40,\"commandRelativeOffset\":1992,\"matchKind\":\"source-segment-relative-offset-field\"},{\"rowIndex\":40,\"commandRelativeOffset\":2024,\"matchKind\":\"source-segment-relative-offset-field\"}],\"roleVectorOffsetAuthorityGate\":{\"basis\":\"fdm-index-role-vector-offset-authority-gate\",\"source\":\"FDMIndex.vectorOffset+FDMIndex role offset fields\",\"decoded\":false,\"sourceBacked\":true,\"roleCandidate\":\"line-candidate\",\"roleVectorOffsetAuthorityDecoded\":false,\"renderPromotionContribution\":\"fdm-index-role-vector-offset-authority-gate\",\"renderPromotionBlockedReason\":\"fdm-index-role-vector-offset-authority-valid-vector-offset-missing\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"invalidVectorOffsetReferenceCount\":2,\"commandRelativeOffsetFieldReferenceCount\":0,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"validCommandRelativeOffsetFieldReferenceCount\":0,\"validSourceSegmentRelativeOffsetFieldReferenceCount\":0,\"invalidCommandRelativeOffsetFieldReferenceCount\":0,\"invalidSourceSegmentRelativeOffsetFieldReferenceCount\":2,\"allValidReferencesUseCommandRelativeOffsetField\":false,\"allValidReferencesUseSourceSegmentRelativeOffsetField\":false,\"mixedOffsetNamespacesAmongValidReferences\":false,\"allReferencesHaveInvalidVectorOffset\":true},\"roleFanoutSegmentOwnerGate\":{\"basis\":\"fdm-index-role-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex role row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true,\"roleCandidate\":\"line-candidate\",\"roleOwnershipDecoded\":false,\"segmentOwnerDecoded\":false,\"renderPromotionContribution\":\"fdm-index-role-row-fanout-segment-owner-gate\",\"renderPromotionBlockedReason\":\"fdm-index-role-row-fanout-multi-command-single-row\",\"referenceCount\":2,\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":1,\"commandRelativeOffsetFieldReferenceCount\":0,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"fanoutRowCount\":1,\"fanoutReferenceCount\":2,\"fanoutCommandRelativeOffsetFieldReferenceCount\":0,\"fanoutSourceSegmentRelativeOffsetFieldReferenceCount\":2,\"maxRowFanout\":2,\"oneToOneRowCommandReferenceCandidate\":false,\"singleRowBacksMultipleCommandsCandidate\":true,\"mixedOffsetFieldNamespaces\":false,\"fanoutRowsUseCommandRelativeOffsetFields\":false,\"fanoutRowsUseSourceSegmentOffsetFields\":true,\"rowsWithMultipleCommandRefs\":[{\"rowIndex\":40,\"commandReferenceCount\":2,\"commandRelativeOffsets\":[1992,2024],\"matchKinds\":[\"source-segment-relative-offset-field\"]}]}"));
    assert!(json.contains("\"paintOrderContinuityProfile\":{\"basis\":\"fdm-index-row-reference-role-command-span\",\"decoded\":false,\"sourceBacked\":true,\"paintOrderDecoded\":false,\"commandRelativeOffsetSpanMin\":1992,\"commandRelativeOffsetSpanMax\":2024,\"roleCommandCount\":2,\"commandCountInSpan\":2,\"interleavedNonRoleCommandCount\":0,\"hasInterleavedNonRoleCommands\":false,\"maxCommandOffsetGap\":32,\"commandOffsetContinuityScore\":1.000,\"spanContiguousCandidate\":true,\"paintOrderAuthorityPending\":true,\"continuityBlocked\":false,\"renderPromotionBlockedReason\":\"role-paint-order-authority-unproven\"}"));
    assert!(json.contains("\"relativeOffset\":1992,\"primitiveKind\":\"polyline\",\"markerHex\":\"ff000160\",\"sourceSegmentBacked\":true,\"sourceSegmentRelativeOffset\":1864,\"roleCandidates\":[\"line-candidate\",\"connector-candidate\"]"));
    assert!(json.contains("\"indexRowReferenceCandidates\":[{\"rowIndex\":40,\"indexOffset\":900,\"vectorOffset\":3729719295,\"validVectorOffset\":false,\"offsetField\":\"bbox.left\",\"offsetValue\":1864,\"matchKind\":\"source-segment-relative-offset-field\",\"decoded\":false}]"));
    assert!(json.contains("\"primitiveKind\":\"cubicBezier\""));
    assert!(json.contains("\"primitiveKind\":\"ellipse\""));
    assert!(json.contains("\"curveSegmentCount\":1"));
    assert!(json.contains("\"ellipse\":{\"center\":{\"x\":-11280,\"y\":-10792},\"radiusX\":556"));
    assert!(json.contains("\"path\":\"/FigureData/ExpandData/main_data/Data/FDMText\""));
    assert!(json.contains("\"fdmTextCount\":15"));
    assert!(json.contains("\"fdmTextIndexEntries\":["));
    assert!(json.contains("\"text\":\"９㎝\""));
    assert!(json.contains("\"textRecordOffset\":6584"));
    assert!(json.contains("\"kind\":\"sparseDocumentTextControlRunTableCandidate\""));
    assert!(json.contains("\"rule\":\"sparse-document-text-001c-cells-with-000e-row-breaks\""));
    assert!(json.contains("\"textPreview\":\"\\t\\t\\t(1)表面積の比"));
    assert!(json.contains("\"sparseObservedTable\":{\"source\":\"sparseDocumentTextControlRows\""));
    assert!(json.contains("\"topologyCandidate\":{\"source\":\"sparseDocumentTextControlRows\""));
    assert!(
        json.contains("\"sparseTopologyCandidate\":{\"source\":\"sparseDocumentTextControlRows\"")
    );
    assert!(json.contains("\"columns\":["));
    assert!(json.contains("\"firstNonEmptyColumnIndex\":3"));
    assert!(json.contains("\"emptyCellCountCandidate\":136"));
    assert!(json.contains("\"rows\":["));
    assert!(json.contains("\"cells\":["));
    assert!(json.contains("\"empty\":true"));
    assert!(json.contains("\"sourceStart\":2902"));
    assert!(json.contains("\"sourceEnd\":5419"));
    assert!(json.contains("\"geometryDecoded\":false"));
}

#[test]
fn local_shanai_lan_exports_fdm_vector_command_diagnostics_when_reference_pdf_is_available() {
    let sample_dir = local_sample_dir();
    let sample_path =
        sample_dir.join("ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd");
    let reference_pdf_path =
        sample_dir.join("ichitaro-20030315134715-success-001-success_data-shanai_lan.pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
    let json = to_json(&document);

    assert!(json.contains("\"path\":\"/FigureData/main_data/FDMVector\""));
    assert!(json.contains("\"fdmIndexEntries\":["));
    assert!(json.contains("\"vectorCommandCount\":"));
    assert!(json.contains("\"vectorCommandBboxCount\":"));
    assert!(json.contains("\"vectorCommands\":[{"));
    assert!(json.contains("\"connectorCandidateCount\":"));
    assert!(json.contains("\"connectorCandidates\":[{"));
    assert!(json.contains("\"candidateBasis\":\"long-open-source-path\""));
    assert!(json.contains("\"sourceEndpoints\":{\"start\":{\"x\":"));
    assert!(json.contains("\"sourceSpan\":"));
    assert!(json.contains("\"endpointDistanceSquared\":"));
    assert!(json.contains("\"fillColor\":"));
    assert!(json.contains("\"strokeColor\":"));
    assert!(json.contains("\"pathSegmentCount\":"));
    assert!(json.contains("\"orthogonalSegmentCount\":"));
    assert!(json.contains("\"diagonalSegmentCount\":"));
    assert!(json.contains("\"compoundChildOffsetCount\":"));
    assert!(json.contains("\"axisAligned\":"));
    assert!(json.contains("\"orientation\":\"horizontal\""));
    assert!(json.contains("\"markerHex\":\"00000960\""));
    assert!(json.contains("\"primitiveKind\":\"cubicBezier\""));
    assert!(json.contains("\"pathPoints\":[{\"x\":"));
    assert!(json.contains("\"curveSegments\":[{\"control1\":"));
    assert!(json.contains("\"compoundChildOffsets\":["));
    assert!(json.contains("\"decoded\":false"));
}

#[test]
fn local_200307_shanai_lan_exports_json_without_fdm_projection_overflow() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let sample_path = project_root
        .join("rjtd-testdata/local-samples")
        .join("ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd");
    let pdf_output_path = project_root
        .join("openjtd-samples/pdf-output")
        .join("ichitaro-20030706232827-success-001-success_data-shanai_lan.pdf");
    if !sample_path.exists() || !pdf_output_path.exists() {
        return;
    }

    let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
    let json = to_json(&document);

    assert!(json.contains("\"objectStreamCandidates\":["));
    assert!(json.contains("\"path\":\"/FigureData/main_data/FDMVector\""));
    assert!(json.contains("\"successDataTestFdmReferenceProjections\":["));
}
