use libfuzzer_sys::arbitrary;

use super::SeekFrom;

#[derive(arbitrary::Arbitrary, Debug)]
pub enum Method {
    Read { buf_len: u32 },
    Write { buf: Vec<u8> },
    Seek { seek_from: SeekFrom },
}
