use std::io::{self, Read, Seek, SeekFrom, Write};

use crate::{min3, CrcStore};

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
            let body_remain = (b - (self.inner_pos % s)) as usize;
            let to_last_checksum = (self.inner_len - 4 - self.inner_pos) as usize;
            let n = min3(buf_remain, body_remain, to_last_checksum);
            let bytes_read = self.read_buf(&mut buf[i .. i + n])?;
            if bytes_read == 0 {
                break;
            }
            i += bytes_read;
            if self.cfg.validate_on_read {
                todo!();
            }
            if self.inner_pos % s == b {
                self.inner_pos = self.inner.seek(SeekFrom::Current(4))?;
            }
        }
        Ok(i)
    }
}
