use std::io::{self, Read, Seek, SeekFrom, Write};

use super::CrcStore;

impl<I: Read + Write + Seek> Seek for CrcStore<I> {
    /// Seek according to given outer position.
    fn seek(&mut self, outer_pos: SeekFrom) -> io::Result<u64> {
        let seek_from: SeekFrom = match outer_pos {
            SeekFrom::Start(outer_n) => {
                let inner_n = self.inner_pos(outer_n);
                self.inner_pos = inner_n;
                SeekFrom::Start(inner_n)
            }
            SeekFrom::Current(outer_n) => {
                let inner_n = self.rel_inner_pos(outer_n, self.inner_pos);
                self.inner_pos = (self.inner_pos as i64 + inner_n) as u64;
                SeekFrom::Current(inner_n)
            }
            SeekFrom::End(outer_n) => {
                let inner_n = self.rel_inner_pos(outer_n, self.inner_len);
                self.inner_pos = (self.inner_pos as i64 + inner_n) as u64;
                SeekFrom::End(inner_n)
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
    pub fn inner_pos(&self, outer_pos: u64) -> u64 {
        let b = self.body_len() as u64;
        let segment = outer_pos / b;
        let offset = outer_pos % b;
        (segment * self.seg_len as u64) + offset + 4
    }

    /// Returns the relative inner position for a given relative outer
    /// position and absolute inner position.
    ///
    /// Precondition: `inner_pos` points to a body byte.
    ///
    /// ## Example
    ///
    /// Here is an example with seg_len = 7:
    ///
    /// seg 0  seg 1  seg 2  seg 3
    /// |      |      |      |
    /// ccccBBBccccBBBccccBBBccccB...
    ///             |
    ///        inner_pos = 12
    ///
    /// - "cccc" is the 4-byte checksum
    /// - "BBB" is the 3-byte body
    ///
    /// | rel_outer_pos | rel_inner_pos() |
    /// | ------------- | --------------- |
    /// |            -3 |              -9 |
    /// |            -2 |              -2 |
    /// |            -1 |              -1 |
    /// |             0 |               0 |
    /// |             1 |               1 |
    /// |             2 |               6 |
    /// |             3 |               7 |
    pub fn rel_inner_pos(&self, rel_outer_pos: i64, inner_pos: u64) -> i64 {
        let b = self.body_len() as i64;
        let s = self.seg_len as u64;
        let offset = (inner_pos % s) as i64;
        assert!(offset >= 4);
        let body_offset = offset - 4;
        let shift = if rel_outer_pos >= 0 {
            body_offset
        } else {
            body_offset - b + 1 // == -((b-1)-body_offset)
        };
        let checksums_to_skip = (rel_outer_pos + shift) / b;
        rel_outer_pos + checksums_to_skip * 4
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
