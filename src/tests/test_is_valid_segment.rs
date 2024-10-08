use super::helpers::Cursor;
use crate::{Config, CrcStore};

pub fn common_config() -> Config {
    Config {
        seg_len: 16,
        buf_len: 8,
        validate_on_read: false,
    }
}

#[rustfmt::skip]
fn valid_15_bytes() -> Vec<u8> {
    vec![
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0xAD, 0x2D, 0x8E, 0xE1
        //       checksum ^^^^  ^^^^  ^^^^  ^^^^
    ]
}

#[rustfmt::skip]
fn valid_16_bytes() -> Vec<u8> {
    vec![
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B, 0x92, 0x70, 0xC9, 0x65
        //             checksum ^^^^  ^^^^  ^^^^  ^^^^
    ]
}

#[rustfmt::skip]
fn valid_21_bytes() -> Vec<u8> {
    vec![
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B, 0x92, 0x70, 0xC9, 0x65,
        //             checksum ^^^^  ^^^^  ^^^^  ^^^^
        0x10, 0xCF, 0xB5, 0xFF, 0xE9
        //    ^^^^  ^^^^  ^^^^  ^^^^ checksum
    ]
}

#[test]
fn test_is_valid_segment_len_0_true() {
    let cursor = Cursor::new(vec![]);
    let mut store = CrcStore::new(common_config(), cursor).unwrap();
    let result = store.is_valid_segment().unwrap();
    assert!(result);
}

#[test]
fn test_is_valid_segment_len_5_false() {
    let cursor = Cursor::new(vec![1, 2, 3, 4, 5]);
    let mut store = CrcStore::new(common_config(), cursor).unwrap();
    let is_valid = store.is_valid_segment().unwrap();
    assert_eq!(is_valid, false);
}

#[test]
fn test_is_valid_segment_len_5_true() {
    let cursor = Cursor::new(vec![0x00, 0xD2, 0x02, 0xEF, 0x8D]);
    let mut store = CrcStore::new(common_config(), cursor).unwrap();
    let is_valid = store.is_valid_segment().unwrap();
    assert_eq!(is_valid, true);
}

#[test]
fn test_is_valid_segment_len_15_true() {
    let data = valid_15_bytes();
    let cursor = Cursor::new(data);
    let mut store = CrcStore::new(common_config(), cursor).unwrap();
    let is_valid = store.is_valid_segment().unwrap();
    assert_eq!(is_valid, true);
}

#[test]
fn test_is_valid_segment_len_16_true() {
    let data = valid_16_bytes();
    let cursor = Cursor::new(data);
    let mut store = CrcStore::new(common_config(), cursor).unwrap();
    let is_valid = store.is_valid_segment().unwrap();
    assert_eq!(is_valid, true);
}

#[test]
fn test_is_valid_segment_len_16_false() {
    let mut data = valid_16_bytes();
    data[7] ^= 0x80;
    let cursor = Cursor::new(data);
    let mut store = CrcStore::new(common_config(), cursor).unwrap();
    let is_valid = store.is_valid_segment().unwrap();
    assert_eq!(is_valid, false);
}

#[test]
fn test_is_valid_segment_len_21_true() {
    let data = valid_21_bytes();
    let cursor = Cursor::new(data);
    let mut store = CrcStore::new(common_config(), cursor).unwrap();
    let is_valid = store.is_valid_segment().unwrap();
    assert_eq!(is_valid, true);
}
