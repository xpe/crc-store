use super::helpers::{valid_crc_store, Cursor};
use crate::CrcStore;

/// Returns a `CrcStore` having seg_len=7 and body_len=3.
fn crc_store_seg_len_7() -> CrcStore<Cursor> {
    let seg_len: u32 = 7; // body_len=3
    let len = 6 * seg_len as usize;
    let mut rng = rand::thread_rng();
    valid_crc_store(&mut rng, seg_len, len)
}

#[test]
/// Example with seg_len=7 (so body_len=3):
///
/// seg 0  seg 1  seg 2  seg 3  seg 4  seg 5
/// |      |      |      |      |      |
/// ccccBBBccccBBBccccBBBccccBBBccccBBBccccBBB
///
/// - "cccc" is the 4-byte checksum
/// - "BBB" is the 3-byte body
fn test_inner_pos_seg_len_7() {
    let store = crc_store_seg_len_7();

    // segment 0
    assert_eq!(store.inner_pos(0), Some(4));
    assert_eq!(store.inner_pos(1), Some(5));
    assert_eq!(store.inner_pos(2), Some(6));
    // segment 1
    assert_eq!(store.inner_pos(3), Some(11));
    assert_eq!(store.inner_pos(4), Some(12));
    assert_eq!(store.inner_pos(5), Some(13));
    // segment 2
    assert_eq!(store.inner_pos(6), Some(18));
    assert_eq!(store.inner_pos(7), Some(19));
    assert_eq!(store.inner_pos(8), Some(20));
    // segment 3
    assert_eq!(store.inner_pos(9), Some(25));
    assert_eq!(store.inner_pos(10), Some(26));
    assert_eq!(store.inner_pos(11), Some(27));
}

/// Example with seg_len=7 (so body_len=3):
///
/// seg 0  seg 1  seg 2  seg 3  seg 4  seg 5
/// |      |      |      |      |      |
/// ccccBBBccccBBBccccBBBccccBBBccccBBBccccBBB
///                    |
///           inner_pos=19
///              offset=5
///         body_offset=1
///
/// - "cccc" is the 4-byte checksum
/// - "BBB" is the 3-byte body
#[test]
fn test_rel_inner_pos_seg_len_7_body_offset_1() {
    let store = crc_store_seg_len_7();

    let inner_pos = 19;
    // segment 1
    assert_eq!(store.rel_inner_pos(-7, inner_pos), Ok(-15));
    assert_eq!(store.rel_inner_pos(-6, inner_pos), Ok(-14));
    assert_eq!(store.rel_inner_pos(-5, inner_pos), Ok(-13));
    // segment 2
    assert_eq!(store.rel_inner_pos(-4, inner_pos), Ok(-8));
    assert_eq!(store.rel_inner_pos(-3, inner_pos), Ok(-7));
    assert_eq!(store.rel_inner_pos(-2, inner_pos), Ok(-6));
    // segment 2
    assert_eq!(store.rel_inner_pos(-1, inner_pos), Ok(-1));
    assert_eq!(store.rel_inner_pos(0, inner_pos), Ok(0));
    assert_eq!(store.rel_inner_pos(1, inner_pos), Ok(1));
    // segment 3
    assert_eq!(store.rel_inner_pos(2, inner_pos), Ok(6));
    assert_eq!(store.rel_inner_pos(3, inner_pos), Ok(7));
    assert_eq!(store.rel_inner_pos(4, inner_pos), Ok(8));
    // segment 4
    assert_eq!(store.rel_inner_pos(5, inner_pos), Ok(13));
    assert_eq!(store.rel_inner_pos(6, inner_pos), Ok(14));
    assert_eq!(store.rel_inner_pos(7, inner_pos), Ok(15));
}

/// Example with seg_len=9 (so body_len=5):
///
/// seg 0    seg 1    seg 2    seg 3    seg 4
/// |        |        |        |        |
/// ccccBBBBBccccBBBBBccccBBBBBccccBBBBBccccBBBBB
///
/// - "cccc" is the 4-byte checksum
/// - "BBBBB" is the 4-byte bodyfn test_outer_pos() {
#[test]
fn test_outer_pos_seg_len_7() {
    let seg_len: u32 = 9;
    let len = 5 * seg_len as usize;
    let mut rng = rand::thread_rng();
    let store = valid_crc_store(&mut rng, seg_len, len);

    // segment 0
    assert_eq!(store.outer_pos(0), None);
    assert_eq!(store.outer_pos(1), None);
    assert_eq!(store.outer_pos(2), None);
    assert_eq!(store.outer_pos(3), None);
    assert_eq!(store.outer_pos(4), Some(0));
    assert_eq!(store.outer_pos(5), Some(1));
    assert_eq!(store.outer_pos(6), Some(2));
    assert_eq!(store.outer_pos(7), Some(3));
    assert_eq!(store.outer_pos(8), Some(4));
    // segment 1
    assert_eq!(store.outer_pos(9), None);
    assert_eq!(store.outer_pos(10), None);
    assert_eq!(store.outer_pos(11), None);
    assert_eq!(store.outer_pos(12), None);
    assert_eq!(store.outer_pos(13), Some(5));
    assert_eq!(store.outer_pos(14), Some(6));
    assert_eq!(store.outer_pos(15), Some(7));
    assert_eq!(store.outer_pos(16), Some(8));
    assert_eq!(store.outer_pos(17), Some(9));
    // segment 2
    assert_eq!(store.outer_pos(18), None);
    assert_eq!(store.outer_pos(19), None);
    assert_eq!(store.outer_pos(20), None);
    assert_eq!(store.outer_pos(21), None);
    assert_eq!(store.outer_pos(22), Some(10));
    assert_eq!(store.outer_pos(23), Some(11));
    assert_eq!(store.outer_pos(24), Some(12));
    assert_eq!(store.outer_pos(25), Some(13));
    assert_eq!(store.outer_pos(26), Some(14));
}

