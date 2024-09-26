use super::super::ValidateError;
use super::helpers::{corrupt_crc_store, valid_crc_store};

#[test]
fn test_validate_seg_len_16_len_64_ok() {
    let seg_len: u32 = 16;
    let len = 4 * seg_len as usize;
    assert_eq!(len, 64);
    let mut rng = rand::thread_rng();
    let mut store = valid_crc_store(&mut rng, seg_len, len);
    let result = store.validate();
    assert!(result.is_ok());
}

// #[test]
// fn test_validate_seg_len_16_len_65_ok() {
//     let seg_len: u32 = 16;
//     let len = 1 + 4 * seg_len as usize;
//     assert_eq!(len, 65);
//     let mut rng = rand::thread_rng();
//     let mut store = valid_crc_store(&mut rng, seg_len, len);
//     let result = store.validate();
//     assert!(result.is_ok());
// }

#[test]
fn test_validate_seg_len_16_len_74_ok() {
    let seg_len: u32 = 16;
    let len = 10 + 4 * seg_len as usize;
    assert_eq!(len, 74);
    let mut rng = rand::thread_rng();
    let mut store = valid_crc_store(&mut rng, seg_len, len);
    let result = store.validate();
    assert!(result.is_ok());
}

#[test]
fn test_validate_err_seg_len_16_changes_1() {
    let seg_len: u32 = 16;
    let len = 4 * seg_len as usize;
    let changes = 1;
    let mut rng = rand::thread_rng();
    let mut store = corrupt_crc_store(&mut rng, seg_len, len, changes);
    match store.validate() {
        Err(ValidateError::Checksum(vec)) => {
            assert_eq!(vec.len(), 1);
        }
        _ => panic!(),
    }
}

#[test]
fn test_validate_err_seg_len_20_changes_3() {
    let seg_len: u32 = 16;
    let len = 5 * seg_len as usize;
    let changes = 3;
    let mut rng = rand::thread_rng();
    let mut store = corrupt_crc_store(&mut rng, seg_len, len, changes);
    assert!(matches!(store.validate(), Err(ValidateError::Checksum(vec)) if vec.len() == changes));
}
