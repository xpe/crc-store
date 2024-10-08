use std::io::Error as IoError;
use std::io::{ErrorKind, Read, Seek, Write};

use crate::CrcStore;

pub fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T {
    v1.min(v2).min(v3)
}

impl<I: Read + Write + Seek> CrcStore<I> {
    /// Read up to `n` bytes from the `inner` I/O object using the provided
    /// buffer. Returns the number of bytes read. Updates `self.inner_pos`
    /// accordingly.
    pub(crate) fn read_buf(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        let mut i = 0; // bytes read
        let n = buf.len();
        while i < n {
            match self.inner.read(&mut buf[i .. n]) {
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
                Ok(0) => break, // EOF
                Ok(m) => i += m,
            }
        }
        self.inner_pos += i as u64;
        Ok(i)
    }

    /// Read up to `n` bytes from the `inner` I/O object using the pre-allocated
    /// `self.buf` buffer. Returns the number of bytes read. Updates
    /// `self.inner_pos` accordingly.
    pub(crate) fn read_up_to(&mut self, n: usize) -> Result<usize, IoError> {
        let mut i = 0; // bytes read
        while i < n {
            match self.inner.read(&mut self.buf[i .. n]) {
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
                Ok(0) => break, // EOF
                Ok(m) => i += m,
            }
        }
        self.inner_pos += i as u64;
        Ok(i)
    }

    /// Reads up to `n` bytes from the `inner` I/O object using the
    /// pre-allocated `self.buf` buffer. Updates the `hasher` accordingly.
    /// Returns the checksum from the last 4 bytes read.
    pub(crate) fn read_checksum(&mut self) -> Result<u32, IoError> {
        let i = self.read_up_to(4)?;
        assert_eq!(i, 4);
        let bytes: [u8; 4] = self.buf[.. 4].try_into().unwrap();
        Ok(u32::from_be_bytes(bytes))
    }
}
