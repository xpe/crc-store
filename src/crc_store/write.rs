use std::cmp;
use std::io::{self, Read, Seek, SeekFrom, Write};

use crate::{crc32_to_be_bytes, CrcStore};

impl<I: Read + Write + Seek> Write for CrcStore<I> {
    /// Writes to the `CrcStore`.
    ///
    /// Precondition: the `inner` position points to a body byte
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
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
            self.inner.write_all(&buf[i .. i + n])?;
            self.inner_pos += n as u64;
            self.inner_len = self.inner_len.max(self.inner_pos);
            self.calc_and_write_prev_checksum()?;
            i += n;
        }
        Ok(i)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<I: Read + Write + Seek> CrcStore<I> {
    /// Updates the checksum of the previous segment.
    ///
    /// Precondition: `inner_pos` must be either
    /// 1. at the start of a segment
    /// 2. at the end of the stream
    ///
    /// Postcondition: same as precondition
    fn calc_and_write_prev_checksum(&mut self) -> io::Result<()> {
        let checksum = self.calc_prev_checksum()?;
        self.write_prev_checksum(checksum)?;
        Ok(())
    }

    /// Updates the checksum of the previous segment.
    ///
    /// Precondition: `inner_pos` must be either
    /// 1. at the start of a segment
    /// 2. at the end of the stream
    ///
    /// Postcondition: same as precondition
    fn write_prev_checksum(&mut self, checksum: [u8; 4]) -> io::Result<()> {
        let s = self.seg_len as u64;
        assert!(self.inner_pos >= s);
        let rel_seek = if self.inner_pos % s == 0 {
            -(s as i64)
        } else if self.inner_pos == self.inner_len {
            -((self.inner_pos % s) as i64)
        } else {
            panic!("failed precondition");
        };
        self.inner.seek(SeekFrom::Current(rel_seek))?;
        self.inner.write_all(&checksum)?;
        self.inner.seek(SeekFrom::Current(-rel_seek - 4))?;
        Ok(())
    }

    /// Calculate the checksum of the previous segment.
    ///
    /// Precondition: `inner_pos` must be either
    /// 1. at the start of a segment
    /// 2. at the end of the stream
    ///
    /// Postcondition: `inner_pos` unchanged
    fn calc_prev_checksum(&mut self) -> io::Result<[u8; 4]> {
        let b = self.body_len() as u64;
        let s = self.seg_len as u64;
        assert!(self.inner_pos >= s);
        let rel_seek = if self.inner_pos % s == 0 {
            -(b as i64)
        } else if self.inner_pos == self.inner_len {
            -((self.inner_pos % s) as i64) + 4
        } else {
            panic!("failed precondition");
        };
        self.inner.seek(SeekFrom::Current(rel_seek))?;
        let mut body = vec![0; -rel_seek as usize];
        self.inner.read_exact(&mut body)?;
        let checksum = crc32_to_be_bytes(&body);
        Ok(checksum)
    }
}
