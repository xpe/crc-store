#![no_main]

use std::io::{self, Cursor, Read, Seek, Write};

use crc_store::{Config, CrcStore};
use crc_store_fuzz::{Method, SeekFrom, Setup};
use libfuzzer_sys::fuzz_target;

const MAX_READ_BUF_LEN: u32 = 4194304; // 4 MB

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
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "read buffer length too large",
                ));
            }
            let mut buf = vec![0u8; buf_len as usize];
            store.read(&mut buf)?;
        }
        Method::Write { mut buf } => {
            store.write(&mut buf)?;
        }
        Method::Seek { seek_from } => {
            let arg = match seek_from {
                SeekFrom::Start(n) => io::SeekFrom::Start(n),
                SeekFrom::End(n) => io::SeekFrom::End(n),
                SeekFrom::Current(n) => io::SeekFrom::Current(n),
            };
            store.seek(arg)?;
        }
    }
    Ok(())
}
