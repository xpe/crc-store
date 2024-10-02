use std::cmp::min;
use std::io::{self, Read, Seek, SeekFrom, Write};

use super::CrcStore;

impl<I: Read + Write + Seek> Read for CrcStore<I> {
    /// Reads from the `CrcStore`.
    ///
    /// Precondition: the `inner` position points to a body byte
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let b = self.body_len() as u64;
        let s = self.cfg.seg_len as u64;
        assert!(self.inner_pos % s < b);
        let mut i = 0;
        while i < buf.len() {
            // each iteration reads as much of a segment as it can
            let buf_remain = buf.len() - i;
            let body_remain = b - (self.inner_pos % s);
            let n = min(buf_remain, body_remain as usize);
            let bytes_read = self.read_buf(&mut buf[i .. i + n])?;
            assert!(bytes_read > 0);
            i += bytes_read;
            self.inner.seek(SeekFrom::Current(4))?;
            self.inner_pos += 4;
        }
        Ok(i)
    }
}
