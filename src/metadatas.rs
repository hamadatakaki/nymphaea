pub mod commit_log;
pub mod objects;
pub mod util;

use crate::manage_file::{read_file_all, read_file_all_as_bytes};
use objects::{Table, ObjectHashTable};


use std::path::Path;

fn latest_commit_hash() -> std::io::Result<Option<String>> {
    let path = Path::new(".nymphaea/current_branch");
    let current_branch = read_file_all(path)?;
    match commit_log::latest_commit_log(&current_branch)? {
        Some(latest_commit) => Ok(Some(latest_commit.hash)),
        None => Ok(None)
    }
}

pub fn particular_object_hash(commit_hash: &str, path: &Path) -> std::io::Result<Option<String>> {
    let ohtable = ObjectHashTable::generate()?;
    match ohtable.search_record(commit_hash, path) {
        Some(record) => Ok(Some(record.object_hash)),
        None => Ok(None)
    }
}

pub fn does_repo_exist() -> bool {
    let path = Path::new(".nymphaea");
    path.exists() && path.is_dir()
}