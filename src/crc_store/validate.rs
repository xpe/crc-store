use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};

use super::CrcStore;
use crate::{crc32_from_be_bytes, ValidateError};

impl<I: Read + Write + Seek> CrcStore<I> {
    /// Validates all checksums. Returns `Ok` if all are valid or an error with
    /// indices of invalid segments. Leaves inner I/O at end.
    pub fn validate(&mut self) -> Result<(), ValidateError> {
        self.inner.seek(SeekFrom::Start(0))?;
        let mut seg_idx: u64 = 0; // segment index
        let mut invalid: Vec<u64> = vec![]; // invalid segment indices
        let n = self.seg_len as usize;
        let mut buf = vec![0; n];
        'outer: loop {
            let mut total = 0; // total bytes read
            while total < n {
                match self.inner.read(&mut buf[total .. n]) {
                    Ok(0) => break 'outer, // EOF
                    Ok(j) if j < 5 => return Err(ValidateError::SegTooShort(seg_idx)),
                    Ok(j) => total += j,
                    Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(ValidateError::Io(e)),
                }
            }
            let header_crc = u32::from_be_bytes(buf[.. 4].try_into().unwrap());
            let crc_of_body = crc32_from_be_bytes(&buf[4 .. total]);
            if header_crc != crc_of_body {
                invalid.push(seg_idx);
            }
            seg_idx += 1;
        }
        if invalid.is_empty() {
            Ok(())
        } else {
            Err(ValidateError::Checksum(invalid))
        }
    }
}
