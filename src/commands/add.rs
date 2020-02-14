use crate::metadatas::objects::{Entry, yield_blob, yield_tree};
use crate::metadatas::util::modelize_entry;

use std::fs;
use std::path::{Path, PathBuf};

pub fn add(path: &Path) -> std::io::Result<()> {
    // start searching new objects
    let hash = rec_search(path)?;
    // renewal index
    let index_path = Path::new(".nymphaea/index");
    fs::write(index_path, &hash)?;
    Ok(())
}

fn rec_search(path: &Path) -> std::io::Result<String> {
    let mut hashes: Vec<(PathBuf, String)> = Vec::new();
    // iterate to generate hash from entry.
    for entry in fs::read_dir(path)? {
        let p = entry?.path();
        if jump_path_condition(&p) { continue; }
        let hash = match modelize_entry(p.as_path()) {
            Entry::File => yield_blob(&p)?,
            Entry::Dir  => rec_search(&p)?
        };
        hashes.push((p, hash));
    };
    yield_tree(path, hashes)
}

fn jump_path_condition(path: &Path) -> bool {
    let judged = path.ends_with(".nymphaea");
    judged
}