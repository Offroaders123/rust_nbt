use flate2::write::{
    DeflateDecoder, DeflateEncoder, GzDecoder, GzEncoder, ZlibDecoder, ZlibEncoder,
};
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

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_and_decompression() {
        let input: &[u8; 37] = b"Hello, compression and decompression!";

        // Test Deflate
        let compressed_deflate: Vec<u8> =
            compress(input, CompressionFormat::Deflate).expect("Compression failed");
        let decompressed_deflate: Vec<u8> =
            decompress(&compressed_deflate, CompressionFormat::Deflate)
                .expect("Decompression failed");
        assert_eq!(
            input,
            &decompressed_deflate[..],
            "Deflate compression/decompression mismatch"
        );

        // Test Gzip
        let compressed_gzip: Vec<u8> =
            compress(input, CompressionFormat::Gzip).expect("Compression failed");
        let decompressed_gzip: Vec<u8> =
            decompress(&compressed_gzip, CompressionFormat::Gzip).expect("Decompression failed");
        assert_eq!(
            input,
            &decompressed_gzip[..],
            "Gzip compression/decompression mismatch"
        );

        // Test DeflateRaw
        let compressed_deflate_raw: Vec<u8> =
            compress(input, CompressionFormat::DeflateRaw).expect("Compression failed");
        let decompressed_deflate_raw: Vec<u8> =
            decompress(&compressed_deflate_raw, CompressionFormat::DeflateRaw)
                .expect("Decompression failed");
        assert_eq!(
            input,
            &decompressed_deflate_raw[..],
            "DeflateRaw compression/decompression mismatch"
        );
    }
}
