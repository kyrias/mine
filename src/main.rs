#[macro_use] extern crate error_chain;
extern crate sequence_trie;
extern crate rand;
extern crate mine;


use mine::Repository;


mod errors {
    use super::mine;
    error_chain! {
        links {
            Mine(mine::Error, mine::ErrorKind);
        }
    }
}

use errors::*;


quick_main!(run);

fn run() -> Result<()> {
    let mut repo = Repository::new();
    repo.insert("foo/bar", &[1,2,3,4]).chain_err(|| "Could not insert 'foo/bar'")?;
    println!("{:?}", repo.get("foo/bar").chain_err(|| "Could not get 'foo/bar'")?);
    repo.delete("foo/bar").chain_err(|| "Could not delete 'foo/bar'")?;
    Ok(())
}
