use rand::Rng;

pub type Cursor = std::io::Cursor<Vec<u8>>;

/// Returns a vector of valid byte data, suitable for backing a `CrcStore`,
/// generated at random.
///
/// Important: not all lengths can be satisfied, because the minimum length
/// for a segment is 5 bytes.
pub fn valid_data<R: Rng>(rng: &mut R, seg_len: u32, len: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(len);
    let full_segments = len / seg_len as usize;
    let partial_segment_len = len % seg_len as usize;
    for _ in 0 .. full_segments {
        data.extend(valid_segment(rng, seg_len as usize));
    }
    if partial_segment_len > 0 {
        data.extend(valid_segment(rng, partial_segment_len));
    }
    data
}

/// Returns a valid segment (full length or partial).
///
/// Returns a random vector of specifed length consisting of:
/// - a 4-byte CRC32 (calculated over the body)
/// - a body of length `len - 4`
pub fn valid_segment<R: Rng>(rng: &mut R, len: usize) -> Vec<u8> {
    assert!(len > 4);
    let mut buf = vec![0; len];
    rng.fill_bytes(&mut buf[.. len - 4]);
    let checksum = crc32fast::hash(&buf[.. len - 4]);
    let checksum_bytes = checksum.to_be_bytes();
    buf[len - 4 ..].copy_from_slice(&checksum_bytes);
    buf
}

/// Returns a vector of random bytes of given length.
pub fn random_bytes<R: Rng>(rng: &mut R, len: usize) -> Vec<u8> {
    let mut buf = vec![0; len];
    rng.fill_bytes(&mut buf);
    buf
}

#[allow(unused)]
pub fn multiline_hex_dump(buf: &[u8], len: usize) -> String {
    buf.chunks(len)
        .map(|chunk| hex_dump(chunk, len))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn hex_dump(bytes: &[u8], len: usize) -> String {
    let a = format!("{:width$}", hex_string(bytes), width = len * 3);
    let b = format!("{:width$}", ascii_string(bytes), width = len);
    format!("| {}| {} |", a, b)
}

#[allow(clippy::format_collect)]

pub fn hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| format!("{:02X} ", b)).collect()
}

pub fn ascii_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
        .collect()
}
