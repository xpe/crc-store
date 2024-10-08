//! RWSV means "read, write, seek, validate"

#![no_main]

use std::io::{self, Cursor, Read, Seek, Write};

use crc_store::{Config, CrcStore, ValidateError};
use io::Error as IoError;
use io::ErrorKind::InvalidInput;
use libfuzzer_sys::{arbitrary, fuzz_target};

const MAX_READ_BUF_LEN: u32 = 4194304; // 4 MB

/// Maximum seek is arbitrarily set to 1 megabyte (1000 ^ 2).
const MAX_SEEK: i64 = 1_000_000;

#[derive(arbitrary::Arbitrary, Debug)]
pub struct Setup {
    pub seg_len: u32,
    pub buf_len: u32,
    pub validate_on_read: bool,
    pub initial_bytes: Vec<u8>,
    pub methods: Vec<Method>,
}

#[derive(arbitrary::Arbitrary, Debug)]
pub enum Method {
    Read { buf_len: u32 },
    Write { buf: Vec<u8> },
    Seek { seek_from: SeekFrom },
    Validate,
}

#[derive(arbitrary::Arbitrary, Debug)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

fuzz_target!(|setup: Setup| {
    let _ = execute_setup(setup);
});

/// Execute the given `Sequence`.
fn execute_setup(setup: Setup) -> Result<(), crc_store::Error> {
    let inner = Cursor::new(setup.initial_bytes);
    let config = Config {
        seg_len: setup.seg_len,
        buf_len: setup.buf_len,
        validate_on_read: setup.validate_on_read,
    };
    let mut store = CrcStore::new(config, inner)?;
    for method in setup.methods {
        call_method(&mut store, method)?;
    }
    Ok(())
}

/// Call the given `Method`.
fn call_method(store: &mut CrcStore<Cursor<Vec<u8>>>, method: Method) -> io::Result<()> {
    match method {
        Method::Read { buf_len } => {
            if buf_len > MAX_READ_BUF_LEN {
                return Err(IoError::new(InvalidInput, "read buffer length too large"));
            }
            let mut buf = vec![0u8; buf_len as usize];
            store.read(&mut buf)?;
            Ok(())
        }
        Method::Write { mut buf } => {
            store.write(&mut buf)?;
            Ok(())
        }
        Method::Seek { seek_from } => {
            let arg = match seek_from {
                SeekFrom::Start(n) => {
                    if n >= MAX_SEEK as u64 {
                        return Err(IoError::new(InvalidInput, "seek too large"));
                    }
                    io::SeekFrom::Start(n)
                }
                SeekFrom::Current(n) => {
                    if n >= MAX_SEEK || n <= -MAX_SEEK {
                        return Err(IoError::new(InvalidInput, "seek too large"));
                    }
                    io::SeekFrom::Current(n)
                }
                SeekFrom::End(n) => {
                    if n >= MAX_SEEK || n <= -MAX_SEEK {
                        return Err(IoError::new(InvalidInput, "seek too large"));
                    }
                    io::SeekFrom::End(n)
                }
            };
            store.seek(arg)?;
            Ok(())
        }
        Method::Validate => match store.validate() {
            Err(ValidateError::Io(e)) => Err(e),
            Err(_) => Err(IoError::new(InvalidInput, "validation error")),
            Ok(_) => Ok(()),
        },
    }
}
