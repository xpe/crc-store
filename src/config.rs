use crate::ConfigError::{self, Buf, Seg};
use crate::LenError::{NotPow2, TooLarge, TooSmall};

/// Minimum segment length, inclusive
pub const MIN_SEG_LEN: u32 = 8;

/// Maximum segment length, inclusive
pub const MAX_SEG_LEN: u32 = 65536;

/// Minimum segment length, inclusive
pub const MIN_BUF_LEN: u32 = 8;

/// Maximum segment length, inclusive
pub const MAX_BUF_LEN: u32 = 65536;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// segment length (for I/O object; e.g. disk)
    pub seg_len: u32,

    /// buffer length (for R/W)
    pub buf_len: u32,

    /// validate checksums on read?
    pub validate_on_read: bool,
}

impl Config {
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.seg_len < MIN_SEG_LEN {
            Err(Seg(TooSmall))
        } else if self.seg_len > MAX_SEG_LEN {
            Err(Seg(TooLarge))
        } else if !self.seg_len.is_power_of_two() {
            Err(Seg(NotPow2))
        } else if self.buf_len < MIN_BUF_LEN {
            Err(Buf(TooSmall))
        } else if self.buf_len > MAX_BUF_LEN {
            Err(Buf(TooLarge))
        } else if !self.buf_len.is_power_of_two() {
            Err(Buf(NotPow2))
        } else {
            if self.validate_on_read {
                todo!();
            }
            Ok(())
        }
    }
}
