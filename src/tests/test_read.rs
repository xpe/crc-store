use std::io::{Read, Seek, SeekFrom};

use super::helpers as h;
use super::helpers::Cursor;
use crate::{Config, CrcStore};

fn crc_store(len: usize) -> CrcStore<Cursor> {
    let cfg = Config {
        seg_len: 16,
        buf_len: 16,
        validate_on_read: false,
    };
    let mut rng = rand::thread_rng();
    let data = h::valid_data(&mut rng, cfg.seg_len, len);
    let cursor = Cursor::new(data);
    CrcStore::new(cfg, cursor).unwrap()
}

#[test]
#[rustfmt::skip]
fn test_read_start_0() {
    let mut store = crc_store(128); // body_len=12
    let mut read_buf = vec![0; 48];
    let result = store.read(&mut read_buf);
    let inner: Vec<u8> = store.inner.into_inner();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 48);
    assert_eq!(read_buf[ 0 .. 12], inner[ 0 .. 12]);
    assert_eq!(read_buf[12 .. 24], inner[16 .. 28]);
    assert_eq!(read_buf[24 .. 36], inner[32 .. 44]);
    assert_eq!(read_buf[36 .. 48], inner[48 .. 60]);
}

#[test]
#[rustfmt::skip]
fn test_read_start_1() {
    let mut store = crc_store(128); // body_len=12
    store.seek(SeekFrom::Start(1)).unwrap();
    let mut read_buf = vec![0; 47];
    let result = store.read(&mut read_buf);
    let inner: Vec<u8> = store.inner.into_inner();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 47);
    assert_eq!(read_buf[ 0     .. 12 - 1], inner[0 + 1 .. 12]);
    assert_eq!(read_buf[12 - 1 .. 24 - 1], inner[16    .. 28]);
    assert_eq!(read_buf[24 - 1 .. 36 - 1], inner[32    .. 44]);
    assert_eq!(read_buf[36 - 1 .. 48 - 1], inner[48    .. 60]);
}

#[test]
#[rustfmt::skip]
fn test_read_current_12() {
    let mut store = crc_store(128); // body_len=12
    store.seek(SeekFrom::Current(12)).unwrap();
    let mut read_buf = vec![0; 24];
    let result = store.read(&mut read_buf);
    let inner: Vec<u8> = store.inner.into_inner();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 24);
    assert_eq!(read_buf[ 0 .. 12], inner[16 .. 28]);
    assert_eq!(read_buf[12 .. 24], inner[32 .. 44]);
}

#[test]
fn test_read_end_8() {
    let mut store = crc_store(128); // body_len=12
    let pos = store.seek(SeekFrom::End(8)).unwrap();
    assert_eq!(pos, 116); // 128 - 8 - 4
    let mut read_buf = vec![0; 8];
    let result = store.read(&mut read_buf);
    let inner: Vec<u8> = store.inner.into_inner();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 8);
    assert_eq!(read_buf[0 .. 8], inner[116 .. 124]);
}

#[test]
#[rustfmt::skip]
fn test_read_end_16() {
    let mut store = crc_store(128); // body_len=12
    let pos = store.seek(SeekFrom::End(16)).unwrap();
    //   0 ..  16: segment 0
    //  96 .. 112: segment 6
    // 112 .. 128: segment 7
    assert_eq!(pos, 104); // 128 - 8 - 4
    let mut read_buf = vec![0; 16];
    let result = store.read(&mut read_buf);
    let inner: Vec<u8> = store.inner.into_inner();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 16);
    assert_eq!(read_buf[0 ..  4], inner[104 .. 108]);
    assert_eq!(read_buf[4 .. 16], inner[112 .. 124]);
}
