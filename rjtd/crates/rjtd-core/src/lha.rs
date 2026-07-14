use crate::{DecompressionBudget, Error, ParseLimits, Result};

const LH5_METHOD: &[u8; 5] = b"-lh5-";
const LH5_DICBIT: usize = 13;
const LH5_DICSIZ: usize = 1 << LH5_DICBIT;
const LH5_NP: usize = LH5_DICBIT + 1;
const LH5_NT: usize = 19;
const LH5_NC: usize = 510;
const LH5_THRESHOLD_BASE: usize = 253;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LhaMember {
    filename: String,
    packed_size: usize,
    original_size: usize,
    bytes: Vec<u8>,
}

impl LhaMember {
    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn packed_size(&self) -> usize {
        self.packed_size
    }

    pub fn original_size(&self) -> usize {
        self.original_size
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

pub fn decompress_lh5_member(data: &[u8]) -> Result<LhaMember> {
    decompress_lh5_member_with_limits(data, ParseLimits::DEFAULT)
}

/// Decompresses one already allocated LH5 member with explicit resource limits.
///
/// The per-member output limit applies to this member; the newly created total budget is also
/// scoped to this call. The input limit checks `data` after the caller has allocated it.
pub fn decompress_lh5_member_with_limits(data: &[u8], limits: ParseLimits) -> Result<LhaMember> {
    let mut budget = limits.decompression_budget();
    decompress_lh5_member_with_budget(data, &mut budget)
}

pub(crate) fn decompress_lh5_member_with_budget(
    data: &[u8],
    budget: &mut DecompressionBudget,
) -> Result<LhaMember> {
    budget.check_input_size(data.len())?;
    let header = parse_lha_header(data)?;
    let packed_end = header
        .data_start
        .checked_add(header.packed_size)
        .ok_or_else(|| Error::InvalidData("LHA packed size overflow".into()))?;
    if packed_end > data.len() {
        return Err(Error::InvalidData(format!(
            "LHA packed data truncated: need {}, have {}",
            packed_end,
            data.len()
        )));
    }

    budget.reserve_lh5_output(header.packed_size, header.original_size)?;
    let bytes = decode_lh5_data(&data[header.data_start..packed_end], header.original_size)?;
    Ok(LhaMember {
        filename: header.filename,
        packed_size: header.packed_size,
        original_size: header.original_size,
        bytes,
    })
}

struct LhaHeader {
    filename: String,
    packed_size: usize,
    original_size: usize,
    data_start: usize,
}

fn parse_lha_header(data: &[u8]) -> Result<LhaHeader> {
    if data.len() < 24 {
        return Err(Error::InvalidData("LHA member header is too short".into()));
    }

    let header_size = data[0] as usize;
    let data_start = 2usize
        .checked_add(header_size)
        .ok_or_else(|| Error::InvalidData("LHA header size overflow".into()))?;
    if data_start > data.len() {
        return Err(Error::InvalidData("LHA member header is truncated".into()));
    }
    if &data[2..7] != LH5_METHOD {
        return Err(Error::Unsupported("LHA method other than -lh5-"));
    }

    let packed_size = read_u32_le(data, 7)? as usize;
    let original_size = read_u32_le(data, 11)? as usize;
    let filename_len = data[21] as usize;
    let filename_start = 22usize;
    let filename_end = filename_start
        .checked_add(filename_len)
        .ok_or_else(|| Error::InvalidData("LHA filename length overflow".into()))?;
    if filename_end > data_start {
        return Err(Error::InvalidData("LHA filename exceeds header".into()));
    }
    let filename = String::from_utf8_lossy(&data[filename_start..filename_end]).into_owned();

    Ok(LhaHeader {
        filename,
        packed_size,
        original_size,
        data_start,
    })
}

fn read_u32_le(data: &[u8], offset: usize) -> Result<u32> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or_else(|| Error::InvalidData("LHA header integer is truncated".into()))?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn decode_lh5_data(data: &[u8], original_size: usize) -> Result<Vec<u8>> {
    let mut reader = BitReader::new(data);
    let mut output = Vec::new();
    output.try_reserve_exact(original_size).map_err(|error| {
        Error::Io(format!(
            "reserve {original_size} bytes for LH5 output failed: {error}"
        ))
    })?;
    let mut dictionary = vec![0u8; LH5_DICSIZ];
    let mut dictionary_pos = 0usize;
    let mut block_remaining = 0usize;
    let mut code_decoder = HuffmanDecoder::single(0);
    let mut position_decoder = HuffmanDecoder::single(0);

    while output.len() < original_size {
        if block_remaining == 0 {
            block_remaining = reader.read_bits(16)? as usize;
            let pt_decoder = read_pt_len(&mut reader, LH5_NT, 5, Some(3))?;
            code_decoder = read_c_len(&mut reader, &pt_decoder)?;
            position_decoder = read_pt_len(&mut reader, LH5_NP, 4, None)?;
        }

        block_remaining = block_remaining.saturating_sub(1);
        let code = code_decoder.decode(&mut reader)?;
        if code < 256 {
            push_decoded_byte(
                &mut output,
                &mut dictionary,
                &mut dictionary_pos,
                code as u8,
            );
            continue;
        }

        let match_len = code
            .checked_sub(LH5_THRESHOLD_BASE)
            .ok_or_else(|| Error::InvalidData("invalid LH5 match length code".into()))?;
        let distance = decode_position(&mut reader, &position_decoder)? + 1;
        copy_from_dictionary(
            &mut output,
            &mut dictionary,
            &mut dictionary_pos,
            distance,
            match_len,
            original_size,
        )?;
    }

    Ok(output)
}

fn push_decoded_byte(
    output: &mut Vec<u8>,
    dictionary: &mut [u8],
    dictionary_pos: &mut usize,
    byte: u8,
) {
    output.push(byte);
    dictionary[*dictionary_pos] = byte;
    *dictionary_pos = (*dictionary_pos + 1) & (LH5_DICSIZ - 1);
}

fn copy_from_dictionary(
    output: &mut Vec<u8>,
    dictionary: &mut [u8],
    dictionary_pos: &mut usize,
    distance: usize,
    match_len: usize,
    original_size: usize,
) -> Result<()> {
    if distance == 0 || distance > LH5_DICSIZ {
        return Err(Error::InvalidData("invalid LH5 match distance".into()));
    }

    let mut read_pos = (*dictionary_pos + LH5_DICSIZ - distance) & (LH5_DICSIZ - 1);
    for _ in 0..match_len {
        if output.len() >= original_size {
            break;
        }
        let byte = dictionary[read_pos];
        read_pos = (read_pos + 1) & (LH5_DICSIZ - 1);
        push_decoded_byte(output, dictionary, dictionary_pos, byte);
    }
    Ok(())
}

fn decode_position(reader: &mut BitReader<'_>, decoder: &HuffmanDecoder) -> Result<usize> {
    let code = decoder.decode(reader)?;
    if code == 0 {
        Ok(0)
    } else {
        Ok((1usize << (code - 1)) + reader.read_bits(code - 1)? as usize)
    }
}

fn read_pt_len(
    reader: &mut BitReader<'_>,
    symbol_count: usize,
    bit_count: usize,
    special_index: Option<usize>,
) -> Result<HuffmanDecoder> {
    let encoded_count = reader.read_bits(bit_count)? as usize;
    if encoded_count == 0 {
        return Ok(HuffmanDecoder::single(reader.read_bits(bit_count)? as usize));
    }

    let mut lengths = vec![0usize; symbol_count];
    let mut index = 0usize;
    while index < encoded_count {
        if index >= symbol_count {
            return Err(Error::InvalidData(
                "LH5 PT length count exceeds table".into(),
            ));
        }

        let mut length = reader.read_bits(3)? as usize;
        if length == 7 {
            while reader.read_bit()? != 0 {
                length += 1;
            }
        }
        lengths[index] = length;
        index += 1;

        if Some(index) == special_index {
            index += reader.read_bits(2)? as usize;
            if index > symbol_count {
                return Err(Error::InvalidData("LH5 PT zero run exceeds table".into()));
            }
        }
    }

    HuffmanDecoder::from_lengths(&lengths)
}

fn read_c_len(reader: &mut BitReader<'_>, pt_decoder: &HuffmanDecoder) -> Result<HuffmanDecoder> {
    let encoded_count = reader.read_bits(9)? as usize;
    if encoded_count == 0 {
        return Ok(HuffmanDecoder::single(reader.read_bits(9)? as usize));
    }

    let mut lengths = vec![0usize; LH5_NC];
    let mut index = 0usize;
    while index < encoded_count {
        if index >= LH5_NC {
            return Err(Error::InvalidData(
                "LH5 C length count exceeds table".into(),
            ));
        }

        let code = pt_decoder.decode(reader)?;
        if code <= 2 {
            let zero_count = match code {
                0 => 1,
                1 => reader.read_bits(4)? as usize + 3,
                2 => reader.read_bits(9)? as usize + 20,
                _ => unreachable!(),
            };
            index += zero_count;
            if index > LH5_NC {
                return Err(Error::InvalidData("LH5 C zero run exceeds table".into()));
            }
        } else {
            lengths[index] = code - 2;
            index += 1;
        }
    }

    HuffmanDecoder::from_lengths(&lengths)
}

struct BitReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    fn read_bits(&mut self, count: usize) -> Result<u32> {
        let mut value = 0u32;
        for _ in 0..count {
            value = (value << 1) | self.read_bit()? as u32;
        }
        Ok(value)
    }

    fn read_bit(&mut self) -> Result<u8> {
        let byte = *self
            .data
            .get(self.byte_pos)
            .ok_or_else(|| Error::InvalidData("LH5 bitstream ended early".into()))?;
        let bit = (byte >> (7 - self.bit_pos)) & 1;
        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }
        Ok(bit)
    }
}

