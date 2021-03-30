use flate2::{bufread::GzDecoder, write::GzEncoder, Compression};
use std::io::prelude::*;

pub fn decompress(data: &[u8]) -> crate::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut decoder = GzDecoder::new(data);
    decoder.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn compress(data: &[u8]) -> crate::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish().map_err(|e| e.into())
}
