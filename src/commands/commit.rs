// TODO: だいたい全部
macro_rules! read_file {
    ($e:expr) => {
        String::new()
    };
}

macro_rules! additionally_write {
    ($path:expr, $body:expr) => {
        println!("Writing `{}` to {}", $body, $path)
    };
}

macro_rules! commit_hash_raw {
    ($hash: expr, $mess: expr) => {
        format!("{} {}", $hash, $mess)
    };
}

pub fn commit(message: String) -> std::io::Result<()> {
    let hash = read_file!(".nymphaea/index");
    let current_branch = read_file!(".nymphaea/current_branch");
    let stack_path = format!(".nymphaea/commit_logs/{}", current_branch);
    let raw = commit_hash_raw!(hash, message);
    additionally_write!(stack_path, raw);
    Ok(())
}