enum HuffmanDecoder {
    Single(usize),
    Tree(Vec<HuffmanNode>),
}

impl HuffmanDecoder {
    fn single(symbol: usize) -> Self {
        Self::Single(symbol)
    }

    fn from_lengths(lengths: &[usize]) -> Result<Self> {
        let used = lengths
            .iter()
            .enumerate()
            .filter(|(_, length)| **length > 0)
            .collect::<Vec<_>>();
        if used.is_empty() {
            return Err(Error::InvalidData("empty LH5 Huffman tree".into()));
        }
        if used.len() == 1 {
            return Ok(Self::Single(used[0].0));
        }

        let max_bits = used.iter().map(|(_, length)| **length).max().unwrap_or(0);
        if max_bits >= usize::BITS as usize {
            return Err(Error::InvalidData(
                "LH5 Huffman code length is too large".into(),
            ));
        }

        let mut counts = vec![0usize; max_bits + 1];
        for (_, length) in &used {
            counts[**length] += 1;
        }

        let mut next_code = vec![0usize; max_bits + 1];
        let mut code = 0usize;
        for bits in 1..=max_bits {
            code = (code + counts[bits - 1]) << 1;
            next_code[bits] = code;
        }

        let mut nodes = vec![HuffmanNode::default()];
        for (symbol, length) in used {
            let code = next_code[*length];
            next_code[*length] += 1;
            insert_code(&mut nodes, code, *length, symbol)?;
        }

        Ok(Self::Tree(nodes))
    }

