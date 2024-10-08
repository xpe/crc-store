use std::io::Error as IoError;
use std::io::ErrorKind::InvalidInput;
use std::io::Result as IoResult;
use std::io::{Read, Seek, SeekFrom, Write};

use super::CrcStore;

/// Maximum seek is arbitrarily set to 1 exabyte (1000 ^ 6).
const MAX_SEEK: i64 = 1_000_000_000_000_000_000;

impl<I: Read + Write + Seek> Seek for CrcStore<I> {
    /// Seek according to given outer position.
    fn seek(&mut self, outer_pos: SeekFrom) -> IoResult<u64> {
        let inner_pos: SeekFrom = match outer_pos {
            SeekFrom::Start(outer_n) => {
                if outer_n >= MAX_SEEK as u64 {
                    return Err(IoError::new(InvalidInput, "exceeded MAX_SEEK_FROM_START"));
                }
                let inner_n: u64 = self
                    .start_pos(outer_n)
                    .ok_or_else(|| IoError::new(InvalidInput, "checked arithmetic"))?;
                SeekFrom::Start(inner_n)
            }
            SeekFrom::Current(outer_n) => {
                if outer_n >= MAX_SEEK || outer_n <= -MAX_SEEK {
                    return Err(IoError::new(InvalidInput, "exceeded MAX_SEEK"));
                }
                let inner_n: i64 = self
                    .current_pos(outer_n)
                    .ok_or_else(|| IoError::new(InvalidInput, "checked arithmetic"))?;
                SeekFrom::Current(inner_n)
            }
            SeekFrom::End(outer_n) => {
                if outer_n >= MAX_SEEK || outer_n <= -MAX_SEEK {
                    return Err(IoError::new(InvalidInput, "exceeded MAX_SEEK"));
                }
                let inner_n: i64 = self
                    .end_pos(outer_n)
                    .ok_or_else(|| IoError::new(InvalidInput, "checked arithmetic"))?;
                SeekFrom::End(inner_n)
            }
        };
        self.inner_pos = self.inner.seek(inner_pos)?;
        Ok(self.inner_pos)
    }
}

impl<I: Read + Write + Seek> CrcStore<I> {
    /// Used with `SeekFrom::Start`. Returns the {inner position from start} for
    /// a given {outer position from start}.
    ///
    /// TODO: write-up what a caller can expect from the returned position. As
    /// explained below, `start_pos` does not guarantee a _readable_ body
    /// position. But I think we can say it guarantees a _writeable_ body
    /// position?
    ///
    /// ## Example
    ///
    /// Here is an example with seg_len=8 and inner_len=22:
    ///
    /// ```text
    /// seg 0   seg 1   seg 2
    /// |       |       |
    /// BBBBccccBBBBccccBBcccc
    /// ^
    ///
    /// ║      outer_n      ║    start_pos()    ║
    /// ╠════╤════╤════╤════╬════╤════╤════╤════╣
    /// ║  0 │  1 │  2 │  3 ║  0 │  1 │  2 │  3 ║
    /// ║  4 │  5 │  6 │  7 ║  8 │  9 │ 10 │ 11 ║
    /// ║  8 │  9 │ 10 │ 11 ║ 16 │ 17 │ 18 │ 19 ║
    /// ```
    ///
    /// Look at `outer_n` > 9. The corresponding `start_pos()` are not readable
    /// body bytes.
    ///
    /// Lesson? `start_pos()` does not guarantee a readable body position.
    pub fn start_pos(&self, outer_n: u64) -> Option<u64> {
        // TODO: review use of `u64::from` below...
        let b = u64::from(self.body_len());
        let s = u64::from(self.cfg.seg_len);
        let segment: u64 = outer_n / b;
        let offset: u64 = outer_n % b;
        // (segment * s) + offset
        segment.checked_mul(s).and_then(|v| v.checked_add(offset))
    }

