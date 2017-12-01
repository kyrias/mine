#[macro_use] extern crate error_chain;
extern crate xdg;
extern crate libmine;


mod errors {
    use super::libmine;
    error_chain! {
        links {
            Libmine(libmine::Error, libmine::ErrorKind);
        }
    }
}


use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;


use libmine::repository::Repository;
use errors::*;


quick_main!(run);

fn run() -> Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("mine").chain_err(|| "could not create XDG instance")?;
    let repo_dir = xdg_dirs.create_data_directory("repository").chain_err(|| "could not create repository directory")?;
    let index_path = xdg_dirs.place_data_file("index").chain_err(|| "could not create index file path")?;
    let mut repo = load_repository(&repo_dir, &index_path)?;
    repo.insert("foo/bar", &[1,2,3,4]).chain_err(|| "could not insert 'foo/bar'")?;
    println!("{:?}", repo.get("foo/bar").chain_err(|| "could not get 'foo/bar'")?);
    println!("{:?}", repo.list("foo").chain_err(|| "could not list 'foo'")?);
    save_index(&repo, &index_path)?;
    Ok(())
}

fn load_repository<P: AsRef<Path>>(repo_path: P, index_path: P) -> Result<Repository> {
    let repo = if index_path.as_ref().exists() {
        let mut file = File::open(index_path).chain_err(|| "failed to open index file")?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).chain_err(|| "failed to read index file")?;
        Repository::deserialize(&buf, repo_path)?
    } else {
        Repository::new(repo_path)?
    };
    Ok(repo)

}

fn save_index<P: AsRef<Path>>(repo: &Repository, index_path: P) -> Result<()> {
    let serialized: Vec<u8> = repo.serialize()
        .chain_err(|| "failed to serialize Repository index")?;
    let mut file = File::create(index_path)
        .chain_err(|| "failed to create Repository index file")?;
    file.write_all(&serialized).chain_err(|| "failed to write Repository index to disk")?;
    Ok(())
}
