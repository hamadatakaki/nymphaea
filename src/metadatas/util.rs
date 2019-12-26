use std::io::{Read, Write};

use libflate::zlib::{Encoder, Decoder};
use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub fn generate_hash(source: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.input(source);
    let s = hasher.result_str();
    String::from(s)
}

pub fn encode_by_zlib(source: &[u8]) -> std::result::Result<Vec<u8>, std::io::Error> {
    let mut encoder = Encoder::new(Vec::new())?;
    encoder.write_all(source)?;
    encoder.finish().into_result()
}

pub fn decode_by_zlib(source: &[u8]) -> std::result::Result<Vec<u8>, std::io::Error> {
    let mut decoder = Decoder::new(source)?;
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn mobilize_lines(lines: Vec<String>) -> String {
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::mobilize_lines;

    #[test]
    fn test_mobilize_lines() {
        let lines = vec! {
            "hoge".to_string(),
            "fuga".to_string(),
            "giga".to_string()
        };
        let joint = mobilize_lines(lines);
        assert_eq!(String::from("hoge\nfuga\ngiga"), joint);
        println!("{}", joint);
    }

}