    /// Used with `SeekFrom::Current`. Returns the {inner position from current}
    /// for a given {outer position from current}.
    ///
    /// Precondition: `inner_pos` points to a body byte.
    ///
    /// ## Example
    ///
    /// Here is an example with seg_len=8 and inner_len=30:
    ///
    /// ```text
    /// seg 0   seg 1   seg 2   seg 3
    /// |       |       |       |
    /// BBBBccccBBBBccccBBBBccccBBcccc
    ///          |
    ///     inner_pos=9
    ///
    /// (where 'B' means body byte and 'c' means checksum byte)
    ///
    /// ║      outer_n      ║   current_pos()   ║
    /// ╠════╤════╤════╤════╬════╤════╤════╤════╣
    /// ║ -5 │ -4 │ -3 │ -2 ║ -9 │ -8 │ -7 │ -6 ║
    /// ║ -1 │  0 │  1 │  2 ║ -1 │  0 │  1 │  2 ║
    /// ║  3 │  3 │  4 │  5 ║  7 │  8 │  9 │ 10 ║
    /// ```
    pub fn current_pos(&self, outer_n: i64) -> Option<i64> {
        self.rel_inner_pos(outer_n, self.inner_pos)
    }

    /// Used with `SeekFrom::End`. Returns the {inner position from end} for a
    /// given {outer position from end}.
    ///
    /// ## Example
    ///
    /// Here is an example with seg_len=8 and inner_len=21:
    ///
    /// ```text
    /// seg 0   seg 1   seg 2
    /// |       |       |
    /// BBBBccccBBBBccccBcccc
    ///                  |   |
    ///         inner_n=17   inner_len=21
    ///
    /// (where 'B' means body byte and 'c' means checksum byte)
    ///
    /// ║      outer_n      ║     end_pos()     ║
    /// ╠════╤════╤════╤════╬════╤════╤════╤════╣
    /// ║ -5 │ -4 │ -3 │ -2 ║ -9 │ -8 │ -7 │ -6 ║
    /// ║ -1 │  0 │  1 │  2 ║ -1 │  0 │  1 │  2 ║
    /// ║  3 │  4 │  5 │  6 ║  7 │  8 │  9 │ 10 ║
    /// ```
    pub fn end_pos(&self, outer_n: i64) -> Option<i64> {
        if self.inner_len == 0 {
            self.rel_inner_pos(outer_n, 0)
        } else if self.inner_len > 4 {
            let m = self.rel_inner_pos(outer_n, self.inner_len - 4)?;
            Some(m - 4)
        } else {
            panic!("internal error");
        }
    }

    /// Helper function. Call from both `SeekFrom::Current` and `SeekFrom::End`.
    /// Returns an updated {inner position from current} for a given {outer
    /// position from current} and a given {inner position from start}.
    ///
    /// Precondition: `inner_pos` points to a body byte.
    ///
    /// ## Example
    ///
    /// Here is an example with seg_len=8, inner_len=22, and inner_n=17:
    ///
    /// ```text
    /// seg 0   seg 1   seg 2
    /// |       |       |
    /// BBBBccccBBBBccccBBcccc
    ///                  |
    ///         inner_n=17
    ///
    /// (where 'B' means body byte and 'c' means checksum byte)
    ///
    /// ║      outer_n      ║  rel_inner_pos()  ║
    /// ╠════╤════╤════╤════╬════╤════╤════╤════╣
    /// ║ -5 │ -4 │ -3 │ -2 ║ -9 │ -8 │ -7 │ -6 ║
    /// ║ -1 │  0 │  1 │  2 ║ -1 │  0 │  1 │  2 ║
    /// ║  3 │  4 │  5 │  6 ║  7 │  8 │  9 │ 10 ║
    /// ```
    pub(crate) fn rel_inner_pos(&self, outer_n: i64, inner_n: u64) -> Option<i64> {
        let b = self.body_len() as i64;
        let s = self.cfg.seg_len as u64;
        // Given that `s == self.cfg.seg_len` has a maximum of `MAX_SEG_LEN`, we can be
        // certain that `inner_n % s` fits into `i64`:
        let offset = (inner_n % s) as i64;
        assert!(offset <= b);
        let shift: i64 = if outer_n >= 0 {
            offset
        } else {
            -((b - 1) - offset)
        };
        let checksums_to_skip = outer_n.checked_add(shift).and_then(|v| v.checked_div(b))?;
        let skip = checksums_to_skip.checked_mul(4)?;
        outer_n.checked_add(skip)
    }
}
