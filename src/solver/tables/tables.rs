/// The zstd compression level used when writing tables; generation is an offline, one-off step, so a slow,
/// high-ratio encode is free - decompression speed at read time is unaffected by the level used to encode.
const COMPRESSION_LEVEL: i32 = 19;

/// Inflates and decodes the given pattern database.
pub fn read<T: Sized>(table: &[u8]) -> T
where
    T: serde::de::DeserializeOwned,
{
    // Inflate the compressed table. `decode_all` would grow its output `Vec` from empty, causing repeated
    // reallocation/copying for these multi-hundred-megabyte tables; reserving capacity up front based on the
    // observed ~4x compression ratio (with headroom) avoids that, since a too-small guess just costs one
    // reallocation rather than breaking correctness.
    let mut decoder = zstd::stream::Decoder::new(table).unwrap();
    let mut encoded = Vec::with_capacity(table.len() * 8);
    std::io::Read::read_to_end(&mut decoder, &mut encoded).unwrap();

    // Decode the encoded table
    bincode::deserialize(&encoded).unwrap()
}

/// Encodes, compresses and writes the given table to disk.
pub fn write<T: ?Sized>(path: &str, table: &T) -> std::io::Result<()>
where
    T: serde::Serialize,
{
    // Serialize the table into binary data
    let encoded = bincode::serialize(&table).unwrap();

    // Compress the binary data
    let compressed = zstd::stream::encode_all(&encoded[..], COMPRESSION_LEVEL)?;

    // Write it out to disk
    std::fs::write(path, compressed)?;

    Ok(())
}