    fn decode(&self, reader: &mut BitReader<'_>) -> Result<usize> {
        match self {
            Self::Single(symbol) => Ok(*symbol),
            Self::Tree(nodes) => {
                let mut index = 0usize;
                loop {
                    if let Some(symbol) = nodes[index].symbol {
                        return Ok(symbol);
                    }
                    let bit = reader.read_bit()? as usize;
                    index = nodes[index].children[bit]
                        .ok_or_else(|| Error::InvalidData("invalid LH5 Huffman code".into()))?;
                }
            }
        }
    }
}

#[derive(Default)]
struct HuffmanNode {
    symbol: Option<usize>,
    children: [Option<usize>; 2],
}

fn insert_code(
    nodes: &mut Vec<HuffmanNode>,
    code: usize,
    length: usize,
    symbol: usize,
) -> Result<()> {
    let mut index = 0usize;
    for shift in (0..length).rev() {
        if nodes[index].symbol.is_some() {
            return Err(Error::InvalidData("ambiguous LH5 Huffman tree".into()));
        }

        let bit = (code >> shift) & 1;
        if nodes[index].children[bit].is_none() {
            nodes[index].children[bit] = Some(nodes.len());
            nodes.push(HuffmanNode::default());
        }
        index = nodes[index].children[bit].unwrap();
    }

    if nodes[index].symbol.is_some() || nodes[index].children.iter().any(Option::is_some) {
        return Err(Error::InvalidData("duplicate LH5 Huffman code".into()));
    }
    nodes[index].symbol = Some(symbol);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        decompress_lh5_member, decompress_lh5_member_with_budget, decompress_lh5_member_with_limits,
    };
    use crate::{Error, ParseLimits};

    #[test]
    fn decompresses_single_literal_lh5_member() {
        let compressed = bits(&[
            (1, 16),
            (0, 5),
            (0, 5),
            (0, 9),
            (b'A' as u32, 9),
            (0, 4),
            (0, 4),
        ]);
        let member = lh5_member(&compressed, 1);

        let decoded = decompress_lh5_member(&member).unwrap();

        assert_eq!(decoded.filename(), "");
        assert_eq!(decoded.packed_size(), compressed.len());
        assert_eq!(decoded.original_size(), 1);
        assert_eq!(decoded.bytes(), b"A");
    }

    #[test]
    fn rejects_lh5_member_before_allocating_output_over_limit() {
        // Given
        let compressed = bits(&[
            (1, 16),
            (0, 5),
            (0, 5),
            (0, 9),
            (b'A' as u32, 9),
            (0, 4),
            (0, 4),
        ]);
        let member = lh5_member(&compressed, 2);
        let limits = ParseLimits::DEFAULT.with_max_decompressed_bytes(1);

        // When
        let result = decompress_lh5_member_with_limits(&member, limits);

        // Then
        assert_eq!(
            result,
            Err(Error::ResourceLimit {
                resource: "LH5 decompressed bytes",
                limit: 1,
                actual: 2,
            })
        );
    }

    #[test]
    fn rejects_second_lh5_member_when_cumulative_budget_is_exhausted() {
        // Given
        let compressed = bits(&[
            (1, 16),
            (0, 5),
            (0, 5),
            (0, 9),
            (b'A' as u32, 9),
            (0, 4),
            (0, 4),
        ]);
        let member = lh5_member(&compressed, 1);
        let limits = ParseLimits::DEFAULT
            .with_max_decompressed_bytes(1)
            .with_max_total_decompressed_bytes(1);
        let mut budget = limits.decompression_budget();

        // When
        let first = decompress_lh5_member_with_budget(&member, &mut budget);
        let second = decompress_lh5_member_with_budget(&member, &mut budget);

        // Then
        assert_eq!(first.unwrap().bytes(), b"A");
        assert_eq!(
            second,
            Err(Error::ResourceLimit {
                resource: "total LH5 decompressed bytes",
                limit: 1,
                actual: 2,
            })
        );
    }

    fn lh5_member(compressed: &[u8], original_size: u32) -> Vec<u8> {
        let header_size = 22u8;
        let mut bytes = vec![header_size, 0, b'-', b'l', b'h', b'5', b'-'];
        bytes.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&original_size.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.push(0x20);
        bytes.push(0);
        bytes.push(0);
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(compressed);
        bytes
    }

    fn bits(values: &[(u32, usize)]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut current = 0u8;
        let mut used = 0u8;
        for (value, count) in values {
            for shift in (0..*count).rev() {
                current = (current << 1) | (((value >> shift) & 1) as u8);
                used += 1;
                if used == 8 {
                    bytes.push(current);
                    current = 0;
                    used = 0;
                }
            }
        }
        if used > 0 {
            bytes.push(current << (8 - used));
        }
        bytes
    }
}
