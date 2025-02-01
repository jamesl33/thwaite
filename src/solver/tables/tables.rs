use std::fs::File;
use std::io::Write;
use std::io::Read;

use bytes::Buf;

/// read - TODO
pub fn read<'a, T: Sized>(table: &[u8]) -> T
where
    T: serde::de::DeserializeOwned,
{
    // TODO
    let mut decoder = snap::read::FrameDecoder::new(table.reader());

    // TODO
    let mut encoded = vec![];

    // TODO
    decoder.read_to_end(&mut encoded).unwrap();

    // TODO
    let decoded = bincode::deserialize(&encoded).unwrap();

    decoded
}


/// write - TODO
pub fn write<T: ?Sized>(path: &str, table: &T) -> std::io::Result<()>
where
    T: serde::Serialize,
{
    // TODO
    let file = File::create(path)?;

    // TODO
    let encoded = bincode::serialize(&table).unwrap();

    // TODO
    let mut compressor = snap::write::FrameEncoder::new(&file);

    compressor.write_all(&encoded)?;

    Ok(())
}
