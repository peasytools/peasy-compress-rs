use flate2::write::{GzEncoder, GzDecoder, DeflateEncoder, DeflateDecoder};
use flate2::Compression;
use std::io::Write;

/// Gzip compress data.
pub fn gzip(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut enc = GzEncoder::new(Vec::new(), Compression::default());
    enc.write_all(data)?;
    enc.finish()
}

/// Gzip decompress data.
pub fn gunzip(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut dec = GzDecoder::new(Vec::new());
    dec.write_all(data)?;
    dec.finish()
}

/// Deflate compress data.
pub fn deflate(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut enc = DeflateEncoder::new(Vec::new(), Compression::default());
    enc.write_all(data)?;
    enc.finish()
}

/// Inflate (decompress) deflated data.
pub fn inflate(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut dec = DeflateDecoder::new(Vec::new());
    dec.write_all(data)?;
    dec.finish()
}

/// Calculate compression ratio as percentage.
pub fn compress_ratio(original_size: usize, compressed_size: usize) -> f64 {
    if original_size == 0 { return 0.0; }
    ((1.0 - compressed_size as f64 / original_size as f64) * 100.0 * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gzip_roundtrip() {
        let data = b"Hello, Peasy Compress!";
        let compressed = gzip(data).unwrap();
        let decompressed = gunzip(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_deflate_roundtrip() {
        let data = b"Hello, deflate!";
        let compressed = deflate(data).unwrap();
        let decompressed = inflate(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_compress_ratio() {
        assert_eq!(compress_ratio(100, 30), 70.0);
        assert_eq!(compress_ratio(0, 0), 0.0);
    }
}
