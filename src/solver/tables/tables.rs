use std::fs::File;
use std::io::Read;
use std::io::Write;

use bytes::Buf;

/// Inflates and decodes the given pattern database.
pub fn read<'a, T: Sized>(table: &[u8]) -> T
where
    T: serde::de::DeserializeOwned,
{
    // Create a snappy decoder
    let mut decoder = snap::read::FrameDecoder::new(table.reader());

    // Allocate the space for the table
    let mut encoded = Vec::with_capacity(snap::raw::decompress_len(table).unwrap());

    // Inflate the compressed table
    decoder.read_to_end(&mut encoded).unwrap();

    // Decode the encoded table
    

    bincode::deserialize(&encoded).unwrap()
}

/// Encodes, compresses and writes the given table to disk.
pub fn write<T: ?Sized>(path: &str, table: &T) -> std::io::Result<()>
where
    T: serde::Serialize,
{
    // Open the target file path
    let file = File::create(path)?;

    // Serialize the table into binary data
    let encoded = bincode::serialize(&table).unwrap();

    // Compress the binary data
    let mut compressor = snap::write::FrameEncoder::new(&file);

    // Write it out to disk
    compressor.write_all(&encoded)?;

    Ok(())
}
