use flate2::write::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use std::io::{Result, Write};

// Compress data
pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder: ZlibEncoder<Vec<u8>> = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

// Decompress data
pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder: ZlibDecoder<Vec<u8>> = ZlibDecoder::new(Vec::new());
    decoder.write_all(data)?;
    decoder.finish()
}

fn main() -> Result<()> {
    let input = b"Hello, zlib!";
    println!("Original: {:?}", input);

    // Compress
    let compressed = compress(input)?;
    println!("Compressed: {:?}", compressed);

    // Decompress
    let decompressed = decompress(&compressed)?;
    println!("Decompressed: {:?}", decompressed);

    Ok(())
}
