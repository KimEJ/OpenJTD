use crate::container::read_cfb_stream;
use crate::{Error, Result};

pub const PAGE_MARK_PATH: &str = "/PageMark";
pub const PAPER_MARK_PATH: &str = "/PaperMark";

const PAGE_MARK_HEADER_BYTES: usize = 12;
const PAGE_MARK_ENTRY_BYTES: usize = 84;
const OBSERVED_PAGE_MARK_STRIDE_VALUE: u32 = 0x10;
const PAPER_MARK_HEADER_BYTES: usize = 12;
const PAPER_MARK_ENTRY_BYTES: usize = 8;
const OBSERVED_PAPER_MARK_STRIDE_VALUE: u32 = 0x0c;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageMark {
    header: PageMarkHeader,
    family: PageMarkFamily,
    entries: Vec<PageMarkEntry>,
    trailing_bytes: Vec<u8>,
}

impl PageMark {
    fn new(
        header: PageMarkHeader,
        family: PageMarkFamily,
        entries: Vec<PageMarkEntry>,
        trailing_bytes: Vec<u8>,
    ) -> Self {
        Self {
            header,
            family,
            entries,
            trailing_bytes,
        }
    }

    pub fn header(&self) -> PageMarkHeader {
        self.header
    }

    pub fn family(&self) -> PageMarkFamily {
        self.family
    }

    pub fn entries(&self) -> &[PageMarkEntry] {
        &self.entries
    }

