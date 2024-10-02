use super::helpers as h;
use super::helpers::Cursor;
use crate::{Config, CrcStore};

#[test]
fn test_validate_len_64() {
    let len = 64;
    let mut rng = rand::thread_rng();
    for seg_len in [8, 16, 32, 64] {
        for buf_len in [8, 16, 32, 64] {
            let config = Config {
                seg_len,
                buf_len,
                validate_on_read: false,
            };
            let data = h::valid_data(&mut rng, seg_len, len);
            let cursor = Cursor::new(data);
            let mut store = CrcStore::new(config, cursor).unwrap();
            let result = store.validate();
            assert!(result.is_ok());
        }
    }
}

#[test]
fn test_validate_len_480() {
    let len = 480;
    let mut rng = rand::thread_rng();
    for seg_len in [16, 32, 64, 128, 256] {
        for buf_len in [16, 32, 64, 128, 256] {
            let config = Config {
                seg_len,
                buf_len,
                validate_on_read: false,
            };
            let data = h::valid_data(&mut rng, seg_len, len);
            let cursor = Cursor::new(data);
            let mut store = CrcStore::new(config, cursor).unwrap();
            let result = store.validate();
            assert!(result.is_ok());
        }
    }
}

#[test]
fn test_validate_len_12600() {
    let len = 12600;
    let mut rng = rand::thread_rng();
    for seg_len in [64, 128, 256, 512, 1024, 2048, 4096] {
        for buf_len in [64, 128, 256, 512, 1024, 2048, 4096] {
            let config = Config {
                seg_len,
                buf_len,
                validate_on_read: false,
            };
            let data = h::valid_data(&mut rng, seg_len, len);
            let cursor = Cursor::new(data);
            let mut store = CrcStore::new(config, cursor).unwrap();
            let result = store.validate();
            assert!(result.is_ok());
        }
    }
}
