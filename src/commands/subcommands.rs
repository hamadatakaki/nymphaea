use crate::metadatas::objects::get_object_data;
use crate::metadatas::objects::create_blob;

use std::path::Path;

pub fn cat_file(hash: &str) -> std::io::Result<()> {
    let data = get_object_data(hash)?;
    println!("cat_file: {}", hash);
    let text = String::from_utf8(data).unwrap();
    println!("{}", text);
    Ok(())
}

pub fn create_object(path: &Path) -> std::io::Result<()> {
    let hash = create_blob(path)?;
    println!("create_object: {}", hash);
    Ok(())
}
