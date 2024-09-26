use rand::seq::SliceRandom;
use rand::Rng;

use crate::{crc32_to_be_bytes, CrcStore};

pub type Cursor = std::io::Cursor<Vec<u8>>;

/// Returns a CrcStore backed by random currupt data. The data start as valid
/// but `changes` to checksums are applied (provided the backing data is large
/// enough). A checksum in a particular segment will be changed at most once.
pub fn corrupt_crc_store<R: Rng>(
    rng: &mut R,
    seg_len: u32,
    len: usize,
    changes: usize,
) -> CrcStore<Cursor> {
    let mut data = valid_data(rng, seg_len, len);
    let actual_changes = corrupt_data(rng, seg_len, &mut data, changes);
    assert_eq!(actual_changes, changes);
    let cursor = Cursor::new(data);
    CrcStore::new(seg_len, cursor).unwrap()
}

/// Mutates its argument. Returns corrupt data by starting with valid data that
/// is corrupted in `changes` checksums, selected at random. Each checksum will
/// be corrupted at most once. Each change occurs via XOR, an operation that
/// ensures the byte will change. Always corrupts `changes` places unless the
/// `data` isn't long enough. Returns the number of bytes that were changed.
///
/// Important: Due to the complexity, it would be best if this helper function
/// itself `was tested!
///
/// History: originally this function could change any byte. But doing this
/// correctly is tricky. In rare cases, a change to a body and its checksum will
/// result in valid data. To make it easier to reason about, this function only
/// changes one bit of a checksum.
pub fn corrupt_data<R: Rng>(
    rng: &mut R,
    seg_len: u32,
    data: &mut [u8],
    mut changes: usize,
) -> usize {
    let segments = data.len() / seg_len as usize;
    changes = changes.min(segments);
    let mut indices: Vec<usize> = (0 .. segments).collect();
    let (shuffled, _) = indices.partial_shuffle(rng, changes);
    for segment_index in shuffled {
        let i = *segment_index * seg_len as usize;
        assert!(i + 4 <= data.len());
        flip_random_bit(rng, &mut data[i .. i + 4]);
    }
    changes
}

fn flip_random_bit<R: Rng>(rng: &mut R, buf: &mut [u8]) {
    let bit: u8 = rng.gen::<u8>() % 32;
    flip_specified_bit(buf, bit);
}

/// Flips a specified bit in a 4-byte buffer.
fn flip_specified_bit(buf: &mut [u8], bit: u8) {
    assert_eq!(buf.len(), 4);
    let index = (3 - (bit / 8)) as usize;
    let bit_in_byte = bit % 8;
    let mask: u8 = 1 << bit_in_byte as usize;
    buf[index] ^= mask;
}

/// Returns a `CrcStore` backed by valid data of specified length.
///
/// Note: `len` does not have to be a multiple of `seg_len`.
pub fn valid_crc_store<R: Rng>(rng: &mut R, seg_len: u32, len: usize) -> CrcStore<Cursor> {
    let data = valid_data(rng, seg_len, len);
    let cursor = Cursor::new(data);
    CrcStore::new(seg_len, cursor).unwrap()
}

/// Returns a vector of valid byte data, suitable for backing a `CrcStore`,
/// generated at random.
pub fn valid_data<R: Rng>(rng: &mut R, seg_len: u32, len: usize) -> Vec<u8> {
    let mut data = Vec::new();
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
    rng.fill_bytes(&mut buf[4 ..]);
    let checksum_bytes = crc32_to_be_bytes(&buf[4 ..]);
    buf[.. 4].copy_from_slice(&checksum_bytes);
    buf
}

/// Returns an empty `CrcStore` backed by a cursor over a vector.
pub fn empty_crc_store(seg_len: u32) -> CrcStore<Cursor> {
    let cursor = Cursor::new(Vec::new());
    CrcStore::new(seg_len, cursor).unwrap()
}

/// Returns a vector of random bytes of given length.
pub fn random_bytes<R: Rng>(rng: &mut R, len: usize) -> Vec<u8> {
    let mut buf = vec![0; len];
    rng.fill_bytes(&mut buf);
    buf
}
