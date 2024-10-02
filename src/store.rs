use std::io::{Read, Seek, SeekFrom, Write};

use crate::{Config, Error};

/// Provides an I/O interface that adds checksums to an inner I/O object.
///
/// The inner I/O object is split into segments. Each segment consists of a
/// body then checksum. The checksum is calculated over the body.
///
/// ## Terminology
///
/// - outer position : position exposed to the user of `CrcStore`
/// - inner position : position of the inner I/O object
///
/// ## Invariants
///
/// For all functions, the following invariants must be true by the time a
/// function returns.
/// - `inner_pos` will:
///    - match the `inner` position
///    - always point to a body byte position
/// - `inner_len` will match the `inner` length
///
/// For `write()`:
/// - `inner` will have correct checksums
///
/// ## Other Notes
///
/// - `read()` only validates checksums when `cfg.validate_on_read` set.
#[derive(Debug)]
pub struct CrcStore<I: Read + Write + Seek> {
    /// config
    pub cfg: Config,

    /// body length
    pub(super) body_len: u32,

    /// buffer
    pub(super) buf: Vec<u8>,

    /// inner I/O object
    pub(super) inner: I,

    /// length of inner I/O object
    pub(super) inner_len: u64,

    /// position of inner I/O object
    pub(super) inner_pos: u64,
}

impl<I: Read + Write + Seek> CrcStore<I> {
    /// Length of a segment. Includes the checksum.
    pub fn cfg(&self) -> Config {
        self.cfg
    }

    /// Length of the body in a segment. Does not include the checksum.
    pub fn body_len(&self) -> u32 {
        self.body_len
    }

    /// Returns a new `CrcStore`. Seeks to the first segment's first body byte
    /// (even if this byte doesn't exist yet), in the inner I/O object.
    pub fn new(config: Config, mut inner: I) -> Result<Self, Error> {
        config.validate()?;
        let inner_len = inner.seek(SeekFrom::End(0))?;
        if inner_len > 0 && inner_len < 5 {
            return Err(Error::BadInnerLen);
        }
        let inner_pos = inner.seek(SeekFrom::Start(0))?;
        Ok(Self {
            cfg: config,
            body_len: config.seg_len - 4,
            buf: vec![0; config.buf_len as usize],
            inner,
            inner_len,
            inner_pos,
        })
    }

    /// Consumes this `CrcStore`, returning the wrapped I/O object.
    pub fn into_inner(self) -> I {
        self.inner
    }
}