/// Returns a `CrcStore` having seg_len=8 and body_len=4.
fn crc_store_seg_len_8() -> CrcStore<Cursor> {
    let seg_len: u32 = 8; // body_len=4
    let len = 6 * seg_len as usize;
    let mut rng = rand::thread_rng();
    valid_crc_store(&mut rng, seg_len, len)
}

/// Example with seg_len=8 (so body_len=4):
///
/// seg 0   seg 1   seg 2   seg 3   seg 4   seg 5
/// |       |       |       |       |       |
/// ccccBBBBccccBBBBccccBBBBccccBBBBccccBBBBccccBBBB
///                     |
///            inner_pos=20
///               offset=4
///          body_offset=0
///
/// - "cccc" is the 4-byte checksum
/// - "BBBB" is the 4-byte body
#[test]
fn test_rel_inner_pos_seg_len_8_body_offset_0() {
    let store = crc_store_seg_len_8();
    let inner_pos = 20;
    // segment 1
    assert_eq!(store.rel_inner_pos(-4, inner_pos), Ok(-8));
    assert_eq!(store.rel_inner_pos(-3, inner_pos), Ok(-7));
    assert_eq!(store.rel_inner_pos(-2, inner_pos), Ok(-6));
    assert_eq!(store.rel_inner_pos(-1, inner_pos), Ok(-5));
    // segment 2
    assert_eq!(store.rel_inner_pos(0, inner_pos), Ok(0));
    assert_eq!(store.rel_inner_pos(1, inner_pos), Ok(1));
    assert_eq!(store.rel_inner_pos(2, inner_pos), Ok(2));
    assert_eq!(store.rel_inner_pos(3, inner_pos), Ok(3));
    // segment 3
    assert_eq!(store.rel_inner_pos(4, inner_pos), Ok(8));
    assert_eq!(store.rel_inner_pos(5, inner_pos), Ok(9));
    assert_eq!(store.rel_inner_pos(6, inner_pos), Ok(10));
    assert_eq!(store.rel_inner_pos(7, inner_pos), Ok(11));
    // segment 4
    assert_eq!(store.rel_inner_pos(8, inner_pos), Ok(16));
    assert_eq!(store.rel_inner_pos(9, inner_pos), Ok(17));
    assert_eq!(store.rel_inner_pos(10, inner_pos), Ok(18));
    assert_eq!(store.rel_inner_pos(11, inner_pos), Ok(19));
}

/// Example with seg_len=8 (so body_len=4):
///
/// seg 0   seg 1   seg 2   seg 3   seg 4   seg 5
/// |       |       |       |       |       |
/// ccccBBBBccccBBBBccccBBBBccccBBBBccccBBBBccccBBBB
///                      |
///             inner_pos=21
///                offset=5
///           body_offset=1
///
/// - "cccc" is the 4-byte checksum
/// - "BBBB" is the 4-byte body
#[test]
fn test_rel_inner_pos_seg_len_8_body_offset_1() {
    let store = crc_store_seg_len_8();
    let inner_pos = 21;
    // segment 1
    assert_eq!(store.rel_inner_pos(-5, inner_pos), Ok(-9));
    assert_eq!(store.rel_inner_pos(-4, inner_pos), Ok(-8));
    assert_eq!(store.rel_inner_pos(-3, inner_pos), Ok(-7));
    assert_eq!(store.rel_inner_pos(-2, inner_pos), Ok(-6));
    // segment 2
    assert_eq!(store.rel_inner_pos(-1, inner_pos), Ok(-1));
    assert_eq!(store.rel_inner_pos(0, inner_pos), Ok(0));
    assert_eq!(store.rel_inner_pos(1, inner_pos), Ok(1));
    assert_eq!(store.rel_inner_pos(2, inner_pos), Ok(2));
    // segment 3
    assert_eq!(store.rel_inner_pos(3, inner_pos), Ok(7));
    assert_eq!(store.rel_inner_pos(4, inner_pos), Ok(8));
    assert_eq!(store.rel_inner_pos(5, inner_pos), Ok(9));
    assert_eq!(store.rel_inner_pos(6, inner_pos), Ok(10));
    // segment 4
    assert_eq!(store.rel_inner_pos(7, inner_pos), Ok(15));
    assert_eq!(store.rel_inner_pos(8, inner_pos), Ok(16));
    assert_eq!(store.rel_inner_pos(9, inner_pos), Ok(17));
    assert_eq!(store.rel_inner_pos(10, inner_pos), Ok(18));
}

