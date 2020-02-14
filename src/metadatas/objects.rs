use crate::manage_file::{create_encoded, read_file_bytes, read_every_line};
use crate::metadatas::latest_commit_hash;
use crate::metadatas::util::{decode_by_zlib, generate_hash, modelize_entry};

use std::path::{Path, PathBuf};

pub enum Entry {
    File,
    Dir,
}

pub struct ObjectHashRecord {
    pub object_hash: String,
    commit_hash: String,
    path: PathBuf
}

impl ObjectHashRecord {
    pub fn new(object_hash: &str, commit_hash: &str, path: PathBuf) -> Self {
        Self {
            object_hash: object_hash.to_string(),
            commit_hash: commit_hash.to_string(),
            path: path
        }
    }

    fn from_line(line: &str) -> Self {
        let mut splited: Vec<&str> = line.splitn(3, ' ').collect();
        let path = match splited.pop() {
            Some(p) => PathBuf::from(p),
            None => panic!("bad object hash table\n")
        };
        let commit_hash = match splited.pop() {
            Some(ch) => ch,
            None => panic!("bad object hash table\n")
        };
        let object_hash = match splited.pop() {
            Some(oh) => oh,
            None => panic!("bad object hash table\n")
        };
        Self::new(object_hash, commit_hash, path)
    }
}

pub type ObjectHashTable = Vec<ObjectHashRecord>;

pub trait Table {
    fn generate() -> std::io::Result<ObjectHashTable>;
    fn search_record(self, commit_hash: &str, path: &Path) -> Option<ObjectHashRecord>;
}

impl Table for ObjectHashTable {
    fn generate() -> std::io::Result<ObjectHashTable> {
        let path = Path::new(".nymphaea/object_hash_table");
        let lines = read_every_line(path)?;
        let table: ObjectHashTable = lines.iter().map(|s| ObjectHashRecord::from_line(s)).collect();
        Ok(table)
    }

    fn search_record(self, commit_hash: &str, path: &Path) -> Option<ObjectHashRecord> {
        self.into_iter().find(|r| r.commit_hash == commit_hash && r.path.as_path() == path)
    }
}

pub fn yield_blob(path: &Path) -> std::io::Result<String> {
    if let Some(latest_hash) = latest_commit_hash()? {
        let table = ObjectHashTable::generate()?;
        if let Some(record) = table.search_record(&latest_hash, path) {
            let object_hash = record.object_hash;
            let decoded = get_object_data(&object_hash)?;
            let latest_file = read_file_bytes(path)?;
            if decoded == latest_file {
                Ok(object_hash)
            } else {
                generate_blob(path)
            }
        } else {
            generate_blob(path)
        }
    } else {
        generate_blob(path)
    }
}

fn generate_blob(path: &Path) -> std::io::Result<String> {
    let src = read_file_bytes(path)?;
    let hash = generate_hash(&src);
    // write object
    let blob_path = format!(".nymphaea/objects/{}", hash);
    let blob_path = Path::new(&blob_path);
    create_encoded(blob_path, &src)?;
    println!("DEBUG in BLOB    : path=>{:?}, hash=>{}", path, hash);  // DEBUG: To check generated hash.
    // return hash
    Ok(hash)
}

pub fn get_object_data(hash: &str) -> std::io::Result<Vec<u8>> {
    let path = format!(".nymphaea/objects/{}", hash);
    let path = Path::new(&path);
    match read_file_bytes(path) {
        Ok(buffer) => decode_by_zlib(&buffer),
        Err(e) => panic!("hash not found: {}", e)
    }
}

pub fn yield_tree(path: &Path, hashes: Vec<(PathBuf, String)>) -> std::io::Result<String> {
    let mut lines = Vec::new();
    let head = format!("{}", path.to_str().unwrap());
    lines.push(head);
    for (path, hash) in hashes {
        println!("DEBUG @yield_tree: path=>{:?}, hash=>{}", path, hash);  // DEBUG: To check tree's row.
        let file_type = match modelize_entry(path.as_path()) {
            Entry::Dir => "tree".to_string(),
            Entry::File => "blob".to_string()
        };
        let path = path.to_str().unwrap();
        let line = format!("{} {} {}", file_type, hash, path);
        lines.push(line);
    };
    let text: String = lines.join("\n");
    let hash = generate_hash(text.as_bytes());
    let tree_path = format!(".nymphaea/objects/{}", hash);
    let tree_path = Path::new(&tree_path);
    create_encoded(tree_path, text.as_bytes())?;
    println!("DEBUG in TREE    : path=>{:?}, hash=>{}", path, hash);  // DEBUG: To check generated hash.
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::{ObjectHashRecord, ObjectHashTable, Table};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_object_hash_record_from_line() {
        let c = ObjectHashRecord::from_line("1234567890abcdef1234 234567890abcdef12345 ./hoge/fuga/toryaaa.app");
        assert_eq!(c.object_hash, "1234567890abcdef1234".to_string());
        assert_eq!(c.commit_hash, "234567890abcdef12345".to_string());
        assert_eq!(c.path, PathBuf::from("./hoge/fuga/toryaaa.app"));
    }

    #[test]
    fn test_table_search_record() {
        let table: ObjectHashTable = vec!{
            ObjectHashRecord::from_line("blobhashblobhash0123 commhashcommhash1234 ./hoge/fuga/toryaaa.app"),
            ObjectHashRecord::from_line("blobhashblobhash1234 commhashcommhash2345 ./hoge/fuga2/taryiii.c"),
            ObjectHashRecord::from_line("blobhashblobhash2345 commhashcommhash3456 ./hoge/fuga3/tiryuuu.hs"),
            ObjectHashRecord::from_line("blobhashblobhash3456 commhashcommhash4567 ./hoge/fuga4/turyeee.rs"),
            ObjectHashRecord::from_line("blobhashblobhash4567 commhashcommhash5678 ./hoge/fuga5/teryooo.tar.xz")
        };
        let result = table.search_record("commhashcommhash2345", Path::new("./hoge/fuga2/taryiii.c"));
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.object_hash, String::from("blobhashblobhash1234"));
        assert_eq!(result.commit_hash, String::from("commhashcommhash2345"));
        assert_eq!(result.path.as_path(), Path::new("./hoge/fuga2/taryiii.c"));
    }
}
