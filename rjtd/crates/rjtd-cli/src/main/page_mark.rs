use rjtd_core::container::{inspect_cfb_stream_location, read_cfb_stream};
use rjtd_core::layout_mark::{read_page_mark, read_paper_mark};
use rjtd_model::page_mark_u16_geometry_profile;

use crate::input::read_file;

use super::page_mark_support::*;
use super::support::*;

pub(crate) fn run_paper_marks(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "paper-marks")?;
    let bytes = read_file(path)?;
    let paper_mark = read_paper_mark(&bytes).map_err(|error| error.to_string())?;
    let header = paper_mark.header();
    write_stdout_line(&format!(
        "header\t{}\t{}\t{}\t{}",
        header.count_value(),
        header.stride_value(),
        header.last_index_value(),
        paper_mark.entries().len()
    ))?;
    for entry in paper_mark.entries() {
        write_stdout_line(&format!(
            "entry\t{}\t0x{:08x}",
            entry.index(),
            entry.flags()
        ))?;
    }
    Ok(())
}

pub(crate) fn run_paper_mark_shape(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "paper-mark-shape")?;
    let bytes = read_file(path)?;
    let location =
        inspect_cfb_stream_location(&bytes, "/PaperMark").map_err(|error| error.to_string())?;
    let stream = read_cfb_stream(&bytes, "/PaperMark").map_err(|error| error.to_string())?;
    write_stdout_line(&format!(
        "stream\t{}\t{}\t{}",
        stream.len(),
        location.size(),
        location.storage().as_str()
    ))?;
    write_stdout_line(&format!(
        "alignment\tu32\t{}",
        stream.len().is_multiple_of(4)
    ))?;

    if stream.len() < 12 {
        write_stdout_line("header\t-\t-\t-")?;
        return Ok(());
    }

    let header_count = read_be32_candidate(&stream, 0);
    let header_stride = read_be32_candidate(&stream, 4);
    let header_last = read_be32_candidate(&stream, 8);
    write_stdout_line(&format!(
        "header\t{}\t{}\t{}",
        header_count, header_stride, header_last
    ))?;

    let tail_bytes = stream.len() - 12;
    let classification =
        classify_paper_mark_shape(tail_bytes, header_count, header_stride, header_last);
    write_stdout_line(&format!(
        "classification\t{}\t{}\t{}\t{}",
        classification.name,
        format_optional_usize(classification.rows),
        format_optional_usize(classification.row_bytes),
        classification.trim_bytes
    ))?;
    write_fixed_row_candidate("fixed8", tail_bytes, 8)?;
    write_header_row_candidate("count-plus-one", tail_bytes, header_count.saturating_add(1))?;
    write_header_row_candidate("count", tail_bytes, header_count)?;
    Ok(())
}

pub(crate) fn run_page_marks(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "page-marks")?;
    let bytes = read_file(path)?;
    let page_mark = read_page_mark(&bytes).map_err(|error| error.to_string())?;
    let header = page_mark.header();
    write_stdout_line(&format!(
        "header\t{}\t{}\t{}\t{}",
        header.count_value(),
        header.stride_value(),
        header.last_index_value(),
        page_mark.entries().len()
    ))?;
    write_stdout_line(&format!(
        "family\t{}\t{}\t{}",
        page_mark.family().as_str(),
        page_mark
            .entries()
            .first()
            .map(|entry| entry.raw().len().to_string())
            .unwrap_or_else(|| "-".to_string()),
        page_mark.trailing_bytes().len()
    ))?;
    for (row, entry) in page_mark.entries().iter().enumerate() {
        let u16_fields = be16_words(entry.raw()).collect::<Vec<_>>();
        let u16_profile = page_mark_u16_geometry_profile(&u16_fields);
        write_stdout_line(&format!(
            "entry\t{}\t{}\t{}\tflags={}\tlineStart={}\tlineEnd={}\tu16Class={}",
            row,
            format_optional_u32(entry.index()),
            bytes_to_hex(entry.raw()),
            format_optional_u32(entry.flags()),
            format_optional_u32(entry.line_start()),
            format_optional_u32(entry.line_end()),
            u16_profile.class_name()
        ))?;
    }
    if !page_mark.trailing_bytes().is_empty() {
        write_stdout_line(&format!(
            "trailing\t{}",
            bytes_to_hex(page_mark.trailing_bytes())
        ))?;
    }
    Ok(())
}

pub(crate) fn run_page_mark_u16_profile(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "page-mark-u16-profile")?;
    let bytes = read_file(path)?;
    let page_mark = read_page_mark(&bytes).map_err(|error| error.to_string())?;
    write_page_mark_u16_profile(&page_mark)
}

pub(crate) fn run_page_mark_pitch_profile(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "page-mark-pitch-profile")?;
    let bytes = read_file(&path)?;
    let page_mark = read_page_mark(&bytes).map_err(|error| error.to_string())?;
    write_page_mark_pitch_profile(&path, &bytes, &page_mark)
}

pub(crate) fn run_page_mark_shape(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "page-mark-shape")?;
    let bytes = read_file(path)?;
    let location =
        inspect_cfb_stream_location(&bytes, "/PageMark").map_err(|error| error.to_string())?;
    let stream = read_cfb_stream(&bytes, "/PageMark").map_err(|error| error.to_string())?;
    write_stdout_line(&format!(
        "stream\t{}\t{}\t{}",
        stream.len(),
        location.size(),
        location.storage().as_str()
    ))?;
    write_stdout_line(&format!(
        "alignment\tu32\t{}",
        stream.len().is_multiple_of(4)
    ))?;

    if stream.len() < 12 {
        write_stdout_line("header\t-\t-\t-")?;
        return Ok(());
    }

    let header_count = read_be32_candidate(&stream, 0);
    let header_stride = read_be32_candidate(&stream, 4);
    let header_last = read_be32_candidate(&stream, 8);
    write_stdout_line(&format!(
        "header\t{}\t{}\t{}",
        header_count, header_stride, header_last
    ))?;

    let tail_bytes = stream.len() - 12;
    let classification =
        classify_page_mark_shape(tail_bytes, header_count, header_stride, header_last);
    write_stdout_line(&format!(
        "classification\t{}\t{}\t{}\t{}",
        classification.name,
        format_optional_usize(classification.rows),
        format_optional_usize(classification.row_bytes),
        classification.trim_bytes
    ))?;
    write_fixed_row_candidate("fixed84", tail_bytes, 84)?;
    write_header_row_candidate("count-plus-one", tail_bytes, header_count.saturating_add(1))?;
    write_header_row_candidate("count", tail_bytes, header_count)?;
    if tail_bytes >= 2 {
        write_fixed_row_candidate("fixed84-trim2", tail_bytes - 2, 84)?;
        write_header_row_candidate(
            "count-plus-one-trim2",
            tail_bytes - 2,
            header_count.saturating_add(1),
        )?;
        write_header_row_candidate("count-trim2", tail_bytes - 2, header_count)?;
    }
    Ok(())
}
