use libfuzzer_sys::arbitrary;

use super::Method;

#[derive(arbitrary::Arbitrary, Debug)]
pub struct Sequence {
    pub seg_len: u32,
    pub initial_bytes: Vec<u8>,
    pub methods: Vec<Method>,
}
