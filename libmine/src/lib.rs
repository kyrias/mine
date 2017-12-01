#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate sequence_trie;
extern crate rand;


pub mod repository;
mod errors {
    error_chain! {}
}


use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub use errors::*;


pub fn load_repository<P: AsRef<Path>>(repo_path: P, index_path: P) -> Result<repository::Repository> {
    let repo = if index_path.as_ref().exists() {
        let mut file = File::open(index_path).chain_err(|| "failed to open index file")?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).chain_err(|| "failed to read index file")?;
        repository::Repository::deserialize(&buf, repo_path)?
    } else {
        repository::Repository::new(repo_path)?
    };
    Ok(repo)

}

pub fn save_index<P: AsRef<Path>>(repo: &repository::Repository, index_path: P) -> Result<()> {
    let serialized: Vec<u8> = repo.serialize()
        .chain_err(|| "failed to serialize Repository index")?;
    let mut file = File::create(index_path)
        .chain_err(|| "failed to create Repository index file")?;
    file.write_all(&serialized).chain_err(|| "failed to write Repository index to disk")?;
    Ok(())
}