    pub fn trailing_bytes(&self) -> &[u8] {
        &self.trailing_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageMarkFamily {
    Fixed84,
    CountPlusOneVariable,
    CountPlusOneTrim2,
    CountVariable,
    Fixed84Tail,
}

impl PageMarkFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed84 => "fixed84",
            Self::CountPlusOneVariable => "count-plus-one-variable",
            Self::CountPlusOneTrim2 => "count-plus-one-trim2",
            Self::CountVariable => "count-variable",
            Self::Fixed84Tail => "fixed84-tail",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageMarkHeader {
    count_value: u32,
    stride_value: u32,
    last_index_value: u32,
}

impl PageMarkHeader {
    fn new(count_value: u32, stride_value: u32, last_index_value: u32) -> Self {
        Self {
            count_value,
            stride_value,
            last_index_value,
        }
    }

    pub fn count_value(self) -> u32 {
        self.count_value
    }

    pub fn stride_value(self) -> u32 {
        self.stride_value
    }

    pub fn last_index_value(self) -> u32 {
        self.last_index_value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageMarkEntry {
    raw: Vec<u8>,
}

impl PageMarkEntry {
    fn new(raw: Vec<u8>) -> Self {
        Self { raw }
    }

    pub fn index(&self) -> Option<u32> {
        self.u32_field(0)
    }

    pub fn flags(&self) -> Option<u32> {
        self.u32_field(1)
    }

    pub fn line_start(&self) -> Option<u32> {
        self.u32_field(2)
    }

    pub fn line_end(&self) -> Option<u32> {
        self.u32_field(3)
    }

    pub fn u32_fields(&self) -> Vec<u32> {
        self.raw
            .chunks_exact(4)
            .map(|chunk| u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect()
    }

    pub fn u32_field(&self, index: usize) -> Option<u32> {
        let offset = index.checked_mul(4)?;
        let bytes = self.raw.get(offset..offset + 4)?;
        Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaperMark {
    header: PaperMarkHeader,
    entries: Vec<PaperMarkEntry>,
}

impl PaperMark {
    fn new(header: PaperMarkHeader, entries: Vec<PaperMarkEntry>) -> Self {
        Self { header, entries }
    }

    pub fn header(&self) -> PaperMarkHeader {
        self.header
    }

    pub fn entries(&self) -> &[PaperMarkEntry] {
        &self.entries
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaperMarkHeader {
    count_value: u32,
    stride_value: u32,
    last_index_value: u32,
}

impl PaperMarkHeader {
    fn new(count_value: u32, stride_value: u32, last_index_value: u32) -> Self {
        Self {
            count_value,
            stride_value,
            last_index_value,
        }
    }

    pub fn count_value(self) -> u32 {
        self.count_value
    }

    pub fn stride_value(self) -> u32 {
        self.stride_value
    }

    pub fn last_index_value(self) -> u32 {
        self.last_index_value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaperMarkEntry {
    index: u32,
    flags: u32,
}

impl PaperMarkEntry {
    fn new(index: u32, flags: u32) -> Self {
        Self { index, flags }
    }

    pub fn index(self) -> u32 {
        self.index
    }

    pub fn flags(self) -> u32 {
        self.flags
    }
}

pub fn read_paper_mark(data: &[u8]) -> Result<PaperMark> {
    let stream = read_cfb_stream(data, PAPER_MARK_PATH)?;
    parse_paper_mark(&stream)
}

pub fn read_page_mark(data: &[u8]) -> Result<PageMark> {
    let stream = read_cfb_stream(data, PAGE_MARK_PATH)?;
    parse_page_mark(&stream)
}

pub fn parse_page_mark(data: &[u8]) -> Result<PageMark> {
    if data.len() < PAGE_MARK_HEADER_BYTES {
        return Err(Error::InvalidData(
            "PageMark shorter than 12-byte header".into(),
        ));
    }

    let header = PageMarkHeader::new(
        read_u32_be(data, 0),
        read_u32_be(data, 4),
        read_u32_be(data, 8),
    );
    if header.stride_value() != OBSERVED_PAGE_MARK_STRIDE_VALUE {
        return Err(Error::InvalidData(format!(
            "unsupported PageMark stride value: {}",
            header.stride_value()
        )));
    }

    let entry_bytes = data.len() - PAGE_MARK_HEADER_BYTES;
    let count_plus_one = header
        .count_value()
        .checked_add(1)
        .ok_or_else(|| Error::InvalidData("PageMark count overflows".into()))?
        as usize;
    let (family, row_bytes, trailing_bytes) = if entry_bytes.is_multiple_of(PAGE_MARK_ENTRY_BYTES) {
        (PageMarkFamily::Fixed84, PAGE_MARK_ENTRY_BYTES, Vec::new())
    } else if count_plus_one > 0 && entry_bytes.is_multiple_of(count_plus_one) {
        (
            PageMarkFamily::CountPlusOneVariable,
            entry_bytes / count_plus_one,
            Vec::new(),
        )
    } else if entry_bytes >= 2
        && count_plus_one > 0
        && (entry_bytes - 2).is_multiple_of(count_plus_one)
    {
        (
            PageMarkFamily::CountPlusOneTrim2,
            (entry_bytes - 2) / count_plus_one,
            data[data.len() - 2..].to_vec(),
        )
    } else if header.count_value() > 0 && entry_bytes.is_multiple_of(header.count_value() as usize)
    {
        (
            PageMarkFamily::CountVariable,
            entry_bytes / header.count_value() as usize,
            Vec::new(),
        )
    } else if entry_bytes >= PAGE_MARK_ENTRY_BYTES {
        let parsed_entry_bytes = entry_bytes / PAGE_MARK_ENTRY_BYTES * PAGE_MARK_ENTRY_BYTES;
        (
            PageMarkFamily::Fixed84Tail,
            PAGE_MARK_ENTRY_BYTES,
            data[PAGE_MARK_HEADER_BYTES + parsed_entry_bytes..].to_vec(),
        )
    } else {
        return Err(Error::InvalidData(
            "PageMark bytes do not match observed row families".into(),
        ));
    };

    if row_bytes == 0 {
        return Err(Error::InvalidData(format!(
            "PageMark row family {} has zero-byte rows",
            family.as_str()
        )));
    }

    let parsed_entry_bytes = entry_bytes - trailing_bytes.len();
    let mut entries = Vec::with_capacity(parsed_entry_bytes / row_bytes);
    let mut offset = PAGE_MARK_HEADER_BYTES;
    while offset + row_bytes <= PAGE_MARK_HEADER_BYTES + parsed_entry_bytes {
        entries.push(PageMarkEntry::new(
            data[offset..offset + row_bytes].to_vec(),
        ));
        offset += row_bytes;
    }

    Ok(PageMark::new(header, family, entries, trailing_bytes))
}

pub fn parse_paper_mark(data: &[u8]) -> Result<PaperMark> {
    if data.len() < PAPER_MARK_HEADER_BYTES {
        return Err(Error::InvalidData(
            "PaperMark shorter than 12-byte header".into(),
        ));
    }
    if !data.len().is_multiple_of(4) {
        return Err(Error::InvalidData("PaperMark is not u32 aligned".into()));
    }

    let header = PaperMarkHeader::new(
        read_u32_be(data, 0),
        read_u32_be(data, 4),
        read_u32_be(data, 8),
    );
    if header.stride_value() != OBSERVED_PAPER_MARK_STRIDE_VALUE {
        return Err(Error::InvalidData(format!(
            "unsupported PaperMark stride value: {}",
            header.stride_value()
        )));
    }

    let entry_bytes = data.len() - PAPER_MARK_HEADER_BYTES;
    if !entry_bytes.is_multiple_of(PAPER_MARK_ENTRY_BYTES) {
        return Err(Error::InvalidData(format!(
            "PaperMark entry bytes are not aligned to {PAPER_MARK_ENTRY_BYTES}-byte rows"
        )));
    }
    let expected_entries = entry_bytes / PAPER_MARK_ENTRY_BYTES;

    let mut entries = Vec::with_capacity(expected_entries);
    let mut offset = PAPER_MARK_HEADER_BYTES;
    while offset + PAPER_MARK_ENTRY_BYTES <= data.len() {
        entries.push(PaperMarkEntry::new(
            read_u32_be(data, offset),
            read_u32_be(data, offset + 4),
        ));
        offset += PAPER_MARK_ENTRY_BYTES;
    }

    Ok(PaperMark::new(header, entries))
}

fn read_u32_be(data: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

#[cfg(test)]
mod tests {
    use super::{
        PAGE_MARK_PATH, PAPER_MARK_PATH, PageMarkFamily, parse_page_mark, parse_paper_mark,
        read_page_mark, read_paper_mark,
    };
    use std::io::{Cursor, Write};

    #[test]
    fn parses_observed_page_mark_shape() {
        let page_mark = parse_page_mark(&page_mark_fixture()).unwrap();

        assert_eq!(page_mark.header().count_value(), 2);
        assert_eq!(page_mark.header().stride_value(), 0x10);
        assert_eq!(page_mark.header().last_index_value(), 1);
        assert_eq!(page_mark.family(), PageMarkFamily::Fixed84);
        assert_eq!(page_mark.entries().len(), 3);
        assert_eq!(page_mark.trailing_bytes(), b"");
        assert_eq!(page_mark.entries()[0].index(), Some(0));
        assert_eq!(page_mark.entries()[0].flags(), Some(0x0001_0000));
        assert_eq!(page_mark.entries()[0].line_start(), Some(0));
        assert_eq!(page_mark.entries()[0].line_end(), Some(2));
        assert_eq!(
            page_mark.entries()[0].u32_fields()[..4],
            [0, 0x0001_0000, 0, 2]
        );
        assert_eq!(page_mark.entries()[1].index(), Some(1));
        assert_eq!(page_mark.entries()[2].raw()[7], 0x02);
    }

    #[test]
    fn parses_count_plus_one_variable_page_mark_shape() {
        let page_mark = parse_page_mark(&page_mark_variable_fixture()).unwrap();

        assert_eq!(page_mark.header().count_value(), 3);
        assert_eq!(page_mark.family(), PageMarkFamily::CountPlusOneVariable);
        assert_eq!(page_mark.entries().len(), 4);
        assert_eq!(page_mark.entries()[0].raw().len(), 20);
        assert_eq!(page_mark.entries()[3].index(), Some(3));
        assert_eq!(page_mark.trailing_bytes(), b"");
    }

    #[test]
    fn parses_count_plus_one_trim2_page_mark_shape() {
        let page_mark = parse_page_mark(&page_mark_trim2_fixture()).unwrap();

        assert_eq!(page_mark.header().count_value(), 3);
        assert_eq!(page_mark.family(), PageMarkFamily::CountPlusOneTrim2);
        assert_eq!(page_mark.entries().len(), 4);
        assert_eq!(page_mark.entries()[0].raw().len(), 20);
        assert_eq!(page_mark.entries()[3].index(), Some(3));
        assert_eq!(page_mark.trailing_bytes(), b"\xaa\x55");
    }

    #[test]
    fn parses_count_variable_page_mark_shape() {
        let page_mark = parse_page_mark(&page_mark_count_variable_fixture()).unwrap();

        assert_eq!(page_mark.header().count_value(), 5);
        assert_eq!(page_mark.family(), PageMarkFamily::CountVariable);
        assert_eq!(page_mark.entries().len(), 5);
        assert_eq!(page_mark.entries()[0].raw().len(), 20);
        assert_eq!(page_mark.entries()[4].index(), Some(4));
        assert_eq!(page_mark.trailing_bytes(), b"");
    }

    #[test]
    fn parses_fixed84_tail_page_mark_shape() {
        let page_mark = parse_page_mark(&page_mark_fixed84_tail_fixture()).unwrap();

        assert_eq!(page_mark.header().count_value(), 6);
        assert_eq!(page_mark.family(), PageMarkFamily::Fixed84Tail);
        assert_eq!(page_mark.entries().len(), 2);
        assert_eq!(page_mark.entries()[0].raw().len(), 84);
        assert_eq!(page_mark.entries()[1].index(), Some(1));
        assert_eq!(page_mark.trailing_bytes(), b"\xde\xad\xbe\xef");
    }

    #[test]
    fn reads_page_mark_from_cfb() {
        let bytes = cfb_with_stream(PAGE_MARK_PATH, &page_mark_fixture());
        let page_mark = read_page_mark(&bytes).unwrap();

        assert_eq!(page_mark.entries().len(), 3);
        assert_eq!(page_mark.entries()[2].index(), Some(2));
    }

    #[test]
    fn rejects_unproven_page_mark_row_family() {
        let error = parse_page_mark(&page_mark_fixture()[..67]).unwrap_err();

        assert!(error.to_string().contains("observed row families"));
    }

    #[test]
    fn rejects_unobserved_page_mark_stride_value() {
        let mut bytes = page_mark_fixture();
        bytes[7] = 0x0c;
        let error = parse_page_mark(&bytes).unwrap_err();

        assert!(error.to_string().contains("unsupported PageMark stride"));
    }

    #[test]
    fn parses_observed_paper_mark_shape() {
        let paper_mark = parse_paper_mark(&paper_mark_fixture()).unwrap();

        assert_eq!(paper_mark.header().count_value(), 2);
        assert_eq!(paper_mark.header().stride_value(), 0x0c);
        assert_eq!(paper_mark.header().last_index_value(), 1);
        assert_eq!(paper_mark.entries().len(), 3);
        assert_eq!(paper_mark.entries()[0].index(), 0);
        assert_eq!(paper_mark.entries()[0].flags(), 0x0001_0010);
        assert_eq!(paper_mark.entries()[2].index(), 2);
        assert_eq!(paper_mark.entries()[2].flags(), 0x0001_0000);
    }

    #[test]
    fn reads_paper_mark_from_cfb() {
        let bytes = cfb_with_stream(PAPER_MARK_PATH, &paper_mark_fixture());
        let paper_mark = read_paper_mark(&bytes).unwrap();

        assert_eq!(paper_mark.entries().len(), 3);
        assert_eq!(paper_mark.entries()[1].index(), 1);
    }

    #[test]
    fn rejects_unproven_row_alignment() {
        let error = parse_paper_mark(&paper_mark_fixture()[..24]).unwrap_err();

        assert!(error.to_string().contains("not aligned"));
    }

    #[test]
    fn rejects_unobserved_stride_value() {
        let mut bytes = paper_mark_fixture();
        bytes[7] = 0x10;
        let error = parse_paper_mark(&bytes).unwrap_err();

        assert!(error.to_string().contains("unsupported PaperMark stride"));
    }

    fn paper_mark_fixture() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&2u32.to_be_bytes());
        bytes.extend_from_slice(&0x0cu32.to_be_bytes());
        bytes.extend_from_slice(&1u32.to_be_bytes());
        bytes.extend_from_slice(&0u32.to_be_bytes());
        bytes.extend_from_slice(&0x0001_0010u32.to_be_bytes());
        bytes.extend_from_slice(&1u32.to_be_bytes());
        bytes.extend_from_slice(&0x0001_0011u32.to_be_bytes());
        bytes.extend_from_slice(&2u32.to_be_bytes());
        bytes.extend_from_slice(&0x0001_0000u32.to_be_bytes());
        bytes
    }

    fn page_mark_fixture() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&2u32.to_be_bytes());
        bytes.extend_from_slice(&0x10u32.to_be_bytes());
        bytes.extend_from_slice(&1u32.to_be_bytes());
        for index in 0..=2u32 {
            let mut entry = [0; 84];
            entry[0..4].copy_from_slice(&index.to_be_bytes());
            entry[4..8].copy_from_slice(&(0x0001_0000u32 + index).to_be_bytes());
            entry[8..12].copy_from_slice(&(index * 10).to_be_bytes());
            entry[12..16].copy_from_slice(&(index * 10 + 2).to_be_bytes());
            bytes.extend_from_slice(&entry);
        }
        bytes
    }

    fn page_mark_variable_fixture() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&3u32.to_be_bytes());
        bytes.extend_from_slice(&0x10u32.to_be_bytes());
        bytes.extend_from_slice(&2u32.to_be_bytes());
        for index in 0..4u32 {
            let mut entry = [0; 20];
            entry[0..4].copy_from_slice(&index.to_be_bytes());
            entry[4..8].copy_from_slice(&(0x0100_0000u32 + index).to_be_bytes());
            bytes.extend_from_slice(&entry);
        }
        bytes
    }

    fn page_mark_trim2_fixture() -> Vec<u8> {
        let mut bytes = page_mark_variable_fixture();
        bytes.extend_from_slice(&[0xaa, 0x55]);
        bytes
    }

    fn page_mark_count_variable_fixture() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&5u32.to_be_bytes());
        bytes.extend_from_slice(&0x10u32.to_be_bytes());
        bytes.extend_from_slice(&4u32.to_be_bytes());
        for index in 0..5u32 {
            let mut entry = [0; 20];
            entry[0..4].copy_from_slice(&index.to_be_bytes());
            entry[4..8].copy_from_slice(&(0x0200_0000u32 + index).to_be_bytes());
            bytes.extend_from_slice(&entry);
        }
        bytes
    }

    fn page_mark_fixed84_tail_fixture() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&6u32.to_be_bytes());
        bytes.extend_from_slice(&0x10u32.to_be_bytes());
        bytes.extend_from_slice(&4u32.to_be_bytes());
        for index in 0..2u32 {
            let mut entry = [0; 84];
            entry[0..4].copy_from_slice(&index.to_be_bytes());
            entry[4..8].copy_from_slice(&(0x0300_0000u32 + index).to_be_bytes());
            bytes.extend_from_slice(&entry);
        }
        bytes.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
        bytes
    }

    fn cfb_with_stream(path: &str, payload: &[u8]) -> Vec<u8> {
        let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
        compound
            .create_stream(path)
            .unwrap()
            .write_all(payload)
            .unwrap();
        compound.into_inner().into_inner()
    }
}
