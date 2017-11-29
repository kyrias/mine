//! Contains the mine filesystem helper
extern crate sequence_trie;


use std::fs::{self, File};
use std::io::{Read, Write};
use rand::{thread_rng, Rng};
use sequence_trie::SequenceTrie;

use super::errors::*;


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

/// The `Repository` struct contains the public interface for storing and retrieving files from
/// disk.
///
/// It manages an internal index that is used to map virtual paths to randomly generated ASCII
/// strings that are then used as the actual filenames.  We do this to provide more privacy than
/// more traditional password managers like [`pass(1)`] can provide by making the on-disk filenames
/// unrelatable to the encrypted password data stored therein.
///
/// Internal
/// --------
///
/// Internally the index is maintained as a [`SequenceTrie`] instance.
///
/// `SequenceTrie` nodes can contain both a value and children at the same time, which means that
/// in e.g. the `list()` output an entry could both contain a valid password, and children, and we
/// don't expose those two possibility in any way.  We might want to limit this in the `insert()`
/// function so that you can't have a node be both to prevent confusion to users.
///
/// TODO: Implement a `fsck` function that checks that all index entries exist on disk, and that
/// all on-disk entries exist in the index.
///
/// [`SequenceTrie`]: ../../sequence_trie/struct.SequenceTrie.html
/// [`pass(1)`]: https://www.passwordstore.org/
pub struct Repository {
    mapper: Mapper,
}

impl Repository {
    /// Returns a new `Repository` instance.
    ///
    /// TODO: This should take a path to create the repository in, and open the existing index if
    /// it already exists.
    pub fn new() -> Repository {
        Repository { mapper: Mapper::new() }
    }

    /// Inserts the given path into the index and then writes the contents of the byte slice to
    /// disk under a randomly generated filename.
    ///
    /// NOTE: This does not serialize the repository index to disk as well, so if we forget to do
    /// that we'll lose track of the new file in the repository directory.
    pub fn insert(&mut self, path: &str, content: &[u8]) -> Result<()> {
        let filename = self.mapper.insert(path);
        fs::create_dir_all("passrep").chain_err(|| "Failed to create repository directory")?;
        let mut file = File::create(format!("passrep/{}", filename)).chain_err(|| "Failed to create file")?;
        file.write_all(content).chain_err(|| "Failed to write content to disk")?;
        Ok(())
    }

    /// Look up the given path in the index and then return the contents of the on-disk entry.
    pub fn get(&self, path: &str) -> Result<Vec<u8>> {
        let filename = self.mapper.find(&path).chain_err(|| "Could not find Mapper entry")?;
        let mut file = File::open(format!("passrep/{}", filename)).chain_err(|| "Failed to open file")?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).chain_err(|| "Could not read file content")?;
        Ok(buf)
    }

    /// List the entries available under a specific path.
    pub fn list(&self, path: &str) -> Option<Vec<String>> {
        let files = self.mapper.list(&path);
        files
    }

    /// Delete an entry from the index and from disk.
    pub fn delete(&mut self, path: &str) -> Result<()> {
        let filename = self.mapper.find(&path).chain_err(|| "Could not find Mapper entry")?;
        self.mapper.remove(&path);
        fs::remove_file(format!("passrep/{}", filename)).chain_err(|| "Failed to remove file")?;
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
