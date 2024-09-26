use std::io::{Read, Seek, SeekFrom};

use super::helpers::{valid_crc_store, Cursor};
use crate::CrcStore;

/// Returns a `CrcStore` having seg_len=9 and body_len=5.
fn crc_store_seg_len_9() -> CrcStore<Cursor> {
    let seg_len: u32 = 9; // body_len = 5;
    let len = 3 * seg_len as usize;
    let mut rng = rand::thread_rng();
    valid_crc_store(&mut rng, seg_len, len)
}

#[test]
fn test_crc_store_read_seg_len_9_start_0() {
    let mut store = crc_store_seg_len_9(); // body_len=5
    let mut read_buf = vec![0; 15];
    let result = store.read(&mut read_buf);
    let inner: Vec<u8> = store.inner.into_inner();
    assert!(result.is_ok());
    assert_eq!(read_buf[0 .. 5], inner[4 .. 9]);
    assert_eq!(read_buf[5 .. 10], inner[13 .. 18]);
    assert_eq!(read_buf[10 .. 15], inner[22 .. 27]);
}

#[test]
fn test_crc_store_read_seg_len_9_start_1() {
    let mut store = crc_store_seg_len_9(); // body_len=5
    let mut read_buf = vec![0; 14];
    store.seek(SeekFrom::Start(1)).unwrap();
    let result = store.read(&mut read_buf);
    let inner: Vec<u8> = store.inner.into_inner();
    assert!(result.is_ok());
    assert_eq!(read_buf[0 .. 4], inner[5 .. 9]);
    assert_eq!(read_buf[4 .. 9], inner[13 .. 18]);
    assert_eq!(read_buf[9 .. 14], inner[22 .. 27]);
}

/// Returns a `CrcStore` having seg_len=10 and body_len=6.
fn crc_store_seg_len_10() -> CrcStore<Cursor> {
    let seg_len: u32 = 10; // body_len = 5;
    let len = 3 * seg_len as usize;
    let mut rng = rand::thread_rng();
    valid_crc_store(&mut rng, seg_len, len)
}

#[test]
fn test_crc_store_read_seg_len_10_start_0() {
    let mut store = crc_store_seg_len_10(); // body_len=6
    let mut read_buf = vec![0; 18];
    let result = store.read(&mut read_buf);
    let data: Vec<u8> = store.inner.into_inner();
    assert!(result.is_ok());
    assert_eq!(read_buf[0 .. 6], data[4 .. 10]);
    assert_eq!(read_buf[6 .. 12], data[14 .. 20]);
    assert_eq!(read_buf[12 .. 18], data[24 .. 30]);
}

#[test]
fn test_crc_store_read_seg_len_10_start_2() {
    let mut store = crc_store_seg_len_10(); // body_len=6
    let mut read_buf = vec![0; 16];
    store.seek(SeekFrom::Start(2)).unwrap();
    let result = store.read(&mut read_buf);
    let data: Vec<u8> = store.inner.into_inner();
    assert!(result.is_ok());
    assert_eq!(read_buf[0 .. 4], data[6 .. 10]);
    assert_eq!(read_buf[4 .. 10], data[14 .. 20]);
    assert_eq!(read_buf[10 .. 16], data[24 .. 30]);
}
