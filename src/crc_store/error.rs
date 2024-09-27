use std::io;

#[derive(Debug)]
pub enum Error {
    SegmentTooSmall,
    SegmentTooLarge,
    Io(io::Error),
}

#[derive(Debug)]
pub enum ValidateError {
    Checksum(Vec<u64>),
    SegTooShort(u64),
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<io::Error> for ValidateError {
    fn from(err: io::Error) -> Self {
        ValidateError::Io(err)
    }
}
