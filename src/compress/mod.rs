use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
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
    let data = encoder.finish()?;
    Ok(data)
}