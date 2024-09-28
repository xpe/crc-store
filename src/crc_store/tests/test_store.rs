use super::helpers::Cursor;
use crate::{CrcStore, Error};

fn crc_store(data: Vec<u8>) -> Result<CrcStore<Cursor>, Error> {
    let cursor = Cursor::new(data);
    let seg_len: u32 = 128; // body_len=124
    CrcStore::new(seg_len, cursor)
}

#[test]
fn test_new_len_0() {
    let result = crc_store(vec![]);
    assert!(matches!(result, Ok(_)));
}

#[test]
fn test_new_len_1() {
    let result = crc_store(vec![1]);
    assert!(matches!(result, Err(Error::InvalidInner)));
}

#[test]
fn test_new_len_2() {
    let result = crc_store(vec![1, 2]);
    assert!(matches!(result, Err(Error::InvalidInner)));
}

#[test]
fn test_new_len_3() {
    let result = crc_store(vec![1, 2, 3]);
    assert!(matches!(result, Err(Error::InvalidInner)));
}

#[test]
fn test_new_len_4() {
    let result = crc_store(vec![1, 2, 3, 4]);
    assert!(matches!(result, Err(Error::InvalidInner)));
}

#[test]
fn test_new_len_5() {
    let result = crc_store(vec![1, 2, 3, 4, 5]);
    assert!(matches!(result, Ok(_)));
}
