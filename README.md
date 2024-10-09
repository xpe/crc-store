# `crc-store`

`crc-store` is a Rust crate that adds and verifies CRC32 checksums over an arbitary I/O object (anything that implements `Read + Write + Seek`).

## Usage Example

```rust
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use crc_store::{Config, CrcStore};

fn main() {
    // create a CrcStore having 16-byte segments
    let config = Config {
        seg_len: 16,
        buf_len: 32,
        validate_on_read: false,
    };
    let inner = Cursor::new(Vec::new());
    let mut store = CrcStore::new(config, inner).unwrap();

    // write some data
    let data = b"A demo of CrcStore"; // 18 bytes
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
    println!("\ndata with checksums:\n{}", multiline_hex_dump(&vec, 16));
}

fn multiline_hex_dump(buf: &[u8], len: usize) -> String {
    buf.chunks(len)
        .map(|chunk| hex_dump(chunk, len))
        .collect::<Vec<String>>()
        .join("\n")
}

fn hex_dump(bytes: &[u8], len: usize) -> String {
    let a = format!("{:width$}", hex_string(bytes), width = len * 3);
    let b = format!("{:width$}", ascii_string(bytes), width = len);
    format!("| {}| {} |", a, b)
}

#[allow(clippy::format_collect)]
fn hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| format!("{:02X} ", b)).collect()
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
| 41 20 64 65 6D 6F 20 6F | A.demo.o |
| 66 20 43 72 63 53 74 6F | f.CrcSto |
| 72 65                   | re       |

data integrity verified

data with checksums:
| 41 20 64 65 6D 6F 20 6F 66 20 43 72 F1 F6 5C E3 | A.demo.of.Cr..\. |
| 63 53 74 6F 72 65 C7 16 5C 39                   | cStore..\9       |
```

## Or Roll Your Own?

It might sound simple, but implementing a correct CRC-based storage system can be tricky. This library is extensively tested and (hopefully) correct.

## Incomplete Features

The `validate_on_read` configuration option only supports `false` at this time.

## Fuzz Testing

After you install [cargo fuzz] as recommended (which involves using Nightly Rust), then you can run fuzz testing with:

```console
cargo fuzz run fuzz_rwsv
```

Note: "rwsv" stands for "read, write, seek, validate" -- the methods exercised by the fuzzer.

[cargo fuzz]: https://github.com/rust-fuzz/cargo-fuzz

## Correctness

As of 2024-10-09, fuzz testing has surfaced no new crashes after exploring more than _1.2 billion_ inputs. In this project, each input is a sequence of operations in the `CrcStore` API, including: `new`, `seek`, `read`, `write`, and `validate`.

Of course, a lack of crashes is not proof of correctness. However, when combined with other testing strategies, this is reassuring.
