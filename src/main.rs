#[macro_use] extern crate error_chain;

extern crate repository;


use repository::Repository;


mod errors {
    use super::repository;
    error_chain! {
        links {
            Repository(repository::Error, repository::ErrorKind);
        }
    }
}

use errors::*;


quick_main!(run);

fn run() -> Result<()> {
    let mut repo = Repository::new();
    repo.insert("foo/bar", &[1,2,3,4]).chain_err(|| "Could not insert 'foo/bar'")?;
    println!("{:?}", repo.get("foo/bar").chain_err(|| "Could not get 'foo/bar'")?);
    println!("{:?}", repo.list("foo").chain_err(|| "Could not list 'foo'")?);
    repo.delete("foo/bar").chain_err(|| "Could not delete 'foo/bar'")?;
    Ok(())
}
