use std::io;

#[derive(Debug)]
pub enum Error {
    Config(ConfigError),
    BadInnerLen,
    Io(io::Error),
}

#[derive(Debug)]
pub enum ConfigError {
    Seg(LenError),
    Buf(LenError),
}

#[derive(Debug)]
pub enum LenError {
    TooSmall,
    TooLarge,
    NotPow2,
}

#[derive(Debug)]
pub enum ValidateError {
    Checksum(Vec<u64>),
    SegTooShort(u64),
    Io(io::Error),
}

impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Self {
        Error::Config(err)
    }
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
