// formerly known as:
// `crc32_from_be_bytes`
pub fn crc32(buf: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(buf);
    hasher.finalize()
}

// pub fn crc32_to_be_bytes(buf: &[u8]) -> [u8; 4] {
//     crc32_from_be_bytes(buf).to_be_bytes()
// }
