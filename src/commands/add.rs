use crate::metadatas::objects::{Entry, create_blob, generate_tree, search_latest_tree};

use std::fs;
use std::path::{Path, PathBuf};

macro_rules! modelize_entry {
    ($e: expr) => {
        if $e.is_file() {
            Entry::File
        } else {
            Entry::Dir
        }
    };
}

macro_rules! path_contains_str {
    ($p: expr, $s: expr) => {
        $p.to_str().unwrap().contains($s)
    };
}

pub fn add() -> std::io::Result<()> {
    // start searching new objects
    let current_path = Path::new(".");
    let hash = rec_search(current_path)?;
    // renewal index
    let index_path = Path::new(".nymphaea/index");
    fs::write(index_path, &hash)?;
    Ok(())
}

fn rec_search(path: &Path) -> std::io::Result<String> {
    let mut hashes: Vec<(Entry, String, PathBuf)> = Vec::new();
    for e in fs::read_dir(path)? {
        let p = e?.path();
        if path_contains_str!(p, ".nymphaea") { continue; }
        let hash = match modelize_entry!(p) {
            Entry::File => create_blob(&p)?,
            Entry::Dir  => rec_search(&p)?
        };
        hashes.push((modelize_entry!(p), hash, p));
    };
    generate_tree(path, hashes)
}
