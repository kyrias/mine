//! Contains the mine filesystem helper
//!
//! The [`Repository`] struct takes care of saving content to disk and retreiving it again, and is
//! responsible for mapping virtual password paths to secret on-disk filenames.
//!
//! [`Repository`]: struct.Repository.html

extern crate sequence_trie;
extern crate serde_json;

use std::path::{Path, PathBuf};
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

/// Thin internal wrapper over [`SequenceTrie`] to make managing it easier.
/// [`SequenceTrie`]: ../../sequence_trie/struct.SequenceTrie.html
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    repo_path: PathBuf,
    mapper: Mapper,
}

impl Repository {
    /// Returns a new `Repository` instance.
    ///
    /// TODO: This should take a path to create the repository in, and open the existing index if
    /// it already exists.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Repository> {
        let repo = Repository {
            repo_path: path.as_ref().join("repository"),
            mapper: Mapper::new()
        };
        repo.create_repo()?;
        Ok(repo)
    }

    /// Deserializes a JSON serialized Repository index.
    pub fn deserialize<P: AsRef<Path>>(serialized: &[u8], path: P) -> Result<Repository> {
        let mapper: Mapper = serde_json::from_slice(serialized)
            .chain_err(|| "failed to deserialize Repository from JSON")?;
        Ok(Repository {
            repo_path: path.as_ref().join("repository"),
            mapper: mapper,
        })
    }

    /// Serializes a Repository index to a JSON byte vector.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(&self.mapper)
            .chain_err(|| "failed to serialize Repository to JSON")
    }

    /// Create the repository directory if it doesn't already exist.
    fn create_repo(&self) -> Result<()> {
        let repo = &self.repo_path;
        if repo.exists() && !repo.is_dir() {
            return Err(format!("repository path '{}' already exists and isn't a directory", repo.display()).into())
        }
        if !repo.exists() {
            fs::create_dir_all(repo).chain_err(|| "failed to create repository path")?;
        }
        Ok(())
    }

    /// Inserts the given path into the index and then writes the contents of the byte slice to
    /// disk under a randomly generated filename.
    ///
    /// NOTE: This does not serialize the repository index to disk as well, so if we forget to do
    /// that we'll lose track of the new file in the repository directory.
    pub fn insert(&mut self, path: &str, content: &[u8]) -> Result<()> {
        fs::create_dir_all(&self.repo_path)
            .chain_err(|| "failed to create repository directory")?;
        let filename = match self.mapper.find(&path) {
            Some(p) => p,
            None    => self.mapper.insert(path),
        };
        let filepath = self.repo_path.join(filename);
        let mut file = File::create(filepath).chain_err(|| "failed to create file")?;
        file.write_all(content).chain_err(|| "failed to write content to disk")?;
        Ok(())
    }

    /// Delete an entry from the index and from disk.
    pub fn delete(&mut self, path: &str) -> Result<()> {
        let filename = self.mapper.find(&path).chain_err(|| "could not find Mapper entry")?;
        let filepath = self.repo_path.join(filename);
        self.mapper.remove(&path);
        fs::remove_file(filepath).chain_err(|| "failed to remove file")?;
        Ok(())
    }

    /// Look up the given path in the index and then return the contents of the on-disk entry.
    pub fn get(&self, path: &str) -> Result<Vec<u8>> {
        let filename = self.mapper.find(&path).chain_err(|| "could not find Mapper entry")?;
        let filepath = self.repo_path.join(filename);
        let mut file = File::open(filepath).chain_err(|| "failed to open file")?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).chain_err(|| "could not read file content")?;
        Ok(buf)
    }

    /// List the entries available under a specific path.
    pub fn list(&self, path: &str) -> Option<Vec<String>> {
        let files = self.mapper.list(&path);
        files
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
