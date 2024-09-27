use std::cmp;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom, Write};

use crate::{crc32_to_be_bytes, CrcStore};

impl<I: Read + Write + Seek> Write for CrcStore<I> {
    /// Writes to the `CrcStore`.
    ///
    /// The precondition and postcondition is the same as a key invariant for
    /// `CrcStore`: the `inner` position points to a body byte.
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
            self.update_prev_checksum()?;
            i += n;
        }
        Ok(i)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<I: Read + Write + Seek> CrcStore<I> {
    /// Updates the "nearest previous" checksum. (The phrase "nearest previous"
    /// is close but not perfect; see cases below.)
    ///
    /// ## Cases
    ///
    /// There are three cases, two of which are valid. They are illustrated
    /// below using an example where seg_len=10 (so body_len=6) where 'c' means
    /// checksum byte and 'B' means body byte:
    ///
    /// ```text
    /// seg 0     seg 1     seg 2     seg 3
    /// |         |         |         |
    /// ccccBBBBBBccccBBBBBBccccBBBBBBccccBBBB...
    /// ```
    ///
    /// The cases below use ^^^^ to show which checksum will be updated.
    ///
    /// ### Case I
    ///
    /// When `inner_pos % seg_len == 0`:
    ///
    /// ```text
    ///                inner_pos=20
    ///                     |
    /// ccccBBBBBBccccBBBBBBccccBBBBBBccccBBBB...
    ///           ^^^^
    /// ```
    ///
    /// ### Case II
    ///
    /// When `inner_pos % seg_len >= 5`:
    ///
    /// ```text
    ///           inner_pos=15
    ///                |
    /// ccccBBBBBBccccBBBBBBccccBBBBBBccccBBBB...
    ///           ^^^^
    /// ```
    ///
    /// ### Case III (fails precondition)
    ///
    /// When `inner_pos % seg_len < 5`:
    ///
    /// ```text
    ///          inner_pos=14
    ///               |
    /// ccccBBBBBBccccBBBBBBccccBBBBBBccccBBBB...
    ///           ^^^^
    /// ```
    ///
    /// ## Precondition
    ///
    /// Either:
    /// - `inner_pos % seg_len == 0 && inner_pos >= seg_len`; or
    /// - `inner_pos % seg_len >= 5`; or
    ///
    /// ## Postcondition
    ///
    /// `inner_pos` unchanged
    fn update_prev_checksum(&mut self) -> io::Result<()> {
        let pos = self.locate_prev_checksum();
        let checksum = self.calc_checksum(pos)?;
        self.write_checksum(checksum)?;
        Ok(())
    }

    /// Locate the relative position of the "nearest previous" checksum
    /// (to be clear, the start of the checksum).
    fn locate_prev_checksum(&self) -> i64 {
        let s = self.seg_len as u64;
        let offset = self.inner_pos % s;
        if offset == 0 && self.inner_pos >= s {
            -(self.seg_len as i64)
        } else if offset >= 5 {
            -(offset as i64)
        } else {
            panic!("failed precondition");
        }
    }

    /// Calculate the checksum, leaving `inner_pos` where this checksum could
    /// be written.
    ///
    /// ## Precondition
    ///
    /// `(inner_pos + pos) % seg_len == 0`
    ///
    /// ## Postconditions
    ///
    /// `inner_pos  % seg_len == 0`
    fn calc_checksum(&mut self, pos: i64) -> io::Result<[u8; 4]> {
        let i = i64::try_from(self.inner_pos)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        // precondition
        assert!((i + pos) % (self.seg_len as i64) == 0);

        // seek to the body start
        self.inner.seek(SeekFrom::Current(pos + 4))?;

        // read the body
        let n = self.body_len() as usize;
        let mut body = vec![0; n];
        let mut total = 0; // total bytes read
        while total < n {
            match self.inner.read(&mut body[total .. n]) {
                Ok(0) => break, // EOF
                Ok(j) => total += j,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }

        // seek back to the start of the segment
        self.inner.seek(SeekFrom::Current(pos))?;
        self.inner_pos = (self.inner_pos as i64 + pos) as u64;

        // postcondition
        assert_eq!(self.inner_pos % self.seg_len as u64, 0);
        Ok(crc32_to_be_bytes(&body[.. total]))
    }

    /// Writes a given checksum to the current location. Then advance to the
    /// start of the next segment, unless EOF.
    fn write_checksum(&mut self, checksum: [u8; 4]) -> io::Result<()> {
        assert_eq!(self.inner_pos % self.seg_len as u64, 0);
        self.inner.write_all(&checksum)?;
        self.inner_pos += 4;

        let to_end = self.inner_len - self.inner_pos;
        let rel_seek = cmp::min(self.seg_len as u64, to_end) as i64;
        self.inner.seek(SeekFrom::Current(rel_seek))?;
        self.inner_pos = (self.inner_pos as i64 + rel_seek) as u64;
        Ok(())
    }
}
