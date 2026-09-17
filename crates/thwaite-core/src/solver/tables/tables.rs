/// Decodes the given pattern database.
///
/// Tables are stored uncompressed: measured on the phase two table (26MiB compressed / 113MiB raw), zstd
/// decompression cost ~150-200ms of the ~160-380ms total load time, while the compression ratio was only ~4x.
/// Since tables are decoded once per process and otherwise sit idle, that's a bad trade - dropping compression
/// cuts phase two's load time to ~10ms. Tables are generated at build time (see `../../../build.rs`) rather than
/// checked in, so the larger on-disk size no longer costs anything in git history either.
pub fn read<T: Sized>(table: &[u8]) -> T
where
    T: serde::de::DeserializeOwned,
{
    bincode::deserialize(table).unwrap()
}

/// Encodes and writes the given table to disk.
pub fn write<T: ?Sized>(path: &str, table: &T) -> std::io::Result<()>
where
    T: serde::Serialize,
{
    let encoded = bincode::serialize(&table).unwrap();
    std::fs::write(path, encoded)?;

    Ok(())
}
