use std::io::{Read, Write};
use std::path::Path;
use std::fs;

use crate::metadatas::util::encode_by_zlib;

pub fn create_file(path: &Path, body: &[u8]) -> std::io::Result<()> {
    let mut file = fs::File::create(path)?;
    let size = file.write(body)?;
    assert_eq!(body.len(), size);
    Ok(())
}

pub fn create_encoded(path: &Path, body: &[u8]) -> std::io::Result<()> {
    let encoded = encode_by_zlib(body)?;
    create_file(path, &encoded)
}

pub fn read_file_str(path: &Path) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

pub fn read_file_bytes(path: &Path) -> std::io::Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn read_every_line(path: &Path) -> std::io::Result<Vec<String>> {
    let buffer = read_file_bytes(path)?;
    Ok(split_lines(buffer))
}

pub fn read_last_line(path: &Path) -> std::io::Result<String> {
    let buffer = read_file_bytes(path)?;
    Ok(extract_last_line(buffer))
}

fn split_lines(buffer: Vec<u8>) -> Vec<String> {
    let lines: Vec<String> = buffer
        .split(|b| b == &10)
        .map(|s| String::from_utf8(s.to_vec()).unwrap())
        .collect();
    lines
}

fn extract_last_line(buffer: Vec<u8>) -> String {
    let last = buffer
        .split(|b| b == &10)
        .into_iter()
        .map(|s| String::from_utf8(s.to_vec()).unwrap())
        .last()
        .unwrap();
    last
}

#[cfg(test)]
mod tests {
    use super::{split_lines, extract_last_line};

    #[test]
    fn test_split_lines() {
        let bytes = vec![
            // 123 456 お腹空いた\n
            49, 50, 51, 32, 52, 53, 54, 32, 227, 129, 138, 232, 133, 185, 231, 169, 186, 227, 129, 132, 227, 129, 159, 10,
            // 234 567 :divide: 10/0
            50, 51, 52, 32, 53, 54, 55, 32, 58, 100, 105, 118, 105, 100, 101, 58, 32, 49, 48, 47, 48
        ];
        let lines = split_lines(bytes);
        let s1 = String::from("123 456 お腹空いた");
        let s2 = String::from("234 567 :divide: 10/0");
        assert_eq!(vec![s1, s2], lines);
    }

    #[test]
    fn test_extract_last_line() {
        let bytes = vec![
            // 123 456 お腹空いた\n
            49, 50, 51, 32, 52, 53, 54, 32, 227, 129, 138, 232, 133, 185, 231, 169, 186, 227, 129, 132, 227, 129, 159, 10,
            // 234 567 :divide: 10/0
            50, 51, 52, 32, 53, 54, 55, 32, 58, 100, 105, 118, 105, 100, 101, 58, 32, 49, 48, 47, 48
        ];
        let line = extract_last_line(bytes);
        let s = String::from("234 567 :divide: 10/0");
        assert_eq!(s, line);
    }

    #[test]
    fn test_extract_last_line2() {
        let bytes = vec![];
        let line = extract_last_line(bytes);
        assert!(line.is_empty());
    }
}
