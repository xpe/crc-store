# `crc-store`

`crc-store` is a Rust crate that adds and verifies CRC32 checksums over an arbitary I/O object (anything that implements `Read + Write + Seek`).

## Usage Example

```rust
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use crc_store::CrcStore;

fn main() {
    // create a CrcStore having 16-byte segments
    let inner = Cursor::new(Vec::new());
    let mut store = CrcStore::new(12, inner).unwrap();

    // write some data
    let data = b"A demo of CrcStore";
    println!("data:\n{}", multiline_hex_dump(data, 8));
    store.write_all(data).unwrap();

    // seek to the beginning and read the data back
    store.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    store.read_to_end(&mut buf).unwrap();
    assert_eq!(buf, data);

    // validate all checksums
    store.seek(SeekFrom::Start(0)).unwrap();
    store.validate().unwrap();
    println!("\ndata integrity verified");

    // display underlying data
    let cursor = store.into_inner();
    let vec = cursor.into_inner();
    println!("\ndata with checksums:\n{}", multiline_hex_dump(&vec, 12));
}

fn multiline_hex_dump(buf: &[u8], len: usize) -> String {
    buf.chunks(len)
        .map(|chunk| hex_dump(chunk, len))
        .collect::<Vec<String>>()
        .join("\n")
}

fn hex_dump(bytes: &[u8], len: usize) -> String {
    let a = format!("{:width$}", hex_string(bytes), width = len * 2);
    let b = format!("{:width$}", ascii_string(bytes), width = len);
    format!("| {} | {} |", a, b)
}

#[allow(clippy::format_collect)]
fn hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| format!("{:02X}", b)).collect()
}

fn ascii_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
        .collect()
}
```

The above program will output:

```
data:
| 412064656D6F206F | A.demo.o |
| 662043726353746F | f.CrcSto |
| 7265             | re       |

data integrity verified

data with checksums:
| 4F516A5F412064656D6F206F | OQj_A.demo.o |
| 67F28C9B662043726353746F | g...f.CrcSto |
| 61089C5C7265             | a..\re       |
```

## Or Roll Your Own?

It might sound simple, but implementing a correct CRC-based storage system can be tricky. This library is extensively tested and (hopefully) correct.
