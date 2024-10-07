use std::io::{Seek, SeekFrom};

use super::helpers as h;
use super::helpers::Cursor;
use crate::{Config, CrcStore};

fn crc_store(len: usize) -> CrcStore<Cursor> {
    let cfg = Config {
        seg_len: 8,
        buf_len: 8,
        validate_on_read: false,
    };
    let mut rng = rand::thread_rng();
    let data = h::valid_data(&mut rng, cfg.seg_len, len);
    let cursor = Cursor::new(data);
    CrcStore::new(cfg, cursor).unwrap()
}

/// Example with seg_len=8 (so body_len=4):
///
/// ```text
/// seg 0   seg 1   seg 2   seg 3   seg  4  seg 5
/// |       |       |       |       |       |
/// BBBBccccBBBBccccBBBBccccBBBBccccBBBBccccBBBBcccc
/// ```
#[test]
fn test_start_pos() {
    let store = crc_store(48);

    // segment 0
    assert_eq!(store.start_pos(0), Some(0));
    assert_eq!(store.start_pos(1), Some(1));
    assert_eq!(store.start_pos(2), Some(2));
    assert_eq!(store.start_pos(3), Some(3));
    // segment 1
    assert_eq!(store.start_pos(4), Some(8));
    assert_eq!(store.start_pos(5), Some(9));
    assert_eq!(store.start_pos(6), Some(10));
    assert_eq!(store.start_pos(7), Some(11));
    // segment 2
    assert_eq!(store.start_pos(8), Some(16));
    assert_eq!(store.start_pos(9), Some(17));
    assert_eq!(store.start_pos(10), Some(18));
    assert_eq!(store.start_pos(11), Some(19));
}

/// Example with seg_len=8 (so body_len=4):
///
/// ```text
/// seg 0   seg 1   seg 2   seg 3   seg 4   seg 5
/// |       |       |       |       |       |
/// BBBBccccBBBBccccBBBBccccBBBBccccBBBBccccBBBBcccc
///                 |
///        inner_pos=16
///           offset=0
/// ```
#[test]
fn test_rel_inner_pos_seg_offset_0() {
    let store = crc_store(48);
    let inner_pos = 16;
    // segment 1
    assert_eq!(store.rel_inner_pos(-4, inner_pos), Some(-8));
    assert_eq!(store.rel_inner_pos(-3, inner_pos), Some(-7));
    assert_eq!(store.rel_inner_pos(-2, inner_pos), Some(-6));
    assert_eq!(store.rel_inner_pos(-1, inner_pos), Some(-5));
    // segment 2
    assert_eq!(store.rel_inner_pos(0, inner_pos), Some(0));
    assert_eq!(store.rel_inner_pos(1, inner_pos), Some(1));
    assert_eq!(store.rel_inner_pos(2, inner_pos), Some(2));
    assert_eq!(store.rel_inner_pos(3, inner_pos), Some(3));
    // segment 3
    assert_eq!(store.rel_inner_pos(4, inner_pos), Some(8));
    assert_eq!(store.rel_inner_pos(5, inner_pos), Some(9));
    assert_eq!(store.rel_inner_pos(6, inner_pos), Some(10));
    assert_eq!(store.rel_inner_pos(7, inner_pos), Some(11));
    // segment 4
    assert_eq!(store.rel_inner_pos(8, inner_pos), Some(16));
    assert_eq!(store.rel_inner_pos(9, inner_pos), Some(17));
    assert_eq!(store.rel_inner_pos(10, inner_pos), Some(18));
    assert_eq!(store.rel_inner_pos(11, inner_pos), Some(19));
}

/// Example with seg_len=8 (so body_len=4):
///
/// ```text
/// seg 0   seg 1   seg 2   seg 3   seg 4   seg 5
/// |       |       |       |       |       |
/// BBBBccccBBBBccccBBBBccccBBBBccccBBBBccccBBBBcccc
///                  |
///         inner_pos=17
///            offset=1
/// ```
#[test]
fn test_rel_inner_pos_seg_offset_1() {
    let store = crc_store(48);
    let inner_pos = 17;
    // segment 1
    assert_eq!(store.rel_inner_pos(-5, inner_pos), Some(-9));
    assert_eq!(store.rel_inner_pos(-4, inner_pos), Some(-8));
    assert_eq!(store.rel_inner_pos(-3, inner_pos), Some(-7));
    assert_eq!(store.rel_inner_pos(-2, inner_pos), Some(-6));
    // segment 2
    assert_eq!(store.rel_inner_pos(-1, inner_pos), Some(-1));
    assert_eq!(store.rel_inner_pos(0, inner_pos), Some(0));
    assert_eq!(store.rel_inner_pos(1, inner_pos), Some(1));
    assert_eq!(store.rel_inner_pos(2, inner_pos), Some(2));
    // segment 3
    assert_eq!(store.rel_inner_pos(3, inner_pos), Some(7));
    assert_eq!(store.rel_inner_pos(4, inner_pos), Some(8));
    assert_eq!(store.rel_inner_pos(5, inner_pos), Some(9));
    assert_eq!(store.rel_inner_pos(6, inner_pos), Some(10));
    // segment 4
    assert_eq!(store.rel_inner_pos(7, inner_pos), Some(15));
    assert_eq!(store.rel_inner_pos(8, inner_pos), Some(16));
    assert_eq!(store.rel_inner_pos(9, inner_pos), Some(17));
    assert_eq!(store.rel_inner_pos(10, inner_pos), Some(18));
}

