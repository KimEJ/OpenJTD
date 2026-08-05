use crate::stream::UnknownStream;
use crate::{Error, Result};
use std::collections::HashSet;
use std::io::{Cursor, Read};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Container {
    entries: Vec<ContainerEntry>,
    streams: Vec<StreamEntry>,
    unknown_streams: Vec<UnknownStream>,
}

impl Container {
    pub fn new(streams: Vec<StreamEntry>, unknown_streams: Vec<UnknownStream>) -> Self {
        Self {
            entries: Vec::new(),
            streams,
            unknown_streams,
        }
    }

    pub fn from_cfb_bytes(data: &[u8]) -> Result<Self> {
        let entries = inspect_cfb_entries(data)?;
        let streams = entries
            .iter()
            .filter(|entry| entry.kind() == EntryKind::Stream)
            .map(|entry| StreamEntry::new(entry.path(), Some(entry.size())))
            .collect();

        Ok(Self {
            entries,
            streams,
            unknown_streams: Vec::new(),
        })
    }

    pub fn entries(&self) -> &[ContainerEntry] {
        &self.entries
    }

    pub fn streams(&self) -> &[StreamEntry] {
        &self.streams
    }

    pub fn unknown_streams(&self) -> &[UnknownStream] {
        &self.unknown_streams
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    Storage,
    Stream,
}

impl EntryKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Storage => "storage",
            Self::Stream => "stream",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerEntry {
    path: String,
    size: u64,
    kind: EntryKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CfbEntryReadMode {
    Strict,
    Lenient,
}

impl ContainerEntry {
    pub fn new(path: impl Into<String>, size: u64, kind: EntryKind) -> Self {
        Self {
            path: path.into(),
            size,
            kind,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn kind(&self) -> EntryKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamEntry {
    name: String,
    size: Option<u64>,
}

impl StreamEntry {
    pub fn new(name: impl Into<String>, size: Option<u64>) -> Self {
        Self {
            name: name.into(),
            size,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CfbDirectoryEntryKind {
    Unknown,
    Storage,
    Stream,
    Root,
}

impl CfbDirectoryEntryKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Storage => "storage",
            Self::Stream => "stream",
            Self::Root => "root",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfbDirectoryEntry {
    id: usize,
    name: String,
    path: Option<String>,
    kind: CfbDirectoryEntryKind,
    left_id: u32,
    right_id: u32,
    child_id: u32,
    start_sector: u32,
    size: u64,
}

impl CfbDirectoryEntry {
    fn from_lenient(id: usize, entry: &LenientDirectoryEntry) -> Self {
        Self {
            id,
            name: entry.name.clone(),
            path: entry.path.clone(),
            kind: CfbDirectoryEntryKind::from(entry.object_type),
            left_id: entry.left_id,
            right_id: entry.right_id,
            child_id: entry.child_id,
            start_sector: entry.start_sector,
            size: entry.size,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn kind(&self) -> CfbDirectoryEntryKind {
        self.kind
    }

    pub fn left_id(&self) -> u32 {
        self.left_id
    }

    pub fn right_id(&self) -> u32 {
        self.right_id
    }

    pub fn child_id(&self) -> u32 {
        self.child_id
    }

    pub fn start_sector(&self) -> u32 {
        self.start_sector
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamStorage {
    Mini,
    Regular,
}

impl StreamStorage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mini => "mini",
            Self::Regular => "regular",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamLocation {
    path: String,
    size: u64,
    start_sector: u32,
    storage: StreamStorage,
    mini_stream_cutoff: u32,
    mini_stream_bytes: usize,
    mini_fat_entries: usize,
}

impl StreamLocation {
    fn new(
        path: impl Into<String>,
        size: u64,
        start_sector: u32,
        storage: StreamStorage,
        mini_stream_cutoff: u32,
        mini_stream_bytes: usize,
        mini_fat_entries: usize,
    ) -> Self {
        Self {
            path: path.into(),
            size,
            start_sector,
            storage,
            mini_stream_cutoff,
            mini_stream_bytes,
            mini_fat_entries,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn start_sector(&self) -> u32 {
        self.start_sector
    }

    pub fn storage(&self) -> StreamStorage {
        self.storage
    }

    pub fn mini_stream_cutoff(&self) -> u32 {
        self.mini_stream_cutoff
    }

    pub fn mini_stream_bytes(&self) -> usize {
        self.mini_stream_bytes
    }

    pub fn mini_fat_entries(&self) -> usize {
        self.mini_fat_entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamSectorChain {
    location: StreamLocation,
    sector_size: usize,
    sectors: Vec<StreamChainSector>,
    status: StreamChainStatus,
}

impl StreamSectorChain {
    fn new(
        location: StreamLocation,
        sector_size: usize,
        sectors: Vec<StreamChainSector>,
        status: StreamChainStatus,
    ) -> Self {
        Self {
            location,
            sector_size,
            sectors,
            status,
        }
    }

    pub fn location(&self) -> &StreamLocation {
        &self.location
    }

    pub fn sector_size(&self) -> usize {
        self.sector_size
    }

    pub fn sectors(&self) -> &[StreamChainSector] {
        &self.sectors
    }

    pub fn status(&self) -> StreamChainStatus {
        self.status
    }

    pub fn capacity_bytes(&self) -> usize {
        self.sectors.len().saturating_mul(self.sector_size)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamChainSector {
    sector_id: u32,
    byte_offset: usize,
    byte_len: usize,
}

impl StreamChainSector {
    fn new(sector_id: u32, byte_offset: usize, byte_len: usize) -> Self {
        Self {
            sector_id,
            byte_offset,
            byte_len,
        }
    }

    pub fn sector_id(self) -> u32 {
        self.sector_id
    }

    pub fn byte_offset(self) -> usize {
        self.byte_offset
    }

    pub fn byte_len(self) -> usize {
        self.byte_len
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamChainStatus {
    Complete,
    Truncated,
    Cycle,
    FatMissing,
    OutOfRange,
}

impl StreamChainStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Truncated => "truncated",
            Self::Cycle => "cycle",
            Self::FatMissing => "fat-missing",
            Self::OutOfRange => "out-of-range",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfbOverview {
    sector_size: usize,
    mini_stream_cutoff: u32,
    fat_sector_ids: Vec<u32>,
    directory_chain: CfbSectorChain,
    mini_fat_chain: CfbSectorChain,
    root_start_sector: u32,
    root_size: u64,
    mini_stream_chain: CfbSectorChain,
}

impl CfbOverview {
    pub fn sector_size(&self) -> usize {
        self.sector_size
    }

    pub fn mini_stream_cutoff(&self) -> u32 {
        self.mini_stream_cutoff
    }

    pub fn fat_sector_ids(&self) -> &[u32] {
        &self.fat_sector_ids
    }

    pub fn directory_chain(&self) -> &CfbSectorChain {
        &self.directory_chain
    }

    pub fn mini_fat_chain(&self) -> &CfbSectorChain {
        &self.mini_fat_chain
    }

    pub fn root_start_sector(&self) -> u32 {
        self.root_start_sector
    }

    pub fn root_size(&self) -> u64 {
        self.root_size
    }

    pub fn mini_stream_chain(&self) -> &CfbSectorChain {
        &self.mini_stream_chain
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfbSectorChain {
    sectors: Vec<u32>,
    status: StreamChainStatus,
}

impl CfbSectorChain {
    fn new(sectors: Vec<u32>, status: StreamChainStatus) -> Self {
        Self { sectors, status }
    }

    fn empty() -> Self {
        Self::new(Vec::new(), StreamChainStatus::Complete)
    }

    pub fn sectors(&self) -> &[u32] {
        &self.sectors
    }

    pub fn status(&self) -> StreamChainStatus {
        self.status
    }
}

pub fn inspect_cfb_entries(data: &[u8]) -> Result<Vec<ContainerEntry>> {
    inspect_cfb_entries_with_mode(data).map(|(entries, _)| entries)
}

pub fn inspect_cfb_entries_with_mode(
    data: &[u8],
) -> Result<(Vec<ContainerEntry>, CfbEntryReadMode)> {
    match inspect_cfb_entries_strict(data) {
        Ok(entries) => Ok((entries, CfbEntryReadMode::Strict)),
        Err(strict_error) if has_cfb_magic(data) => match inspect_cfb_entries_lenient(data) {
            Ok(entries) => Ok((entries, CfbEntryReadMode::Lenient)),
            Err(_) => Err(strict_error),
        },
        Err(error) => Err(error),
    }
}

pub fn inspect_cfb_stream_location(data: &[u8], path: &str) -> Result<StreamLocation> {
    LenientCfb::open(data)?.stream_location(path)
}

pub fn inspect_cfb_stream_chain(data: &[u8], path: &str) -> Result<StreamSectorChain> {
    LenientCfb::open(data)?.stream_chain(path)
}

pub fn inspect_cfb_directory(data: &[u8]) -> Result<Vec<CfbDirectoryEntry>> {
    Ok(LenientCfb::open(data)?
        .entries
        .iter()
        .enumerate()
        .map(|(id, entry)| CfbDirectoryEntry::from_lenient(id, entry))
        .collect())
}

pub fn inspect_cfb_overview(data: &[u8]) -> Result<CfbOverview> {
    LenientCfb::open(data).map(|compound| compound.overview)
}

fn inspect_cfb_entries_strict(data: &[u8]) -> Result<Vec<ContainerEntry>> {
    let compound = open_cfb(data)?;

    let mut entries = compound
        .walk()
        .filter(|entry| !entry.is_root())
        .map(|entry| {
            let kind = if entry.is_stream() {
                EntryKind::Stream
            } else {
                EntryKind::Storage
            };
            ContainerEntry::new(normalized_path(entry.path()), entry.len(), kind)
        })
        .collect::<Vec<_>>();

    entries.sort_by(|left, right| left.path().cmp(right.path()));
    Ok(entries)
}

pub fn read_cfb_stream(data: &[u8], path: &str) -> Result<Vec<u8>> {
    match read_cfb_stream_strict(data, path) {
        Ok(stream) => {
            if stream.is_empty()
                && has_cfb_magic(data)
                && strict_stream_has_nonzero_size(data, path)
            {
                return read_cfb_stream_lenient(data, path).or(Ok(stream));
            }
            Ok(stream)
        }
        Err(strict_error) if has_cfb_magic(data) => match read_cfb_stream_lenient(data, path) {
            Ok(stream) => Ok(stream),
            Err(Error::NotFound(message)) => Err(Error::NotFound(message)),
            Err(_) => Err(strict_error),
        },
        Err(error) => Err(error),
    }
}

fn read_cfb_stream_strict(data: &[u8], path: &str) -> Result<Vec<u8>> {
    let mut compound = open_cfb(data)?;
    if !compound.is_stream(path) {
        return Err(Error::NotFound(format!("stream `{path}`")));
    }

    let mut stream = compound
        .open_stream(path)
        .map_err(|error| Error::Io(format!("open stream `{path}` failed: {error}")))?;
    let mut bytes = Vec::new();
    stream
        .read_to_end(&mut bytes)
        .map_err(|error| Error::Io(format!("read stream `{path}` failed: {error}")))?;
    Ok(bytes)
}

fn strict_stream_has_nonzero_size(data: &[u8], path: &str) -> bool {
    let normalized = normalize_requested_path(path);
    inspect_cfb_entries_strict(data)
        .ok()
        .and_then(|entries| {
            entries
                .into_iter()
                .find(|entry| entry.kind() == EntryKind::Stream && entry.path() == normalized)
        })
        .is_some_and(|entry| entry.size() > 0)
}

fn open_cfb(data: &[u8]) -> Result<cfb::CompoundFile<Cursor<Vec<u8>>>> {
    let cursor = Cursor::new(data.to_vec());
    cfb::CompoundFile::open(cursor)
        .map_err(|error| Error::InvalidData(format!("CFB open failed: {error}")))
}

fn has_cfb_magic(data: &[u8]) -> bool {
    data.starts_with(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1")
}

fn normalized_path(path: &std::path::Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn inspect_cfb_entries_lenient(data: &[u8]) -> Result<Vec<ContainerEntry>> {
    let compound = LenientCfb::open(data)?;
    let mut entries = compound
        .entries
        .iter()
        .filter(|entry| {
            matches!(
                entry.object_type,
                CfbObjectType::Storage | CfbObjectType::Stream
            )
        })
        .filter_map(|entry| {
            let path = entry.path.as_ref()?;
            let kind = match entry.object_type {
                CfbObjectType::Storage => EntryKind::Storage,
                CfbObjectType::Stream => EntryKind::Stream,
                CfbObjectType::Root | CfbObjectType::Unknown => return None,
            };
            let size = if kind == EntryKind::Stream {
                entry.size
            } else {
                0
            };
            Some(ContainerEntry::new(path, size, kind))
        })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.path().cmp(right.path()));
    Ok(entries)
}

fn read_cfb_stream_lenient(data: &[u8], path: &str) -> Result<Vec<u8>> {
    LenientCfb::open(data)?.read_stream(path)
}

struct LenientCfb {
    data: Vec<u8>,
    sector_size: usize,
    overview: CfbOverview,
    entries: Vec<LenientDirectoryEntry>,
    fat: Vec<u32>,
    mini_fat: Vec<u32>,
    mini_stream: Vec<u8>,
    mini_stream_cutoff: u32,
}

impl LenientCfb {
    const END_OF_CHAIN: u32 = 0xffff_fffe;
    const FREE_SECT: u32 = 0xffff_ffff;

    fn open(data: &[u8]) -> Result<Self> {
        if data.len() < 512 || !has_cfb_magic(data) {
            return Err(Error::InvalidData("not a CFB file".into()));
        }

        let sector_size = 1usize
            .checked_shl(read_u16_le(data, 30)? as u32)
            .ok_or_else(|| Error::InvalidData("invalid CFB sector size".into()))?;
        let fat_sector_count = read_u32_le(data, 44)? as usize;
        let first_directory_sector = read_u32_le(data, 48)?;
        let mini_stream_cutoff = read_u32_le(data, 56)?;
        let first_mini_fat_sector = read_u32_le(data, 60)?;
        let mini_fat_sector_count = read_u32_le(data, 64)? as usize;
        let first_difat_sector = read_u32_le(data, 68)?;
        let difat_sector_count = read_u32_le(data, 72)? as usize;

        let fat_sector_ids = read_difat_sector_ids(
            data,
            sector_size,
            fat_sector_count,
            first_difat_sector,
            difat_sector_count,
        );
        let fat = read_fat(data, sector_size, &fat_sector_ids);
        let directory_chain = collect_sector_ids(&fat, first_directory_sector, None);
        let directory_data =
            read_sector_chain(data, &fat, first_directory_sector, sector_size, None);
        let mut entries = parse_directory_entries(&directory_data)?;
        assign_directory_paths(&mut entries);

        let (mini_fat, mini_fat_chain) = if mini_fat_sector_count > 0
            && first_mini_fat_sector != Self::END_OF_CHAIN
            && first_mini_fat_sector != Self::FREE_SECT
        {
            let mini_fat_chain =
                collect_sector_ids(&fat, first_mini_fat_sector, Some(mini_fat_sector_count));
            let mini_fat_data = read_sector_chain(
                data,
                &fat,
                first_mini_fat_sector,
                sector_size,
                Some(mini_fat_sector_count),
            );
            (read_u32_table(&mini_fat_data), mini_fat_chain)
        } else {
            (Vec::new(), CfbSectorChain::empty())
        };
        let root_entry = entries
            .iter()
            .find(|entry| entry.object_type == CfbObjectType::Root);
        let mini_stream = root_entry
            .map(|root| {
                let mut bytes = read_sector_chain(data, &fat, root.start_sector, sector_size, None);
                truncate_to_u64(&mut bytes, root.size);
                bytes
            })
            .unwrap_or_default();
        let root_start_sector = root_entry
            .map(|root| root.start_sector)
            .unwrap_or(Self::END_OF_CHAIN);
        let root_size = root_entry.map(|root| root.size).unwrap_or(0);
        let mini_stream_chain = root_entry
            .map(|root| {
                if root.size == 0 {
                    CfbSectorChain::empty()
                } else {
                    collect_sector_ids(&fat, root.start_sector, None)
                }
            })
            .unwrap_or_else(CfbSectorChain::empty);
        let overview = CfbOverview {
            sector_size,
            mini_stream_cutoff,
            fat_sector_ids,
            directory_chain,
            mini_fat_chain,
            root_start_sector,
            root_size,
            mini_stream_chain,
        };

        Ok(Self {
            data: data.to_vec(),
            sector_size,
            overview,
            entries,
            fat,
            mini_fat,
            mini_stream,
            mini_stream_cutoff,
        })
    }

    fn read_stream(&self, path: &str) -> Result<Vec<u8>> {
        let normalized = normalize_requested_path(path);
        let entry = self
            .entries
            .iter()
            .find(|entry| {
                entry.object_type == CfbObjectType::Stream
                    && entry
                        .path
                        .as_deref()
                        .is_some_and(|candidate| candidate == normalized)
            })
            .ok_or_else(|| Error::NotFound(format!("stream `{path}`")))?;

        if entry.size == 0 {
            return Ok(Vec::new());
        }

        if entry.size < self.mini_stream_cutoff as u64 {
            Ok(read_mini_sector_chain(
                &self.mini_stream,
                &self.mini_fat,
                entry.start_sector,
                entry.size,
            ))
        } else {
            let mut bytes = read_sector_chain(
                &self.data,
                &self.fat,
                entry.start_sector,
                self.sector_size,
                None,
            );
            truncate_to_u64(&mut bytes, entry.size);
            Ok(bytes)
        }
    }

    fn stream_location(&self, path: &str) -> Result<StreamLocation> {
        let normalized = normalize_requested_path(path);
        let entry = self
            .entries
            .iter()
            .find(|entry| {
                entry.object_type == CfbObjectType::Stream
                    && entry
                        .path
                        .as_deref()
                        .is_some_and(|candidate| candidate == normalized)
            })
            .ok_or_else(|| Error::NotFound(format!("stream `{path}`")))?;
        let storage = if entry.size < self.mini_stream_cutoff as u64 {
            StreamStorage::Mini
        } else {
            StreamStorage::Regular
        };

        Ok(StreamLocation::new(
            entry.path.clone().unwrap_or_else(|| normalized.clone()),
            entry.size,
            entry.start_sector,
            storage,
            self.mini_stream_cutoff,
            self.mini_stream.len(),
            self.mini_fat.len(),
        ))
    }

    fn stream_chain(&self, path: &str) -> Result<StreamSectorChain> {
        let location = self.stream_location(path)?;
        let (sector_size, sectors, status) = match location.storage() {
            StreamStorage::Mini => {
                let sector_size = 64;
                let (sectors, status) = collect_sector_chain(
                    &self.mini_fat,
                    location.start_sector(),
                    sector_size,
                    location.size(),
                    |sector_id| {
                        let offset = (sector_id as usize).checked_mul(sector_size)?;
                        let end = offset.checked_add(sector_size)?;
                        (end <= self.mini_stream.len()).then_some(offset)
                    },
                );
                (sector_size, sectors, status)
            }
            StreamStorage::Regular => {
                let sector_size = self.sector_size;
                let (sectors, status) = collect_sector_chain(
                    &self.fat,
                    location.start_sector(),
                    sector_size,
                    location.size(),
                    |sector_id| sector_offset(&self.data, sector_id, sector_size),
                );
                (sector_size, sectors, status)
            }
        };

        Ok(StreamSectorChain::new(
            location,
            sector_size,
            sectors,
            status,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CfbObjectType {
    Unknown,
    Storage,
    Stream,
    Root,
}

impl From<CfbObjectType> for CfbDirectoryEntryKind {
    fn from(value: CfbObjectType) -> Self {
        match value {
            CfbObjectType::Unknown => Self::Unknown,
            CfbObjectType::Storage => Self::Storage,
            CfbObjectType::Stream => Self::Stream,
            CfbObjectType::Root => Self::Root,
        }
    }
}

#[derive(Debug, Clone)]
struct LenientDirectoryEntry {
    name: String,
    object_type: CfbObjectType,
    left_id: u32,
    right_id: u32,
    child_id: u32,
    start_sector: u32,
    size: u64,
    path: Option<String>,
}

fn read_difat_sector_ids(
    data: &[u8],
    sector_size: usize,
    fat_sector_count: usize,
    first_difat_sector: u32,
    difat_sector_count: usize,
) -> Vec<u32> {
    let mut sector_ids = Vec::new();
    for index in 0..109.min(fat_sector_count) {
        let offset = 76 + index * 4;
        if let Ok(sector_id) = read_u32_le(data, offset)
            && is_real_sector(sector_id)
        {
            sector_ids.push(sector_id);
        }
    }

    let mut sector_id = first_difat_sector;
    for _ in 0..difat_sector_count {
        if !is_real_sector(sector_id) {
            break;
        }
        let Some(offset) = sector_offset(data, sector_id, sector_size) else {
            break;
        };
        let entries_per_sector = sector_size / 4 - 1;
        for index in 0..entries_per_sector {
            let entry_offset = offset + index * 4;
            if let Ok(fat_sector_id) = read_u32_le(data, entry_offset)
                && is_real_sector(fat_sector_id)
            {
                sector_ids.push(fat_sector_id);
            }
        }
        let next_offset = offset + entries_per_sector * 4;
        match read_u32_le(data, next_offset) {
            Ok(next) => sector_id = next,
            Err(_) => break,
        }
    }

    sector_ids.truncate(fat_sector_count);
    sector_ids
}

fn read_fat(data: &[u8], sector_size: usize, fat_sector_ids: &[u32]) -> Vec<u32> {
    let mut fat = Vec::new();
    for &sector_id in fat_sector_ids {
        let Some(offset) = sector_offset(data, sector_id, sector_size) else {
            continue;
        };
        let entries_per_sector = sector_size / 4;
        for index in 0..entries_per_sector {
            let entry_offset = offset + index * 4;
            if let Ok(value) = read_u32_le(data, entry_offset) {
                fat.push(value);
            }
        }
    }
    fat
}

fn read_sector_chain(
    data: &[u8],
    fat: &[u32],
    start_sector: u32,
    sector_size: usize,
    max_sectors: Option<usize>,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut sector_id = start_sector;
    let mut visited = HashSet::new();
    while is_real_sector(sector_id) {
        if !visited.insert(sector_id) {
            break;
        }
        if max_sectors.is_some_and(|limit| visited.len() > limit) {
            break;
        }
        let Some(offset) = sector_offset(data, sector_id, sector_size) else {
            break;
        };
        bytes.extend_from_slice(&data[offset..offset + sector_size]);
        match fat.get(sector_id as usize) {
            Some(next) => sector_id = *next,
            None => break,
        }
    }
    bytes
}

fn read_mini_sector_chain(
    mini_stream: &[u8],
    mini_fat: &[u32],
    start_sector: u32,
    size: u64,
) -> Vec<u8> {
    const MINI_SECTOR_SIZE: usize = 64;

    let mut bytes = Vec::new();
    let mut sector_id = start_sector;
    let mut visited = HashSet::new();
    while is_real_sector(sector_id) {
        if !visited.insert(sector_id) {
            break;
        }
        let offset = sector_id as usize * MINI_SECTOR_SIZE;
        let Some(end) = offset.checked_add(MINI_SECTOR_SIZE) else {
            break;
        };
        if end > mini_stream.len() {
            break;
        }
        bytes.extend_from_slice(&mini_stream[offset..end]);
        match mini_fat.get(sector_id as usize) {
            Some(next) => sector_id = *next,
            None => break,
        }
    }
    truncate_to_u64(&mut bytes, size);
    bytes
}

fn collect_sector_chain(
    fat: &[u32],
    start_sector: u32,
    sector_size: usize,
    declared_size: u64,
    mut sector_offset: impl FnMut(u32) -> Option<usize>,
) -> (Vec<StreamChainSector>, StreamChainStatus) {
    let mut sectors = Vec::new();
    if declared_size == 0 {
        return (sectors, StreamChainStatus::Complete);
    }

    let mut sector_id = start_sector;
    let mut visited = HashSet::new();
    while is_real_sector(sector_id) {
        if !visited.insert(sector_id) {
            return (sectors, StreamChainStatus::Cycle);
        }
        let Some(offset) = sector_offset(sector_id) else {
            return (sectors, StreamChainStatus::OutOfRange);
        };
        sectors.push(StreamChainSector::new(sector_id, offset, sector_size));
        match fat.get(sector_id as usize) {
            Some(next) => sector_id = *next,
            None => return (sectors, StreamChainStatus::FatMissing),
        }
    }

    let capacity = (sectors.len() as u64).saturating_mul(sector_size as u64);
    let status = if capacity >= declared_size {
        StreamChainStatus::Complete
    } else {
        StreamChainStatus::Truncated
    };
    (sectors, status)
}

fn collect_sector_ids(
    fat: &[u32],
    start_sector: u32,
    max_sectors: Option<usize>,
) -> CfbSectorChain {
    let mut sectors = Vec::new();
    let mut sector_id = start_sector;
    let mut visited = HashSet::new();
    while is_real_sector(sector_id) {
        if !visited.insert(sector_id) {
            return CfbSectorChain::new(sectors, StreamChainStatus::Cycle);
        }
        sectors.push(sector_id);
        if max_sectors.is_some_and(|limit| sectors.len() >= limit) {
            return CfbSectorChain::new(sectors, StreamChainStatus::Complete);
        }
        match fat.get(sector_id as usize) {
            Some(next) => sector_id = *next,
            None => return CfbSectorChain::new(sectors, StreamChainStatus::FatMissing),
        }
    }

    CfbSectorChain::new(sectors, StreamChainStatus::Complete)
}

fn parse_directory_entries(data: &[u8]) -> Result<Vec<LenientDirectoryEntry>> {
    let mut entries = Vec::new();
    for entry_data in data.chunks_exact(128) {
        let name_length = u16::from_le_bytes([entry_data[64], entry_data[65]]) as usize;
        let name_bytes_end = name_length.saturating_sub(2).min(64);
        let name_units = entry_data[..name_bytes_end]
            .chunks_exact(2)
            .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
            .collect::<Vec<_>>();
        let name = String::from_utf16_lossy(&name_units);
        let object_type = match entry_data[66] {
            1 => CfbObjectType::Storage,
            2 => CfbObjectType::Stream,
            5 => CfbObjectType::Root,
            _ => CfbObjectType::Unknown,
        };

        entries.push(LenientDirectoryEntry {
            name,
            object_type,
            left_id: u32::from_le_bytes([
                entry_data[68],
                entry_data[69],
                entry_data[70],
                entry_data[71],
            ]),
            right_id: u32::from_le_bytes([
                entry_data[72],
                entry_data[73],
                entry_data[74],
                entry_data[75],
            ]),
            child_id: u32::from_le_bytes([
                entry_data[76],
                entry_data[77],
                entry_data[78],
                entry_data[79],
            ]),
            start_sector: u32::from_le_bytes([
                entry_data[116],
                entry_data[117],
                entry_data[118],
                entry_data[119],
            ]),
            size: u64::from_le_bytes([
                entry_data[120],
                entry_data[121],
                entry_data[122],
                entry_data[123],
                entry_data[124],
                entry_data[125],
                entry_data[126],
                entry_data[127],
            ]),
            path: None,
        });
    }

    if entries.is_empty() {
        return Err(Error::InvalidData("CFB directory is empty".into()));
    }
    Ok(entries)
}

fn assign_directory_paths(entries: &mut [LenientDirectoryEntry]) {
    let root_index = entries
        .iter()
        .position(|entry| entry.object_type == CfbObjectType::Root)
        .unwrap_or(0);
    let root_child = entries[root_index].child_id;
    let mut visited = HashSet::new();
    assign_child_tree_paths(entries, root_child, "", &mut visited);

    for entry in entries {
        if entry.path.is_none()
            && matches!(
                entry.object_type,
                CfbObjectType::Storage | CfbObjectType::Stream
            )
            && !entry.name.is_empty()
        {
            entry.path = Some(format!("/{}", entry.name));
        }
    }
}

fn assign_child_tree_paths(
    entries: &mut [LenientDirectoryEntry],
    entry_id: u32,
    parent_path: &str,
    visited: &mut HashSet<u32>,
) {
    if !is_real_sector(entry_id) || entry_id as usize >= entries.len() || !visited.insert(entry_id)
    {
        return;
    }

    let index = entry_id as usize;
    let left_id = entries[index].left_id;
    let right_id = entries[index].right_id;
    let child_id = entries[index].child_id;
    assign_child_tree_paths(entries, left_id, parent_path, visited);

    if matches!(
        entries[index].object_type,
        CfbObjectType::Storage | CfbObjectType::Stream
    ) {
        let path = if parent_path.is_empty() {
            format!("/{}", entries[index].name)
        } else {
            format!("{parent_path}/{}", entries[index].name)
        };
        entries[index].path = Some(path.clone());
        if entries[index].object_type == CfbObjectType::Storage {
            assign_child_tree_paths(entries, child_id, &path, visited);
        }
    }

    assign_child_tree_paths(entries, right_id, parent_path, visited);
}

fn read_u16_le(data: &[u8], offset: usize) -> Result<u16> {
    let bytes = data
        .get(offset..offset + 2)
        .ok_or_else(|| Error::InvalidData("CFB u16 field is truncated".into()))?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32_le(data: &[u8], offset: usize) -> Result<u32> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or_else(|| Error::InvalidData("CFB u32 field is truncated".into()))?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_u32_table(data: &[u8]) -> Vec<u32> {
    data.chunks_exact(4)
        .map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        .collect()
}

fn is_real_sector(sector_id: u32) -> bool {
    sector_id != LenientCfb::END_OF_CHAIN && sector_id != LenientCfb::FREE_SECT
}

fn sector_offset(data: &[u8], sector_id: u32, sector_size: usize) -> Option<usize> {
    let offset = (sector_id as usize)
        .checked_add(1)?
        .checked_mul(sector_size)?;
    let end = offset.checked_add(sector_size)?;
    (end <= data.len()).then_some(offset)
}

fn truncate_to_u64(bytes: &mut Vec<u8>, size: u64) {
    let Ok(size) = usize::try_from(size) else {
        return;
    };
    bytes.truncate(size);
}

fn normalize_requested_path(path: &str) -> String {
    let path = path.replace('\\', "/");
    if path.starts_with('/') {
        path
    } else {
        format!("/{path}")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CfbDirectoryEntryKind, CfbEntryReadMode, ContainerEntry, EntryKind, StreamChainStatus,
        StreamStorage, inspect_cfb_directory, inspect_cfb_entries, inspect_cfb_entries_with_mode,
        inspect_cfb_overview, inspect_cfb_stream_chain, inspect_cfb_stream_location, open_cfb,
        read_cfb_stream, read_u32_le, sector_offset,
    };
    use std::io::{Cursor, Write};

    fn tiny_cfb() -> Vec<u8> {
        let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
        compound
            .create_stream("/\u{4}JSRV_SegmentInformation")
            .unwrap()
            .write_all(b"segment")
            .unwrap();
        compound
            .create_stream("/DocInfo")
            .unwrap()
            .write_all(b"doc")
            .unwrap();
        compound.create_storage("/BodyText").unwrap();
        compound
            .create_stream("/BodyText/Section0")
            .unwrap()
            .write_all(b"hello")
            .unwrap();

        compound.into_inner().into_inner()
    }

    fn cfb_with_duplicate_fat_pointer() -> Vec<u8> {
        let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
        compound
            .create_stream("/A")
            .unwrap()
            .write_all(&vec![b'a'; 5000])
            .unwrap();
        compound
            .create_stream("/B")
            .unwrap()
            .write_all(&vec![b'b'; 5000])
            .unwrap();
        let mut bytes = compound.into_inner().into_inner();
        let sector_size = 1usize << u16::from_le_bytes([bytes[30], bytes[31]]);
        let first_dir_sector = read_u32_le(&bytes, 48).unwrap();
        let dir_offset = sector_offset(&bytes, first_dir_sector, sector_size).unwrap();
        let a_offset = find_directory_entry_offset(&bytes, dir_offset, "A");
        let b_offset = find_directory_entry_offset(&bytes, dir_offset, "B");
        let a_start = u32::from_le_bytes([
            bytes[a_offset + 116],
            bytes[a_offset + 117],
            bytes[a_offset + 118],
            bytes[a_offset + 119],
        ]);
        let b_start = u32::from_le_bytes([
            bytes[b_offset + 116],
            bytes[b_offset + 117],
            bytes[b_offset + 118],
            bytes[b_offset + 119],
        ]);
        let a_fat_offset = fat_entry_offset(&bytes, sector_size, a_start);
        let b_fat_offset = fat_entry_offset(&bytes, sector_size, b_start);
        let a_next = bytes[a_fat_offset..a_fat_offset + 4].to_vec();
        bytes[b_fat_offset..b_fat_offset + 4].copy_from_slice(&a_next);
        bytes
    }

    fn fat_entry_offset(bytes: &[u8], sector_size: usize, sector_id: u32) -> usize {
        let fat_sector_id = read_u32_le(bytes, 76).unwrap();
        let fat_offset = sector_offset(bytes, fat_sector_id, sector_size).unwrap();
        fat_offset + sector_id as usize * 4
    }

    fn find_directory_entry_offset(bytes: &[u8], directory_offset: usize, name: &str) -> usize {
        for offset in (directory_offset..bytes.len().saturating_sub(128)).step_by(128) {
            if !matches!(bytes[offset + 66], 1 | 2 | 5) {
                continue;
            }
            let name_length = u16::from_le_bytes([bytes[offset + 64], bytes[offset + 65]]) as usize;
            let name_bytes_end = name_length.saturating_sub(2).min(64);
            let units = bytes[offset..offset + name_bytes_end]
                .chunks_exact(2)
                .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
                .collect::<Vec<_>>();
            if String::from_utf16_lossy(&units) == name {
                return offset;
            }
        }
        panic!("directory entry `{name}` not found");
    }

    #[test]
    fn inspects_cfb_entries_with_kind_size_and_normalized_path() {
        let bytes = tiny_cfb();
        let (entries, mode) = inspect_cfb_entries_with_mode(&bytes).unwrap();

        assert_eq!(mode, CfbEntryReadMode::Strict);
        assert_eq!(
            entries,
            vec![
                ContainerEntry::new("/\u{4}JSRV_SegmentInformation", 7, EntryKind::Stream),
                ContainerEntry::new("/BodyText", 0, EntryKind::Storage),
                ContainerEntry::new("/BodyText/Section0", 5, EntryKind::Stream),
                ContainerEntry::new("/DocInfo", 3, EntryKind::Stream),
            ]
        );
    }

    #[test]
    fn rejects_non_cfb_data() {
        let error = inspect_cfb_entries(b"not a cfb file").unwrap_err();

        assert!(error.to_string().contains("invalid data"));
    }

    #[test]
    fn reads_cfb_stream_payload() {
        let payload = read_cfb_stream(&tiny_cfb(), "/BodyText/Section0").unwrap();

        assert_eq!(payload, b"hello");
    }

    #[test]
    fn inspects_stream_storage_location() {
        let location = inspect_cfb_stream_location(&tiny_cfb(), "/DocInfo").unwrap();

        assert_eq!(location.path(), "/DocInfo");
        assert_eq!(location.size(), 3);
        assert_eq!(location.storage(), StreamStorage::Mini);
        assert!(location.mini_stream_cutoff() > 0);
        assert!(location.mini_stream_bytes() > 0);
    }

    #[test]
    fn inspects_mini_stream_sector_chain() {
        let chain = inspect_cfb_stream_chain(&tiny_cfb(), "/DocInfo").unwrap();

        assert_eq!(chain.location().path(), "/DocInfo");
        assert_eq!(chain.location().storage(), StreamStorage::Mini);
        assert_eq!(chain.status(), StreamChainStatus::Complete);
        assert_eq!(chain.sector_size(), 64);
        assert!(chain.capacity_bytes() >= chain.location().size() as usize);
        assert!(!chain.sectors().is_empty());
        assert_eq!(chain.sectors()[0].byte_len(), 64);
    }

    #[test]
    fn inspects_cfb_overview_chains() {
        let overview = inspect_cfb_overview(&tiny_cfb()).unwrap();

        assert!(overview.sector_size() > 0);
        assert!(overview.mini_stream_cutoff() > 0);
        assert!(!overview.fat_sector_ids().is_empty());
        assert_eq!(
            overview.directory_chain().status(),
            StreamChainStatus::Complete
        );
        assert!(overview.root_size() > 0);
        assert_eq!(
            overview.mini_stream_chain().status(),
            StreamChainStatus::Complete
        );
        assert!(!overview.mini_stream_chain().sectors().is_empty());
    }

    #[test]
    fn inspects_raw_cfb_directory_entries() {
        let entries = inspect_cfb_directory(&tiny_cfb()).unwrap();

        assert!(entries.iter().any(|entry| {
            entry.kind() == CfbDirectoryEntryKind::Root && entry.name() == "Root Entry"
        }));
        let doc_info = entries
            .iter()
            .find(|entry| entry.path() == Some("/DocInfo"))
            .unwrap();
        assert_eq!(doc_info.kind(), CfbDirectoryEntryKind::Stream);
        assert_eq!(doc_info.size(), 3);
        assert_eq!(doc_info.name(), "DocInfo");
    }

    #[test]
    fn falls_back_to_lenient_reader_for_malformed_fat_inventory() {
        let bytes = cfb_with_duplicate_fat_pointer();
        assert!(open_cfb(&bytes).is_err());

        let (entries, mode) = inspect_cfb_entries_with_mode(&bytes).unwrap();

        assert_eq!(mode, CfbEntryReadMode::Lenient);
        assert!(entries.contains(&ContainerEntry::new("/A", 5000, EntryKind::Stream)));
        assert!(entries.contains(&ContainerEntry::new("/B", 5000, EntryKind::Stream)));
    }

    #[test]
    fn falls_back_to_lenient_reader_for_malformed_fat_streams() {
        let bytes = cfb_with_duplicate_fat_pointer();
        assert!(open_cfb(&bytes).is_err());

        let payload = read_cfb_stream(&bytes, "/A").unwrap();

        assert_eq!(payload, vec![b'a'; 5000]);
    }

    #[test]
    fn lenient_reader_preserves_missing_stream_errors_after_malformed_fat() {
        let bytes = cfb_with_duplicate_fat_pointer();
        assert!(open_cfb(&bytes).is_err());

        let error = read_cfb_stream(&bytes, "/Missing").unwrap_err();

        assert!(error.to_string().contains("not found"));
    }

    #[test]
    fn reports_missing_stream() {
        let error = read_cfb_stream(&tiny_cfb(), "/Missing").unwrap_err();

        assert!(error.to_string().contains("not found"));
    }
}
