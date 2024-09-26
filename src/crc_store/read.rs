use std::cmp;
use std::io::{self, Read, Seek, SeekFrom, Write};

use super::CrcStore;

impl<I: Read + Write + Seek> Read for CrcStore<I> {
    /// Reads from the `CrcStore`.
    ///
    /// Precondition: the `inner` position points to a body byte
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let b = self.body_len() as u64;
        let s = self.seg_len as u64;
        let mut i = 0;
        while i < buf.len() {
            // each iteration reads part of a segment.
            if self.inner_pos % s == 0 {
                self.inner.seek(SeekFrom::Current(4))?;
                self.inner_pos += 4;
            }
            assert!(self.inner_pos % s >= 4);
            let body_remain = (4 + b - self.inner_pos % s) as usize;
            let buf_remain = buf.len() - i;
            let n = cmp::min(body_remain, buf_remain);
            match self.inner.read(&mut buf[i .. i + n]) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    self.inner_pos += n as u64;
                    i += n;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(i)
    }
}
