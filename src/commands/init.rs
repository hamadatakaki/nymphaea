use crate::manage_file::create_file;
use crate::metadatas::does_repo_exist;

use std::fs::create_dir;
use std::path::Path;

macro_rules! create_file {
    ($path:expr, $body:expr) => {
        {
            let path = Path::new($path);
            create_file(path, $body.as_bytes())?;
        }
    };
}

pub fn init() -> std::io::Result<()> {
    if does_repo_exist() {
        println!("The repository has already exsisted.");
    } else {
        create_dir(".nymphaea")?;
        create_file!(".nymphaea/current_branch", "master");
        create_file!(".nymphaea/commit_metadatas", "");
        create_file!(".nymphaea/object_hash_table", "");
        create_dir(".nymphaea/commit_logs")?;
        create_file!(".nymphaea/commit_logs/master", "");
        create_dir(".nymphaea/objects")?;
    }
    Ok(())
}
