use flate2::write::{DeflateDecoder, DeflateEncoder, GzDecoder, GzEncoder, ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use std::io::{Result, Write};

// Enum for compression formats
pub enum CompressionFormat {
    Deflate,
    Gzip,
    DeflateRaw,
}

// Compress data
pub fn compress(data: &[u8], format: CompressionFormat) -> Result<Vec<u8>> {
    match format {
        CompressionFormat::Deflate => {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data)?;
            encoder.finish()
        }
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data)?;
            encoder.finish()
        }
        CompressionFormat::DeflateRaw => {
            let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data)?;
            encoder.finish()
        }
    }
}

// Decompress data
pub fn decompress(data: &[u8], format: CompressionFormat) -> Result<Vec<u8>> {
    match format {
        CompressionFormat::Deflate => {
            let mut decoder = ZlibDecoder::new(Vec::new());
            decoder.write_all(data)?;
            decoder.finish()
        }
        CompressionFormat::Gzip => {
            let mut decoder = GzDecoder::new(Vec::new());
            decoder.write_all(data)?;
            decoder.finish()
        }
        CompressionFormat::DeflateRaw => {
            let mut decoder = DeflateDecoder::new(Vec::new());
            decoder.write_all(data)?;
            decoder.finish()
        }
    }
}

fn main() -> Result<()> {
    let input = b"Hello, compression formats!";
    println!("Original: {:?}", input);

    // Compress and decompress with Zlib
    let compressed_zlib = compress(input, CompressionFormat::Deflate)?;
    println!("Compressed with Zlib: {:?}", compressed_zlib);
    let decompressed_zlib = decompress(&compressed_zlib, CompressionFormat::Deflate)?;
    println!("Decompressed with Zlib: {:?}", decompressed_zlib);

    // Compress and decompress with Gzip
    let compressed_gzip = compress(input, CompressionFormat::Gzip)?;
    println!("Compressed with Gzip: {:?}", compressed_gzip);
    let decompressed_gzip = decompress(&compressed_gzip, CompressionFormat::Gzip)?;
    println!("Decompressed with Gzip: {:?}", decompressed_gzip);

    // Compress and decompress with Deflate
    let compressed_deflate = compress(input, CompressionFormat::DeflateRaw)?;
    println!("Compressed with Deflate: {:?}", compressed_deflate);
    let decompressed_deflate = decompress(&compressed_deflate, CompressionFormat::DeflateRaw)?;
    println!("Decompressed with Deflate: {:?}", decompressed_deflate);

    Ok(())
}
