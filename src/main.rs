use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use crc_store::{self, CrcStore};

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

fn hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| format!("{:02X}", b)).collect()
}

fn ascii_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
        .collect()
}
