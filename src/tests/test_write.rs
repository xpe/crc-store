use std::io::{Seek, SeekFrom, Write};

use super::helpers as h;
use super::helpers::Cursor;
use crate::{Config, CrcStore};

fn empty_crc_store() -> CrcStore<Cursor> {
    let cfg = Config {
        seg_len: 16,
        buf_len: 16,
        validate_on_read: false,
    };
    let data = vec![];
    let cursor = Cursor::new(data);
    CrcStore::new(cfg, cursor).unwrap()
}

/// Writes 24 bytes, which corresponds to 2 full segments. (Here, a segment is
/// 16 bytes, so the body of each is 12 bytes.)
#[test]
fn test_write_24() {
    let mut store = empty_crc_store();

    let mut rng = rand::thread_rng();
    let data = h::random_bytes(&mut rng, 24);
    store.write_all(&data).unwrap();
    let inner = store.inner.into_inner();

    // segment 0
    let body = &data[0 .. 12];
    assert_eq!(inner[0 .. 12], *body);
    let cs_bytes = crc32fast::hash(body).to_be_bytes();
    assert_eq!(inner[12 .. 16], cs_bytes);

    // segment 1
    let body = &data[12 .. 24];
    assert_eq!(inner[16 .. 28], *body);
    let cs_bytes = crc32fast::hash(body).to_be_bytes();
    assert_eq!(inner[28 .. 32], cs_bytes);
}

/// Writes 24 bytes first. Then 4 bytes in the first segment. (Here, a segment
/// is 16 bytes, so the body of each is 12 bytes.)
#[test]
#[rustfmt::skip]
fn test_write_24_seek_start_4_write_4() {
    let mut store = empty_crc_store();

    let mut rng = rand::thread_rng();
    let data_0 = h::random_bytes(&mut rng, 24);
    store.write_all(&data_0).unwrap();

    store.seek(SeekFrom::Start(4)).unwrap();

    let data_1 = h::random_bytes(&mut rng, 4);
    store.write_all(&data_1).unwrap();
    let written = store.inner.into_inner();

    assert_eq!(written[0 ..  4], data_0[0 ..  4]);
    assert_eq!(written[4 ..  8], data_1[0 ..  4]);
    assert_eq!(written[8 .. 12], data_0[8 .. 12]);
    let cs_bytes = crc32fast::hash(&written[0 .. 12]).to_be_bytes();
    assert_eq!(written[12 .. 16], cs_bytes);
}

/// Writes 18 bytes, which corresponds to one full segment (12 bytes) followed
/// by a partial segment of 6 bytes.
#[test]
fn test_write_18() {
    let mut store = empty_crc_store();

    let mut rng = rand::thread_rng();
    let data = h::random_bytes(&mut rng, 18);
    store.write_all(&data).unwrap();
    let inner = store.inner.into_inner();

    // segment 0
    let body = &data[0 .. 12];
    assert_eq!(inner[0 .. 12], *body);
    let cs_bytes = crc32fast::hash(body).to_be_bytes();
    assert_eq!(inner[12 .. 16], cs_bytes);

    // segment 1
    let body = &data[12 .. 18];
    assert_eq!(inner[16 .. 22], *body);
    let cs_bytes = crc32fast::hash(body).to_be_bytes();
    assert_eq!(inner[22 .. 26], cs_bytes);
}
