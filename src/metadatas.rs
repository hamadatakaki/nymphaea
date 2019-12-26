pub mod commit_log;
pub mod objects;
pub mod util;

use crate::manage_file::read_file_str;
use objects::{Table, ObjectHashTable};

use std::path::Path;

fn latest_commit_hash() -> std::io::Result<Option<String>> {
    let path = Path::new(".nymphaea/current_branch");
    let current_branch = read_file_str(path)?;
    match commit_log::latest_commit_log(&current_branch)? {
        Some(latest_commit) => Ok(Some(latest_commit.hash)),
        None => Ok(None)
    }
}

pub fn does_repo_exist() -> bool {
    let path = Path::new(".nymphaea");
    path.exists() && path.is_dir()
}
