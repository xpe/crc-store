use std::cmp::min;
use std::io::Error as IoError;
use std::io::{Read, Seek, SeekFrom, Write};

use crc32fast::Hasher;

use crate::{CrcStore, ValidateError};

impl<I: Read + Write + Seek> CrcStore<I> {
    /// Does the inner I/O object contain valid data? This happens when the
    /// checksums all match.
    pub fn validate(&mut self) -> Result<(), ValidateError> {
        self.inner_pos = self.inner.seek(SeekFrom::Start(0))?;
        if self.cfg.seg_len <= self.cfg.buf_len {
            self.validate_smaller_segments()
        } else {
            self.validate_larger_segments()
        }
    }

    /// Call this when `seg_len` <= `buf_len`. Validate by processing one buffer
    /// at a time.
    fn validate_smaller_segments(&mut self) -> Result<(), ValidateError> {
        assert!(self.cfg.seg_len <= self.cfg.buf_len);
        let mut invalid: Option<Vec<u64>> = None;
        while self.inner_pos < self.inner_len {
            match self.validate_next_segments() {
                Ok(()) => continue,
                Err(ValidateError::Checksum(mut vec)) => {
                    invalid = Some(match invalid {
                        Some(mut existing) => {
                            existing.append(&mut vec);
                            existing
                        }
                        None => vec,
                    });
                }
                Err(e) => return Err(e),
            }
        }
        match invalid {
            None => Ok(()),
            Some(vec) => Err(ValidateError::Checksum(vec)),
        }
    }

    /// Validate next segments (whatever number fit in one buffer), starting at
    /// `inner_pos`.
    ///
    /// Preconditions:
    /// - inner_pos % seg_len == 0
    /// - buf_len >= seg_len
    /// - buf_len % seg_len == 0
    ///
    /// Postcondition: Either
    /// - inner_pos % seg_len == 0
    /// - inner_pos @ EOF
    fn validate_next_segments(&mut self) -> Result<(), ValidateError> {
        assert_eq!(self.inner_pos % self.cfg.seg_len as u64, 0);
        assert!(self.cfg.buf_len >= self.cfg.seg_len);
        assert_eq!(self.cfg.buf_len % self.cfg.seg_len, 0);
        let s = self.cfg.seg_len as usize;
        let first_seg_idx = self.inner_pos / self.cfg.seg_len as u64;
        let n = self.read_up_to(self.cfg.buf_len as usize)?;
        let mut invalid: Option<Vec<u64>> = None;
        let mut i: usize = 0;
        while i < n {
            let end = min(i + s, n);
            let body = &self.buf[i .. end - 4];
            let checksum_bytes = &self.buf[end - 4 .. end];
            let read_checksum = u32::from_be_bytes(checksum_bytes.try_into().unwrap());
            let calc_checksum = crc32fast::hash(body);
            if read_checksum != calc_checksum {
                invalid
                    .get_or_insert_with(Vec::new)
                    .push(i as u64 + first_seg_idx);
            }
            i += s;
        }
        match invalid {
            None => Ok(()),
            Some(vec) => Err(ValidateError::Checksum(vec)),
        }
    }

    /// Call this when `seg_len` > `buf_len`. Validate by processing one segment
    /// at a time.
    fn validate_larger_segments(&mut self) -> Result<(), ValidateError> {
        assert!(self.cfg.seg_len > self.cfg.buf_len);
        let mut seg_index: u64 = 0;
        let mut invalid: Option<Vec<u64>> = None;
        while self.inner_pos < self.inner_len {
            if !self.is_valid_segment()? {
                invalid.get_or_insert_with(Vec::new).push(seg_index);
            }
            seg_index += 1;
        }
        match invalid {
            None => Ok(()),
            Some(vec) => Err(ValidateError::Checksum(vec)),
        }
    }

    /// Validate next segment, starting at `inner_pos`.
    ///
    /// Precondition: inner_pos % seg_len == 0
    ///
    /// Postcondition: Either
    /// - inner_pos % seg_len == 0
    /// - inner_pos @ EOF
    pub(crate) fn is_valid_segment(&mut self) -> Result<bool, IoError> {
        assert_eq!(self.inner_pos % self.cfg.seg_len as u64, 0);
        if self.inner_len == 0 {
            return Ok(true);
        }
        let mut hasher = Hasher::new();
        let read_checksum = self.process_segment(&mut hasher)?;
        let calc_checksum = hasher.finalize();
        Ok(read_checksum == calc_checksum)
    }

    /// Processes the rest of the current segment, one buffer at a time. Updates
    /// the checksum `hasher` state as it goes. Returns the checksum in the
    /// last 4 bytes.
    fn process_segment(&mut self, hasher: &mut Hasher) -> Result<u32, IoError> {
        let s = self.cfg.seg_len as u64;
        let end_of_seg = s - (self.inner_pos % s);
        let end_of_inner = self.inner_len - self.inner_pos;
        let n = min(end_of_inner, end_of_seg) as usize;
        let b = self.cfg.buf_len as usize;
        if n <= b {
            self.process_segment_end(hasher, n)
        } else {
            assert!(n > b);
            let i = self.read_up_to(b)?;
            let body = &self.buf[.. i];
            hasher.update(body);
            self.process_segment(hasher)
        }
    }

    /// Reads up to `n` bytes from the `inner` I/O object using the
    /// pre-allocated `self.buf` buffer. Updates the `hasher` accordingly.
    /// Returns the checksum from the last 4 bytes read.
    fn process_segment_end(&mut self, hasher: &mut Hasher, n: usize) -> Result<u32, IoError> {
        assert!(n <= self.cfg.buf_len as usize);
        let i = self.read_up_to(n)?;
        let body = &self.buf[.. i - 4];
        hasher.update(body);
        let last4: [u8; 4] = self.buf[i - 4 .. i].try_into().unwrap();
        let read_checksum = u32::from_be_bytes(last4);
        Ok(read_checksum)
    }
}
