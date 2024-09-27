use libfuzzer_sys::arbitrary;

#[derive(arbitrary::Arbitrary, Debug)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}
