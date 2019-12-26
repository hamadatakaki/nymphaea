#[cfg(test)]
mod tests {
    extern crate nymphaea;
    use nymphaea::metadatas::util;

    #[test]
    fn hello() {
        // 91207d309a1946f2d32fa571819eccdbecc9fd69
        let s = "hogehogehogehogehogehhogehoge";
        let hash = util::generate_hash(&Vec::from(s));
        println!("{}", hash);
    }
}