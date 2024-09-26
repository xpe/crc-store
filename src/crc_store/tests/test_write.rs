use std::io::Write;

use super::helpers::{empty_crc_store, random_bytes};
use crate::crc32_to_be_bytes;

#[test]
/// Writes 10 bytes, which corresponds to 2 full segments. Here, a segment is 9
/// bytes, so the body of each is 5 bytes.
fn test_crc_store_write_seg_len_9_ok_1() {
    let seg_len: u32 = 9;
    let mut store = empty_crc_store(seg_len); // body_len=5

    let mut rng = rand::thread_rng();
    let data = random_bytes(&mut rng, 10);
    store.write_all(&data).unwrap();
    let written = store.inner.into_inner();

    // segment 0
    assert_eq!(written[0 .. 4], crc32_to_be_bytes(&data[0 .. 5]));
    assert_eq!(written[4 .. 9], data[0 .. 5]);
    // segment 1
    assert_eq!(written[9 .. 13], crc32_to_be_bytes(&data[5 .. 10]));
    assert_eq!(written[13 .. 18], data[5 .. 10]);
}

#[test]
/// Writes 8 bytes, which is 2 bytes less than what 2 full segments would
/// hold. Here, a segment is 9 bytes, so the body of each is 5 bytes.
fn test_crc_store_write_seg_len_9_ok_2() {
    let seg_len: u32 = 9;
    let mut store = empty_crc_store(seg_len); // body_len=5

    let mut rng = rand::thread_rng();
    let data = random_bytes(&mut rng, 8);
    store.write_all(&data).unwrap();
    let written = store.inner.into_inner();

    // segment 0
    assert_eq!(written[0 .. 4], crc32_to_be_bytes(&data[0 .. 5]));
    assert_eq!(written[4 .. 9], data[0 .. 5]);
    // segment 1
    assert_eq!(written[9 .. 13], crc32_to_be_bytes(&data[5 .. 8]));
    assert_eq!(written[13 .. 16], data[5 .. 8]);
}