/// Example with seg_len=8 (so body_len=4):
///
/// seg 0   seg 1   seg 2   seg 3   seg 4   seg 5
/// |       |       |       |       |       |
/// ccccBBBBccccBBBBccccBBBBccccBBBBccccBBBBccccBBBB
///                       |
///              inner_pos=22
///                 offset=6
///            body_offset=2
///
/// - "cccc" is the 4-byte checksum
/// - "BBBB" is the 4-byte body
#[test]
fn test_rel_inner_pos_seg_len_8_body_offset_2() {
    let store = crc_store_seg_len_8();
    let inner_pos = 22;
    // segment 1
    assert_eq!(store.rel_inner_pos(-6, inner_pos), Ok(-10));
    assert_eq!(store.rel_inner_pos(-5, inner_pos), Ok(-9));
    assert_eq!(store.rel_inner_pos(-4, inner_pos), Ok(-8));
    assert_eq!(store.rel_inner_pos(-3, inner_pos), Ok(-7));
    // segment 2
    assert_eq!(store.rel_inner_pos(-2, inner_pos), Ok(-2));
    assert_eq!(store.rel_inner_pos(-1, inner_pos), Ok(-1));
    assert_eq!(store.rel_inner_pos(0, inner_pos), Ok(0));
    assert_eq!(store.rel_inner_pos(1, inner_pos), Ok(1));
    // segment 3
    assert_eq!(store.rel_inner_pos(2, inner_pos), Ok(6));
    assert_eq!(store.rel_inner_pos(3, inner_pos), Ok(7));
    assert_eq!(store.rel_inner_pos(4, inner_pos), Ok(8));
    assert_eq!(store.rel_inner_pos(5, inner_pos), Ok(9));
    // segment 4
    assert_eq!(store.rel_inner_pos(6, inner_pos), Ok(14));
    assert_eq!(store.rel_inner_pos(7, inner_pos), Ok(15));
    assert_eq!(store.rel_inner_pos(8, inner_pos), Ok(16));
    assert_eq!(store.rel_inner_pos(9, inner_pos), Ok(17));
}

/// Example with seg_len=8 (so body_len=4):
///
/// seg 0   seg 1   seg 2   seg 3   seg 4   seg 5
/// |       |       |       |       |       |
/// ccccBBBBccccBBBBccccBBBBccccBBBBccccBBBBccccBBBB
///                        |
///               inner_pos=23
///                  offset=7
///             body_offset=3
///
/// - "cccc" is the 4-byte checksum
/// - "BBBB" is the 4-byte body
#[test]
fn test_rel_inner_pos_seg_len_8_body_offset_3() {
    let store = crc_store_seg_len_8();
    let inner_pos = 23;
    // segment 1
    assert_eq!(store.rel_inner_pos(-7, inner_pos), Ok(-11));
    assert_eq!(store.rel_inner_pos(-6, inner_pos), Ok(-10));
    assert_eq!(store.rel_inner_pos(-5, inner_pos), Ok(-9));
    assert_eq!(store.rel_inner_pos(-4, inner_pos), Ok(-8));
    // segment 2
    assert_eq!(store.rel_inner_pos(-3, inner_pos), Ok(-3));
    assert_eq!(store.rel_inner_pos(-2, inner_pos), Ok(-2));
    assert_eq!(store.rel_inner_pos(-1, inner_pos), Ok(-1));
    assert_eq!(store.rel_inner_pos(0, inner_pos), Ok(0));
    // segment 3
    assert_eq!(store.rel_inner_pos(1, inner_pos), Ok(5));
    assert_eq!(store.rel_inner_pos(2, inner_pos), Ok(6));
    assert_eq!(store.rel_inner_pos(3, inner_pos), Ok(7));
    assert_eq!(store.rel_inner_pos(4, inner_pos), Ok(8));
    // segment 4
    assert_eq!(store.rel_inner_pos(5, inner_pos), Ok(13));
    assert_eq!(store.rel_inner_pos(6, inner_pos), Ok(14));
    assert_eq!(store.rel_inner_pos(7, inner_pos), Ok(15));
    assert_eq!(store.rel_inner_pos(8, inner_pos), Ok(16));
}
