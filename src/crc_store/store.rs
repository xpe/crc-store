use std::io::{Read, Seek, SeekFrom, Write};

use crate::Error;

/// Minimum payload length
pub const MIN_BODY_LEN: u32 = 1;

/// Minimum segment length
const MIN_SEG_LEN: u32 = MIN_BODY_LEN + 4;

/// Provides an I/O interface that adds checksums to an inner I/O object.
///
/// The inner I/O object is split into segments. Each segment consists of a
/// checksum then a body. The checksum is calculated over the body.
///
/// ## Terminology
///
/// - outer position : position exposed to the user of `CrcStore`
/// - inner position : position of the inner I/O object
///
/// ## Invariants
///
/// The following invariants must be true by the time a function returns.
///
/// For all functions:
/// - `inner_len` will match the `inner` length
/// - `inner_pos` will match the `inner` position
///
/// For `write()`:
/// - `inner` will have correct checksums
///
/// ## Other Notes
///
/// - `read()` does _not_ validate `inner` checksums
/// - call `validate()` to validate checksums
///
/// ## Undecided
///
/// - will `inner_pos` always point to a body byte?
/// - a `read` does _not_ validate checksums
#[derive(Debug)]
pub struct CrcStore<I: Read + Write + Seek> {
    /// inner I/O object
    pub(super) inner: I,

    /// length of inner I/O object
    pub(super) inner_len: u64,

    /// position of inner I/O object
    pub(super) inner_pos: u64,

    /// segment length
    pub(super) seg_len: u32,
}

impl<I: Read + Write + Seek> CrcStore<I> {
    /// Length of a segment. Includes the checksum.
    pub fn seg_len(&self) -> u32 {
        self.seg_len
    }

    /// Length of the body in a segment. Does not include the checksum.
    pub fn body_len(&self) -> u32 {
        self.seg_len - 4
    }

    /// Returns a new `CrcStore`. Seeks to the start of the inner I/O object.
    pub fn new(seg_len: u32, mut inner: I) -> Result<Self, Error> {
        if seg_len < MIN_SEG_LEN {
            return Err(Error::SegmentTooShort);
        }
        let inner_len = inner.seek(SeekFrom::End(0))?;
        let inner_pos = inner.seek(SeekFrom::Start(0))?;
        Ok(Self {
            inner,
            inner_len,
            inner_pos,
            seg_len,
        })
    }

    /// Consumes this `CrcStore`, returning the wrapped I/O object.
    pub fn into_inner(self) -> I {
        self.inner
    }
}