/// Example with seg_len=8 (so body_len=4):
///
/// ```text
/// seg 0   seg 1   seg 2   seg 3   seg 4   seg 5
/// |       |       |       |       |       |
/// BBBBccccBBBBccccBBBBccccBBBBccccBBBBccccBBBBcccc
///                   |
///          inner_pos=18
///             offset=2
/// ```
#[test]
fn test_rel_inner_pos_seg_offset_2() {
    let store = crc_store(48);
    let inner_pos = 18;
    // segment 1
    assert_eq!(store.rel_inner_pos(-6, inner_pos), Some(-10));
    assert_eq!(store.rel_inner_pos(-5, inner_pos), Some(-9));
    assert_eq!(store.rel_inner_pos(-4, inner_pos), Some(-8));
    assert_eq!(store.rel_inner_pos(-3, inner_pos), Some(-7));
    // segment 2
    assert_eq!(store.rel_inner_pos(-2, inner_pos), Some(-2));
    assert_eq!(store.rel_inner_pos(-1, inner_pos), Some(-1));
    assert_eq!(store.rel_inner_pos(0, inner_pos), Some(0));
    assert_eq!(store.rel_inner_pos(1, inner_pos), Some(1));
    // segment 3
    assert_eq!(store.rel_inner_pos(2, inner_pos), Some(6));
    assert_eq!(store.rel_inner_pos(3, inner_pos), Some(7));
    assert_eq!(store.rel_inner_pos(4, inner_pos), Some(8));
    assert_eq!(store.rel_inner_pos(5, inner_pos), Some(9));
    // segment 4
    assert_eq!(store.rel_inner_pos(6, inner_pos), Some(14));
    assert_eq!(store.rel_inner_pos(7, inner_pos), Some(15));
    assert_eq!(store.rel_inner_pos(8, inner_pos), Some(16));
    assert_eq!(store.rel_inner_pos(9, inner_pos), Some(17));
}

/// Example with seg_len=8 (so body_len=4):
///
/// ```text
/// seg 0   seg 1   seg 2   seg 3   seg 4   seg 5
/// |       |       |       |       |       |
/// BBBBccccBBBBccccBBBBccccBBBBccccBBBBccccBBBBcccc
///                    |
///           inner_pos=19
///              offset=3
/// ```
#[test]
fn test_rel_inner_pos_offset_3() {
    let store = crc_store(48);
    let inner_pos = 19;
    // segment 1
    assert_eq!(store.rel_inner_pos(-7, inner_pos), Some(-11));
    assert_eq!(store.rel_inner_pos(-6, inner_pos), Some(-10));
    assert_eq!(store.rel_inner_pos(-5, inner_pos), Some(-9));
    assert_eq!(store.rel_inner_pos(-4, inner_pos), Some(-8));
    // segment 2
    assert_eq!(store.rel_inner_pos(-3, inner_pos), Some(-3));
    assert_eq!(store.rel_inner_pos(-2, inner_pos), Some(-2));
    assert_eq!(store.rel_inner_pos(-1, inner_pos), Some(-1));
    assert_eq!(store.rel_inner_pos(0, inner_pos), Some(0));
    // segment 3
    assert_eq!(store.rel_inner_pos(1, inner_pos), Some(5));
    assert_eq!(store.rel_inner_pos(2, inner_pos), Some(6));
    assert_eq!(store.rel_inner_pos(3, inner_pos), Some(7));
    assert_eq!(store.rel_inner_pos(4, inner_pos), Some(8));
    // segment 4
    assert_eq!(store.rel_inner_pos(5, inner_pos), Some(13));
    assert_eq!(store.rel_inner_pos(6, inner_pos), Some(14));
    assert_eq!(store.rel_inner_pos(7, inner_pos), Some(15));
    assert_eq!(store.rel_inner_pos(8, inner_pos), Some(16));
}

/// ```text
/// seg 0
/// |
/// Bcccc
///  |   |
///  ^   inner_len=5
/// ```
#[test]
fn test_len_5_seek_from_end_0() {
    let mut store = crc_store(5);
    let result = store.seek(SeekFrom::End(0));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

/// ```text
/// seg 0
/// |
/// Bcccc
/// |    |
/// ^    inner_len=5
/// ```
#[test]
fn test_len_5_seek_from_end_1() {
    let mut store = crc_store(5);
    let result = store.seek(SeekFrom::End(-1));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

/// ```text
/// seg 0   seg 1   seg 2
/// |       |       |
/// BBBBccccBBBBccccBBcccc
///                   |   |
///                   ^   inner_len=22
/// ```
#[test]
fn test_len_22_seek_from_end_0() {
    let mut store = crc_store(22);
    let result = store.seek(SeekFrom::End(0));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 18);
}

/// ```text
/// seg 0   seg 1   seg 2
/// |       |       |
/// BBBBccccBBBBccccBBcccc
///                  |    |
///                  ^    inner_len=22
/// ```
#[test]
fn test_len_22_seek_from_end_1() {
    let mut store = crc_store(22);
    let result = store.seek(SeekFrom::End(-1));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 17);
}
