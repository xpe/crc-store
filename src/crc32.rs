use crc32fast::Hasher;

pub fn crc32_from_be_bytes(buf: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(buf);
    hasher.finalize()
}

pub fn crc32_to_be_bytes(buf: &[u8]) -> [u8; 4] {
    crc32_from_be_bytes(buf).to_be_bytes()
}
