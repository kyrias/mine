#[macro_use] extern crate error_chain;
extern crate rand;
extern crate sequence_trie;


use std::fs::{self, File};
use std::io::{Read, Write};
use rand::{thread_rng, Rng};
use sequence_trie::SequenceTrie;


mod errors {
    error_chain! {}
}

pub use errors::*;


fn split_path(path: &str) -> Vec<String> {
    path.split("/").map(|v| v.to_string()).filter(|v| !v.is_empty()).collect()
}

fn generate_random_string() -> String {
    thread_rng().gen_ascii_chars().take(100).collect()
}

struct Mapper {
    st: SequenceTrie<String, String>,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper { st: SequenceTrie::new() }
    }

    fn insert(&mut self, path: &str) -> String {
        let random_path = generate_random_string();
        let parts = split_path(path);
        self.st.insert(&parts, random_path.clone());
        random_path
    }

    fn remove(&mut self, path: &str) {
        let parts = split_path(path);
        self.st.remove(&parts);
    }

    fn find(&self, path: &str) -> Option<String> {
        let parts = split_path(path);
        self.st.get(&parts).cloned()
    }

    fn list(&self, path: &str) -> Option<Vec<String>> {
        let parts = split_path(path);
        match self.st.get_node(&parts) {
            Some(node) => {
                let children_with_keys = node.children_with_keys();
                let keys: Vec<String> = children_with_keys.iter().map(|&(k, _)| k.to_string()).collect();
                Some(keys)
            },
            None => None,
        }
    }
}

pub struct Repository {
    mapper: Mapper,
}

impl Repository {
    pub fn new() -> Repository {
        Repository { mapper: Mapper::new() }
    }

    pub fn insert(&mut self, path: &str, content: &[u8]) -> Result<()> {
        let filename = self.mapper.insert(path);
        fs::create_dir_all("passrep")
            .chain_err(|| "Failed to create repository directory")?;
        let mut file = File::create(format!("passrep/{}", filename))
            .chain_err(|| "Failed to create file")?;
        file.write_all(content)
            .chain_err(|| "Failed to write content to disk")?;
        Ok(())
    }

    pub fn get(&self, path: &str) -> Result<Vec<u8>> {
        let filename = self.mapper.find(&path)
            .chain_err(|| "Could not find Mapper entry")?;
        let mut file = File::open(format!("passrep/{}", filename))
            .chain_err(|| "Failed to open file")?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .chain_err(|| "Could not read file content")?;
        Ok(buf)
    }

    pub fn delete(&mut self, path: &str) -> Result<()> {
        let filename = self.mapper.find(&path)
            .chain_err(|| "Could not find Mapper entry")?;
        self.mapper.remove(&path);
        fs::remove_file(format!("passrep/{}", filename))
            .chain_err(|| "Failed to remove file")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Mapper;

    #[test]
    fn exists_after_insert() {
        let mut m = Mapper::new();
        m.insert("foo");

        let entries = m.list("");
        assert_eq!(entries.is_some(), true);
        assert!(entries.unwrap().iter().any(|e| e == "foo"));
    }

    #[test]
    fn gone_after_remove(){
        let mut m = Mapper::new();
        m.insert("foo");
        m.remove("foo");

        let entries = m.list("");
        assert_eq!(entries.is_some(), true);
        assert!(entries.unwrap().is_empty());
    }
}
