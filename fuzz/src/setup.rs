use libfuzzer_sys::arbitrary;

use super::Method;

#[derive(arbitrary::Arbitrary, Debug)]
pub struct Setup {
    pub seg_len: u32,
    pub buf_len: u32,
    pub validate_on_read: bool,
    pub initial_bytes: Vec<u8>,
    pub methods: Vec<Method>,
}
