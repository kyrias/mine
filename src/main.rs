extern crate rand;
extern crate sequence_trie;

use std::fs::{self, File};
use std::io::prelude::*;

use rand::{thread_rng, Rng};
use sequence_trie::SequenceTrie;


struct Mapper {
    st: SequenceTrie<String, String>,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper { st: SequenceTrie::new() }
    }

    fn insert(&mut self, path: &str) -> String {
        let random: String = thread_rng().gen_ascii_chars().take(100).collect();
        let parts = split_path(path);
        self.st.insert(&parts, random.clone());
        random
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
            None     => None,
        }
    }
}

fn split_path(path: &str) -> Vec<String> {
    path.split("/").map(|v| v.to_string()).filter(|v| !v.is_empty()).collect()
}

struct Repository {
    mapper: Mapper,
}

impl Repository {
    fn new() -> Repository {
        Repository { mapper: Mapper::new() }
    }

    fn insert(&mut self, path: &str, content: &[u8]) {
        let filename = self.mapper.insert(path);
        fs::create_dir("passrep");
        let mut file = File::create(format!("passrep/{}", filename)).unwrap();
        file.write_all(content).unwrap();
    }

    fn get(&self, path: &str) -> Option<Vec<u8>> {
        let filename = self.mapper.find(&path).unwrap();
        let mut file = File::open(format!("passrep/{}", filename)).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf);
        Some(buf)
    }

    fn delete(&mut self, path: &str) {
        let filename = self.mapper.find(&path).unwrap();
        self.mapper.remove(&path);
        fs::remove_file(format!("passrep/{}", filename)).unwrap();
    }
}

fn main() {
    let mut repo = Repository::new();
    repo.insert("foo/bar", &[1,2,3,4]);
    println!("{:?}", repo.get("foo/bar"));
    repo.delete("foo/bar");
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
