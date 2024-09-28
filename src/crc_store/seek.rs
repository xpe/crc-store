use std::io::{self, Read, Seek, SeekFrom, Write};
use std::num::TryFromIntError;

use super::CrcStore;

impl<I: Read + Write + Seek> Seek for CrcStore<I> {
    /// Seek according to given outer position.
    fn seek(&mut self, outer_pos: SeekFrom) -> io::Result<u64> {
        let seek_from: SeekFrom = match outer_pos {
            SeekFrom::Start(outer_n) => {
                let inner_n = self
                    .inner_pos(outer_n)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "overflow"))?;
                self.inner_pos = inner_n;
                SeekFrom::Start(inner_n)
            }
            SeekFrom::Current(outer_n) => {
                let inner_n = self
                    .rel_inner_pos(outer_n, self.inner_pos)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
                self.inner_pos = (self.inner_pos as i64 + inner_n) as u64;
                SeekFrom::Current(inner_n)
            }
            SeekFrom::End(outer_n) => {
                if self.inner_len == 0 {
                    SeekFrom::End(4)
                } else {
                    let inner_n = self
                        .rel_inner_pos(outer_n, self.inner_len)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
                    self.inner_pos = (self.inner_pos as i64 + inner_n) as u64;
                    SeekFrom::End(inner_n)
                }
            }
        };
        self.inner.seek(seek_from)
    }
}

impl<I: Read + Write + Seek> CrcStore<I> {
    /// Returns the absolute inner position for a given absolute outer position.
    ///
    /// Note: this can be thought of a specialized version of `rel_inner_pos()`
    /// where `inner_pos` is an integer multiple of the segment length.
    pub fn inner_pos(&self, outer_pos: u64) -> Option<u64> {
        let b = self.body_len() as u64;
        let segment = outer_pos / b;
        let offset = outer_pos % b;
        segment
            .checked_mul(self.seg_len as u64)
            .and_then(|v| v.checked_add(offset))
            .and_then(|v| v.checked_add(4))
    }

    /// Returns the relative inner position for a given relative outer
    /// position and absolute inner position.
    ///
    /// Precondition: `inner_pos` points to a body byte.
    ///
    /// ## Example
    ///
    /// Here is an example with seg_len=7 where 'c' means checksum byte and
    /// 'B' means body byte:
    ///
    /// ```text
    /// seg 0  seg 1  seg 2  seg 3
    /// |      |      |      |
    /// ccccBBBccccBBBccccBBBccccB...
    ///             |
    ///        inner_pos=12
    ///
    /// | rel_outer_pos | rel_inner_pos() |
    /// |---------------|-----------------|
    /// |            -4 |              -8 |
    /// |            -3 |              -7 |
    /// |            -2 |              -6 |
    /// |            -1 |              -1 |
    /// |             0 |               0 |
    /// |             1 |               1 |
    /// |             2 |               6 |
    /// |             3 |               7 |
    /// |             4 |               8 |
    /// ```
    pub fn rel_inner_pos(
        &self,
        rel_outer_pos: i64,
        inner_pos: u64,
    ) -> Result<i64, TryFromIntError> {
        let b = i64::from(self.body_len());
        let s = u64::from(self.seg_len);
        let offset = i64::try_from(inner_pos % s)?;
        assert!(offset >= 4);
        let body_offset = offset - 4;
        let shift = if rel_outer_pos >= 0 {
            body_offset
        } else {
            body_offset - b + 1 // == -((b-1)-body_offset)
        };
        let checksums_to_skip = (rel_outer_pos + shift) / b;
        Ok(rel_outer_pos + checksums_to_skip * 4)
    }

    /// Returns the absolute outer position for a given absolute inner position.
    /// Some inner positions correspond to checksum bytes; these are not valid
    /// outer positions so they return `None`.
    pub fn outer_pos(&self, inner_pos: u64) -> Option<u64> {
        let b = self.body_len() as u64;
        let s = self.seg_len as u64;
        let segment = inner_pos / s;
        let offset = inner_pos % s;
        if offset < 4 {
            None
        } else {
            Some((segment * b) + offset - 4)
        }
    }
}
