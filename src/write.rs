use std::cmp::{max, min};
use std::io::ErrorKind::InvalidData;
// use std::io::{self, ErrorKind, Read, Seek, SeekFrom, Write};
use std::io::{self, Read, Seek, SeekFrom, Write};

use crc32fast::Hasher;

use crate::CrcStore;

impl<I: Read + Write + Seek> Write for CrcStore<I> {
    /// Writes to the `CrcStore`. Returns the number of bytes from `buf`
    /// written (corresponding to 'outer' bytes).
    ///
    /// The precondition and postcondition is the same as a key invariant for
    /// `CrcStore`: the `inner` position points to a body byte.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut hasher = self.read_start_of_segment()?;
        self.write_with_checksums(buf, &mut hasher)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<I: Read + Write + Seek> CrcStore<I> {
    /// Read the part of the segment before the current location.
    ///
    /// Note: `inner_pos` is unchanged by this function.
    fn read_start_of_segment(&mut self) -> io::Result<Hasher> {
        let s = self.cfg.seg_len as u64;

        // Rewind to start of the segment
        let offset: u64 = self.inner_pos % s;
        self.inner.seek(SeekFrom::Current(-(offset as i64)))?;
        self.inner_pos -= offset;

        // Begin calculating the checksum
        let mut hasher = Hasher::new();
        let mut remain = offset as usize;
        while remain > 0 {
            let k = min(remain, self.cfg.buf_len as usize);
            let j = self.read_up_to(k)?;
            if j != k {
                return Err(io::Error::new(InvalidData, "internal error"));
            }
            remain -= j;
            hasher.update(&self.buf[0 .. j]);
        }
        Ok(hasher)
    }

    fn write_with_checksums(&mut self, buf: &[u8], hasher: &mut Hasher) -> io::Result<usize> {
        let b = self.body_len() as u64;
        let s = self.cfg.seg_len as u64;
        let offset: u64 = self.inner_pos % s;
        assert!(offset <= b);

        let mut i = 0;
        while i < buf.len() {
            let buf_remain = buf.len() - i;
            let body_remain = b - (self.inner_pos % s);
            let k = min(buf_remain, body_remain as usize);
            hasher.update(&buf[i .. i + k]);
            self.inner.write_all(&buf[i .. i + k])?;
            self.inner_pos += k as u64;
            self.inner_len = max(self.inner_len, self.inner_pos);
            i += k;

            self.read_end_of_body(hasher)?;

            // write checksum
            let checksum = hasher.clone().finalize();
            let checksum_bytes = checksum.to_be_bytes();
            self.inner.write_all(&checksum_bytes)?;
            self.inner_pos += 4;
            self.inner_len = max(self.inner_len, self.inner_pos);
            hasher.reset();
        }
        Ok(i)
    }

    /// Read the rest of the body in the current segment.
    fn read_end_of_body(&mut self, hasher: &mut Hasher) -> io::Result<()> {
        let b = self.body_len() as u64;
        let s = self.cfg.seg_len as u64;
        let offset: u64 = self.inner_pos % s;
        assert!(offset <= b);

        let mut remain = (b - offset) as usize;
        while remain > 0 {
            let k = min(remain, self.cfg.buf_len as usize);
            let j = self.read_up_to(k)?;
            if j != k {
                return Err(io::Error::new(InvalidData, "internal error"));
            }
            hasher.update(&self.buf[0 .. j]);
            remain -= j;
        }
        Ok(())
    }
}
