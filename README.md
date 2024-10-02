# `crc-store`

`crc-store` is a Rust crate that adds and verifies CRC32 checksums over an arbitary I/O object (anything that implements `Read + Write + Seek`).

## Usage Example

```rust
// TODO
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
